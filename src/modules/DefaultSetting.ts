export interface TTElement {
  time: number;
  active: boolean;
  effect: boolean;
  voice: string;
  volume: number;
}

export interface Settings {
  master_volume: number;
  master_mute: boolean;
  time_table: TTElement[];
}

export const defaultSettings: Settings = {
  master_volume: 80,
  master_mute: false,
  time_table: [
    { time: 0,    active: true  , effect: true, voice: "つくよみちゃん-れいせい", volume: 100},
    { time: 100,  active: false , effect: true, voice: "つくよみちゃん-れいせい", volume: 100},
    { time: 200,  active: false , effect: true, voice: "つくよみちゃん-れいせい", volume: 100},
    { time: 300,  active: false , effect: true, voice: "つくよみちゃん-れいせい", volume: 100},
    { time: 400,  active: false , effect: true, voice: "つくよみちゃん-れいせい", volume: 100},
    { time: 500,  active: false , effect: true, voice: "つくよみちゃん-れいせい", volume: 100},
    { time: 600,  active: false , effect: true, voice: "つくよみちゃん-れいせい", volume: 100},
    { time: 700,  active: false , effect: true, voice: "つくよみちゃん-れいせい", volume: 100},
    { time: 800,  active: true  , effect: true, voice: "つくよみちゃん-れいせい", volume: 100},
    { time: 900,  active: true  , effect: true, voice: "つくよみちゃん-れいせい", volume: 100},
    { time: 1000, active: true  , effect: true, voice: "つくよみちゃん-れいせい", volume: 100},
    { time: 1100, active: true  , effect: true, voice: "つくよみちゃん-れいせい", volume: 100},
    { time: 1200, active: true  , effect: true, voice: "つくよみちゃん-れいせい", volume: 100},
    { time: 1300, active: true  , effect: true, voice: "つくよみちゃん-れいせい", volume: 100},
    { time: 1400, active: true  , effect: true, voice: "つくよみちゃん-れいせい", volume: 100},
    { time: 1500, active: true  , effect: true, voice: "つくよみちゃん-れいせい", volume: 100},
    { time: 1600, active: true  , effect: true, voice: "つくよみちゃん-れいせい", volume: 100},
    { time: 1700, active: true  , effect: true, voice: "つくよみちゃん-れいせい", volume: 100},
    { time: 1800, active: true  , effect: true, voice: "つくよみちゃん-れいせい", volume: 100},
    { time: 1900, active: true  , effect: true, voice: "つくよみちゃん-れいせい", volume: 100},
    { time: 2000, active: true  , effect: true, voice: "つくよみちゃん-れいせい", volume: 100},
    { time: 2100, active: true  , effect: true, voice: "つくよみちゃん-れいせい", volume: 100},
    { time: 2200, active: true  , effect: true, voice: "つくよみちゃん-れいせい", volume: 100},
    { time: 2300, active: true  , effect: true, voice: "つくよみちゃん-れいせい", volume: 100},
  ],
};

export const getMasterVolumeStorage = () => {
    const json = localStorage.getItem("hatodokeiMasterVolume");
    return json === null ? null : JSON.parse(json);
  };

export const setMasterVolumeStorage = (masterVolume: number) => {
    localStorage.setItem("hatodokeiMasterVolume", JSON.stringify(masterVolume));
  };

export const getTimeTableStorage = () => {
    const json = localStorage.getItem("hatodokeiTimeTable2.0");
    return json === null ? null : JSON.parse(json);
  };

export const setTimeTableStorage = (timeTable: TTElement[]) => {
    localStorage.setItem("hatodokeiTimeTable2.0", JSON.stringify(timeTable));
  };
