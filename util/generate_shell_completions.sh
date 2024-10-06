#!/bin/bash

set -e  # If any command fails, stop the script immediately

# Go to the project directory
cd ..

# Build your Rust program
cargo build

# Path to the built binary
spin_path=./target/debug/tspin

# Generate shell completions
$spin_path --hidden-generate-shell-completions zsh > completions/tspin.zsh
$spin_path --hidden-generate-shell-completions bash > completions/tspin.bash
$spin_path --hidden-generate-shell-completions fish > completions/tspin.fish
