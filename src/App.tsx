import { useState, useEffect } from "react";
import { emit, listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api";
import "./App.css";
import { defaultSettings, TTElement, Settings } from "./defaultSetting";

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
import PlayArrow from "@mui/icons-material/PlayArrow";
import { styled } from "@mui/material/styles";

import List from "@mui/material/List";
import ListItem from "@mui/material/ListItem";
import ListItemButton from "@mui/material/ListItemButton";
import ListItemIcon from "@mui/material/ListItemIcon";
import ListItemText from "@mui/material/ListItemText";
import Checkbox from "@mui/material/Checkbox";
import IconButton from "@mui/material/IconButton";
import CommentIcon from "@mui/icons-material/Comment";
import Divider from "@mui/material/Divider";

function App() {
  const [count, setCount] = useState(0);
  const [masterVolume, setMasterVolume] = useState(60);
  const [masterMute, setMasterMute] = useState(false);
  const [timeTable, setTimeTable] = useState(Array<TTElement>);

  const getMasterVolumeStorage = () => {
    const json = localStorage.getItem("hatodokeiMasterVolume");
    return json === null ? null : JSON.parse(json);
  };

  const getMasterMuteStorage = () => {
    const json = localStorage.getItem("hatodokeiMasterMute");
    return json === null ? null : JSON.parse(json);
  };

  const getTimeTableStorage = () => {
    const json = localStorage.getItem("hatodokeiTimeTable");
    return json === null ? null : JSON.parse(json);
  };

  const setMasterVolumeStorage = (masterVolume: number) => {
    localStorage.setItem("hatodokeiMasterVolume", JSON.stringify(masterVolume));
  };

  const setMasterMuteStorage = (masterMute: boolean) => {
    localStorage.setItem("hatodokeiMasterMute", JSON.stringify(masterMute));
  };

  const setTimeTableStorage = (timeTable: TTElement[]) => {
    localStorage.setItem("hatodokeiTimeTable", JSON.stringify(timeTable));
  };

  useEffect(() => {
    // Get system default setting from backend.
    let initSettings: Settings = defaultSettings;

    // If any value in local storage, over write default.
    const savedMasterVolume = getMasterVolumeStorage();
    if (savedMasterVolume !== null) {
      initSettings.master_volume = savedMasterVolume;
    }

    const savedMasterMute = getMasterMuteStorage();
    if (savedMasterMute !== null) {
      initSettings.master_mute = savedMasterMute;
    }

    const savedTimeTable = getTimeTableStorage();
    if (savedTimeTable !== null) {
      initSettings.time_table = savedTimeTable;
    }

    // Refrect settings to backend and frontend
    setMasterVolume(initSettings.master_volume);
    invoke("set_master_volume", { volume: initSettings.master_volume });

    setMasterMute(initSettings.master_mute);
    invoke("set_master_mute", { mute: initSettings.master_mute });

    let timeTableChild = [...timeTable];
    for (const i in initSettings.time_table) {
      timeTableChild.push(initSettings.time_table[i]);
      invoke("set_table_row", {
        row: {
          time: initSettings.time_table[i].time,
          active: initSettings.time_table[i].active,
        },
      });
      console.log(initSettings.time_table[i]);
    }
    setTimeTable(timeTableChild);
  }, []);

  const handleVolumeChange = (event: Event, value: number | number[]) => {
    invoke("set_master_volume", { volume: value });
    setMasterVolume(value as number);
    setMasterVolumeStorage(value as number);

    console.log("set master volume: %d", value);
  };

  const handleMuteMasterChange = (
    event: React.ChangeEvent<HTMLInputElement>
  ) => {
    invoke("set_master_mute", { mute: !event.target.checked });
    setMasterMute(!event.target.checked);
    setMasterMuteStorage(!event.target.checked);

    console.log(!event.target.checked);
  };

  const handlePlayClick = (time: number) => {
    invoke("play", { index: (time / 100) as number });

    console.log("play clicked %d", (time / 100) as number);
  };

  const handleMuteChildChange = (time: number) => {
    let timeTableChild = [...timeTable];
    let rowChild = timeTableChild.find((v) => v.time === time);
    if (rowChild !== undefined) {
      rowChild.active = !rowChild.active;
      invoke("set_table_row", {
        row: { time: rowChild.time, active: rowChild.active },
      });

      console.log("time: %d", time);
      console.log(rowChild);
    }

    setTimeTable(timeTableChild);
    setTimeTableStorage(timeTableChild);
  };

  const toTimeString = (hhmm: number) => {
    return (
      ("00" + ((hhmm / 100) >> 0).toString()).slice(-2) +
      ":" +
      ("00" + (hhmm % 100).toString()).slice(-2)
    );
  };

  const CardTimeSwitch: React.VFC<TTElement> = (props) => {
    return (
      <Card variant="outlined">
        <Stack
          spacing={1}
          direction="row"
          sx={{ mx: 1, my: 0 }}
          alignItems="center"
          justifyContent="center"
        >
          <IconButton
            aria-label="play"
            onClick={handlePlayClick.bind(null, props.time)}
          >
            <PlayArrow />
          </IconButton>
          <Box component="span"> {toTimeString(props.time)} </Box>
          <Switch
            aria-label={`mute-child-switch-${props.time}`}
            checked={props.active}
            onChange={handleMuteChildChange.bind(null, props.time)}
            disabled={masterMute}
          />
        </Stack>
      </Card>
    );
  };

  return (
    <div className="App">
      <header className="App-header">
        <AppBar color="primary">
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
                <Switch
                  color="default"
                  aria-label="mute_master"
                  checked={!masterMute}
                  onChange={handleMuteMasterChange}
                />
              </Stack>
            </Box>
          </Toolbar>
        </AppBar>

        <Stack
          sx={{ mt: 8 }}
          spacing={0.3}
          alignItems="center"
          justifyContent="center"
        >
          {timeTable.length > 0 &&
            timeTable.map((row) => {
              const labelId = `stack-list-label-${row["time"]}`;
              return <CardTimeSwitch time={row.time} active={row.active} />;
            })}
        </Stack>
      </header>
    </div>
  );
}

export default App;
