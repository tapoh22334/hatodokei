import {invoke} from "@tauri-apps/api";
import * as React from "react";
import IconButton from "@mui/material/IconButton";
import PlayArrow from "@mui/icons-material/PlayArrow";
import Typography from '@mui/material/Typography';
import Stack from "@mui/material/Stack";
import Switch from "@mui/material/Switch";
import InputLabel from '@mui/material/InputLabel';
import MenuItem from '@mui/material/MenuItem';
import FormControl from '@mui/material/FormControl';
import Select, {SelectChangeEvent} from '@mui/material/Select';
import VolumeDown from "@mui/icons-material/VolumeDown";
import VolumeUp from "@mui/icons-material/VolumeUp";
import Slider from "@mui/material/Slider";
import TableCell from '@mui/material/TableCell';
import TableRow from '@mui/material/TableRow';

import {voices} from "./Voices";
import {defaultSettings, TTElement, Settings, getTimeTableStorage, setTimeTableStorage} from "./DefaultSetting";
import Checkbox from "@mui/material/Checkbox";

const toTimeString = (hhmm: number) => {
  return (
    ("00" + ((hhmm / 100) >> 0).toString()).slice(-2) +
    ":" +
    ("00" + (hhmm % 100).toString()).slice(-2)
  );
};

export type CardProps = {
  timeTable: TTElement[],
  setTimeTable: React.Dispatch<TTElement[]>
  index: number
}

export const CardTimeSwitch: React.VFC<CardProps> = ({timeTable, setTimeTable, index}) => {
  const time = timeTable[index].time

  React.useEffect(() => {
    const active = timeTable[index].active
    const effect = timeTable[index].effect
    const voice = timeTable[index].voice
    const volume = timeTable[index].volume

    invoke("set_table_row", {
      row: {time: time, active: active, effect: effect, voice: voice, volume: volume},
    });

  }, [timeTable]);

  const handleMuteChildChange = (_: React.ChangeEvent<HTMLInputElement>) => {
    let newTimeTable: TTElement[] = [...timeTable]
    newTimeTable[index].active = !timeTable[index].active
    setTimeTable(newTimeTable)
    setTimeTableStorage(newTimeTable)
  };

  const onEffectChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    let newTimeTable: TTElement[] = [...timeTable]
    newTimeTable[index].effect = event.target.checked as boolean
    setTimeTable(newTimeTable)
    setTimeTableStorage(newTimeTable)
  };

  const onVoiceChange = (event: SelectChangeEvent) => {
    let newTimeTable: TTElement[] = [...timeTable]
    newTimeTable[index].voice = event.target.value as string
    setTimeTable(newTimeTable)
    setTimeTableStorage(newTimeTable)
  };

  const onVolumeChange = (_: Event, value: number | number[]) => {
    let newTimeTable: TTElement[] = [...timeTable]
    newTimeTable[index].volume = value as number
    setTimeTable(newTimeTable)
    setTimeTableStorage(newTimeTable)
  };

  const handlePlayClick = (event: React.MouseEvent<HTMLButtonElement, MouseEvent>) => {
    event.stopPropagation()
    invoke("play", {voice: timeTable[index].voice, index: (time / 100) as number, effect: timeTable[index].effect, volume: timeTable[index].volume});

    console.log("play clicked %d", (time / 100) as number);
  };

  return (
    <TableRow
      sx={{
        backgroundColor: timeTable[index].active ? "white" : "lightgray"
      }}
    >
      <TableCell
        padding='none'
        align='center'
      >
        <Switch
          aria-label={`mute-child-switch-${time}`}
          checked={timeTable[index].active}
          onChange={handleMuteChildChange}
          onClick={(e) => e.stopPropagation()}
        />
      </TableCell>

      <TableCell
        padding="checkbox"
      >
        <IconButton
          aria-label="play"
          onClick={handlePlayClick}
        >
          <PlayArrow />
        </IconButton>
      </TableCell>

      <TableCell
        padding='none'
        align='center'
      >
        <Typography noWrap>
          {toTimeString(time)}
        </Typography>
      </TableCell>

      <TableCell
        padding='none'
        align='center'
      >
        <Checkbox checked={timeTable[index].effect} onChange={onEffectChange} />
      </TableCell>

      <TableCell
        padding='none'
        align='center'
      >
        <FormControl sx={{minWidth: 160, maxWidth: 160}} size="small">
          <InputLabel id="voice-select-small-label"></InputLabel>
          <Select
            sx={{
              '& legend': {display: 'none'},
              '& fieldset': {top: 0},
            }}
            labelId="voice-select"
            id="voice-select"
            value={timeTable[index].voice}
            onChange={onVoiceChange}
            label="Voice"
          >
            {voices.map((option) => (
              <MenuItem key={option.value} value={option.value}>
                {option.label}
              </MenuItem>
            ))}
          </Select>
        </FormControl>
      </TableCell>

      <TableCell
        padding='none'
        align='center'
      >
        <Stack
          direction="row"
          alignItems="center"
          justifyContent="center"
          sx={{margin: 0}}
        >
          <VolumeDown />
          <Slider
            sx={{mx: 2, minWidth: 160, maxWidth: 160}}
            aria-label="volume"
            value={timeTable[index].volume}
            onChange={onVolumeChange}
            valueLabelDisplay="auto"
            step={5}
          />
          <VolumeUp />
        </Stack>
      </TableCell>

    </TableRow>
  );
};

