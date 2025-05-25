#!/usr/bin/env bash

# On Windows, the stderr output of clippy is slightly different to the one on
# Unix systems. This script patches the stderr output to match the one on Unix
# systems. It's designed to be run before tests in the CI pipeline.
#
# Usage: ./scripts/patch-stderrs-windows.bash

find lints/ -type f -name '*.stderr' | while IFS= read -r file; do
  # from "$line" to "##[warning]$line" or "##[error]$line"
  # where "$line" starts with "  --> $DIR/"
  new_content=""
  inside=""

  while IFS= read -r line; do
    new_line="$line"
    if [[ "$line" == warning:\ * ]]; then
      inside="warning"
    fi;

    if [[ "$line" == error:\ * ]]; then
      inside="error"
    fi;

    if [[ "$line" == \ \ --\>\ \$DIR/* ]]; then
      new_line="##[$inside]$line"
    fi;
    new_content+="$new_line"$'\n'
  done < "$file"
  new_content+=$''

  echo -n "$new_content" > "$file"
done

