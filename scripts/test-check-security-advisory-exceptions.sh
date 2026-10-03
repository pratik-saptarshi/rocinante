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
echo "Advisory exception governance checker contract passed."
