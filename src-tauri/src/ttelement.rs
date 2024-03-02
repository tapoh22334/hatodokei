#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TTElement {
    pub time: u32,
    pub active: bool,
    pub effect: bool,
    pub voice: String,
    pub volume: u32,
}

impl Default for TTElement {
    fn default() -> Self {
        Self {
            time: 0,
            active: true,
            effect: true,
            voice: "つくよみちゃん-れいせい".to_string(),
            volume: 100,
        }
    }
}
