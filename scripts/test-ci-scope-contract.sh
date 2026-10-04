#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
classifier="$repo_root/scripts/detect-ci-scope.sh"
workflow="$repo_root/.github/workflows/ci.yml"

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
assert_scope "rust-toolchain.toml" false true code-surface-touched
assert_scope ".cargo/config.toml" false true code-surface-touched
assert_scope ".cargo/config" false true code-surface-touched
assert_scope "docs/roadmap/readiness.md" false false docs-only-tweak
assert_scope "README.md" false false docs-only-tweak
assert_scope "docs/roadmap/readiness.md" true true release-docs-tweak
assert_scope ".github/workflows/ci.yml" false true code-surface-touched

workflow_contents="$(<"$workflow")"
if [[ "$workflow_contents" != *'bash scripts/detect-ci-scope.sh true >> "$GITHUB_OUTPUT"'* ]]; then
  echo "Missing complete conservative scope outputs in the CI fallback path." >&2
  exit 1
fi

test_job_contents="$(awk '
  $0 == "  test:" { in_job = 1; next }
  in_job && /^  [[:alnum:]_-]+:$/ { exit }
  in_job { print }
' "$workflow")"
if [[ "$test_job_contents" != *"- tauri-runtime-bundle"* ]]; then
  echo "The aggregate test job must depend on the Tauri runtime bundle matrix." >&2
  exit 1
fi
if [[ "$test_job_contents" != *'needs.tauri-runtime-bundle.result'* || "$test_job_contents" != *'!= "success"'* ]]; then
  echo "The aggregate test job must fail unless all Tauri bundle matrix legs succeed." >&2
  exit 1
fi

assert_cargo_test_runtime_staging() {
  local job_name="$1"
  local job_body

  job_body="$(awk -v job_name="$job_name" '
    $0 == "  " job_name ":" { in_job = 1; next }
    in_job && /^  [[:alnum:]_-]+:$/ { exit }
    in_job { print }
  ' "$workflow")"

  if [[ "$job_body" != *'python3 scripts/provision_duckdb.py --stage-runtime-for-cargo-tests'* ]]; then
    printf 'Rust test job `%s` does not stage DuckDB in Cargo\x27s debug/deps directory.\n' "$job_name" >&2
    exit 1
  fi
}

for job in rust-workspace-tests rust-quality-gates rust-tests rust-coverage; do
  assert_cargo_test_runtime_staging "$job"
done

echo "Rust CI scope contract passed."
