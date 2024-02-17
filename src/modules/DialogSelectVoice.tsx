import * as React from 'react';
import Box from '@mui/material/Box';
import Button from '@mui/material/Button';
import Dialog from '@mui/material/Dialog';
import DialogActions from '@mui/material/DialogActions';
import DialogContent from '@mui/material/DialogContent';
import DialogTitle from '@mui/material/DialogTitle';
import InputLabel from '@mui/material/InputLabel';
import OutlinedInput from '@mui/material/OutlinedInput';
import MenuItem from '@mui/material/MenuItem';
import FormControl from '@mui/material/FormControl';
import Select, {SelectChangeEvent} from '@mui/material/Select';

import {defaultSettings, TTElement, Settings, getTimeTableStorage, setTimeTableStorage} from "./DefaultSetting";
import {voices} from "./Voices";

export type DialogSelectVoiceProps = {
  timeTable: TTElement[],
  setTimeTable: React.Dispatch<TTElement[]>
}

export const DialogSelectVoice: React.VFC<DialogSelectVoiceProps> = ({timeTable, setTimeTable}) => {
  const [open, setOpen] = React.useState(false);
  const [voice, setVoice] = React.useState<string>('');

  const handleChange = (event: SelectChangeEvent<typeof voice>) => {
    setVoice(event.target.value);
  };

  const handleClickOpen = () => {
    setOpen(true);
  };

  const handleClose = (event: React.SyntheticEvent<unknown>, reason?: string) => {
    setOpen(false);
  };

  const handleVoiceAllChange = () => {
    setOpen(false);
    setVoice('');

    if (voice == '') {
      return
    }

    let newTimeTable: TTElement[] = [...timeTable]

    for (const row of newTimeTable) {
      row.voice = voice
    }

    setTimeTable(newTimeTable)
    setTimeTableStorage(newTimeTable);
  };


  return (
    <div>
      <Button onClick={handleClickOpen}>声</Button>
      <Dialog open={open} onClose={handleClose}>
        <DialogTitle>声一括変更</DialogTitle>
        <DialogContent>
          <Box component="form" sx={{display: 'flex', flexWrap: 'wrap'}}>
            <FormControl sx={{m: 1, minWidth: 120}}>
              <InputLabel id="demo-dialog-select-label">声</InputLabel>
              <Select
                labelId="demo-dialog-select-label"
                id="demo-dialog-select"
                value={voice}
                onChange={handleChange}
                input={<OutlinedInput label="voice" />}
              >
                {voices.map((v) => (
                  <MenuItem key={v.value} value={v.value}>
                    {v.label}
                  </MenuItem>
                ))}
              </Select>
            </FormControl>
          </Box>
        </DialogContent>
        <DialogActions>
          <Button onClick={handleVoiceAllChange}>Ok</Button>
        </DialogActions>
      </Dialog>
    </div>
  );
}
