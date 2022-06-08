use chrono::Timelike;

#[derive(serde::Deserialize, serde::Serialize, Copy, Clone, Debug)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TTElement {
    h: u32,
    m: u32,
    active: bool,
}

impl Default for TTElement {
    fn default() -> Self {
        Self {
            h: 0,
            m: 0,
            active: true,
        }
    }
}

impl TTElement {
    pub fn time(&self) -> u32 {
        (Self::join(self.h, self.m)).into()
    }

    pub fn join(h: u32, m: u32) -> u32 {
        h * 100 + m
    }

    pub fn sub(h1: u32, m1: u32, h2: u32, m2: u32) -> u32 {
        let v1 = Self::join(h1, m1) as i32;
        let v2 = Self::join(h2, m2) as i32;
        ((v1 - v2 + 2359) % 2359) as u32
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct Settings {
    master_volume: u32,
    master_mute: bool,
    time_table: Vec<TTElement>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            master_volume: 100,
            master_mute: false,
            time_table: vec![
                TTElement {
                    h: 0,
                    m: 0,
                    active: true,
                },
                TTElement {
                    h: 1,
                    m: 0,
                    active: false,
                },
                TTElement {
                    h: 2,
                    m: 0,
                    active: false,
                },
                TTElement {
                    h: 3,
                    m: 0,
                    active: false,
                },
                TTElement {
                    h: 4,
                    m: 0,
                    active: false,
                },
                TTElement {
                    h: 5,
                    m: 0,
                    active: false,
                },
                TTElement {
                    h: 6,
                    m: 0,
                    active: false,
                },
                TTElement {
                    h: 7,
                    m: 0,
                    active: false,
                },
                TTElement {
                    h: 8,
                    m: 0,
                    active: true,
                },
                TTElement {
                    h: 9,
                    m: 0,
                    active: true,
                },
                TTElement {
                    h: 10,
                    m: 0,
                    active: true,
                },
                TTElement {
                    h: 11,
                    m: 0,
                    active: true,
                },
                TTElement {
                    h: 12,
                    m: 0,
                    active: true,
                },
                TTElement {
                    h: 13,
                    m: 0,
                    active: true,
                },
                TTElement {
                    h: 14,
                    m: 0,
                    active: true,
                },
                TTElement {
                    h: 15,
                    m: 0,
                    active: true,
                },
                TTElement {
                    h: 16,
                    m: 0,
                    active: true,
                },
                TTElement {
                    h: 17,
                    m: 0,
                    active: true,
                },
                TTElement {
                    h: 18,
                    m: 0,
                    active: true,
                },
                TTElement {
                    h: 19,
                    m: 0,
                    active: true,
                },
                TTElement {
                    h: 20,
                    m: 0,
                    active: true,
                },
                TTElement {
                    h: 21,
                    m: 0,
                    active: true,
                },
                TTElement {
                    h: 22,
                    m: 0,
                    active: true,
                },
                TTElement {
                    h: 23,
                    m: 0,
                    active: true,
                },
            ],
        }
    }
}

pub enum SMessage {
    Overwrite(TTElement),
    // Not impremented
    // this element is reserved for adding/deleting schedule.
    #[allow(unused)]
    Delete(TTElement),
}

pub struct Scheduler {}

use chrono::DateTime;
use chrono::Local;

const MAX_FPS: f32 = 1.0;
impl Scheduler {
    fn activate(tx_sc: std::sync::mpsc::Sender<SCMessage>) -> std::sync::mpsc::Sender<SMessage> {
        use std::sync::mpsc;
        let (tx, rx) = mpsc::channel::<SMessage>();
        let mut time_table: Vec<TTElement> = Vec::new();
        let mut next_play: Option<TTElement> = None;

        std::thread::spawn(move || {
            let mut last_frame: DateTime<Local> = Local::now();

            loop {
                let now: DateTime<Local> = Local::now();

                // sleep fixed time and remove jitter.
                //
                let sleep_milliseconds = (1000. / MAX_FPS) as u32;
                //std::thread::sleep(std::time::Duration::new(0, sleep_milliseconds * 1000000));
                //let sleep_milliseconds = ((1000.0 / MAX_FPS) as i64) - (last_frame - now).num_milliseconds();

                print!("{}:", now);
                print!("{}:", last_frame);
                print!("[{}]", sleep_milliseconds);
                print!("[next:{:?}]", next_play);

                if sleep_milliseconds > 0 {
                    let rv = rx
                        .recv_timeout(std::time::Duration::from_millis(sleep_milliseconds as u64));
                    if let Ok(msg) = rv {
                        Self::process_message(&mut time_table, msg);
                        next_play = None;

                        println!("message received");
                    } else if let Err(e) = rv {
                        println!("sleep but no message {:?}", e);
                    }
                } else {
                    println!("Note: Slow down");
                    let rv = rx.try_recv();
                    if let Ok(msg) = rv {
                        Self::process_message(&mut time_table, msg);
                        next_play = None;
                    }
                };

                if next_play.is_none() {
                    let target = now + chrono::Duration::minutes(1);
                    next_play = Some(
                        *time_table
                            .iter()
                            .min_by_key(|x| {
                                TTElement::sub(x.h, x.m, target.hour(), target.minute())
                            })
                            .unwrap(),
                    );
                } else {
                    let index = next_play.unwrap().h;
                    let h = chrono::Duration::hours(next_play.unwrap().h.into());
                    let m = chrono::Duration::minutes(next_play.unwrap().m.into());
                    let now_h = chrono::Duration::hours(now.hour().into());
                    let now_m = chrono::Duration::minutes(now.minute().into());

                    let sub = (h + m) - (now_h + now_m);

                    if sub.num_minutes() == 0 {
                        SoundCoordinator::play_full_set_list(&tx_sc, index, 100);
                        next_play = None;
                    }
                }

                last_frame = now;
            }
        });

        tx
    }

    fn process_message(timetable: &mut Vec<TTElement>, message: SMessage) {
        match message {
            SMessage::Overwrite(src) => {
                println!("{:?}", src);
                if let Some(row) = timetable.iter_mut().find(|e| e.h == src.h && e.m == src.m) {
                    row.active = src.active;
                } else {
                    timetable.push(src);
                }
            }
            SMessage::Delete(target) => {
                println!("{:?}", target);
                timetable.retain(|e| e.h == target.h && e.m == target.m);
            }
        };

        timetable.sort_by(|a, b| (a.time()).partial_cmp(&b.time()).unwrap());

        println!("{:?}", timetable);
    }

    fn edit(tx_s: &std::sync::mpsc::Sender<SMessage>, row: &TTElement) {
        tx_s.send(SMessage::Overwrite(row.clone())).unwrap();
    }
}

pub struct PresetVoice {}

impl PresetVoice {
    // TODO: Improve this function to not use clone
    pub fn voice_data(index: u32) -> Vec<u8> {
        let voice_data = vec![
            include_bytes!("data/0000.wav").to_vec(),
            include_bytes!("data/0100.wav").to_vec(),
            include_bytes!("data/0200.wav").to_vec(),
            include_bytes!("data/0300.wav").to_vec(),
            include_bytes!("data/0400.wav").to_vec(),
            include_bytes!("data/0500.wav").to_vec(),
            include_bytes!("data/0600.wav").to_vec(),
            include_bytes!("data/0700.wav").to_vec(),
            include_bytes!("data/0800.wav").to_vec(),
            include_bytes!("data/0900.wav").to_vec(),
            include_bytes!("data/1000.wav").to_vec(),
            include_bytes!("data/1100.wav").to_vec(),
            include_bytes!("data/1200.wav").to_vec(),
            include_bytes!("data/1300.wav").to_vec(),
            include_bytes!("data/1400.wav").to_vec(),
            include_bytes!("data/1500.wav").to_vec(),
            include_bytes!("data/1600.wav").to_vec(),
            include_bytes!("data/1700.wav").to_vec(),
            include_bytes!("data/1800.wav").to_vec(),
            include_bytes!("data/1900.wav").to_vec(),
            include_bytes!("data/2000.wav").to_vec(),
            include_bytes!("data/2100.wav").to_vec(),
            include_bytes!("data/2200.wav").to_vec(),
            include_bytes!("data/2300.wav").to_vec(),
        ];
        voice_data[index as usize].clone()
    }
}

use rodio;
use rodio::source::{SineWave, Source};
use rodio::{OutputStream, OutputStreamHandle, Sink};
use std::time::Duration;

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
    volume: u32,
    sources: Vec<SoundSource>,
}

#[derive(Default)]
pub struct SCMessage {
    master_volume: Option<u32>,
    play_info: Option<PlayInfo>,
}

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
                let message = rx.recv().unwrap_or(SCMessage::default());
                if let Some(playinfo) = message.play_info {
                    let sink = Self::_play(&playinfo.sources, &sh);
                    sink.set_volume(Self::to_volume_magnification(
                        master_volume,
                        playinfo.volume,
                    ));

                    let exsink: ExSink = ExSink {
                        volume: playinfo.volume,
                        sink: sink,
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
        let play_info = PlayInfo {
            volume: volume,
            sources: sources,
        };

        Self::play(&tx, play_info);
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
        let source =
            rodio::Decoder::new(std::io::Cursor::new(PresetVoice::voice_data(index))).unwrap();

        sink.append(source);
    }
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    settings: Settings,
    #[serde(skip)]
    time_table_diff_base: Vec<TTElement>,
    #[serde(skip)]
    tx_sc: Option<std::sync::mpsc::Sender<SCMessage>>,
    #[serde(skip)]
    tx_s: Option<std::sync::mpsc::Sender<SMessage>>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            settings: Settings::default(),
            time_table_diff_base: Vec::new(),
            tx_sc: None,
            tx_s: None,
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        let tx_sc = SoundCoordinator::activate();
        let tx_s = Scheduler::activate(tx_sc.clone());
        let tx_s_for_init = tx_s.clone();

        let mut app;

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        //
        if let Some(storage) = cc.storage {
            let mut stored: Self = eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
            stored.tx_sc = Some(tx_sc);
            stored.tx_s = Some(tx_s);
            app = stored;
        } else {
            app = TemplateApp {
                tx_sc: Some(tx_sc),
                tx_s: Some(tx_s),
                ..Default::default()
            }
        }

        for row in &app.settings.time_table {
            app.time_table_diff_base.push(row.clone());
            Scheduler::edit(&tx_s_for_init, row);
        }

        app
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
        let Settings {
            master_volume,
            master_mute,
            time_table,
        } = &mut self.settings;

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
                        ui.checkbox(master_mute, "Muting");
                        SoundCoordinator::set_master_volume(self.tx_sc.as_ref().unwrap(), 0);
                    } else {
                        ui.checkbox(master_mute, "");
                        SoundCoordinator::set_master_volume(
                            self.tx_sc.as_ref().unwrap(),
                            *master_volume,
                        );
                    }
                });
            });

            use egui_extras::{Size, TableBuilder};
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
                            ui.label(clock_emoji(time_table[row_index].h as usize));
                        });
                        row.col(|ui| {
                            let time = format!(
                                "{0:>02}:{1:>02}",
                                time_table[row_index].h, time_table[row_index].m,
                            );
                            ui.label(time);
                        });
                        row.col(|ui| {
                            ui.checkbox(&mut time_table[row_index].active, "");
                            if time_table[row_index].active
                                != self.time_table_diff_base[row_index].active
                            {
                                Scheduler::edit(
                                    self.tx_s.as_ref().unwrap(),
                                    &time_table[row_index],
                                );
                                self.time_table_diff_base[row_index].active =
                                    time_table[row_index].active;
                            }
                        });
                        row.col(|ui| {
                            let voice_index = time_table[row_index].h;

                            let icon = emojis::get_by_shortcode("arrow_forward").unwrap().as_str();
                            if ui.small_button(icon).clicked() {
                                SoundCoordinator::play_full_set_list(
                                    self.tx_sc.as_ref().unwrap(),
                                    voice_index,
                                    100,
                                );
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
