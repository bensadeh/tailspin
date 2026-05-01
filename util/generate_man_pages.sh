#!/bin/bash

set -e  # If any command fails, stop the script immediately

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

full_version=$(cargo run --manifest-path "$PROJECT_DIR/Cargo.toml" -- -V)
version_number=$(echo "$full_version" | awk '{print $2}')

touch "$SCRIPT_DIR/tspin.adoc"

asciidoctor -b manpage "$SCRIPT_DIR/tspin.adoc" \
  --destination="$PROJECT_DIR/man/" \
  --attribute release-version="$version_number"

# Inject a do-not-edit header after the preprocessor hint line.
manpage="$PROJECT_DIR/man/tspin.1"
{
  head -n 1 "$manpage"
  printf '.\\"\n.\\" === DO NOT EDIT — generated from util/tspin.adoc by util/generate_man_pages.sh ===\n.\\"\n'
  tail -n +2 "$manpage"
} > "$manpage.tmp" && mv "$manpage.tmp" "$manpage"