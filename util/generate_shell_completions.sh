#!/bin/bash

set -e  # If any command fails, stop the script immediately

# Go to the project directory
cd ..

# Build your Rust program
cargo build

# Path to the built binary
spin_path=./target/debug/tspin

# Generate shell completions
$spin_path --generate-zsh-completions > completions/tspin.zsh
$spin_path --generate-bash-completions > completions/tspin.bash
$spin_path --generate-fish-completions > completions/tspin.fish
