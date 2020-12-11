#!/bin/bash
YEAR=2020
FILE=day_$1
if [ ! -f "$FILE" ]; then
  echo "$FILE does not exists."
  echo "downloading..."
  SESSION=`cat .session-cookie`
  curl -o $FILE --cookie "session=$SESSION" https://adventofcode.com/$YEAR/day/$1/input
fi;