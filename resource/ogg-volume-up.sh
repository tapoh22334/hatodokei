#!/bin/bash
mkdir ogg_volume_uped
for file in find ./ogg/* ; do ffmpeg -i $file -af volume=2 ./ogg_volume_uped/$(basename $file) -y ; done
