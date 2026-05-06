#!/usr/bin/env bash

set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib.sh"

yes_arg=()
if [[ "${1:-}" == "-y" || "${1:-}" == "--yes" ]]; then
  yes_arg=("$1")
  shift
fi

dry_run_arg=()
if [[ "${1:-}" == "--dry-run" ]]; then
  dry_run_arg=("$1")
  shift
fi

src="$(resolve_source "${1:-main.cpp}")"
task_dir="$(dirname "$src")"

cd "$task_dir"
python3 "$script_dir/atcoder_submit.py" "${yes_arg[@]}" "${dry_run_arg[@]}" "$src"
