use crate::ttelement;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct Settings {
    pub master_volume: u32,
    pub master_mute: bool,
    pub time_table: Vec<ttelement::TTElement>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            master_volume: 80,
            master_mute: false,
            time_table: vec![],
        }
    }
}
