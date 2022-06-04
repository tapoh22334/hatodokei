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


/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    settings: Settings,

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

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        let (s, sh) = OutputStream::try_default().unwrap();
        let snk = Sink::try_new(&sh).unwrap();

        if let Some(storage) = cc.storage {
            let mut stored: Self = eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();

            stored.stream        = Some(s);
            stored.stream_handle = Some(sh);
            stored.sink          = Some(snk);

            return stored;
        }

        TemplateApp {
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
            .column(Size::initial(40.0).at_least(40.0))
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
                        ui.add(
                            egui::Label::new(time),
                            );
                    });
                    row.col(|ui| {
                        ui.checkbox(&mut time_table[row_index].mute, "");
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
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
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

