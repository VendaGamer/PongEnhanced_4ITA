#!/bin/bash

# Output file
OUTPUT="merged.rs"

# Remove old merged file if it exists
rm -f "$OUTPUT"

# Find all .rs files recursively, sort them for consistent order, and merge
find . -type f -name "*.rs" | sort | while read -r file; do
  echo "// --- $file ---" >> "$OUTPUT"
  cat "$file" >> "$OUTPUT"
  echo -e "\n" >> "$OUTPUT"
done

echo "Merged all .rs files into $OUTPUT"
