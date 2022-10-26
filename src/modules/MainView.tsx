import { useState, useEffect } from "react";
import { emit, listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api";
import {
    defaultSettings,
    TTElement,
    Settings,
    getMasterMuteStorage,
    getMasterVolumeStorage,
    getVoiceStorage,
    getTimeTableStorage,
    setMasterMuteStorage,
    setMasterVolumeStorage,
    setVoiceStorage,
    setTimeTableStorage
    } from "./DefaultSetting";
import { CardTimeSwitch } from "./CardTimeSwitch";

import * as React from "react";
import AppBar from "@mui/material/AppBar";
import Toolbar from "@mui/material/Toolbar";
import Box from "@mui/material/Box";
import Stack from "@mui/material/Stack";
import Grid from "@mui/material/Grid";
import Card from "@mui/material/Card";
import Switch from "@mui/material/Switch";
import Slider from "@mui/material/Slider";
import VolumeDown from "@mui/icons-material/VolumeDown";
import VolumeUp from "@mui/icons-material/VolumeUp";
import { styled } from "@mui/material/styles";

import InputLabel from '@mui/material/InputLabel';
import MenuItem from '@mui/material/MenuItem';
import FormControl from '@mui/material/FormControl';
import Select, { SelectChangeEvent } from '@mui/material/Select';

import List from "@mui/material/List";
import ListItem from "@mui/material/ListItem";
import ListItemButton from "@mui/material/ListItemButton";
import ListItemIcon from "@mui/material/ListItemIcon";
import SettingsIcon from '@mui/icons-material/Settings';
import ListItemText from "@mui/material/ListItemText";
import Checkbox from "@mui/material/Checkbox";
import IconButton from "@mui/material/IconButton";
import CommentIcon from "@mui/icons-material/Comment";
import Divider from "@mui/material/Divider";

import Fab from "@mui/material/Fab";
import EditIcon from '@mui/icons-material/Edit';

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
    <FormControl sx={{ m: 1 , minWidth: 150}} size="small">
      <InputLabel id="voice-select">声</InputLabel>
      <Select
        labelId="voice-select"
        id="voice-select"
        value={voice}
        label="Voice"
        onChange={onVoiceChange}
        >
        <MenuItem value="Tsukuyomichan">つくよみちゃん</MenuItem>
        <MenuItem value="Oftonp">おふとんP</MenuItem>
        <MenuItem value="Mana">MANA</MenuItem>
        <MenuItem value="Kana">KANA</MenuItem>
        <MenuItem value="Shikokumetan">四国めたん</MenuItem>
        <MenuItem value="Zundamon">ずんだもん</MenuItem>
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
