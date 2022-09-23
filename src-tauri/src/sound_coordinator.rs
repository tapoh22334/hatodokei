use crate::preset_voice;
use crate::println;

use rodio::source::{SineWave, Source};
use rodio::{OutputStream, OutputStreamHandle, Sink};
use std::time::Duration;

pub struct ExSink {
    volume: u32,
    sink: Sink,
}

pub enum SoundSource {
    Popopopin(),
    Silence(f32),
    Voice(u32),
    // Not impremented.
    // this element is reserved for playing user prepared voice data.
    #[allow(unused)]
    Path(String),
}

pub struct PlayInfo {
    pub volume: u32,
    pub sources: Vec<SoundSource>,
}

#[derive(Default)]
pub struct SCMessage {
    pub master_volume: Option<u32>,
    pub master_mute: Option<bool>,
    pub play_info: Option<PlayInfo>,
}

pub struct SoundCoordinator {}

impl SoundCoordinator {
    pub fn activate() -> std::sync::mpsc::SyncSender<SCMessage> {
        use std::sync::mpsc;
        let (tx, rx) = mpsc::sync_channel::<SCMessage>(1);
        let mut master_volume: u32 = 100;

        std::thread::spawn(move || {
            let mut exsinks = Vec::<ExSink>::default();
            let (mut _output_stream, mut output_stream_handle) = OutputStream::try_default().unwrap();

            loop {
                let message = rx.recv().unwrap_or_default();

                // Cleanup playback list
                while let Some(index) = exsinks
                    .iter()
                    .position(|ExSink { volume: _, sink }| sink.empty())
                {
                    exsinks.remove(index);
                    println!("SoundCoordinator: Remove empty sink {:?}", index);
                }

                // Received play request
                if let Some(playinfo) = message.play_info {
                    if exsinks.is_empty() {
                        // output_stream_handle won't work when _output_stream is dropped.
                        println!("SoundCoordinator: Opened new output stream");
                        (_output_stream, output_stream_handle) = OutputStream::try_default().unwrap_or_else(
                            |e| {
                                println!("SoundCoordinator: Failed to open device {:?}", e);
                                (_output_stream, output_stream_handle)
                            });
                    }

                    let sink = Self::_play(&playinfo.sources, &output_stream_handle);
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
                if let Some(vol) = message.master_volume {
                    master_volume = vol;
                    for ExSink { volume, sink } in &exsinks {
                        sink.set_volume(Self::to_volume_magnification(master_volume, *volume));
                    }

                    println!("SoundCoordinator: update master volume {:?}", master_volume);
                }

                // Received mute request
                if let Some(master_mute) = message.master_mute {
                    let volume_mute = if master_mute { 0 } else { master_volume };

                    for ExSink { volume, sink } in &exsinks {
                        sink.set_volume(Self::to_volume_magnification(volume_mute, *volume));
                    }

                    println!("SoundCoordinator: update master volume {:?}", volume_mute);
                }
            }
        });

        tx
    }

    fn _play(sources: &Vec<SoundSource>, streamhandle: &OutputStreamHandle) -> Sink {
        let mut sink = Sink::try_new(streamhandle).unwrap();

        for source in sources {
            match source {
                SoundSource::Popopopin() => {
                    Self::play_popopopin(&mut sink);
                }
                SoundSource::Silence(sec) => {
                    Self::play_none(&mut sink, *sec);
                }
                SoundSource::Voice(index) => {
                    Self::play_preset_voice(&mut sink, *index);
                }
                SoundSource::Path(_path) => {} // Not implemented
            };
        }
        sink
    }

    pub fn play(tx: &std::sync::mpsc::SyncSender<SCMessage>, play_info: PlayInfo) {
        tx.send(SCMessage {
            master_volume: None,
            master_mute: None,
            play_info: Some(play_info),
        })
        .unwrap();
    }

    pub fn play_full_set_list(
        tx: &std::sync::mpsc::SyncSender<SCMessage>,
        voice_index: u32,
        volume: u32,
    ) {
        let sources: Vec<SoundSource> = vec![
            SoundSource::Popopopin(),
            SoundSource::Silence(0.75),
            SoundSource::Voice(voice_index),
        ];
        let play_info = PlayInfo { volume, sources };

        Self::play(tx, play_info);

        println!("SoundCoordinator: played full set list {:?}", voice_index);
    }

    pub fn set_master_volume(tx: &std::sync::mpsc::SyncSender<SCMessage>, mv: u32) {
        tx.send(SCMessage {
            master_volume: Some(mv),
            master_mute: None,
            play_info: None,
        })
        .unwrap();
    }

    pub fn set_master_mute(tx: &std::sync::mpsc::SyncSender<SCMessage>, mute: bool) {
        tx.send(SCMessage {
            master_volume: None,
            master_mute: Some(mute),
            play_info: None,
        })
        .unwrap();
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

    fn play_preset_voice(sink: &mut Sink, index: u32) {
        let source = rodio::Decoder::new(std::io::Cursor::new(
            preset_voice::PresetVoice::voice_data(index),
        ))
        .unwrap();

        sink.append(source);
    }
}
