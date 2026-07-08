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
if ! RAW_ALERTS_JSON="$(gh api "repos/$REPO/dependabot/alerts?state=$STATE" --jq '.' 2>/dev/null || true)"; then
  echo "skip: unable to query dependabot alerts in this environment"
  exit 0
fi

if ! ALERTHITS="$(printf '%s' "$RAW_ALERTS_JSON" | jq --arg advisory "$ADVISORY" --arg state "$STATE" 'map(select(.security_advisory.ghsa_id == $advisory and .state == $state)) | length' 2>/dev/null)"; then
  echo "fail: malformed Dependabot API response; cannot evaluate advisory state"
  exit 1
fi

if [[ "$ALERTHITS" != "0" ]]; then
  echo "fail: open Dependabot alert $ADVISORY is still present"
  printf '%s\n' "$RAW_ALERTS_JSON" | jq --arg advisory "$ADVISORY" --arg state "$STATE" 'map(select(.security_advisory.ghsa_id == $advisory and .state == $state)) | .[] | "#\(.number) \(.dependency.package.name) \(.state)"'
  exit 1
fi

echo "pass: Dependabot alert $ADVISORY is not open"
