#!/bin/bash

BANG=$(wofi --dmenu --prompt "Bang!")
[ -z "$BANG" ] && exit 0
URL=$(bang $BANG)
xdg-open "$URL"
