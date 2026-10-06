# Frontend UX Phase 1 validation — 2026-10-06

## Scope

This record covers the Phase 1 changes for telemetry payload completeness and
recovery. Imported payloads now preserve the distinction between omitted and
explicitly empty collections. Only the dashboard's sample/reset path supplies
sample telemetry. The UI reports partial imports separately from complete
empty imports. Browser coverage also exercises malformed JSON recovery after a
valid import.

The aggregate `test` job now depends on `ui-playwright` and requires its result
to be `success`. The job currently has no condition and applies to all workflow
events; a skipped or failed browser-test job therefore fails the aggregate.
The workflow contract rejects adding a conditional skip without updating that
applicability contract.

## Local validation evidence

| Check | Result |
|---|---|
| `bash scripts/test-ci-scope-contract.sh` | Passed. The aggregate dependency and fail-closed UI Playwright contract are asserted. |
| `cargo test --locked -p rocinante-repo-analyzer --test ci_gate_tests ci_workflow_has_aggregate_test_gate -- --exact` | Passed: 1 targeted Rust contract test. It now expects `ui-playwright` in the aggregate dependencies. DuckDB was staged from the SHA-256-pinned official prebuilt; no DuckDB source build was used. |
| `bash scripts/test-roadmap-doc-contracts.sh` | Passed: 10 tests across parity, command inventory, publish gate, and roadmap coherence. |
| `actionlint -oneline -ignore 'unknown permission scope "vulnerability-alerts"' .github/workflows/ci.yml .github/workflows/security.yml` | Passed using the repository-configured ignore. Unignored actionlint reports this unsupported-permission-scope diagnostic at `.github/workflows/ci.yml:429`. |
| TypeScript project build (`ui/node_modules/.bin/tsc -b`) | Passed. |
| Vitest (`ui/node_modules/.bin/vitest run`) | Passed after PR-review remediation: 13 files, 76 tests, including no-data risk and bottleneck trace assertions. |
| Production build (`ui/node_modules/.bin/vite build`) | Passed with Vite 8.3.2. Vite emitted the existing warning that one minified chunk exceeds 500 kB. |
| `git diff --check` | Passed after implementation and documentation updates. |
| Playwright (`ui/node_modules/.bin/playwright test`) | Blocked by the sandbox: the configured web server cannot bind `127.0.0.1:4173` (`listen EPERM`). No browser tests ran locally. |
| Playwright discovery (`ui/node_modules/.bin/playwright test --list`, from `ui/`) | Passed: all 8 browser tests were discovered, including malformed JSON recovery. |

The worktree pins pnpm 12.9.1. The available host executable reports pnpm
12.8.1 and hangs when invoked, so the UI checks above ran through the existing
`node_modules/.bin` executables. They do not establish that the pinned pnpm
installation path works in this environment.

## Remaining exit criteria

- Hosted `ui-playwright` and aggregate `test` checks must pass on the exact PR
  head before this phase can be marked complete.
- The pinned pnpm 12.9.1 UI lane must pass in hosted CI; no hosted result is
  available in this record.
- The protected `main` branch at base `d4bc8e7` requires `test` and `codeql`. The aggregate `test` job depends on `ui-playwright`, making the browser gate transitively required; `ui-playwright` is not a separate branch-protection context.
- Local Playwright remains blocked by sandbox networking; this is not a test
  pass and does not replace hosted browser validation.

## Payload compatibility note

The public envelope and field names are unchanged, and the legacy/null envelope
form remains supported. Import validation is now stricter: present rows must
contain all required fields with valid finite numeric values, and supplied
limits must be finite and non-negative where applicable. Callers that previously
sent incomplete rows or malformed limits must correct those payloads; the UI now
keeps the last good view and reports the validation error instead of accepting
bad values that could later crash rendering.


## PR review follow-up

PR #114 review found that an empty or partial import could still display the
quality-pulse sample recommendations and hard-coded sample action routes. The
implementation now derives available recommendations from present telemetry,
suppresses sample fallback recommendations for imported data, and leaves action
routes empty with an “Awaiting telemetry” window until data-grounded routing is
available. The pulse score requires at least one commit-risk and one bottleneck record. Empty arrays mean no observations and show an unavailable score; partial imports missing either input also show unavailable instead of 100/100. Visual, security, recommendation, score, and bottleneck empty states now use source-neutral copy. Empty-import component, helper, and browser assertions cover these review findings. A fourth P2 review comment found that the bottleneck metric and explanation still said “review” for no-data imports; both now show Unavailable and explain that no bottleneck records are available, while sample mode retains its fallback.
The next P2 review comment found that the Top Risk Commit trace rendered a blank identifier with a good status when imports had no commits. The trace now says Unavailable, explains that no commit-risk records are available, and uses a neutral status; sample and populated imported modes retain their behavior. Bead `BI-5u3.1.5` tracks the new acceptance criteria and test plan.

A sixth P2 review comment found empty commit/stage trend cards still read as green zero/healthy results. Those cards now show Unavailable with neutral status when there are no records; observed healthy results remain distinct. Bead `BI-5u3.1.6` carries the acceptance criteria and test plan. TypeScript, Vitest, production build, and discovery of all 8 Playwright tests pass locally after both updates.
Hosted same-head validation is required before marking the findings complete.
