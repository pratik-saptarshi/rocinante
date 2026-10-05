#!/usr/bin/env bash
set -euo pipefail

force_rust="${1:-false}"
needs_rust=false
run_rust_storage_lanes=false
run_rust_coverage_lanes=false
rust_test_lanes='["core"]'
scope_profile=docs-only-tweak

enable_rust_lanes() {
  needs_rust=true
  run_rust_storage_lanes=true
  run_rust_coverage_lanes=true
  rust_test_lanes='["core","storage"]'
  scope_profile=code-surface-touched
}

while IFS= read -r path; do
  [[ -z "$path" ]] && continue
  case "$path" in
    .github/workflows/ci.yml|src-tauri/*|tools/sled-migration/*|rust-toolchain.toml|.cargo/config.toml|.cargo/config)
      enable_rust_lanes
      break
      ;;
    docs/*|README.md|README.*|CHANGELOG*|*.md|*.txt|*.rst|LICENSE*|SECURITY*|CODE_OF_CONDUCT*|.github/*|ui/*|*.toml|*.yml|*.yaml|*.json|*.lock|*.png|*.jpg|*.jpeg|*.gif|*.svg|*.ico)
      ;;
    *)
      enable_rust_lanes
      break
      ;;
  esac
done

if [[ "$force_rust" == "true" ]]; then
  needs_rust=true
  run_rust_storage_lanes=true
  run_rust_coverage_lanes=true
  rust_test_lanes='["core","storage"]'
  if [[ "$scope_profile" == "docs-only-tweak" ]]; then
    scope_profile=release-docs-tweak
  fi
fi

printf 'needs_rust=%s\n' "$needs_rust"
printf 'scope_profile=%s\n' "$scope_profile"
printf 'run_rust_storage_lanes=%s\n' "$run_rust_storage_lanes"
printf 'run_rust_coverage_lanes=%s\n' "$run_rust_coverage_lanes"
printf 'rust_test_lanes=%s\n' "$rust_test_lanes"
