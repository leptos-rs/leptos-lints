#!/usr/bin/env bash

# On Windows, the stderr output of clippy is slightly different to the one on
# Unix systems. This script patches stderr outputs to match the ones on Unix
# systems. It's designed to be run before tests in the CI pipeline.
#
# Usage: ./scripts/patch-windows-tests.sh

find lints/ -type f -name '*.stderr' | while IFS= read -r file; do
  # from "$line" to "##[warning]$line\n$line" or "##[error]$line\n$line"
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

    if [[ "$line" == \ \ --\>\ \$DIR/* ]] && [ -n "$inside" ] ; then
      new_content+="##[$inside]$line"$'\n'
      inside=""
    else
      new_content+="$line"$'\n'
    fi;
  done < "$file"
  new_content+=$''  # additional newline at the end

  echo -n "$new_content" > "$file"
done

