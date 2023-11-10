#!/bin/bash

fd  . | tree --fromfile
echo -e "\n---\n"
# List all files (excluding directories) while respecting .gitignore
fd --type f --exclude '*.sh' --exclude '*.txt' --exclude '*.lock' --exclude 'target' . | while read -r file; do
    # Display the file's path
    echo "Path: $file:"
    echo -e "\n---\n"
    # Display the contents of the file
    cat "$file"

    # Optionally, print a separator for readability
    echo -e "\n---\n"
done
