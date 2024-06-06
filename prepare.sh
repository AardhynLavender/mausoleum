#!/bin/bash

# Remove save data and copy the assets folder to the target directory

target=target/$1;

if [ ! -d "$target" ]; then
  echo "Target directory not found";
  exit 1;
fi;

rm -rf "$target"
mkdir -p "$target/asset" "$target/data";
cp -r asset "$target";
cp -r data "$target";

exit 0;
