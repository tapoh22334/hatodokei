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

