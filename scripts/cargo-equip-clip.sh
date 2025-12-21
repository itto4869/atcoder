#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'EOF'
Usage:
  cargo-equip-clip.sh <bin-name> [--manifest-path PATH] [-- extra args...]
  cargo-equip-clip.sh --bin <bin-name> [--manifest-path PATH] [-- extra args...]

Environment:
  EQUIP_MINE   Override --mine (default: github.com/itto4869)

  Notes:
  If src/bin/<bin>.rs exists, resolve the bin name from Cargo.toml.
  Installs a clipboard command automatically: pbcopy, wl-copy, xclip, or xsel.
EOF
}

if [[ "${1:-}" == "-h" || "${1:-}" == "--help" ]]; then
  usage
  exit 0
fi

bin_name=""
manifest_path=""
extra_args=()

while [[ $# -gt 0 ]]; do
  case "$1" in
    -m|--manifest-path)
      manifest_path="${2:-}"
      shift 2
      ;;
    --bin)
      bin_name="${2:-}"
      shift 2
      ;;
    --)
      shift
      extra_args+=("$@")
      break
      ;;
    -*)
      extra_args+=("$1")
      shift
      ;;
    *)
      if [[ -z "$bin_name" ]]; then
        bin_name="$1"
      else
        extra_args+=("$1")
      fi
      shift
      ;;
  esac
done

if [[ -z "$bin_name" ]]; then
  echo "error: bin name is required" >&2
  usage >&2
  exit 1
fi

manifest_dir="."
if [[ -n "$manifest_path" ]]; then
  manifest_dir="$(dirname "$manifest_path")"
fi

if [[ -f "$manifest_dir/src/bin/${bin_name}.rs" ]]; then
  manifest_file="$manifest_path"
  if [[ -z "$manifest_file" ]]; then
    manifest_file="$manifest_dir/Cargo.toml"
  fi

  if [[ ! -f "$manifest_file" ]]; then
    echo "error: Cargo.toml not found at $manifest_file" >&2
    exit 1
  fi

  resolved_bin="$(
    awk -v target="src/bin/${bin_name}.rs" '
      /^\[\[bin\]\]/ { inbin=1; name=""; path=""; next }
      inbin && $0 ~ /^name[[:space:]]*=/ {
        if (match($0, /"[^"]+"/)) {
          name=substr($0, RSTART+1, RLENGTH-2)
        }
        next
      }
      inbin && $0 ~ /^path[[:space:]]*=/ {
        if (match($0, /"[^"]+"/)) {
          path=substr($0, RSTART+1, RLENGTH-2)
        }
        next
      }
      inbin && name != "" && path != "" {
        if (path == target) {
          print name
          exit
        }
      }
    ' "$manifest_file"
  )"

  if [[ -z "$resolved_bin" ]]; then
    echo "error: failed to resolve bin name for src/bin/${bin_name}.rs in $manifest_file" >&2
    exit 1
  fi

  bin_name="$resolved_bin"
fi

if command -v pbcopy >/dev/null 2>&1; then
  clip_cmd=(pbcopy)
elif command -v wl-copy >/dev/null 2>&1; then
  clip_cmd=(wl-copy)
elif command -v xclip >/dev/null 2>&1; then
  clip_cmd=(xclip -selection clipboard)
elif command -v xsel >/dev/null 2>&1; then
  clip_cmd=(xsel --clipboard --input)
else
  echo "error: clipboard command not found (pbcopy, wl-copy, xclip, xsel)" >&2
  exit 1
fi

equip_args=(
  --exclude-atcoder-crates
  --resolve-cfgs
  --remove docs
  --minify libs
  --rustfmt
  --check
  --bin "$bin_name"
  --mine "${EQUIP_MINE:-github.com/itto4869}"
)

if [[ -n "$manifest_path" ]]; then
  equip_args+=(--manifest-path "$manifest_path")
fi

equip_args+=("${extra_args[@]}")

tmpfile="$(mktemp)"
trap 'rm -f "$tmpfile"' EXIT

cargo equip "${equip_args[@]}" | tee "$tmpfile" | "${clip_cmd[@]}" >/dev/null

bytes="$(wc -c <"$tmpfile" | tr -d ' ')"
echo "Copied ${bytes} bytes to clipboard."
