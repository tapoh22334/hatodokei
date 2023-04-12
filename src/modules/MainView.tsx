import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api";
import {
    defaultSettings,
    TTElement,
    getMasterVolumeStorage,
    getEffectStorage,
    getVoiceStorage,
    getTimeTableStorage,
    setMasterVolumeStorage,
    setEffectStorage,
    setVoiceStorage,
    setTimeTableStorage
    } from "./DefaultSetting";
import { CardTimeSwitch } from "./CardTimeSwitch";

import * as React from "react";
import AppBar from "@mui/material/AppBar";
import Toolbar from "@mui/material/Toolbar";
import Box from "@mui/material/Box";
import Stack from "@mui/material/Stack";
import Switch from "@mui/material/Switch";
import Slider from "@mui/material/Slider";
import VolumeDown from "@mui/icons-material/VolumeDown";
import VolumeUp from "@mui/icons-material/VolumeUp";
import FormControlLabel from '@mui/material/FormControlLabel';

import InputLabel from '@mui/material/InputLabel';
import MenuItem from '@mui/material/MenuItem';
import FormControl from '@mui/material/FormControl';
import Select, { SelectChangeEvent } from '@mui/material/Select';

export const MainView: React.VFC = (props) => {
  const [masterVolume, setMasterVolume] = useState(() => {
    let initMasterVolume = getMasterVolumeStorage();
    if (initMasterVolume === null) {
      initMasterVolume = defaultSettings.master_volume;
      console.log("Not exist stored master volume");
    }
    console.log("initial master volume: %d", initMasterVolume);
    return initMasterVolume;
  });

  const [effect, setEffect] = useState(() => {
    let initEffect = getEffectStorage();
    if (initEffect === null) {
      initEffect = defaultSettings.effect;
      console.log("Not exist stored effect");
    }
    console.log("initial effect: %s", initEffect);
    return initEffect;
  });

  const [voice, setVoice] = useState(() => {
    let initVoice = getVoiceStorage();
    if (initVoice === null) {
      initVoice = defaultSettings.voice;
      console.log("Not exist stored voice");
    }
    console.log("initial voice: %s", initVoice);
    return initVoice;
  });

  const [timeTable, setTimeTable] = useState<Array<TTElement>>(() => {
      let initTimeTable = getTimeTableStorage();
      if (initTimeTable === null) {
          initTimeTable = defaultSettings.time_table

          // TimeTbleStorage is not null able, Must be initialized here.
          setTimeTableStorage(initTimeTable);
          console.log("Not exist stored table");
      }
      console.log("time table: %o",initTimeTable);
      return initTimeTable;
  });

  useEffect(() => {
    invoke("set_master_volume", { volume: masterVolume });
    setMasterVolumeStorage(masterVolume);
    console.log("set master volume: %d", masterVolume);
  }, [masterVolume]);

  const handleVolumeChange = (event: Event, value: number | number[]) => {
    setMasterVolume(value as number);
  };

  useEffect(() => {
    invoke("set_effect", { effect: effect });
    setEffectStorage(effect);
    console.log("set effect: %s", effect);
  }, [effect]);

  const onEffectChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setEffect(event.target.checked as boolean);
  };

  useEffect(() => {
    invoke("set_voice", { voice: voice });
    setVoiceStorage(voice);
    console.log("set voice: %s", voice);
  }, [voice]);

  const onVoiceChange = (event: SelectChangeEvent) => {
    setVoice(event.target.value as string);
  };

  useEffect(() => {
    console.log("TimeTable Changed!! %o", timeTable);
  }, [timeTable]);

  return (
    <React.Fragment>
    <AppBar sx={{overflow: 'hidden'}}>
      <Toolbar>
        <Box sx={{ width: 220 }} alignItems="center" position="sticky">
          <Stack
            spacing={0}
            direction="row"
            sx={{ my: 0 }}
            alignItems="center"
            justifyContent="center"
          >
            <VolumeDown />
            <Slider
              sx={{ color: "#fff" }}
              aria-label="MasterVolume"
              value={masterVolume}
              onChange={handleVolumeChange}
            />
            <VolumeUp />
          </Stack>
        </Box>
      </Toolbar>

    </AppBar>

    {/*<Box sx={{ mt: 8, mb: 7, overflow: 'hidden', overflowY: 'scroll', height: 540}}>*/}
    <Box sx={{ mt: 8, overflow: 'hidden', overflowY: 'scroll', height: 575}}>

    <FormControlLabel control={<Switch value={effect} onChange={onEffectChange}/>} label="効果音" />
    <FormControl sx={{ m: 1 , minWidth: 150}} size="small">
      <InputLabel id="voice-select">声</InputLabel>
      <Select
        labelId="voice-select"
        id="voice-select"
        value={voice}
        label="Voice"
        onChange={onVoiceChange}
        >
            <MenuItem value="つくよみちゃん-れいせい">つくよみちゃん</MenuItem> 
            <MenuItem value="MANA-のーまる">MANA</MenuItem> 
            <MenuItem value="おふとんP-のーまるv2">おふとんP</MenuItem> 
            <MenuItem value="ディアちゃん-のーまる">ディアちゃん</MenuItem> 
            <MenuItem value="アルマちゃん-表-v2">アルマちゃん</MenuItem> 
            <MenuItem value="KANA-のーまる">KANA</MenuItem> 
            <MenuItem value="MANA+-ないしょばなし">MANA+-ないしょばなし</MenuItem> 
            <MenuItem value="AI声優-朱花-のーまる">AI声優-朱花</MenuItem> 
            <MenuItem value="AI声優-青葉-のーまる">AI声優-青葉</MenuItem> 
            <MenuItem value="AI声優-銀芽-のーまる">AI声優-銀芽</MenuItem> 
            <MenuItem value="伊能いお-ふつう">伊能いお</MenuItem> 
            <MenuItem value="あみたろ-のーまるv4">あみたろ</MenuItem> 
            <MenuItem value="お星-テンション↑↑">お星</MenuItem> 
            <MenuItem value="四国めたん-ノーマル">四国めたん</MenuItem> 
            <MenuItem value="ずんだもん-ノーマル">ずんだもん</MenuItem> 
            <MenuItem value="ずんだもん-ヒソヒソ">ずんだもん-ヒソヒソ</MenuItem> 
            <MenuItem value="春日部つむぎ-ノーマル">春日部つむぎ</MenuItem> 
            <MenuItem value="雨晴はう-ノーマル">雨晴はう</MenuItem> 
            <MenuItem value="波音リツ-ノーマル">波音リツ</MenuItem> 
            <MenuItem value="玄野武宏-ノーマル">玄野武宏</MenuItem> 
            <MenuItem value="白上虎太郎-ふつう">白上虎太郎</MenuItem> 
            <MenuItem value="青山龍星-ノーマル">青山龍星</MenuItem> 
            <MenuItem value="冥鳴ひまり-ノーマル">冥鳴ひまり</MenuItem> 
            <MenuItem value="九州そら-ノーマル">九州そら</MenuItem> 
            <MenuItem value="剣崎雌雄-ノーマル">剣崎雌雄</MenuItem> 
            <MenuItem value="WhiteCUL-ノーマル">WhiteCUL</MenuItem> 
            <MenuItem value="後鬼-人間ver.">後鬼-人間ver.</MenuItem> 
            <MenuItem value="ちび式じい-ノーマル">ちび式じい</MenuItem> 
            <MenuItem value="櫻歌ミコ-ノーマル">櫻歌ミコ</MenuItem> 
            <MenuItem value="小夜/SAYO-ノーマル">小夜/SAYO</MenuItem> 
            <MenuItem value="ナースロボ＿タイプＴ-ノーマル">ナースロボ＿タイプＴ</MenuItem> 
            <MenuItem value="†聖騎士 紅桜†-ノーマル">†聖騎士 紅桜†</MenuItem> 
            <MenuItem value="雀松朱司-ノーマル">雀松朱司</MenuItem> 
            <MenuItem value="麒ヶ島宗麟-ノーマル">麒ヶ島宗麟</MenuItem> 
            <MenuItem value="春歌ナナ-ノーマル">春歌ナナ</MenuItem> 
            <MenuItem value="猫使アル-ノーマル">猫使アル</MenuItem> 
            <MenuItem value="猫使ビィ-ノーマル">猫使ビィ</MenuItem> 
            <MenuItem value="#ランダム">#ランダム</MenuItem> 
      </Select>
    </FormControl>


    <Stack
      spacing={0.3}
      alignItems="center"
      sx={{ mb: 7 }}
      //justifyContent="center"
    >
      {timeTable.length > 0 &&
        timeTable.map((row) => {
          const labelId = `stack-list-label-${row["time"]}`;
          return <CardTimeSwitch row={row}/>;
        })}
    </Stack>
    </Box>

    </React.Fragment>
  );

}
