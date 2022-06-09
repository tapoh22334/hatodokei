#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

const IMAGE: &[u8] = include_bytes!("../icon.png");

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let icon = image::load_from_memory(IMAGE).expect("Couldn't load image").to_rgba8();
    let (icon_width, icon_height) = icon.dimensions();

    let native_options = eframe::NativeOptions {
        icon_data: Some(eframe::IconData {
            rgba: icon.into_raw(),
            width: icon_width,
            height: icon_height,
        }),
        resizable: false,
        ..eframe::NativeOptions::default()
    };
    eframe::run_native(
        "hatodokei",
        native_options,
        Box::new(|cc| Box::new(hatodokei::TemplateApp::new(cc))),
    );
}
