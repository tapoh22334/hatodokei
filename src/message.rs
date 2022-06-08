use crate::ttelement::TTElement;

pub enum SMessage {
    Overwrite(TTElement),
    // Not impremented
    // this element is reserved for adding/deleting schedule.
    #[allow(unused)]
    Delete(TTElement),
}

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
    pub volume: u32,
    pub sources: Vec<SoundSource>,
}

#[derive(Default)]
pub struct SCMessage {
    pub master_volume: Option<u32>,
    pub play_info: Option<PlayInfo>,
}
