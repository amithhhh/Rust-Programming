#!/bin/bash

# Commit message prefix
PREFIX="Auto commit:"

# Loop through all tracked & untracked modified files
git ls-files -m -o --exclude-standard | while read file
do
    echo "Processing $file"

    git add "$file"
    git commit -m "$PREFIX $file"
done

echo "Done committing files individually."
