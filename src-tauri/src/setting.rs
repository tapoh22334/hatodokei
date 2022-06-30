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
            time_table: vec![
                ttelement::TTElement { time:    0, active: true, },
                ttelement::TTElement { time:  100, active: false, },
                ttelement::TTElement { time:  200, active: false, },
                ttelement::TTElement { time:  300, active: false, },
                ttelement::TTElement { time:  400, active: false, },
                ttelement::TTElement { time:  500, active: false, },
                ttelement::TTElement { time:  600, active: false, },
                ttelement::TTElement { time:  700, active: false, },
                ttelement::TTElement { time:  800, active: true, },
                ttelement::TTElement { time:  900, active: true, },
                ttelement::TTElement { time: 1000, active: true, },
                ttelement::TTElement { time: 1100, active: true, },
                ttelement::TTElement { time: 1200, active: true, },
                ttelement::TTElement { time: 1300, active: true, },
                ttelement::TTElement { time: 1400, active: true, },
                ttelement::TTElement { time: 1500, active: true, },
                ttelement::TTElement { time: 1600, active: true, },
                ttelement::TTElement { time: 1700, active: true, },
                ttelement::TTElement { time: 1800, active: true, },
                ttelement::TTElement { time: 1900, active: true, },
                ttelement::TTElement { time: 2000, active: true, },
                ttelement::TTElement { time: 2100, active: true, },
                ttelement::TTElement { time: 2200, active: true, },
                ttelement::TTElement { time: 2300, active: true, },
            ],
        }
    }
}
