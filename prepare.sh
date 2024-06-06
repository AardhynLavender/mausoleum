#!/bin/bash

# Remove save data and copy the assets folder to the target directory

target=target/$1;
save_file="user_save.json";

if [ ! -d "$target" ]; then
  echo "Target directory not found";
  exit 1;
fi;

rm -f "$save_file" "$target/$save_file"
cp -r asset "$target";
cp -r data "$target";

exit 0;
