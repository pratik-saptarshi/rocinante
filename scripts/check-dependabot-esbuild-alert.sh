#!/usr/bin/env bash
set -euo pipefail

REPO="${1:-pratik-saptarshi/rocinante}"
ADVISORY="${2:-GHSA-g7r4-m6w7-qqqr}"
STATE="${3:-open}"

if ! command -v gh >/dev/null 2>&1; then
  echo "skip: gh CLI unavailable in this environment"
  exit 0
fi

echo "Checking dependabot alerts for $ADVISORY in $REPO (state=$STATE)"
if ! ALERTHITS="$(gh api "repos/$REPO/dependabot/alerts?state=$STATE" --jq "if type==\"array\" then map(select((.security_advisory.ghsa_id // \"\") == \"$ADVISORY\" and .state == \"$STATE\")) | length else 0 end" 2>/dev/null || true)"; then
  echo "skip: unable to query dependabot alerts in this environment"
  exit 0
fi

if [[ "${ALERTHITS:-0}" != "0" ]]; then
  echo "fail: open Dependabot alert $ADVISORY is still present"
  gh api "repos/$REPO/dependabot/alerts?state=$STATE" --jq "if type==\"array\" then map(select((.security_advisory.ghsa_id // \"\") == \"$ADVISORY\" and .state == \"$STATE\"))[] | \"#\(.number) \(.dependency.package.name) \(.state)\" else empty end"
  exit 1
fi

echo "pass: Dependabot alert $ADVISORY is not open"
