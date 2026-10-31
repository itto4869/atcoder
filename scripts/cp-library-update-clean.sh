#!/usr/bin/env bash
set -euo pipefail

root_dir="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root_dir"

# Discover only the genre crates used by the current workspace.
# Capture the pipeline first so metadata errors are not mistaken for no dependencies.
packages="$(
    cargo metadata --no-deps --format-version 1 |
        python3 -c '
import json, sys
data = json.load(sys.stdin)
members = set(data["workspace_members"])
known = {"cp_algorithm", "cp_data_structure", "cp_graph", "cp_grid", "cp_math", "cp_utils"}
used = {dep["name"] for pkg in data["packages"] if pkg["id"] in members
        for dep in pkg["dependencies"] if dep["name"] in known}
print("\n".join(sorted(used)))
'
)"
if [[ -z "$packages" ]]; then
    echo "現在の workspace に cp_* クレートの依存はありません。"
    exit 0
fi

update_args=()
clean_args=()
while IFS= read -r package; do
    update_args+=(-p "$package")
    clean_args+=(-p "$package")
done <<< "$packages"
cargo update "${update_args[@]}"
cargo clean "${clean_args[@]}"
