#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TimeElement {
    h: u32,
    m: u32,
    mute: bool,
}

impl Default for TimeElement {
    fn default() -> Self {
        Self { h: 0, m: 0, mute: false, }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct Settings {
    master_volume: u32,
    master_mute: bool,
    time_table: Vec<TimeElement>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            master_volume: 100,
            master_mute: false,
            time_table: vec![
                TimeElement {h: 0,  m: 0, mute: false},
                TimeElement {h: 1,  m: 0, mute: false},
                TimeElement {h: 2,  m: 0, mute: false},
                TimeElement {h: 3,  m: 0, mute: false},
                TimeElement {h: 4,  m: 0, mute: false},
                TimeElement {h: 5,  m: 0, mute: false},
                TimeElement {h: 6,  m: 0, mute: false},
                TimeElement {h: 7,  m: 0, mute: false},
                TimeElement {h: 8,  m: 0, mute: false},
                TimeElement {h: 9,  m: 0, mute: false},
                TimeElement {h: 10, m: 0, mute: false},
                TimeElement {h: 11, m: 0, mute: false},
                TimeElement {h: 12, m: 0, mute: false},
                TimeElement {h: 13, m: 0, mute: false},
                TimeElement {h: 14, m: 0, mute: false},
                TimeElement {h: 15, m: 0, mute: false},
                TimeElement {h: 16, m: 0, mute: false},
                TimeElement {h: 17, m: 0, mute: false},
                TimeElement {h: 18, m: 0, mute: false},
                TimeElement {h: 19, m: 0, mute: false},
                TimeElement {h: 20, m: 0, mute: false},
                TimeElement {h: 21, m: 0, mute: false},
                TimeElement {h: 22, m: 0, mute: false},
                TimeElement {h: 23, m: 0, mute: false},
            ],
        }
    }
}

//pub struct Dispatcher {
//}
//
//impl Default for Dispatcher {
//    fn default() -> Self {
//        Self { ..Default::default() }
//    }
//        //std::thread::spawn( move || {
//        //    let mut last_frame: DateTime<Local> = Local::now();
//
//        //    loop {
//        //        let now: DateTime<Local> = Local::now();
//        //        let sleep_milliseconds = (1000. / MAX_FPS) as u32;
//
//        //        std::thread::sleep(std::time::Duration::new(0, sleep_milliseconds * 1000000));
//        //        let sleep_milliseconds = (last_frame - now).num_milliseconds()
//        //            - ((1000.0 / MAX_FPS) as i64);
//
//        //        if  sleep_milliseconds > 0 {
//        //            std::thread::sleep(std::time::Duration::new(0, sleep_milliseconds as u32 * 1000000));
//        //            last_frame = now;
//        //        }
//        //    }
//        //});
//
//}

pub struct PresetVoice {}

impl PresetVoice {
    pub fn voice_data(index: u32) -> Vec<u8> {
            let voice_data = vec![
                include_bytes!("data/0000.wav").to_vec(), include_bytes!("data/0100.wav").to_vec(),
                include_bytes!("data/0200.wav").to_vec(), include_bytes!("data/0300.wav").to_vec(),
                include_bytes!("data/0400.wav").to_vec(), include_bytes!("data/0500.wav").to_vec(),
                include_bytes!("data/0600.wav").to_vec(), include_bytes!("data/0700.wav").to_vec(),
                include_bytes!("data/0800.wav").to_vec(), include_bytes!("data/0900.wav").to_vec(),
                include_bytes!("data/1000.wav").to_vec(), include_bytes!("data/1100.wav").to_vec(),
                include_bytes!("data/1200.wav").to_vec(), include_bytes!("data/1300.wav").to_vec(),
                include_bytes!("data/1400.wav").to_vec(), include_bytes!("data/1500.wav").to_vec(),
                include_bytes!("data/1600.wav").to_vec(), include_bytes!("data/1700.wav").to_vec(),
                include_bytes!("data/1800.wav").to_vec(), include_bytes!("data/1900.wav").to_vec(),
                include_bytes!("data/2000.wav").to_vec(), include_bytes!("data/2100.wav").to_vec(),
                include_bytes!("data/2200.wav").to_vec(), include_bytes!("data/2300.wav").to_vec(),
            ];
            voice_data[index as usize].clone()
    }
}

pub enum SoundSource {
    Popopopin(),
    Silence(f32),
    Voice(u32),
    Path(String),
}

pub struct PlayInfo {
    volume: u32,
    sources: Vec<SoundSource>,
}

pub struct ExSink {
    volume: u32,
    sink: Sink,
}

pub struct SoundCoordinator {
    master_volume: u32,
    stream: OutputStream,
    stream_handle: OutputStreamHandle,
    exsinks: Vec<ExSink>,
}

impl Default for SoundCoordinator {
    fn default() -> Self {
        let (s, sh) = OutputStream::try_default().unwrap();

        Self {
            master_volume: 100,
            stream: s,
            stream_handle: sh,
            exsinks: Vec::<ExSink>::default(),
        }

    }
}

use std::time::Duration;
use rodio;
use rodio::{OutputStream, OutputStreamHandle, Sink};
use rodio::source::{SineWave, Source};

impl SoundCoordinator {
    pub fn play(&mut self, playinfo: PlayInfo) {
        let PlayInfo {volume, sources} = playinfo;
        let mut sink = Sink::try_new(&self.stream_handle).unwrap();

        let mut exsink: ExSink = ExSink {volume: volume, sink: sink};

        self.update_exsinks();
        for source in sources {
            match source {
                SoundSource::Popopopin()        => {Self::play_popopopin(&mut exsink.sink);}
                SoundSource::Silence(sec)       => {Self::play_none(&mut exsink.sink, sec);}
                SoundSource::Voice(index) => { Self::play_preset_voice(&mut exsink.sink, index); }
                SoundSource::Path(_path)         => {} // Not implemented
            };

        }

        exsink.sink.set_volume(
            Self::to_volume_magnification(self.master_volume, volume));
        self.exsinks.push(exsink);
    }

    pub fn set_master_volume(&mut self, mv: u32) {
        self.master_volume = mv;
        for ExSink {volume, sink} in &self.exsinks {
            sink.set_volume(Self::to_volume_magnification(mv, *volume));
        }
    }

    fn update_exsinks(&mut self) {
        while let Some(index) = self.exsinks.iter().position( |ExSink{volume: _, sink}|  sink.empty() ) {
            self.exsinks.remove(index);
        }
    }

    fn to_volume_magnification(master_volume: u32, volume: u32) -> f32 {
        (master_volume as f32) / 100. *
        (volume as f32) / 100.
    }

    fn play_popopopin(sink: &mut Sink) {
        let popopopin = vec![
            SineWave::new(440.0).take_duration(Duration::from_secs_f32(0.25)).amplify(0.20),
            SineWave::new(0.).take_duration(Duration::from_secs_f32(0.75)).amplify(0.20),
            SineWave::new(440.0).take_duration(Duration::from_secs_f32(0.25)).amplify(0.20),
            SineWave::new(0.).take_duration(Duration::from_secs_f32(0.75)).amplify(0.20),
            SineWave::new(440.0).take_duration(Duration::from_secs_f32(0.25)).amplify(0.20),
            SineWave::new(0.).take_duration(Duration::from_secs_f32(0.75)).amplify(0.20),
            SineWave::new(880.0).take_duration(Duration::from_secs_f32(2.)).amplify(0.20),
        ];
        for s in popopopin {
            sink.append(s);
        }
    }

    fn play_none(sink: &mut Sink, sec: f32) {
        let sinwave = SineWave::new(0.).take_duration(Duration::from_secs_f32(sec)).amplify(0.0);
        sink.append(sinwave);
    }

    fn play_preset_voice(sink: &mut Sink, index: u32) {
        let source = rodio::Decoder::new(
            std::io::Cursor::new(PresetVoice::voice_data(index))).unwrap();

        sink.append(source);
    }
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    settings: Settings,
    #[serde(skip)]
    sound_coordinator: SoundCoordinator,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            settings: Settings::default(),
            sound_coordinator: SoundCoordinator::default(),
        }
    }
}

impl TemplateApp {
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            let mut stored: Self = eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
            return stored;
        }

        TemplateApp {
            ..Default::default()
        }
    }

}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let Settings { master_volume, master_mute, time_table } = &mut self.settings;

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {

                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }

                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.group(|ui| {

                    // UI view
                    ui.add(egui::Slider::new(master_volume, 0..=100));
                    if *master_mute {
                        ui.checkbox(master_mute, "Muting" );
                    } else {
                        ui.checkbox(master_mute, "");
                    }

                    // Write device setting
                    {
                        if *master_mute {
                            self.sound_coordinator.set_master_volume(0);
                        }
                        else {
                            self.sound_coordinator.set_master_volume(*master_volume);
                        }
                    }

                });
            });

        use egui_extras::{TableBuilder, Size};
        TableBuilder::new(ui)
            .striped(true)
            .cell_layout(egui::Layout::left_to_right().with_cross_align(egui::Align::Center))
            .column(Size::initial(10.0).at_least(10.0))
            .column(Size::initial(35.0).at_least(35.0))
            .column(Size::initial(15.0).at_least(15.0))
            .column(Size::initial(15.0).at_least(15.0))

            .header(0.0, |mut header| {
                header.col(|ui| {
                    ui.heading("");
                });
                header.col(|ui| {
                    ui.heading("");
                });
            })

            .body(|body| {
                body.rows(20., time_table.len(), |row_index, mut row| {
                    row.col(|ui| {
                        ui.label(
                            clock_emoji(time_table[row_index].h as usize)
                            );
                    });
                    row.col(|ui| {
                        let time = format!("{0:>02}:{1:>02}",
                                           time_table[row_index].h,
                                           time_table[row_index].m,
                                           );
                        ui.label(time);
                    });
                    row.col(|ui| {
                        ui.checkbox(&mut time_table[row_index].mute, "");
                    });
                    row.col(|ui| {
                        let voice_num = time_table[row_index].h;

                        let icon = emojis::get_by_shortcode("arrow_forward").unwrap().as_str();
                        let b = ui.small_button(icon).clicked();
                        if b {
                            let sources: Vec<SoundSource> = vec![
                                        SoundSource::Popopopin(),
                                        SoundSource::Silence(0.75),
                                        SoundSource::Voice(voice_num),
                                    ];
                            self.sound_coordinator.play(PlayInfo {
                                    volume: 100,
                                    sources: sources,
                            });
                        }
                    });
                });
            });

        });

    }
}

fn clock_emoji(row_index: usize) -> String {
    char::from_u32(0x1f550 + row_index as u32 % 24)
        .unwrap()
        .to_string()
}
