import { invoke } from "@tauri-apps/api";
import * as React from "react";
import Card from "@mui/material/Card";
import IconButton from "@mui/material/IconButton";
import PlayArrow from "@mui/icons-material/PlayArrow";
import Typography from '@mui/material/Typography';
import Stack from "@mui/material/Stack";
import Switch from "@mui/material/Switch";
import Accordion from '@mui/material/Accordion';
import AccordionSummary from '@mui/material/AccordionSummary';
import AccordionDetails from '@mui/material/AccordionDetails';
import ExpandMoreIcon from '@mui/icons-material/ExpandMore';
import InputLabel from '@mui/material/InputLabel';
import MenuItem from '@mui/material/MenuItem';
import FormControl from '@mui/material/FormControl';
import FormControlLabel from '@mui/material/FormControlLabel';
import Select, { SelectChangeEvent } from '@mui/material/Select';
import VolumeDown from "@mui/icons-material/VolumeDown";
import VolumeUp from "@mui/icons-material/VolumeUp";
import Slider from "@mui/material/Slider";

import { defaultSettings, TTElement, Settings, getTimeTableStorage, setTimeTableStorage} from "./DefaultSetting";

const toTimeString = (hhmm: number) => {
    return (
        ("00" + ((hhmm / 100) >> 0).toString()).slice(-2) +
            ":" +
            ("00" + (hhmm % 100).toString()).slice(-2)
    );
};

export type CardProps = {
    row: TTElement;
}

const voices = [
  { value: "つくよみちゃん-れいせい", label: 'つくよみちゃん' },
  { value: "MANA-のーまる", label: 'MANA' },
  { value: "おふとんP-のーまるv2", label: 'おふとんP' },
  { value: "ディアちゃん-のーまる", label: 'ディアちゃん' },
  { value: "アルマちゃん-表-v2", label: 'アルマちゃん' },
  { value: "KANA-のーまる", label: 'KANA' },
  { value: "MANA+-ないしょばなし", label: 'MANA+-ないしょばなし' },
  { value: "AI声優-朱花-のーまる", label: 'AI声優-朱花' },
  { value: "AI声優-青葉-のーまる", label: 'AI声優-青葉' },
  { value: "AI声優-銀芽-のーまる", label: 'AI声優-銀芽' },
  { value: "伊能いお-ふつう", label: '伊能いお' },
  { value: "あみたろ-のーまるv4", label: 'あみたろ' },
  { value: "お星-テンション↑↑", label: 'お星' },
  { value: "四国めたん-ノーマル", label: '四国めたん' },
  { value: "ずんだもん-ノーマル", label: 'ずんだもん' },
  { value: "ずんだもん-ヒソヒソ", label: 'ずんだもん-ヒソヒソ' },
  { value: "春日部つむぎ-ノーマル", label: '春日部つむぎ' },
  { value: "雨晴はう-ノーマル", label: '雨晴はう' },
  { value: "波音リツ-ノーマル", label: '波音リツ' },
  { value: "玄野武宏-ノーマル", label: '玄野武宏' },
  { value: "白上虎太郎-ふつう", label: '白上虎太郎' },
  { value: "青山龍星-ノーマル", label: '青山龍星' },
  { value: "冥鳴ひまり-ノーマル", label: '冥鳴ひまり' },
  { value: "九州そら-ノーマル", label: '九州そら' },
  { value: "剣崎雌雄-ノーマル", label: '剣崎雌雄' },
  { value: "WhiteCUL-ノーマル", label: 'WhiteCUL' },
  { value: "後鬼-人間ver.", label: '後鬼-人間ver.' },
  { value: "ちび式じい-ノーマル", label: 'ちび式じい' },
  { value: "櫻歌ミコ-ノーマル", label: '櫻歌ミコ' },
  { value: "小夜/SAYO-ノーマル", label: '小夜/SAYO' },
  { value: "ナースロボ＿タイプＴ-ノーマル", label: 'ナースロボ＿タイプＴ' },
  { value: "†聖騎士 紅桜†-ノーマル", label: '†聖騎士 紅桜†' },
  { value: "雀松朱司-ノーマル", label: '雀松朱司' },
  { value: "麒ヶ島宗麟-ノーマル", label: '麒ヶ島宗麟' },
  { value: "春歌ナナ-ノーマル", label: '春歌ナナ' },
  { value: "猫使アル-ノーマル", label: '猫使アル' },
  { value: "猫使ビィ-ノーマル", label: '猫使ビィ' },
  { value: "#ランダム", label: '#ランダム' },
]

export const CardTimeSwitch: React.VFC<CardProps> = (props) => {
    const [time, setTime] = React.useState(props.row.time);
    const [active, setActive] = React.useState(props.row.active);
    const [effect, setEffect] = React.useState(props.row.effect);
    const [voice, setVoice] = React.useState(props.row.voice);
    const [volume, setVolume] = React.useState(props.row.volume);

    console.log("card refresh %o", props);

    React.useEffect(() => {
        invoke("set_table_row", {
          row: {time: time, active: active, effect: effect, voice: voice, volume: volume},
        });

        let newTimeTable = getTimeTableStorage();
        let rewriteIndex = newTimeTable.map((x: TTElement) => x.time).indexOf(time);
        newTimeTable[rewriteIndex].active = active;
        newTimeTable[rewriteIndex].voice = voice;
        newTimeTable[rewriteIndex].effect = effect;
        newTimeTable[rewriteIndex].volume = volume;
        setTimeTableStorage(newTimeTable);
    }, [active, effect, voice, volume]);

    const handleMuteChildChange = (_: React.ChangeEvent<HTMLInputElement>) => {
        setActive(!active);
    };

    const onEffectChange = (event: React.ChangeEvent<HTMLInputElement>) => {
      setEffect(event.target.checked as boolean);
    };

    const onVoiceChange = (event: SelectChangeEvent) => {
      setVoice(event.target.value as string);
    };

    const onVolumeChange = (_: Event, value: number | number[]) => {
      setVolume(value as number);
    };

    const handlePlayClick = (event: React.MouseEvent<HTMLButtonElement, MouseEvent>) => {
        event.stopPropagation()
        invoke("play", {voice: voice, index: (time / 100) as number , effect: effect, volume: volume});

        console.log("play clicked %d", (time / 100) as number);
    };

    return (
      <Card variant="outlined">
        <Accordion
        >
            <AccordionSummary
              expandIcon={<ExpandMoreIcon />}
            >
                <Stack
                  sx={{width: '100%'}}
                  spacing={2}
                  direction="row"
                  alignItems="center"
                  justifyContent="center"
                >
                  <IconButton
                    aria-label="play"
                    onClick={ handlePlayClick }
                  >
                    <PlayArrow />
                  </IconButton>

                  <Typography noWrap>
                    {toTimeString(time)}
                  </Typography>

                  <Switch
                    aria-label={`mute-child-switch-${time}`}
                    checked={active}
                    onChange={handleMuteChildChange}
                    onClick={(e) => e.stopPropagation()}
                  />
                </Stack>
            </AccordionSummary>
            <AccordionDetails>
              <Stack
                direction="row"
                sx={{ mb: 1, mx: 1 }}
                alignItems="center"
                justifyContent="center"
              >
                <VolumeDown />
                <Slider
                  //sx={{ color: "#fff" }}
                  aria-label="volume"
                  value={volume}
                  onChange={onVolumeChange}
                />
                <VolumeUp />
              </Stack>
                <Stack
                  direction="row"
                  alignItems="center"
                  justifyContent="center"
                >

                    <FormControlLabel
                      value="top"
                      control={<Switch color="primary" checked={effect} onChange={onEffectChange}/>}
                      label="効果音"
                      labelPlacement="top"
                    />
                    <FormControl sx={{ minWidth: 160 , maxWidth: 160}} size="small">
                      <InputLabel id="voice-select">声</InputLabel>
                      <Select
                        labelId="voice-select"
                        id="voice-select"
                        value={voice}
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

                </Stack>
            </AccordionDetails>
        </Accordion>
      </Card>
    );
  };

