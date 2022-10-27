import * as React from "react";
import LicensesYarn from "../../resource/THIRD-PARTY-NOTICES-yarn.txt";
import LicensesCargo from "../../resource/THIRD-PARTY-NOTICES-cargo.txt";
import Typography from '@mui/material/Typography';
import Box from '@mui/material/Box';

export const Licenses = () => {
    const [textYarn, setTextYarn] = React.useState("");
    const [textCargo, setTextCargo] = React.useState("");

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
                dangerouslySetInnerHTML={{ __html: textCargo }}
            />
            <Box className="LicensesYarn">
                <pre >{textYarn}</pre>
            </Box>
        </Box>
    );
}

export default Licenses;
