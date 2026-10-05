#!/usr/bin/env python3
"""Report invalid or overdue entries in the Rust audit exception registry."""

from __future__ import annotations

import argparse
import json
import re
import sys
import tomllib
from datetime import date
from pathlib import Path


REQUIRED_FIELDS = (
    "id",
    "kind",
    "owner",
    "reason",
    "affected_path",
    "review_by",
    "exit_condition",
    "tracking_id",
)
DATE_PATTERN = re.compile(r"\d{4}-\d{2}-\d{2}")
ADVISORY_ID_PATTERN = re.compile(r"RUSTSEC-\d{4}-\d{4}")


def read_entries(registry_path: Path) -> list[dict[str, object]]:
    payload = json.loads(registry_path.read_text(encoding="utf-8"))
    if not isinstance(payload, list):
        raise ValueError("registry must contain a JSON array")

    ids: set[str] = set()
    tracking_ids: set[str] = set()
    for index, entry in enumerate(payload):
        if not isinstance(entry, dict):
            raise ValueError(f"entry {index} must be an object")
        for field in REQUIRED_FIELDS:
            if not isinstance(entry.get(field), str) or not entry[field].strip():
                raise ValueError(f"entry {index} has missing or empty {field}")
        if entry["kind"] != "audit_ignore":
            raise ValueError(f"entry {index} kind must be audit_ignore")
        if not DATE_PATTERN.fullmatch(entry["review_by"]):
            raise ValueError(f"entry {index} review_by must use YYYY-MM-DD")
        try:
            date.fromisoformat(entry["review_by"])
        except ValueError as error:
            raise ValueError(f"entry {index} review_by is not a real date") from error
        if entry["id"] in ids:
            raise ValueError(f"duplicate advisory id: {entry['id']}")
        if entry["tracking_id"] in tracking_ids:
            raise ValueError(f"duplicate tracking id: {entry['tracking_id']}")
        ids.add(entry["id"])
        tracking_ids.add(entry["tracking_id"])
    return payload


def read_audit_ignore_ids(audit_config_path: Path) -> set[str]:
    try:
        with audit_config_path.open("rb") as source:
            payload = tomllib.load(source)
    except (OSError, tomllib.TOMLDecodeError) as error:
        raise ValueError(f"could not read Cargo audit config {audit_config_path}: {error}") from error

    advisories = payload.get("advisories", {})
    if not isinstance(advisories, dict):
        raise ValueError("Cargo audit config [advisories] must be a table")
    ignored = advisories.get("ignore", [])
    if not isinstance(ignored, list):
        raise ValueError("Cargo audit [advisories].ignore must be an array")

    ids: set[str] = set()
    for index, advisory_id in enumerate(ignored):
        if not isinstance(advisory_id, str) or not ADVISORY_ID_PATTERN.fullmatch(advisory_id):
            raise ValueError(f"Cargo audit ignore {index} must be a RustSec advisory ID")
        if advisory_id in ids:
            raise ValueError(f"duplicate Cargo audit ignore: {advisory_id}")
        ids.add(advisory_id)
    return ids


def validate_registry_matches_audit(entries: list[dict[str, object]], ignored_ids: set[str]) -> None:
    registry_ids = {str(entry["id"]) for entry in entries}
    missing_from_audit = sorted(registry_ids - ignored_ids)
    missing_from_registry = sorted(ignored_ids - registry_ids)
    if not missing_from_audit and not missing_from_registry:
        return

    problems = []
    if missing_from_audit:
        problems.append(
            "registry advisories missing from Cargo audit ignores: "
            + ", ".join(missing_from_audit)
        )
    if missing_from_registry:
        problems.append(
            "Cargo audit ignores missing from the exception registry: "
            + ", ".join(missing_from_registry)
        )
    raise ValueError("; ".join(problems))


def main() -> int:
    repo_root = Path(__file__).resolve().parents[1]
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--registry",
        type=Path,
        default=repo_root / "docs/roadmap/security-advisory-exceptions.json",
    )
    parser.add_argument(
        "--audit-config",
        type=Path,
        default=repo_root / ".cargo/audit.toml",
    )
    parser.add_argument("--as-of", type=date.fromisoformat, default=date.today())
    args = parser.parse_args()

    try:
        entries = read_entries(args.registry)
        ignored_ids = read_audit_ignore_ids(args.audit_config)
        validate_registry_matches_audit(entries, ignored_ids)
    except (OSError, json.JSONDecodeError, ValueError) as error:
        print(f"Invalid advisory exception registry: {error}", file=sys.stderr)
        return 2

    expired = [
        entry for entry in entries if date.fromisoformat(entry["review_by"]) < args.as_of
    ]
    print(
        f"Validated {len(entries)} advisory exception entries against "
        f"{len(ignored_ids)} Cargo audit ignores as of {args.as_of}."
    )
    if not expired:
        print("No review dates are overdue.")
        return 0

    print(f"{len(expired)} exception review dates are overdue:")
    for entry in expired:
        print(f"- {entry['id']} ({entry['tracking_id']}): due {entry['review_by']}")
    print("No risk acceptance or review-date renewal is inferred; release remains blocked.")
    return 1


if __name__ == "__main__":
    raise SystemExit(main())
