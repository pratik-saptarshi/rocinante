#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
temporary_dir="$(mktemp -d)"
trap 'rm -rf "$temporary_dir"' EXIT
rust_compiler="${RUSTC:-rustc}"

compile_and_run() {
  local test_source="$1"
  local test_name="$2"
  CARGO_MANIFEST_DIR="$repo_root/src-tauri" \
    "$rust_compiler" --edition=2021 --test \
    "$repo_root/src-tauri/tests/$test_source.rs" \
    -o "$temporary_dir/$test_name"
  "$temporary_dir/$test_name"
}

compile_and_run gtk_free_host_migration_plan_tests gtk_free_host_migration_plan
compile_and_run publish_gate_docs_tests publish_gate_docs
compile_and_run roadmap_coherence_tests roadmap_coherence
