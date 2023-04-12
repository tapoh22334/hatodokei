#!/bin/bash

TEMPLATE=$(cat << EOS
{% for speaker in speakers %}
<MenuItem value="{{speaker.name}}-{{speaker.style}}">{{speaker.name}}-{{speaker.style}}</MenuItem> {% endfor %}
EOS
)

j2 <(echo "$TEMPLATE") speakers.json > items-fragment.js

