#!/usr/bin/env bash

set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib.sh"

root="$(repo_root)"
src="$(resolve_source "${1:-main.cpp}")"
bin="$(binary_path_for "$src" "$root")"
task_dir="$(dirname "$src")"

build_cpp "$src" "$root" "$bin"
cd "$task_dir"
oj test -d tests -c "$bin"
