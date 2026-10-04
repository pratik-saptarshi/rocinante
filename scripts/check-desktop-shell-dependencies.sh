#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
manifest="$repo_root/src-tauri/crates/rocinante-desktop-shell/Cargo.toml"
target="x86_64-unknown-linux-gnu"

dependency_tree="$(cargo tree \
  --manifest-path "$manifest" \
  --locked \
  --target "$target" \
  --prefix none \
  --format '{p}')"

if grep -En '^(gtk|gtk4|glib|gobject|gio|pango|atk)(-[[:alnum:]_-]+)? v|^(tauri|wry) v' <<< "$dependency_tree"; then
  echo "error: native desktop shell dependency tree contains a forbidden host dependency" >&2
  exit 1
fi

echo "pass: native desktop shell $target dependency tree excludes gtk, glib, tauri, and wry"
