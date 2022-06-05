use std::fs::File;
use std::io::BufReader;

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

//static VOICIES: (&[u8; 71956],
//    &[u8; 64160], &[u8; 56992], &[u8; 66208], &[u8; 56992], &[u8; 56992],
//    &[u8; 64160], &[u8; 70304], &[u8; 68256], &[u8; 59040], &[u8; 62112],
//    &[u8; 82592], &[u8; 74400], &[u8; 79520], &[u8; 70304], &[u8; 73376],
//    &[u8; 80544], &[u8; 82592], &[u8; 84640], &[u8; 74400], &[u8; 73376],
//    &[u8; 88736], &[u8; 78496], &[u8; 90784]) = (
//    include_bytes!("data/0000.wav"), include_bytes!("data/0100.wav"), include_bytes!("data/0200.wav"),
//    include_bytes!("data/0300.wav"), include_bytes!("data/0400.wav"), include_bytes!("data/0500.wav"),
//    include_bytes!("data/0600.wav"), include_bytes!("data/0700.wav"), include_bytes!("data/0800.wav"),
//    include_bytes!("data/0900.wav"), include_bytes!("data/1000.wav"), include_bytes!("data/1100.wav"),
//    include_bytes!("data/1200.wav"), include_bytes!("data/1300.wav"), include_bytes!("data/1400.wav"),
//    include_bytes!("data/1500.wav"), include_bytes!("data/1600.wav"), include_bytes!("data/1700.wav"),
//    include_bytes!("data/1800.wav"), include_bytes!("data/1900.wav"), include_bytes!("data/2000.wav"),
//    include_bytes!("data/2100.wav"), include_bytes!("data/2200.wav"), include_bytes!("data/2300.wav"),
//    );


/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    settings: Settings,

    #[serde(skip)]
    voice_data: Vec<Vec<u8>>,
    #[serde(skip)]
    stream: Option<OutputStream>,
    #[serde(skip)]
    stream_handle: Option<OutputStreamHandle>,
    #[serde(skip)]
    sink: Option<Sink>,

}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            settings: Settings::default(),
            voice_data: Vec::default(),
            stream: None,
            stream_handle: None,
            sink: None,
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        let voice_data: Vec<Vec<u8>> = vec![
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

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        let (s, sh) = OutputStream::try_default().unwrap();
        let snk = Sink::try_new(&sh).unwrap();

        if let Some(storage) = cc.storage {
            let mut stored: Self = eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();

            stored.voice_data    = voice_data;
            stored.stream        = Some(s);
            stored.stream_handle = Some(sh);
            stored.sink          = Some(snk);

            return stored;
        }

        TemplateApp {
            voice_data:    voice_data,
            stream:        Some(s),
            stream_handle: Some(sh),
            sink:          Some(snk),

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
        let Settings { master_volume, master_mute, time_table} = &mut self.settings;

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {

                    if ui.button("Play sound 3").clicked() {
                        let popopopin = popopopin_generator();
                        for s in popopopin {
                           self.sink.as_ref().unwrap().append(s)
                        }
                    }

                    else if ui.button("Play sound 5").clicked() {
                       // Load a sound from a file, using a path relative to Cargo.toml
                       let file = File::open("sound.mp3").unwrap();
                       let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
                       self.sink.as_ref().unwrap().append(source);
                    }

                    else if ui.button("Quit").clicked() {
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
                        let current_volume = self.sink.as_ref().unwrap().volume();
                        let volume_magnification = (*master_volume as f32) / 100.;

                        if *master_mute && current_volume != 0.0 {
                            self.sink.as_ref().unwrap().set_volume(0.0);
                        }

                        if ! *master_mute
                            && (volume_magnification - current_volume).abs() >= 0.01 {
                            self.sink.as_ref().unwrap().set_volume(volume_magnification);
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

            .body(|mut body| {
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
                        let icon = emojis::get_by_shortcode("arrow_forward").unwrap().as_str();
                        let b = ui.small_button(icon).clicked();
                        if b {
                            let popopopin = popopopin_generator();
                            for s in popopopin {
                                self.sink.as_ref().unwrap().append(s)
                            }

                            self.sink.as_ref().unwrap().append(
                                silence_generator(0.75)
                                );

                            //let name = format!("{0:>02}{1:>02}.wav",
                            //                   time_table[row_index].h,
                            //                   time_table[row_index].m,);
                            //let file = File::open("data/".to_owned() + &name).unwrap();
                            //let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
                            let index = time_table[row_index].h as usize;
                            //let cursor = std::io::Cursor::new(&self.voice_data[index]);
                            let source = rodio::Decoder::new(
                                std::io::Cursor::new( self.voice_data[index].clone() )).unwrap();
                            self.sink.as_ref().unwrap().append(source);
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

use std::time::Duration;
use rodio;
use rodio::{OutputStream, OutputStreamHandle, Sink};
use rodio::source::{SineWave, Source, Amplify, TakeDuration};

fn popopopin_generator() -> std::vec::Vec<Amplify<TakeDuration<SineWave>>> {
    let v = vec![
        SineWave::new(440.0).take_duration(Duration::from_secs_f32(0.25)).amplify(0.20),
        SineWave::new(0.).take_duration(Duration::from_secs_f32(0.75)).amplify(0.20),
        SineWave::new(440.0).take_duration(Duration::from_secs_f32(0.25)).amplify(0.20),
        SineWave::new(0.).take_duration(Duration::from_secs_f32(0.75)).amplify(0.20),
        SineWave::new(440.0).take_duration(Duration::from_secs_f32(0.25)).amplify(0.20),
        SineWave::new(0.).take_duration(Duration::from_secs_f32(0.75)).amplify(0.20),
        SineWave::new(880.0).take_duration(Duration::from_secs_f32(2.)).amplify(0.20),
    ];
    v
}

fn silence_generator(sec: f32) -> Amplify<TakeDuration<SineWave>> {
    SineWave::new(0.).take_duration(Duration::from_secs_f32(sec)).amplify(0.0)
}
