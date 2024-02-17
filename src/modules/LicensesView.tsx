import * as React from "react";
import Readme from "../../resource/README.txt";
import Credit from "../../resource/CREDIT.txt";
import MIT from "../../resource/MIT.txt";
import LicensesYarn from "../../resource/THIRD-PARTY-NOTICES-yarn.txt";
import LicensesCargo from "../../resource/THIRD-PARTY-NOTICES-cargo.txt";
import Box from '@mui/material/Box';

export const Licenses = () => {
  const [textReadme, setTextReadme] = React.useState("");
  const [textCredit, setTextCredit] = React.useState("");
  const [textMIT, setTextMIT] = React.useState("");
  const [textYarn, setTextYarn] = React.useState("");
  const [textCargo, setTextCargo] = React.useState("");

  fetch(Readme)
    .then((response) => response.text())
    .then((textContent) => {
      setTextReadme(textContent);
    });

  fetch(Credit)
    .then((response) => response.text())
    .then((textContent) => {
      setTextCredit(textContent);
    });

  fetch(MIT)
    .then((response) => response.text())
    .then((textContent) => {
      setTextMIT(textContent);
    });

  fetch(LicensesYarn)
    .then((response) => response.text())
    .then((textContent) => {
      setTextYarn(textContent);
    });

  fetch(LicensesCargo)
    .then((response) => response.text())
    .then((textContent) => {
      setTextCargo(textContent);
    });

  return (

    <Box>
      <div
        dangerouslySetInnerHTML={{__html: textReadme}}
      />
      <div>
        <h2>使用している音声</h2>
        <pre >{textCredit}</pre>
      </div>
      <div
        dangerouslySetInnerHTML={{__html: textMIT}}
      />
      <div
        dangerouslySetInnerHTML={{__html: textCargo}}
      />
      <Box className="LicensesYarn">
        <pre >{textYarn}</pre>
      </Box>
    </Box>
  );
}

export default Licenses;
