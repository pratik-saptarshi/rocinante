#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

printf '%s\n' "cargo tree --manifest-path src-tauri/Cargo.toml -i glib --locked --target all"
cargo tree --manifest-path "$repo_root/src-tauri/Cargo.toml" -i glib --locked --target all
printf '\n%s\n' "cargo tree --manifest-path src-tauri/Cargo.toml -i gtk --locked --target all"
cargo tree --manifest-path "$repo_root/src-tauri/Cargo.toml" -i gtk --locked --target all
