import { invoke } from "@tauri-apps/api";
import * as React from "react";
import Box from "@mui/material/Box";
import Card from "@mui/material/Card";
import Paper from '@mui/material/Paper';
import IconButton from "@mui/material/IconButton";
import PlayArrow from "@mui/icons-material/PlayArrow";
import Typography from '@mui/material/Typography';
import Stack from "@mui/material/Stack";
import Switch from "@mui/material/Switch";

import { defaultSettings, TTElement, Settings, getTimeTableStorage, setTimeTableStorage} from "./DefaultSetting";

const toTimeString = (hhmm: number) => {
    return (
        ("00" + ((hhmm / 100) >> 0).toString()).slice(-2) +
            ":" +
            ("00" + (hhmm % 100).toString()).slice(-2)
    );
};

const handlePlayClick = (time: number) => {
    invoke("play", {index: (time / 100) as number });

    console.log("play clicked %d", (time / 100) as number);
};

export type CardProps = {
    row: TTElement;
}
export const CardTimeSwitch: React.VFC<CardProps> = (props) => {
    const [time, setTime] = React.useState(props.row.time);
    const [active, setActive] = React.useState(props.row.active);

    React.useEffect(() => {
        invoke("set_table_row", {
            row: {time: time, active: active },
        });

        let newTimeTable = getTimeTableStorage();
        let rewriteIndex = newTimeTable.map((x: TTElement) => x.time).indexOf(time);
        newTimeTable[rewriteIndex].active = active;
        setTimeTableStorage(newTimeTable);
    }, [active]);

    const handleMuteChildChange = () => {
        setActive(!active);
    };

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
            onClick={handlePlayClick.bind(null, time)}
          >
            <PlayArrow />
          </IconButton>
            <Typography noWrap>
              {toTimeString(time)}
            </Typography>
          <Switch
            aria-label={`mute-child-switch-${time}`}
            checked={active}
            onChange={handleMuteChildChange.bind(null, time)}
          />
        </Stack>
      </Card>
    );
  };

