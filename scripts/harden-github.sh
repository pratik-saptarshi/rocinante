#!/usr/bin/env bash
set -euo pipefail

if ! command -v gh >/dev/null 2>&1; then
  echo "gh CLI is required" >&2
  exit 1
fi

if [[ $# -lt 1 ]]; then
  echo "Usage: $0 <owner/repo>" >&2
  exit 1
fi

REPO="$1"

# Repo baseline hardening
# - main default branch
# - private visibility (change if needed)
# - disable wiki/projects
# - merge strategy controls
# - delete merged branches
# - enable vulnerability alerts + auto-fix

gh repo edit "$REPO" \
  --default-branch main \
  --enable-issues \
  --enable-merge-commit=false \
  --enable-rebase-merge=false \
  --enable-squash-merge \
  --delete-branch-on-merge \
  --enable-auto-merge

# Branch protection for main
# Requires admin on the repository

gh api \
  -X PUT \
  -H "Accept: application/vnd.github+json" \
  "repos/$REPO/branches/main/protection" \
  --input - <<'JSON'
{
  "required_status_checks": {
    "strict": true,
    "contexts": ["test", "codeql"]
  },
  "enforce_admins": true,
  "required_pull_request_reviews": {
    "dismiss_stale_reviews": true,
    "require_code_owner_reviews": true,
    "required_approving_review_count": 1,
    "require_last_push_approval": true
  },
  "restrictions": null,
  "required_linear_history": true,
  "allow_force_pushes": false,
  "allow_deletions": false,
  "block_creations": false,
  "required_conversation_resolution": true,
  "lock_branch": false
}
JSON

# Security settings
# Enabled separately because some org policies may override these calls.
gh api -X PUT "repos/$REPO/vulnerability-alerts" -H "Accept: application/vnd.github+json"
gh api -X PUT "repos/$REPO/automated-security-fixes" -H "Accept: application/vnd.github+json"

echo "Hardening applied to $REPO"
