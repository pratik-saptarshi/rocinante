#!/usr/bin/env bash
set -euo pipefail

REPO="${1:-pratik-saptarshi/rocinante}"
ADVISORY="${2:-GHSA-g7r4-m6w7-qqqr}"
STATE="${3:-open}"

if ! command -v gh >/dev/null 2>&1; then
  echo "error: gh CLI unavailable; cannot verify Dependabot alert state" >&2
  exit 2
fi

echo "Checking dependabot alerts for $ADVISORY in $REPO (state=$STATE)"
if ! ALERT_PAGES="$(gh api "repos/$REPO/dependabot/alerts?state=$STATE" --paginate --slurp 2>&1)"; then
  echo "error: unable to query Dependabot alerts: $ALERT_PAGES" >&2
  exit 2
fi

if ! command -v jq >/dev/null 2>&1; then
  echo "error: jq unavailable; cannot evaluate Dependabot alert state" >&2
  exit 2
fi

if ! ALERTHITS="$(jq -er --arg advisory "$ADVISORY" --arg state "$STATE" 'if (type == "array" and all(.[]; type == "array")) then add | map(select((.security_advisory.ghsa_id // "") == $advisory and .state == $state)) | length else error("expected paginated alert arrays") end' <<<"$ALERT_PAGES" 2>&1)"; then
  echo "error: unable to evaluate Dependabot alert response: $ALERTHITS" >&2
  exit 2
fi

if [[ ! "$ALERTHITS" =~ ^[0-9]+$ ]]; then
  echo "error: malformed Dependabot alert count; cannot evaluate advisory state: $ALERTHITS" >&2
  exit 2
fi

if [[ "$ALERTHITS" != "0" ]]; then
  echo "fail: open Dependabot alert $ADVISORY is still present"
  exit 1
fi

echo "pass: Dependabot alert $ADVISORY is not open"
