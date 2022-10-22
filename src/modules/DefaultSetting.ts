export interface TTElement {
  time: number;
  active: boolean;
}

export interface Settings {
  master_volume: number;
  master_mute: boolean;
  voice: string;
  time_table: TTElement[];
}

export const defaultSettings: Settings = {
  master_volume: 80,
  master_mute: false,
  voice: "Tsukuyomichan",
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
  ],
};

export const getMasterVolumeStorage = () => {
    const json = localStorage.getItem("hatodokeiMasterVolume");
    return json === null ? null : JSON.parse(json);
  };

export const getMasterMuteStorage = () => {
    const json = localStorage.getItem("hatodokeiMasterMute");
    return json === null ? null : JSON.parse(json);
  };

export const getVoiceStorage = () => {
    const json = localStorage.getItem("hatodokeiVoice");
    return json === null ? null : JSON.parse(json);
  };

export const getTimeTableStorage = () => {
    const json = localStorage.getItem("hatodokeiTimeTable");
    return json === null ? null : JSON.parse(json);
  };

export const setMasterVolumeStorage = (masterVolume: number) => {
    localStorage.setItem("hatodokeiMasterVolume", JSON.stringify(masterVolume));
  };

export const setMasterMuteStorage = (masterMute: boolean) => {
    localStorage.setItem("hatodokeiMasterMute", JSON.stringify(masterMute));
  };

export const setVoiceStorage = (voice: string) => {
    localStorage.setItem("hatodokeiVoice", JSON.stringify(voice));
  };

export const setTimeTableStorage = (timeTable: TTElement[]) => {
    localStorage.setItem("hatodokeiTimeTable", JSON.stringify(timeTable));
  };
