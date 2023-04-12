use crate::preset_voice;
use crate::println;

use rodio::source::{SineWave, Source};
use rodio::{OutputStream, OutputStreamHandle, Sink};
use std::time::Duration;

pub struct ExSink {
    volume: u32,
    sink: Sink,
}

#[derive(Clone)]
pub enum SoundSource {
    Popopopin(),
    Silence(f32),
    VoiceIndex(String, usize),
    // Not impremented.
    // this element is reserved for playing user prepared voice data.
    #[allow(unused)]
    Path(String),
}

pub struct PlayInfo {
    pub volume: u32,
    pub sources: Vec<SoundSource>,
}

pub enum SCMessage {
    MasterVolume(u32),
    PlayInfo(PlayInfo),
}

pub struct SoundCoordinator {
}

impl SoundCoordinator {
    pub fn activate() -> std::sync::mpsc::SyncSender<SCMessage> {
        use std::sync::mpsc;
        let (tx, rx) = mpsc::sync_channel::<SCMessage>(3);
        let preset_voice = preset_voice::PresetVoice::new();

        // Default Configuration
        let mut master_volume: u32 = 100;

        std::thread::spawn(move || {
            let mut exsinks = Vec::<ExSink>::default();
            let (mut _output_stream, mut output_stream_handle) = OutputStream::try_default().unwrap();

            loop {
                let message = rx.recv().expect("sound_coordinator: disconnected");

                // Cleanup playback list
                while let Some(index) = exsinks
                    .iter()
                    .position(|ExSink { volume: _, sink }| sink.empty())
                {
                    exsinks.remove(index);
                    println!("SoundCoordinator: Remove empty sink {:?}", index);
                }

                // Received play request
                match message {
                    SCMessage::PlayInfo(playinfo) => {
                        if exsinks.is_empty() {
                            // output_stream_handle won't work when _output_stream is dropped.
                            println!("SoundCoordinator: Opened new output stream");
                            (_output_stream, output_stream_handle) = OutputStream::try_default().unwrap_or_else(
                                |e| {
                                    println!("SoundCoordinator: Failed to open device {:?}", e);
                                    (_output_stream, output_stream_handle)
                                });
                        }

                        let sink = Self::_play(&preset_voice, &playinfo.sources, &output_stream_handle);
                        sink.set_volume(Self::to_volume_magnification(
                                master_volume,
                                playinfo.volume,
                                ));

                        exsinks.push(ExSink {
                            volume: playinfo.volume,
                            sink,
                        });

                        println!("SoundCoordinator: playinfo accepted");
                    }

                    // Received volume change request
                    SCMessage::MasterVolume(vol) => {
                        master_volume = vol;
                        for ExSink { volume, sink } in &exsinks {
                            sink.set_volume(Self::to_volume_magnification(master_volume, *volume));
                        }

                        println!("SoundCoordinator: update master volume {:?}", master_volume);
                    }

                }
            }
        });

        tx
    }

    pub fn play_index(
        tx: &std::sync::mpsc::SyncSender<SCMessage>,
        voice: String,
        index: usize,
        effect: bool,
        volume: u32,
    ) {
        println!("SoundCoordinator: played full set list {:?} {:?} {:?} {:?}", voice, index, effect, volume);

        let sources = if effect {
            vec![
                SoundSource::Popopopin(),
                SoundSource::Silence(0.75),
                SoundSource::VoiceIndex(voice, index),
            ]
        } else {
            vec![
                SoundSource::VoiceIndex(voice, index),
            ]
        };

        let play_info = PlayInfo { volume, sources };
        tx.send(SCMessage::PlayInfo(play_info)).unwrap();
    }

    pub fn set_master_volume(tx: &std::sync::mpsc::SyncSender<SCMessage>, mv: u32) {
        tx.send(SCMessage::MasterVolume(mv)).unwrap();
    }

    fn _play(preset_voice: &preset_voice::PresetVoice, sources: &Vec<SoundSource>, streamhandle: &OutputStreamHandle) -> Sink {
        let mut sink = Sink::try_new(streamhandle).unwrap();

        for source in sources {
            match source {
                SoundSource::Popopopin() => {
                    Self::play_popopopin(&mut sink);
                }
                SoundSource::Silence(sec) => {
                    Self::play_none(&mut sink, *sec);
                }
                SoundSource::VoiceIndex(voice, index) => {
                    Self::play_preset_voice(preset_voice, &mut sink, voice, *index);
                }
                SoundSource::Path(_path) => {} // Not implemented
            };
        }
        sink
    }

    fn to_volume_magnification(master_volume: u32, volume: u32) -> f32 {
        (master_volume as f32) / 100. * (volume as f32) / 100.
    }

    fn play_popopopin(sink: &mut Sink) {
        let popopopin = vec![
            SineWave::new(440.0)
                .take_duration(Duration::from_secs_f32(0.25))
                .amplify(0.20),
            SineWave::new(0.)
                .take_duration(Duration::from_secs_f32(0.75))
                .amplify(0.20),
            SineWave::new(440.0)
                .take_duration(Duration::from_secs_f32(0.25))
                .amplify(0.20),
            SineWave::new(0.)
                .take_duration(Duration::from_secs_f32(0.75))
                .amplify(0.20),
            SineWave::new(440.0)
                .take_duration(Duration::from_secs_f32(0.25))
                .amplify(0.20),
            SineWave::new(0.)
                .take_duration(Duration::from_secs_f32(0.75))
                .amplify(0.20),
            SineWave::new(880.0)
                .take_duration(Duration::from_secs_f32(2.))
                .amplify(0.20),
        ];
        for s in popopopin {
            sink.append(s);
        }
    }

    fn play_none(sink: &mut Sink, sec: f32) {
        let sinwave = SineWave::new(0.)
            .take_duration(Duration::from_secs_f32(sec))
            .amplify(0.0);
        sink.append(sinwave);
    }

    fn play_preset_voice(preset_voice: &preset_voice::PresetVoice, sink: &mut Sink, voice: &String, index: usize) {
        let source = rodio::Decoder::new(std::io::Cursor::new(
            preset_voice.get_data(preset_voice::Voice::from(voice.as_str()), index).clone(),
        ))
        .unwrap();

        sink.append(source);
    }
}
