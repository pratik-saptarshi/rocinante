#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
classifier="$repo_root/scripts/detect-ci-scope.sh"

assert_scope() {
  local paths="$1"
  local force_rust="$2"
  local expected_rust="$3"
  local expected_profile="$4"
  local output

  output="$(printf '%s\n' "$paths" | bash "$classifier" "$force_rust")"
  if [[ "$output" != *"needs_rust=$expected_rust"* ]]; then
    printf 'Unexpected Rust selection for [%s]:\n%s\n' "$paths" "$output" >&2
    exit 1
  fi
  if [[ "$output" != *"scope_profile=$expected_profile"* ]]; then
    printf 'Unexpected scope profile for [%s]:\n%s\n' "$paths" "$output" >&2
    exit 1
  fi
  if [[ "$expected_rust" == "true" && "$output" != *'rust_test_lanes=["core","storage"]'* ]]; then
    printf 'Rust code changes must select core and storage test lanes:\n%s\n' "$output" >&2
    exit 1
  fi
}

assert_scope "src-tauri/Cargo.toml" false true code-surface-touched
assert_scope "src-tauri/Cargo.lock" false true code-surface-touched
assert_scope "src-tauri/crates/rocinante-storage/Cargo.toml" false true code-surface-touched
assert_scope "docs/roadmap/readiness.md" false false docs-only-tweak
assert_scope "README.md" false false docs-only-tweak
assert_scope "docs/roadmap/readiness.md" true true release-docs-tweak
assert_scope ".github/workflows/ci.yml" false true code-surface-touched

echo "Rust CI scope contract passed."
