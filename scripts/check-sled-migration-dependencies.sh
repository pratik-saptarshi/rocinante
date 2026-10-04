#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
application_manifest="$repo_root/src-tauri/Cargo.toml"
migration_manifest="$repo_root/tools/sled-migration/Cargo.toml"

application_tree="$(cargo tree --locked --manifest-path "$application_manifest" --workspace --all-features --edges normal)"
migration_tree="$(cargo tree --locked --manifest-path "$migration_manifest" --edges normal)"

if grep -Eq '(^|[[:space:]])(sled|fxhash|instant) v' <<<"$application_tree"; then
  echo "The application graph must not depend on Sled, fxhash, or instant." >&2
  grep -E '(^|[[:space:]])(sled|fxhash|instant) v' <<<"$application_tree" >&2
  exit 1
fi

if ! grep -Fq "/tools/sled-migration/vendor/sled)" <<<"$migration_tree"; then
  echo "The migration graph must use the locally audited Sled 0.34.7 patch." >&2
  grep -E '(^|[[:space:]])sled v' <<<"$migration_tree" >&2 || true
  exit 1
fi

if grep -Eq '(^|[[:space:]])(fxhash|instant) v' <<<"$migration_tree"; then
  echo "The migration graph must not resolve the fxhash or instant advisories." >&2
  grep -E '(^|[[:space:]])(fxhash|instant) v' <<<"$migration_tree" >&2
  exit 1
fi

echo "Sled migration is isolated to its patched reader graph; fxhash and instant are absent."
