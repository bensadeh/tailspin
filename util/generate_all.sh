#!/bin/bash

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

"$SCRIPT_DIR/generate_man_pages.sh"
"$SCRIPT_DIR/generate_shell_completions.sh"
