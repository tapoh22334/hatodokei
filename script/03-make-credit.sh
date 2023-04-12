#!/bin/bash
cat speakers.txt | cut -d':' -f 1,2 | sed -e 's/50021/VOICEVOX/g;s/50031/COEIROINK/g' | uniq > CREDIT.txt
