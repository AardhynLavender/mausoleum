#!/bin/bash

# Remove save data and copy the assets folder to the target directory


if [ "$1" != "debug" ] && [ "$1" != "release" ]; then
  echo "Invalid target";
  exit 1;
fi;

target=target/$1;
if [ ! -d "$target" ]; then
  echo "Target directory not found";
  exit 1;
fi;

rm -rf "$target"

asset_dir=$target/asset;
data_dir=$target/data;
dev_save=$data_dir/dev_save.json;

mkdir -p "$asset_dir" "$data_dir";
cp -r asset "$target";
cp -r data "$target";

[ "$1" == "release" ] && rm -f "$dev_save"

exit 0;
