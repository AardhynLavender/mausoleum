#!/bin/bash

# Remove save data and copy the assets folder to the target directory

target=target/$1;

if [ ! -d "$target" ]; then
  echo "Target directory not found";
  exit 1;
fi;

rm -f "$target/user_save.json";
cp -r asset "$target";
exit 0;
