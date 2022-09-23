#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem, WindowEvent};

mod debug;
mod preset_voice;
mod scheduler;
mod setting;
mod sound_coordinator;
mod ttelement;

use crate::scheduler::{SMessage, Scheduler};
use crate::sound_coordinator::{SCMessage, SoundCoordinator};

#[tauri::command]
fn set_master_volume(volume: u32, tx: tauri::State<std::sync::mpsc::SyncSender<SCMessage>>) {
    SoundCoordinator::set_master_volume(&tx, volume);
}

#[tauri::command]
fn set_master_mute(mute: bool, tx: tauri::State<std::sync::mpsc::SyncSender<SCMessage>>) {
    SoundCoordinator::set_master_mute(&tx, mute);
}

#[tauri::command]
fn set_table_row(
    row: ttelement::TTElement,
    tx: tauri::State<std::sync::mpsc::SyncSender<SMessage>>,
) {
    Scheduler::edit(&tx, &row);
}

#[tauri::command]
fn play(index: u32, tx: tauri::State<std::sync::mpsc::SyncSender<SCMessage>>) {
    SoundCoordinator::play_full_set_list(&tx, index, 100);
}

fn main() {
    // Start Backend
    let tx_sound_coordinator = sound_coordinator::SoundCoordinator::activate();
    let tx_scheduler = scheduler::Scheduler::activate(tx_sound_coordinator.clone());

    // Load settings
    // TODO: implement save, load feature
    let settings = setting::Settings::default();

    // System tray icon
    let quit = CustomMenuItem::new("Quit".to_string(), "Quit");
    let about = CustomMenuItem::new("About".to_string(), "About");
    let tray_menu = SystemTrayMenu::new()
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(about);
    let system_tray = SystemTray::new().with_menu(tray_menu);

    // Show main window
    let context = tauri::generate_context!();
    tauri::Builder::default()
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick { .. } => {
                let window = app.get_window("main").unwrap();
                if !window.is_visible().unwrap() {
                    window.show().unwrap();
                    window.unminimize().unwrap();
                    window.set_focus().unwrap();
                } else {
                    window.minimize().unwrap();
                }
            }
            SystemTrayEvent::MenuItemClick { id, .. } => {
                match id.as_str() {
                    "Quit" => {
                        std::process::exit(0);
                    }
                    "About" => {
                        let window = app.get_window("main").unwrap();
                        tauri::api::dialog::message(Some(&window), "Hatodokei", "鳩時計時報 v1.3.3");
                    }
                    _ => {}
                }
            }
            _ => {}
        })
        .on_window_event(|event| {
            if let WindowEvent::Resized(size) = event.event() {
                if size.width == 0 && size.height == 0 {
                    event.window().hide().unwrap();
                }
            }
        })
        .system_tray(system_tray)
        .manage(settings)
        .manage(tx_scheduler)
        .manage(tx_sound_coordinator)
        .invoke_handler(tauri::generate_handler![
            set_master_volume,
            set_master_mute,
            set_table_row,
            play,
        ])
        .run(context)
        .expect("error while running tauri application");
}
