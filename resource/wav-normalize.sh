#!/bin/bash
mkdir ogg_normalized
#for file in find ./wav/* ; do ffmpeg -i $file -ab 128k -ar 44100 -filter:a dynaudnorm ./wav_normalized/$(basename $file .wav).ogg -y ; done
for file in find ./wav/* ; do ffmpeg -i $file -vn -ac 2 -ar 44100 -ab 128k -acodec libvorbis -f ogg ./ogg_normalized/$(basename $file .wav).ogg -y ; done
