#!/bin/bash

set -e  # If any command fails, stop the script immediately

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

# Build your Rust program
cargo build --manifest-path "$PROJECT_DIR/Cargo.toml"

# Path to the built binary
spin_path="$PROJECT_DIR/target/debug/tspin"

# The binary emits its own do-not-edit header
"$spin_path" --generate-default-theme > "$PROJECT_DIR/default-theme.toml"
