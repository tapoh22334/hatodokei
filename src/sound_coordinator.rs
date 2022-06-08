use crate::message::{PlayInfo, SCMessage, SoundSource};
use crate::preset_voice;

use rodio::source::{SineWave, Source};
use rodio::{OutputStream, OutputStreamHandle, Sink};
use std::time::Duration;

pub struct ExSink {
    volume: u32,
    sink: Sink,
}

pub struct SoundCoordinator {}

impl SoundCoordinator {
    pub fn activate() -> std::sync::mpsc::Sender<SCMessage> {
        use std::sync::mpsc;
        let (tx, rx) = mpsc::channel::<SCMessage>();
        let mut master_volume: u32 = 100;

        std::thread::spawn(move || {
            let mut exsinks = Vec::<ExSink>::default();
            #[allow(unused)]
            let (s, sh) = OutputStream::try_default().unwrap();

            loop {
                let message = rx.recv().unwrap_or_default();
                if let Some(playinfo) = message.play_info {
                    let sink = Self::_play(&playinfo.sources, &sh);
                    sink.set_volume(Self::to_volume_magnification(
                        master_volume,
                        playinfo.volume,
                    ));

                    let exsink: ExSink = ExSink {
                        volume: playinfo.volume,
                        sink,
                    };
                    exsinks.push(exsink);
                } else {
                    for ExSink { volume, sink } in &exsinks {
                        master_volume = message.master_volume.unwrap();
                        sink.set_volume(Self::to_volume_magnification(master_volume, *volume));
                    }
                }

                while let Some(index) = exsinks
                    .iter()
                    .position(|ExSink { volume: _, sink }| sink.empty())
                {
                    exsinks.remove(index);
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

    pub fn play(tx: &std::sync::mpsc::Sender<SCMessage>, play_info: PlayInfo) {
        tx.send(SCMessage {
            master_volume: None,
            play_info: Some(play_info),
        })
        .unwrap();
    }

    pub fn play_full_set_list(
        tx: &std::sync::mpsc::Sender<SCMessage>,
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
    }

    pub fn set_master_volume(tx: &std::sync::mpsc::Sender<SCMessage>, mv: u32) {
        tx.send(SCMessage {
            master_volume: Some(mv),
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
