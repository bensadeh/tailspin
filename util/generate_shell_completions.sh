#!/bin/bash

set -e  # If any command fails, stop the script immediately

# Go to the project directory
cd ..

# Build your Rust program
cargo build

# Path to the built binary
spin_path=./target/debug/spin

# Generate shell completions
$spin_path --z-generate-shell-completions zsh > completions/spin.zsh
$spin_path --z-generate-shell-completions bash > completions/spin.bash
$spin_path --z-generate-shell-completions fish > completions/spin.fish
