#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
checker="$repo_root/scripts/check-security-advisory-exceptions.py"
registry="$repo_root/docs/roadmap/security-advisory-exceptions.json"
audit_config="$repo_root/.cargo/audit.toml"

checker_status=0
current="$(python3 "$checker" --as-of 2026-10-05)" || checker_status=$?
if [[ "$checker_status" -ne 0 ]]; then
  echo "the governed registry and audit ignore list must pass" >&2
  printf '%s\n' "$current" >&2
  exit 1
fi
if [[ "$current" =~ Validated[[:space:]]([0-9]+)[[:space:]]advisory[[:space:]]exception[[:space:]]entries[[:space:]]against[[:space:]]([0-9]+)[[:space:]]Cargo[[:space:]]audit[[:space:]]ignores ]]; then
  validated_count="${BASH_REMATCH[1]}"
  ignore_count="${BASH_REMATCH[2]}"
else
  echo "checker output did not report parsed entry/ignore counts" >&2
  printf '%s\n' "$current" >&2
  exit 1
fi
registry_count="$(python3 -c 'import json, sys; print(len(json.load(open(sys.argv[1], encoding="utf-8"))))' "$registry")"
if [[ "$validated_count" != "$registry_count" || "$ignore_count" != "$registry_count" ]]; then
  echo "parsed checker counts must match the registry entry count" >&2
  printf '%s\n' "$current" >&2
  exit 1
fi

python3 - "$checker" <<'PY'
import json
import re
import subprocess
import sys
import tempfile
from pathlib import Path

checker = Path(sys.argv[1])


def validated_counts(stdout: str) -> tuple[int, int] | None:
    match = re.search(
        r"Validated (\d+) advisory exception entries against (\d+) Cargo audit ignores",
        stdout,
    )
    if match is None:
        return None
    return (int(match.group(1)), int(match.group(2)))


def entry(advisory_id: str) -> dict[str, str]:
    return {
        "id": advisory_id,
        "kind": "audit_ignore",
        "owner": "test-owner",
        "crate": "test-crate",
        "reason": "test-only registry fixture for test-crate",
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
    if empty.returncode != 0 or validated_counts(empty.stdout) != (0, 0):
        raise SystemExit("an empty registry and empty ignore list should pass")

    registry.write_text(json.dumps([entry(first)]), encoding="utf-8")
    audit_config.write_text(f'[advisories]\nignore = ["{first}"]\n', encoding="utf-8")
    matched = invoke(registry, audit_config)
    if matched.returncode != 0 or validated_counts(matched.stdout) != (1, 1):
        raise SystemExit(f"matching registry and Cargo ignores should pass: {matched.stderr}")

    expired = invoke(registry, audit_config, "2026-08-07")
    if expired.returncode != 1 or first not in expired.stdout:
        raise SystemExit("an expired exception must fail closed and identify the advisory")

    audit_config.write_text(
        f'[advisories]\nignore = ["{first}", "{second}"]\n', encoding="utf-8"
    )
    cargo_only = invoke(registry, audit_config)
    if cargo_only.returncode != 2:
        raise SystemExit("Cargo-only ignore must fail the governance checker")

    registry.write_text(json.dumps([entry(first), entry(second)]), encoding="utf-8")
    audit_config.write_text(f'[advisories]\nignore = ["{first}"]\n', encoding="utf-8")
    registry_only = invoke(registry, audit_config)
    if registry_only.returncode != 2:
        raise SystemExit("registry-only exception must fail the governance checker")

    audit_config.write_text(f'[advisories]\nignore = ["{first}"]\n', encoding="utf-8")
    bot_owned = entry(first)
    bot_owned["owner"] = "Musebot-Astro"
    registry.write_text(json.dumps([bot_owned]), encoding="utf-8")
    bot_owner = invoke(registry, audit_config)
    if bot_owner.returncode != 2:
        raise SystemExit("a bot-owned exception must fail the governance checker")

    reason_omits_crate = entry(first)
    reason_omits_crate["reason"] = "test-only registry fixture naming no crate"
    registry.write_text(json.dumps([reason_omits_crate]), encoding="utf-8")
    omitted_crate = invoke(registry, audit_config)
    if omitted_crate.returncode != 2:
        raise SystemExit("a reason that omits its crate must fail the governance checker")

    missing_crate = entry(first)
    del missing_crate["crate"]
    registry.write_text(json.dumps([missing_crate]), encoding="utf-8")
    no_crate_field = invoke(registry, audit_config)
    if no_crate_field.returncode != 2:
        raise SystemExit("an entry without a crate field must fail the governance checker")

print("Advisory registry/Cargo ignore parity contract passed.")
PY

echo "Advisory exception governance checker contract passed."
