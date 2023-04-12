export interface TTElement {
  time: number;
  active: boolean;
  effect: boolean;
  voice: string;
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
    { time: 0,    active: true  , effect: true, voice: "つくよみちゃん-れいせい"},
    { time: 100,  active: false , effect: true, voice: "つくよみちゃん-れいせい"},
    { time: 200,  active: false , effect: true, voice: "つくよみちゃん-れいせい"},
    { time: 300,  active: false , effect: true, voice: "つくよみちゃん-れいせい"},
    { time: 400,  active: false , effect: true, voice: "つくよみちゃん-れいせい"},
    { time: 500,  active: false , effect: true, voice: "つくよみちゃん-れいせい"},
    { time: 600,  active: false , effect: true, voice: "つくよみちゃん-れいせい"},
    { time: 700,  active: false , effect: true, voice: "つくよみちゃん-れいせい"},
    { time: 800,  active: true  , effect: true, voice: "つくよみちゃん-れいせい"},
    { time: 900,  active: true  , effect: true, voice: "つくよみちゃん-れいせい"},
    { time: 1000, active: true  , effect: true, voice: "つくよみちゃん-れいせい"},
    { time: 1100, active: true  , effect: true, voice: "つくよみちゃん-れいせい"},
    { time: 1200, active: true  , effect: true, voice: "つくよみちゃん-れいせい"},
    { time: 1300, active: true  , effect: true, voice: "つくよみちゃん-れいせい"},
    { time: 1400, active: true  , effect: true, voice: "つくよみちゃん-れいせい"},
    { time: 1500, active: true  , effect: true, voice: "つくよみちゃん-れいせい"},
    { time: 1600, active: true  , effect: true, voice: "つくよみちゃん-れいせい"},
    { time: 1700, active: true  , effect: true, voice: "つくよみちゃん-れいせい"},
    { time: 1800, active: true  , effect: true, voice: "つくよみちゃん-れいせい"},
    { time: 1900, active: true  , effect: true, voice: "つくよみちゃん-れいせい"},
    { time: 2000, active: true  , effect: true, voice: "つくよみちゃん-れいせい"},
    { time: 2100, active: true  , effect: true, voice: "つくよみちゃん-れいせい"},
    { time: 2200, active: true  , effect: true, voice: "つくよみちゃん-れいせい"},
    { time: 2300, active: true  , effect: true, voice: "つくよみちゃん-れいせい"},
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

export const getEffectStorage = () => {
    const json = localStorage.getItem("hatodokeiEffect");
    return json === null ? null : JSON.parse(json);
  };

export const getVoiceStorage = () => {
    const json = localStorage.getItem("hatodokeiVoice");
    return json === null ? null : JSON.parse(json);
  };

export const getTimeTableStorage = () => {
    const json = localStorage.getItem("hatodokeiTimeTable2");
    return json === null ? null : JSON.parse(json);
  };

export const setMasterVolumeStorage = (masterVolume: number) => {
    localStorage.setItem("hatodokeiMasterVolume", JSON.stringify(masterVolume));
  };

export const setMasterMuteStorage = (masterMute: boolean) => {
    localStorage.setItem("hatodokeiMasterMute", JSON.stringify(masterMute));
  };

export const setEffectStorage = (effect: boolean) => {
    localStorage.setItem("hatodokeiEffect", JSON.stringify(effect));
  };

export const setVoiceStorage = (voice: string) => {
    localStorage.setItem("hatodokeiVoice", JSON.stringify(voice));
  };

export const setTimeTableStorage = (timeTable: TTElement[]) => {
    localStorage.setItem("hatodokeiTimeTable2", JSON.stringify(timeTable));
  };
