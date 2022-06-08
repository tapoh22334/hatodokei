#[derive(serde::Deserialize, serde::Serialize, Copy, Clone, Debug)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TTElement {
    pub h: u32,
    pub m: u32,
    pub active: bool,
}

impl Default for TTElement {
    fn default() -> Self {
        Self {
            h: 0,
            m: 0,
            active: true,
        }
    }
}

impl TTElement {
    pub fn time(&self) -> u32 {
        Self::join(self.h, self.m)
    }

    pub fn join(h: u32, m: u32) -> u32 {
        h * 100 + m
    }

    pub fn sub(h1: u32, m1: u32, h2: u32, m2: u32) -> u32 {
        let v1 = Self::join(h1, m1) as i32;
        let v2 = Self::join(h2, m2) as i32;
        ((v1 - v2 + 2359) % 2359) as u32
    }
}
