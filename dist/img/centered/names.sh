#!/bin/bash

# Initialize an empty array to store unique names
unique_names=()

# Loop through each file in the current directory
for file in *.jpg; do
  # Extract the base name without the number and extension
  base_name=$(echo "$file" | sed -E 's/_[0-9]+\.jpg//')
  
  # Check if the base name is already in the array
  if [[ ! " ${unique_names[@]} " =~ " ${base_name} " ]]; then
    # If not, add it to the array
    unique_names+=("$base_name")
  fi
done

# Print out the unique base names
for name in "${unique_names[@]}"; do
  echo "$name"
done
