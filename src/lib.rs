#![warn(clippy::all, rust_2018_idioms)]

mod sound_coordinator;
pub use sound_coordinator::SoundCoordinator;

mod ttelement;
pub use ttelement::TTElement;

mod scheduler;
pub use scheduler::Scheduler;

mod preset_voice;
pub use preset_voice::PresetVoice;

mod message;
pub use message::{PlayInfo, SCMessage, SMessage, SoundSource};

mod app;
pub use app::TemplateApp;

// ----------------------------------------------------------------------------
// When compiling for web:

#[cfg(target_arch = "wasm32")]
use eframe::wasm_bindgen::{self, prelude::*};

/// This is the entry-point for all the web-assembly.
/// This is called once from the HTML.
/// It loads the app, installs some callbacks, then returns.
/// You can add more callbacks like this if you want to call in to your code.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn start(canvas_id: &str) -> Result<(), eframe::wasm_bindgen::JsValue> {
    // Make sure panics are logged using `console.error`.
    console_error_panic_hook::set_once();

    // Redirect tracing to console.log and friends:
    tracing_wasm::set_as_global_default();

    eframe::start_web(canvas_id, Box::new(|cc| Box::new(TemplateApp::new(cc))))
}
