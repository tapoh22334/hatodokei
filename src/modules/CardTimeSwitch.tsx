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
export const CardTimeSwitch: React.VFC<CardProps> = (props) => {
    const [time, setTime] = React.useState(props.row.time);
    const [active, setActive] = React.useState(props.row.active);
    const [effect, setEffect] = React.useState(props.row.effect);
    const [voice, setVoice] = React.useState(props.row.voice);

    console.log("card refresh %o", props);

    React.useEffect(() => {
        invoke("set_table_row", {
            row: {time: time, active: active, effect: effect, voice: voice},
        });

        let newTimeTable = getTimeTableStorage();
        let rewriteIndex = newTimeTable.map((x: TTElement) => x.time).indexOf(time);
        newTimeTable[rewriteIndex].active = active;
        newTimeTable[rewriteIndex].voice = voice;
        newTimeTable[rewriteIndex].effect = effect;
        setTimeTableStorage(newTimeTable);
    }, [active, effect, voice]);

    const handleMuteChildChange = (event: React.ChangeEvent<HTMLInputElement>) => {
        setActive(!active);
    };

    const onEffectChange = (event: React.ChangeEvent<HTMLInputElement>) => {
      setEffect(event.target.checked as boolean);
    };

    const onVoiceChange = (event: SelectChangeEvent) => {
      setVoice(event.target.value as string);
    };

    const handlePlayClick = (event: React.MouseEvent<HTMLButtonElement, MouseEvent>) => {
        event.stopPropagation()
        invoke("play", {voice: voice, index: (time / 100) as number , effect: effect});

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
                        defaultValue={voice}
                        value={voice}
                        onChange={onVoiceChange}
                        label="Voice"
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

                </Stack>
            </AccordionDetails>
        </Accordion>
      </Card>
    );
  };

