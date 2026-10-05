#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
if [[ "${1:-}" == "--tree-file" ]]; then
  tree_file="$2"
  if [[ ! -f "$tree_file" ]]; then
    echo "DuckDB feature tree file not found: $tree_file" >&2
    exit 2
  fi
  tree="$(cat "$tree_file")"
else
  tree="$(cargo tree --locked --manifest-path "$repo_root/src-tauri/Cargo.toml" --workspace --all-features --target all -e features -i libduckdb-sys)"
fi

if ! grep -Eq '^libduckdb-sys (v|feature )' <<<"$tree"; then
  echo "libduckdb-sys is missing from the all-target workspace feature graph" >&2
  exit 1
fi

if grep -Eq 'libduckdb-sys feature "(bundled|bundled-cmake|cc)"|duckdb feature "(bundled|bundled-cmake|parquet|json)"' <<<"$tree"; then
  echo "A DuckDB source-build feature is active:" >&2
  grep -E 'libduckdb-sys feature "(bundled|bundled-cmake|cc)"|duckdb feature "(bundled|bundled-cmake|parquet|json)"' <<<"$tree" >&2
  exit 1
fi

echo "DuckDB resolves without source-build features."
