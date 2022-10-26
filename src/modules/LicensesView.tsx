import * as React from "react";
import licenses from "../../resource/THIRD-PARTY-NOTICES.txt"
import Typography from '@mui/material/Typography';
import Box from '@mui/material/Box';

export const Licenses = () => {
    const [text, setText] = React.useState("");

    fetch(licenses)
        .then((response) => response.text())
        .then((textContent) => {
                setText(textContent);
                });

    return (
        <Box className="Licenses">
            <pre >{text}</pre>
        </Box>
    );
}

export default Licenses;
