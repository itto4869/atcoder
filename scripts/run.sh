#!/usr/bin/env bash

set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib.sh"

root="$(repo_root)"
src="$(resolve_source "${1:-main.cpp}")"
if [[ $# -gt 0 ]]; then
  shift
fi
bin="$(binary_path_for "$src" "$root")"

build_cpp "$src" "$root" "$bin"
"$bin" "$@"
