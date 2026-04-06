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