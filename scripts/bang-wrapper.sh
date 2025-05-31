#!/bin/bash
## This wraps around wofi executing both drun and dmenu
## It'll redirect to the bang operator if finds a bang as the first letter
## Otherwise it'll exec the .desktop entry
## This assumes bang exits in your $PATH, usually $HOME/.cargo/bin/bang
## if installed through cargo

ENTRY=$(wofi -S drun,dmenu -p "Run or !bang")
[ -z "$ENTRY" ] && exit 0

# If entry starts with a bang (!tag query)
if [[ "$ENTRY" == !* ]]; then
  URL=$(bang $ENTRY)
  [ -n "$URL" ] && xdg-open "$URL"
  exit 0
fi
# Otherwise, run the selected app using gtk-launch or plain exec
gtk-launch "$ENTRY" 2>/dev/null || exec "$ENTRY"
