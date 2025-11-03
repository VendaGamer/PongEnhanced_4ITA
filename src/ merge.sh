#!/bin/bash

# Output file
OUTPUT="merged.rs"


rm -f "$OUTPUT"


find . -type f -name "*.rs" | sort | while read -r file; do
  {
  echo "// --- $file ---"
  cat "$file"
  echo -e "\n"
  } >> $OUTPUT
done

echo "Merged all .rs files into $OUTPUT"
