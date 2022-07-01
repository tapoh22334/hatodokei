#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{CustomMenuItem, WindowEvent, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu};

mod debug;
mod preset_voice;
mod scheduler;
mod setting;
mod sound_coordinator;
mod ttelement;

use crate::scheduler::{SMessage, Scheduler};
use crate::setting::Settings;
use crate::sound_coordinator::{SCMessage, SoundCoordinator};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello {} !", name)
}

#[tauri::command]
fn get_settings(settings: tauri::State<Settings>) -> Settings {
    settings.clone().inner().to_owned()
}

#[tauri::command]
fn set_master_volume(volume: u32, tx: tauri::State<std::sync::mpsc::SyncSender<SCMessage>>) {
    SoundCoordinator::set_master_volume(&tx, volume);
}

#[tauri::command]
fn set_master_mute(mute: bool, tx: tauri::State<std::sync::mpsc::SyncSender<SCMessage>>) {
    SoundCoordinator::set_master_mute(&tx, mute);
}

#[tauri::command]
fn play(index: u32, tx: tauri::State<std::sync::mpsc::SyncSender<SCMessage>>) {
    SoundCoordinator::play_full_set_list(&tx, index, 100);
}

#[tauri::command]
fn set_table_row(
    row: ttelement::TTElement,
    tx: tauri::State<std::sync::mpsc::SyncSender<SMessage>>,
) {
    Scheduler::edit(&tx, &row);
}

fn main() {
    // Start Backend
    let tx_sound_coordinator = sound_coordinator::SoundCoordinator::activate();
    let tx_scheduler = scheduler::Scheduler::activate(tx_sound_coordinator.clone());

    // Load settings
    // TODO: implement save, load feature
    let settings = setting::Settings::default();
    for row in &settings.time_table {
        Scheduler::edit(&tx_scheduler, row);
    }

    // System tray icon
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu = SystemTrayMenu::new().add_item(quit);
    let system_tray = SystemTray::new().with_menu(tray_menu);

    // Show main window
    let context = tauri::generate_context!();
    tauri::Builder::default()
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick { .. } => {
                let window = app.get_window("main").unwrap();
                if window.is_visible().unwrap() {
                    window.hide().unwrap();
                    window.minimize().unwrap();
                } else {
                    window.show().unwrap();
                    window.unminimize().unwrap();
                    window.set_focus().unwrap();
                }
            }
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                _ => {}
            },
            _ => {}
        })
        .on_window_event(|event| match event.event() {
            WindowEvent::Focused(focus) => {
                if !focus {
                    event.window().hide().unwrap();
                    println!("main: UnFocused");
                }
            }
            _ => {}
        })
        .system_tray(system_tray)
        .manage(settings)
        .manage(tx_scheduler)
        .manage(tx_sound_coordinator)
        .invoke_handler(tauri::generate_handler![
            greet,
            get_settings,
            set_master_volume,
            set_master_mute,
            play,
            set_table_row
        ])
        .run(context)
        .expect("error while running tauri application");
}
