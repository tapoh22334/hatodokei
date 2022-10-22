#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TTElement {
    pub time: u32,
    pub active: bool,
}

impl Default for TTElement {
    fn default() -> Self {
        Self {
            time: 0,
            active: true,
        }
    }
}

impl TTElement {
    pub fn sub(t1: u32, t2: u32) -> u32 {
        ((t1 + 2359 - t2) % 2359) as u32
    }
}
