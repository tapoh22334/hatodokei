export interface TTElement {
  time: number;
  active: boolean;
};

export interface Settings {
  master_volume: number;
  master_mute: boolean;
  time_table: TTElement[];
};

export const defaultSettings: Settings = {
  master_volume: 80,
  master_mute: false,
  time_table: [
    { time: 0, active: true },
    { time: 100, active: false },
    { time: 200, active: false },
    { time: 300, active: false },
    { time: 400, active: false },
    { time: 500, active: false },
    { time: 600, active: false },
    { time: 700, active: false },
    { time: 800, active: true },
    { time: 900, active: true },
    { time: 1000, active: true },
    { time: 1100, active: true },
    { time: 1200, active: true },
    { time: 1300, active: true },
    { time: 1400, active: true },
    { time: 1500, active: true },
    { time: 1600, active: true },
    { time: 1700, active: true },
    { time: 1800, active: true },
    { time: 1900, active: true },
    { time: 2000, active: true },
    { time: 2100, active: true },
    { time: 2200, active: true },
    { time: 2300, active: true },
    ]
};
