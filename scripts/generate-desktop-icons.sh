#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
icon_dir="$repo_root/src-tauri/crates/rocinante-desktop-shell/packaging/icons"
module_cache="${TMPDIR:-/tmp}/rocinante-swift-module-cache"

mkdir -p "$icon_dir" "$module_cache"
swift -module-cache-path "$module_cache" \
  "$repo_root/scripts/generate-desktop-icons.swift" "$icon_dir"
sips -s format icns "$icon_dir/rocinante.png" --out "$icon_dir/Rocinante.icns" >/dev/null
