#!/bin/bash
FILE=day_$1
if [ ! -f "$FILE" ]; then
  echo "$FILE does not exists."
  echo "downloading..."
fi;