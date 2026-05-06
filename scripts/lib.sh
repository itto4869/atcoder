#!/usr/bin/env bash

set -euo pipefail

repo_root() {
  git rev-parse --show-toplevel 2>/dev/null || pwd
}

resolve_source() {
  local input="${1:-main.cpp}"

  if [[ -d "$input" ]]; then
    input="$input/main.cpp"
  fi

  if [[ ! -f "$input" ]]; then
    echo "source file not found: $input" >&2
    echo "usage: $0 [source.cpp|task-dir]" >&2
    exit 2
  fi

  realpath "$input"
}

binary_path_for() {
  local src="$1"
  local root="$2"
  local rel
  rel="$(realpath --relative-to="$root" "$src")"

  local dir
  local base
  local stem
  dir="$(dirname "$rel")"
  base="$(basename "$src")"
  stem="${base%.*}"

  printf '%s/target/cpp/%s/%s\n' "$root" "$dir" "$stem"
}

build_cpp() {
  local src="$1"
  local root="$2"
  local bin="$3"

  mkdir -p "$(dirname "$bin")"
  g++ -std=gnu++20 -O2 -Wall -Wextra -Wshadow -Wconversion -DLOCAL \
    -I "$root/lib/ac-library" \
    "$src" -o "$bin"
}
