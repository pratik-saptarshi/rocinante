#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
manifest="$repo_root/src-tauri/Cargo.toml"

dependency_tree="$(cargo tree \
  --manifest-path "$manifest" \
  --locked \
  --workspace \
  --all-features \
  --target all \
  --prefix none \
  --format '{p}')"

if grep -En '^(gtk|gtk4|gtk3-macros|gtk-sys|gtk4-sys|glib|glib-sys|gobject-sys|gobject|gio-sys|gio|pango-sys|pango|atk-sys|atk|gdk-sys|gdkwayland-sys|gdk|webkit2gtk|tauri|tauri-build|wry|proc-macro-error|proc-macro-error-attr|libproc-macro-error-attr) v' <<< "$dependency_tree"; then
  echo "error: supported desktop workspace contains a retired GTK/Tauri dependency" >&2
  exit 1
fi

echo "pass: all supported desktop workspace targets exclude GTK, GLib, Wry, Tauri, and proc-macro-error"
