#!/bin/bash

# Source folder name
src_folder="PXYTRP"

# Loop to create 38 copies
for i in {1..2}
do
    # Generate a random 6-letter title
    new_title=$(cat /dev/urandom | LC_CTYPE=C tr -dc 'A-Z' | fold -w 6 | head -n 1)

    # Create a copy of the source folder with the new title
    cp -r "$src_folder" "$new_title"

    # Replace folder name in the text file
    sed -i '' "s/$src_folder/$new_title/g" "$new_title/branch_weekly_sales.txt"
done

echo "Done copying and updating folders."

