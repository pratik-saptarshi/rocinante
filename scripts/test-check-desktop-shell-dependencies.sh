#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
temporary_root="$(mktemp -d "${TMPDIR:-/tmp}/rocinante-shell-deps-test.XXXXXX")"
trap 'rm -rf "$temporary_root"' EXIT
mkdir -p "$temporary_root/bin"
cat > "$temporary_root/bin/cargo" <<'CARGO'
#!/usr/bin/env bash
set -euo pipefail
[[ " $* " == *" --locked "* ]] || exit 2
[[ " $* " == *" --workspace "* ]] || exit 2
[[ " $* " == *" --all-features "* ]] || exit 2
[[ " $* " == *" --target all "* ]] || exit 2
cat "$CARGO_TREE_FIXTURE"
CARGO
chmod +x "$temporary_root/bin/cargo"

printf 'serde v1.0.0\nrocinante-desktop-shell v0.1.0\n' > "$temporary_root/clean-tree"
CARGO_TREE_FIXTURE="$temporary_root/clean-tree" \
PATH="$temporary_root/bin:$PATH" \
  bash "$repo_root/scripts/check-desktop-shell-dependencies.sh" >/dev/null

printf 'tauri-winrt-notification v0.8.1\n' > "$temporary_root/allowed-platform-helper-tree"
CARGO_TREE_FIXTURE="$temporary_root/allowed-platform-helper-tree" \
PATH="$temporary_root/bin:$PATH" \
  bash "$repo_root/scripts/check-desktop-shell-dependencies.sh" >/dev/null

for dependency in gtk gtk4 gtk3-macros gtk-sys gtk4-sys glib glib-sys gobject-sys gio-sys pango-sys atk-sys atk tauri tauri-build wry proc-macro-error proc-macro-error-attr; do
  printf '%s v1.0.0\n' "$dependency" > "$temporary_root/forbidden-tree"
  if CARGO_TREE_FIXTURE="$temporary_root/forbidden-tree" \
    PATH="$temporary_root/bin:$PATH" \
    bash "$repo_root/scripts/check-desktop-shell-dependencies.sh" \
      >"$temporary_root/stdout" 2>"$temporary_root/stderr"; then
    echo "error: dependency guard accepted forbidden package $dependency" >&2
    exit 1
  fi
  if ! grep -F "$dependency v1.0.0" "$temporary_root/stdout" >/dev/null; then
    echo "error: dependency guard did not report forbidden package $dependency" >&2
    cat "$temporary_root/stdout" >&2
    exit 1
  fi
done

echo "pass: workspace dependency guard accepts clean trees and rejects retired GTK/Tauri dependencies"
