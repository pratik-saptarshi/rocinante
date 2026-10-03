#!/usr/bin/env python3
"""Report invalid or overdue entries in the Rust audit exception registry."""

from __future__ import annotations

import argparse
import json
import re
import sys
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


def main() -> int:
    repo_root = Path(__file__).resolve().parents[1]
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--registry",
        type=Path,
        default=repo_root / "docs/roadmap/security-advisory-exceptions.json",
    )
    parser.add_argument("--as-of", type=date.fromisoformat, default=date.today())
    args = parser.parse_args()

    try:
        entries = read_entries(args.registry)
    except (OSError, json.JSONDecodeError, ValueError) as error:
        print(f"Invalid advisory exception registry: {error}", file=sys.stderr)
        return 2

    expired = [
        entry for entry in entries if date.fromisoformat(entry["review_by"]) < args.as_of
    ]
    print(f"Validated {len(entries)} advisory exception entries as of {args.as_of}.")
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
