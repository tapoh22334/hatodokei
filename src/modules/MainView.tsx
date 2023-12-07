import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api";
import {
    defaultSettings,
    TTElement,
    getMasterVolumeStorage,
    getTimeTableStorage,
    setMasterVolumeStorage,
    setTimeTableStorage
    } from "./DefaultSetting";
import { CardTimeSwitch } from "./CardTimeSwitch";

import * as React from "react";
import AppBar from "@mui/material/AppBar";
import Toolbar from "@mui/material/Toolbar";
import Box from "@mui/material/Box";
import Stack from "@mui/material/Stack";
import Slider from "@mui/material/Slider";
import VolumeDown from "@mui/icons-material/VolumeDown";
import VolumeUp from "@mui/icons-material/VolumeUp";
import Table from '@mui/material/Table';
import TableBody from '@mui/material/TableBody';
import TableCell from '@mui/material/TableCell';
import TableContainer from '@mui/material/TableContainer';
import TableHead from '@mui/material/TableHead';
import TableRow from '@mui/material/TableRow';
import Paper from '@mui/material/Paper';

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
    console.log("TimeTable Changed!! %o", timeTable);
  }, [timeTable]);

  const toolbarStyle = {
    minHeight: 48
  };

  return (
    <React.Fragment>
    <AppBar style={toolbarStyle} sx={{overflow: 'hidden'}}>
      <Toolbar style={toolbarStyle}>
        <Box sx={{ width: 340 }} alignItems="center" position="sticky">
          <Stack
            spacing={0}
            direction="row"
            sx={{ my: 0 }}
            alignItems="center"
            justifyContent="center"
          >
            <VolumeDown />
            <Slider
              sx={{ mx: 2, color: "#fff" }}
              aria-label="MasterVolume"
              value={masterVolume}
              onChange={handleVolumeChange}
            />
            <VolumeUp />
          </Stack>
        </Box>
      </Toolbar>

    </AppBar>

    <Box sx={{ mt: 6, overflow: 'hidden', overflowY: 'scroll', height: 420}}>
      <TableContainer sx={{ pb: 1, px: 1 }} component={Paper}>
        <Table aria-label="table">
          <TableHead>
            <TableRow>
              <TableCell align='center'></TableCell>
              <TableCell align='center'></TableCell>
              <TableCell align='center'>時間</TableCell>
              <TableCell align='center'>効果音</TableCell>
              <TableCell align='center'>声</TableCell>
              <TableCell align='center'>音量</TableCell>
            </TableRow>
          </TableHead>
          <TableBody>
            {timeTable.length > 0 &&
              timeTable.map((row) => {
              const labelId = `stack-list-label-${row["time"]}`;
              return <CardTimeSwitch row={row}/>;
            })}
          </TableBody>
        </Table>
      </TableContainer>
    </Box>

    </React.Fragment>
  );

}
