import { useState, useEffect } from "react";
import { emit, listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api";
import "./App.css";
import {
    defaultSettings,
    TTElement,
    Settings,
    getMasterMuteStorage,
    getMasterVolumeStorage,
    getTimeTableStorage,
    setMasterMuteStorage,
    setMasterVolumeStorage,
    setTimeTableStorage
    } from "./modules/DefaultSetting";
import { CardTimeSwitch } from "./modules/CardTimeSwitch";

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

  const [masterVolume, setMasterVolume] = useState(() => {
    let initMasterVolume = getMasterVolumeStorage();
    if (initMasterVolume === null) {
      initMasterVolume = defaultSettings.master_volume;
      console.log("Not exist stored master volume");
    }
    console.log("initial master volume: %d", initMasterVolume);
    return initMasterVolume;
  });

  const [masterMute, setMasterMute] = useState<boolean>(() => {
    let initMasterMute = getMasterMuteStorage();
    if (initMasterMute === null) {
      initMasterMute = defaultSettings.master_mute;
    }
    return initMasterMute;
  });

  const [timeTable, setTimeTable] = useState<Array<TTElement>>(() => {
      let initTimeTable = getTimeTableStorage();
      if (initTimeTable === null) {
          initTimeTable = defaultSettings.time_table
      }
      return initTimeTable;
  });

  //useEffect(() => {
  //  invoke("set_master_volume", { volume: masterVolume });
  //  invoke("set_master_mute", { mute: masterMute});
  //}, []);

  useEffect(() => {
    invoke("set_master_volume", { volume: masterVolume });
    setMasterVolumeStorage(masterVolume);
    console.log("set master volume: %d", masterVolume);
  }, [masterVolume]);

  useEffect(() => {
    invoke("set_master_mute", { mute: masterMute });
    setMasterMuteStorage(masterMute);
    console.log(masterMute);
  }, [masterMute]);

  const handleVolumeChange = (event: Event, value: number | number[]) => {
    setMasterVolume(value as number);
  };

  const handleMuteMasterChange = (
    event: React.ChangeEvent<HTMLInputElement>
  ) => {
    setMasterMute(!event.target.checked);
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
              </Stack>
            </Box>
          </Toolbar>
        </AppBar>

        <Stack
          sx={{ mt: 8, mb: 8 }}
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
