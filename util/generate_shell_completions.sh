#!/bin/bash

set -e  # If any command fails, stop the script immediately

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

# Build your Rust program
cargo build --manifest-path "$PROJECT_DIR/Cargo.toml"

# Path to the built binary
spin_path="$PROJECT_DIR/target/debug/tspin"

# Generate shell completions
"$spin_path" --generate-zsh-completions > "$PROJECT_DIR/completions/tspin.zsh"
"$spin_path" --generate-bash-completions > "$PROJECT_DIR/completions/tspin.bash"
"$spin_path" --generate-fish-completions > "$PROJECT_DIR/completions/tspin.fish"
