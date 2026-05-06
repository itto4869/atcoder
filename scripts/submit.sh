#!/usr/bin/env bash

set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib.sh"

src="$(resolve_source "${1:-main.cpp}")"
task_dir="$(dirname "$src")"
file_name="$(basename "$src")"

cd "$task_dir"
acc submit "$file_name"
