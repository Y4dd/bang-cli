#!/bin/bash

QUERY=$(rofi -dmenu -p "Search")
echo $QUERY
URL=$(bang $QUERY)
echo $URL
xdg-open "$URL"
