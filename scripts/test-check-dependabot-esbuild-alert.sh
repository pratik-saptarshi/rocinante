#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CHECK="$ROOT/scripts/check-dependabot-esbuild-alert.sh"
TEMP_DIR="$(mktemp -d)"
REAL_JQ="$(command -v jq || true)"
trap 'rm -rf "$TEMP_DIR"' EXIT
mkdir -p "$TEMP_DIR/bin" "$TEMP_DIR/gh-only"

cat > "$TEMP_DIR/bin/gh" <<'MOCK_GH'
#!/bin/sh
has_paginate=0
has_slurp=0
has_jq=0
for argument do
    [ "$argument" = "--paginate" ] && has_paginate=1
    [ "$argument" = "--slurp" ] && has_slurp=1
    [ "$argument" = "--jq" ] && has_jq=1
done
[ "$has_paginate" -eq 1 ] && [ "$has_slurp" -eq 1 ] && [ "$has_jq" -eq 0 ] || exit 9
if [ "${MOCK_GH_STATUS:-0}" -ne 0 ]; then
    exit "$MOCK_GH_STATUS"
fi
printf '%s\n' "${MOCK_GH_OUTPUT:-}"
MOCK_GH
chmod +x "$TEMP_DIR/bin/gh"

cat > "$TEMP_DIR/bin/jq" <<'MOCK_JQ'
#!/bin/sh
if [ "${MOCK_JQ_STATUS:-0}" -ne 0 ]; then
    exit "$MOCK_JQ_STATUS"
fi
if [ "${MOCK_JQ_OVERRIDE:-0}" -eq 1 ]; then
    printf '%s\n' "${MOCK_JQ_OUTPUT:-}"
    exit 0
fi
exec "$MOCK_REAL_JQ" "$@"
MOCK_JQ
chmod +x "$TEMP_DIR/bin/jq"
cp "$TEMP_DIR/bin/gh" "$TEMP_DIR/gh-only/gh"

run_case() {
    local name="$1"
    local expected_status="$2"
    local gh_status="$3"
    local gh_output="$4"
    local jq_status="${5:-0}"
    local jq_output="${6:-}"
    local jq_override="${7:-0}"
    local actual_status=0

    if PATH="$TEMP_DIR/bin" MOCK_GH_STATUS="$gh_status" MOCK_GH_OUTPUT="$gh_output" \
        MOCK_JQ_STATUS="$jq_status" MOCK_JQ_OUTPUT="$jq_output" \
        MOCK_JQ_OVERRIDE="$jq_override" MOCK_REAL_JQ="$REAL_JQ" \
        /bin/bash "$CHECK" owner/repo GHSA-g7r4-m6w7-qqqr open >"$TEMP_DIR/output" 2>&1; then
        actual_status=0
    else
        actual_status=$?
    fi
    if [[ "$actual_status" -ne "$expected_status" ]]; then
        cat "$TEMP_DIR/output" >&2
        echo "FAIL: $name expected exit $expected_status, got $actual_status" >&2
        exit 1
    fi
}

run_case "closed alert" 0 0 '[[],[]]' 0
run_case "open alert on later page" 1 0 '[[],[{"state":"open","security_advisory":{"ghsa_id":"GHSA-g7r4-m6w7-qqqr"}}]]' 0 1
run_case "API failure" 2 7 ''
run_case "malformed API error payload" 2 0 '{"message":"Resource not accessible by integration"}'
run_case "malformed paginated response" 2 0 '[{"state":"open","security_advisory":{"ghsa_id":"GHSA-g7r4-m6w7-qqqr"}}]'
run_case "jq failure" 2 0 '[[],[]]' 4
run_case "malformed count" 2 0 '[[],[]]' 0 'not-a-count' 1

if PATH="$TEMP_DIR/empty-bin" /bin/bash "$CHECK" owner/repo GHSA-g7r4-m6w7-qqqr open \
    >"$TEMP_DIR/output" 2>&1; then
    cat "$TEMP_DIR/output" >&2
    echo "FAIL: missing gh CLI unexpectedly passed" >&2
    exit 1
else
    actual_status=$?
    if [[ "$actual_status" -ne 2 ]]; then
        cat "$TEMP_DIR/output" >&2
        echo "FAIL: missing gh CLI expected exit 2, got $actual_status" >&2
        exit 1
    fi
fi

if PATH="$TEMP_DIR/gh-only" MOCK_GH_STATUS=0 MOCK_GH_OUTPUT='[[],[]]' \
    /bin/bash "$CHECK" owner/repo GHSA-g7r4-m6w7-qqqr open >"$TEMP_DIR/output" 2>&1; then
    cat "$TEMP_DIR/output" >&2
    echo "FAIL: missing jq unexpectedly passed" >&2
    exit 1
else
    actual_status=$?
    if [[ "$actual_status" -ne 2 ]]; then
        cat "$TEMP_DIR/output" >&2
        echo "FAIL: missing jq expected exit 2, got $actual_status" >&2
        exit 1
    fi
fi

echo "pass: Dependabot alert checker fails closed across all contract cases"
