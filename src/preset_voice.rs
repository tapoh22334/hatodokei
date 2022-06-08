pub struct PresetVoice {}

impl PresetVoice {
    // TODO: Improve this function to not use clone
    pub fn voice_data(index: u32) -> Vec<u8> {
        let voice_data = vec![
            include_bytes!("data/0000.wav").to_vec(),
            include_bytes!("data/0100.wav").to_vec(),
            include_bytes!("data/0200.wav").to_vec(),
            include_bytes!("data/0300.wav").to_vec(),
            include_bytes!("data/0400.wav").to_vec(),
            include_bytes!("data/0500.wav").to_vec(),
            include_bytes!("data/0600.wav").to_vec(),
            include_bytes!("data/0700.wav").to_vec(),
            include_bytes!("data/0800.wav").to_vec(),
            include_bytes!("data/0900.wav").to_vec(),
            include_bytes!("data/1000.wav").to_vec(),
            include_bytes!("data/1100.wav").to_vec(),
            include_bytes!("data/1200.wav").to_vec(),
            include_bytes!("data/1300.wav").to_vec(),
            include_bytes!("data/1400.wav").to_vec(),
            include_bytes!("data/1500.wav").to_vec(),
            include_bytes!("data/1600.wav").to_vec(),
            include_bytes!("data/1700.wav").to_vec(),
            include_bytes!("data/1800.wav").to_vec(),
            include_bytes!("data/1900.wav").to_vec(),
            include_bytes!("data/2000.wav").to_vec(),
            include_bytes!("data/2100.wav").to_vec(),
            include_bytes!("data/2200.wav").to_vec(),
            include_bytes!("data/2300.wav").to_vec(),
        ];
        voice_data[index as usize].clone()
    }
}
