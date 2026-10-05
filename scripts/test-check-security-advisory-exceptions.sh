#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
checker="$repo_root/scripts/check-security-advisory-exceptions.py"
registry="$repo_root/docs/roadmap/security-advisory-exceptions.json"
audit_config="$repo_root/.cargo/audit.toml"

current="$(python3 "$checker" --as-of 2026-10-05)"
if [[ "$current" != *"Validated 0 advisory exception entries against 0 Cargo audit ignores"* || \
  "$current" != *"No review dates are overdue."* ]]; then
  echo "the empty registry and empty audit ignore list must pass" >&2
  printf '%s\n' "$current" >&2
  exit 1
fi

python3 - "$checker" <<'PY'
import json
import subprocess
import sys
import tempfile
from pathlib import Path

checker = Path(sys.argv[1])


def entry(advisory_id: str) -> dict[str, str]:
    return {
        "id": advisory_id,
        "kind": "audit_ignore",
        "owner": "test-owner",
        "reason": "test-only registry fixture",
        "affected_path": "Cargo.lock",
        "review_by": "2026-08-06",
        "exit_condition": "remove the test fixture",
        "tracking_id": f"TEST-{advisory_id}",
    }


def invoke(registry: Path, audit_config: Path, as_of: str = "2026-08-06") -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        [
            sys.executable,
            str(checker),
            "--registry",
            str(registry),
            "--audit-config",
            str(audit_config),
            "--as-of",
            as_of,
        ],
        capture_output=True,
        text=True,
        check=False,
    )


first = "RUSTSEC-2026-0001"
second = "RUSTSEC-2026-0002"
with tempfile.TemporaryDirectory() as directory:
    root = Path(directory)
    registry = root / "registry.json"
    audit_config = root / "audit.toml"

    registry.write_text("[]", encoding="utf-8")
    audit_config.write_text("[advisories]\nignore = []\n", encoding="utf-8")
    empty = invoke(registry, audit_config)
    if empty.returncode != 0 or "Validated 0 advisory exception entries" not in empty.stdout:
        raise SystemExit("an empty registry and empty ignore list should pass")

    registry.write_text(json.dumps([entry(first)]), encoding="utf-8")
    audit_config.write_text(f'[advisories]\nignore = ["{first}"]\n', encoding="utf-8")
    matched = invoke(registry, audit_config)
    if matched.returncode != 0:
        raise SystemExit(f"matching registry and Cargo ignores should pass: {matched.stderr}")

    expired = invoke(registry, audit_config, "2026-08-07")
    if expired.returncode != 1 or first not in expired.stdout:
        raise SystemExit("an expired exception must fail closed and identify the advisory")

    audit_config.write_text(
        f'[advisories]\nignore = ["{first}", "{second}"]\n', encoding="utf-8"
    )
    cargo_only = invoke(registry, audit_config)
    if cargo_only.returncode != 2 or f"missing from the exception registry: {second}" not in cargo_only.stderr:
        raise SystemExit("Cargo-only ignore must fail the governance checker")

    registry.write_text(json.dumps([entry(first), entry(second)]), encoding="utf-8")
    audit_config.write_text(f'[advisories]\nignore = ["{first}"]\n', encoding="utf-8")
    registry_only = invoke(registry, audit_config)
    if registry_only.returncode != 2 or f"missing from Cargo audit ignores: {second}" not in registry_only.stderr:
        raise SystemExit("registry-only exception must fail the governance checker")

print("Advisory registry/Cargo ignore parity contract passed.")
PY

echo "Advisory exception governance checker contract passed."
