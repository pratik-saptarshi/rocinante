#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
checker="$repo_root/scripts/check-duckdb-features.sh"
temp_dir="$(mktemp -d)"
trap 'rm -rf "$temp_dir"' EXIT

cat >"$temp_dir/clean-tree.txt" <<'TREE'
libduckdb-sys v1.10506.0
libduckdb-sys feature "pkg-config"
duckdb feature "default"
TREE
bash "$checker" --tree-file "$temp_dir/clean-tree.txt"

for feature in bundled bundled-cmake cc; do
  cat >"$temp_dir/forbidden-tree.txt" <<TREE
libduckdb-sys feature "$feature"
TREE
  set +e
  output="$(bash "$checker" --tree-file "$temp_dir/forbidden-tree.txt" 2>&1)"
  status=$?
  set -e
  if [[ "$status" -ne 1 || "$output" != *"A DuckDB source-build feature is active"* ]]; then
    printf 'The DuckDB source-build guard accepted feature %s:\n%s\n' "$feature" "$output" >&2
    exit 1
  fi
done

cat >"$temp_dir/missing-tree.txt" <<'TREE'
libduckdb-sys is absent
TREE
set +e
output="$(bash "$checker" --tree-file "$temp_dir/missing-tree.txt" 2>&1)"
status=$?
set -e
if [[ "$status" -ne 1 || "$output" != *"libduckdb-sys is missing"* ]]; then
  printf 'The DuckDB source-build guard accepted a missing crate:\n%s\n' "$output" >&2
  exit 1
fi

echo "DuckDB source-build feature guard contract passed."
