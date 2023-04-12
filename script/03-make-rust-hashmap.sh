#!/bin/bash
function main {
    FILE=$1

    echo -n '{"speakers": ['

    awk -F ':' '
    {
        printf("{\"port\":\""$1"\",\"name\":\""$2"\",\"type\":\""$3"\",\"id\":\""$4"\"}, ")
    }
    END {
        printf("{\"port\":\""$1"\",\"name\":\""$2"\",\"type\":\""$3"\",\"id\":\""$4"\"}")
    }
    ' < "$FILE"

    echo "]}"
}

main speakers.txt > speakers.json
j2 ./preset_voice.rs.j2 speakers.json > preset_voice.rs

