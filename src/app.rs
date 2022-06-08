use crate::message;
use crate::scheduler;
use crate::sound_coordinator;
use crate::ttelement;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct Settings {
    master_volume: u32,
    master_mute: bool,
    time_table: Vec<ttelement::TTElement>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            master_volume: 100,
            master_mute: false,
            time_table: vec![
                ttelement::TTElement {
                    h: 0,
                    m: 0,
                    active: true,
                },
                ttelement::TTElement {
                    h: 1,
                    m: 0,
                    active: false,
                },
                ttelement::TTElement {
                    h: 2,
                    m: 0,
                    active: false,
                },
                ttelement::TTElement {
                    h: 3,
                    m: 0,
                    active: false,
                },
                ttelement::TTElement {
                    h: 4,
                    m: 0,
                    active: false,
                },
                ttelement::TTElement {
                    h: 5,
                    m: 0,
                    active: false,
                },
                ttelement::TTElement {
                    h: 6,
                    m: 0,
                    active: false,
                },
                ttelement::TTElement {
                    h: 7,
                    m: 0,
                    active: false,
                },
                ttelement::TTElement {
                    h: 8,
                    m: 0,
                    active: true,
                },
                ttelement::TTElement {
                    h: 9,
                    m: 0,
                    active: true,
                },
                ttelement::TTElement {
                    h: 10,
                    m: 0,
                    active: true,
                },
                ttelement::TTElement {
                    h: 11,
                    m: 0,
                    active: true,
                },
                ttelement::TTElement {
                    h: 12,
                    m: 0,
                    active: true,
                },
                ttelement::TTElement {
                    h: 13,
                    m: 0,
                    active: true,
                },
                ttelement::TTElement {
                    h: 14,
                    m: 0,
                    active: true,
                },
                ttelement::TTElement {
                    h: 15,
                    m: 0,
                    active: true,
                },
                ttelement::TTElement {
                    h: 16,
                    m: 0,
                    active: true,
                },
                ttelement::TTElement {
                    h: 17,
                    m: 0,
                    active: true,
                },
                ttelement::TTElement {
                    h: 18,
                    m: 0,
                    active: true,
                },
                ttelement::TTElement {
                    h: 19,
                    m: 0,
                    active: true,
                },
                ttelement::TTElement {
                    h: 20,
                    m: 0,
                    active: true,
                },
                ttelement::TTElement {
                    h: 21,
                    m: 0,
                    active: true,
                },
                ttelement::TTElement {
                    h: 22,
                    m: 0,
                    active: true,
                },
                ttelement::TTElement {
                    h: 23,
                    m: 0,
                    active: true,
                },
            ],
        }
    }
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize, Default)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    settings: Settings,
    #[serde(skip)]
    time_table_diff_base: Vec<ttelement::TTElement>,
    #[serde(skip)]
    tx_sc: Option<std::sync::mpsc::Sender<message::SCMessage>>,
    #[serde(skip)]
    tx_s: Option<std::sync::mpsc::Sender<message::SMessage>>,
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        let tx_sc = sound_coordinator::SoundCoordinator::activate();
        let tx_s = scheduler::Scheduler::activate(tx_sc.clone());
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
            app.time_table_diff_base.push(*row);
            scheduler::Scheduler::edit(&tx_s_for_init, row);
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
        ctx.set_visuals(egui::style::Visuals::dark());

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
            ui.vertical(|ui| {
                ui.group(|ui| {
                    // UI view
                    ui.add(egui::Slider::new(master_volume, 0..=100).suffix("%"));

                    if *master_mute {
                        ui.checkbox(master_mute, "Muting");
                        sound_coordinator::SoundCoordinator::set_master_volume(
                            self.tx_sc.as_ref().unwrap(),
                            0,
                            );
                    } else {
                        ui.checkbox(master_mute, "Mute");
                        sound_coordinator::SoundCoordinator::set_master_volume(
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
                                scheduler::Scheduler::edit(
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
                                sound_coordinator::SoundCoordinator::play_full_set_list(
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
    char::from_u32(0x1f550 + (row_index + 23) as u32 % 12)
        .unwrap()
        .to_string()
}
