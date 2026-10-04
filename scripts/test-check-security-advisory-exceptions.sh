#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
checker="$repo_root/scripts/check-security-advisory-exceptions.py"
before_deadline="$(python3 "$checker" --as-of 2026-08-06)"
if [[ "$before_deadline" != *"No review dates are overdue."* ]]; then
  echo "review date should remain valid on its due date" >&2
  exit 1
fi

set +e
after_deadline="$(python3 "$checker" --as-of 2026-08-07 2>&1)"
status=$?
set -e
if [[ $status -ne 1 ]]; then
  echo "expired review dates must fail closed (status $status)" >&2
  printf '%s\n' "$after_deadline" >&2
  exit 1
fi
if [[ "$after_deadline" != *"17 exception review dates are overdue:"* ]]; then
  echo "expected all current exceptions to be reported as overdue" >&2
  printf '%s\n' "$after_deadline" >&2
  exit 1
fi
if [[ "$after_deadline" != *"No risk acceptance or review-date renewal is inferred"* ]]; then
  echo "expired exceptions must not imply risk acceptance" >&2
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

def invoke(registry: Path, audit_config: Path) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        [
            sys.executable,
            str(checker),
            "--registry",
            str(registry),
            "--audit-config",
            str(audit_config),
            "--as-of",
            "2026-08-06",
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

    registry.write_text(json.dumps([entry(first)]), encoding="utf-8")
    audit_config.write_text(f'[advisories]\nignore = ["{first}"]\n', encoding="utf-8")
    matched = invoke(registry, audit_config)
    if matched.returncode != 0:
        raise SystemExit(f"matching registry and Cargo ignores should pass: {matched.stderr}")

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
