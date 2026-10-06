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
| Vitest (`ui/node_modules/.bin/vitest run`) | Passed after the security display-limit remediation: 13 files, 82 tests, including no-data, imported-route, and hidden-security-signal assertions. |
| Production build (`ui/node_modules/.bin/vite build`) | Passed with Vite 8.3.2. Vite emitted the existing warning that one minified chunk exceeds 500 kB. |
| `git diff --check` | Passed after implementation and documentation updates. |
| Playwright (`ui/node_modules/.bin/playwright test`) | Blocked by the sandbox: the configured web server cannot bind `127.0.0.1:4173` (`listen EPERM`). No browser tests ran locally. |
| Playwright discovery (`ui/node_modules/.bin/playwright test --list`, from `ui/`) | Passed: all 8 browser tests were discovered, including malformed JSON recovery. |
| Earlier hosted UI quality, Playwright, aggregate CI, Rust workspace/lints, and three-platform package/URL lanes | Passed on the earlier PR #114 code head `71bf9e5990428ac0603eeb370837bd954c4ef1f2` in CI run [37411802567](https://github.com/pratik-saptarshi/rocinante/actions/runs/37411802567). This is historical evidence and does not cover the subsequent security display-limit fix. |
| Earlier hosted RustSec audit, CodeQL, and secret scan | Passed on the earlier code head `71bf9e5990428ac0603eeb370837bd954c4ef1f2` in Security run [37411802526](https://github.com/pratik-saptarshi/rocinante/actions/runs/37411802526). |
| Earlier Dependency Review | Passed on the earlier code head `71bf9e5990428ac0603eeb370837bd954c4ef1f2` in run [37411802611](https://github.com/pratik-saptarshi/rocinante/actions/runs/37411802611). |
| Current hosted UI quality, Playwright, aggregate CI, Rust workspace/lints, and platform lanes | Passed on exact security-fix code head `75edab266f1fbc65d7fdacb55abca49f25125e35` in CI run [37416787765](https://github.com/pratik-saptarshi/rocinante/actions/runs/37416787765). |
| Current hosted RustSec audit, CodeQL, and secret scan | Passed on code head `75edab266f1fbc65d7fdacb55abca49f25125e35` in Security run [37416787763](https://github.com/pratik-saptarshi/rocinante/actions/runs/37416787763). |
| Current Dependency Review | Passed on code head `75edab266f1fbc65d7fdacb55abca49f25125e35` in run [37416787782](https://github.com/pratik-saptarshi/rocinante/actions/runs/37416787782). |

The worktree pins pnpm 12.9.1. The available host executable reports pnpm
12.8.1 and hangs when invoked, so the UI checks above ran through the existing
`node_modules/.bin` executables. They do not establish that the pinned pnpm
installation path works in this environment.

## Hosted acceptance and remaining exit criteria

- Hosted checks on the original code head `71bf9e5990428ac0603eeb370837bd954c4ef1f2`
  are historical. After the security display-limit fix, hosted UI quality,
  Playwright, aggregate CI, Rust workspace/lints, platform lanes, security
  governance, RustSec audit, CodeQL, secret scan, and Dependency Review passed
  on exact code head `75edab266f1fbc65d7fdacb55abca49f25125e35` in the three runs
  listed above. This document-only update is later than that tested head; its
  hosted checks must be rerun before Phase 1 can close.
- Hosted `ui-quality` passed with the pinned pnpm 12.9.1. The local pinned
  executable remained unavailable; local checks used existing `node_modules/.bin`
  tools.
- The protected `main` branch at base `d4bc8e7` requires `test` and `codeql`. The aggregate `test` job depends on `ui-playwright`, making the browser gate transitively required; `ui-playwright` is not a separate branch-protection context.
- Child Bead `BI-5u3.1.9` (preserve security signals beyond the display limit)
  is implemented and the fix passed hosted checks on code head
  `75edab266f1fbc65d7fdacb55abca49f25125e35`. Leave it open until the updated
  documentation commit passes hosted checks and the related review thread is
  resolved. Keep milestone `BI-5u3.1` open until PR #114 merges through the
  protected flow; this record does not claim a merge.
- Local Playwright remains blocked by sandbox networking; this is not a test
  pass; hosted Playwright passed on the recorded code head.

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
suppresses sample fallback recommendations for imported data, and keeps routes
in “Awaiting telemetry” only while the relevant audience records are absent.
Populated routes derive from current commit-risk, bottleneck, opportunity, and
security records. Healthy imported stages and absent security signals are
reported as observed results, not replaced by sample instructions. The pulse
score requires at least one commit-risk and one bottleneck record. Empty arrays
mean no observations and show an unavailable score; partial imports missing
either input also show unavailable instead of 100/100. Visual, security,
recommendation, score, and bottleneck empty states now use source-neutral copy.
Empty-import and populated-route component, helper, and browser assertions
cover these review findings. A fourth P2 review comment found that the
bottleneck metric and explanation still said “review” for no-data imports; both
now show Unavailable and explain that no bottleneck records are available,
while sample mode retains its fallback.
The next P2 review comment found that the Top Risk Commit trace rendered a blank identifier with a good status when imports had no commits. The trace now says Unavailable, explains that no commit-risk records are available, and uses a neutral status; sample and populated imported modes retain their behavior. Bead `BI-5u3.1.5` tracks the new acceptance criteria and test plan.

A sixth P2 review comment found empty commit/stage trend cards still read as green zero/healthy results. Those cards now show Unavailable with neutral status when there are no records; observed healthy results remain distinct. Bead `BI-5u3.1.6` carries the acceptance criteria and test plan.

A seventh P2 review comment found populated imports were still routed to Awaiting telemetry. Bead `BI-5u3.1.8` captures the follow-up. Per-audience routes now use the imported records; absent audience data remains awaiting, sample routes are unchanged. At that milestone, TypeScript, Vitest (13 files / 79 tests), production build, and discovery of all 8 Playwright tests passed locally. The build retains the existing >500 kB chunk warning. Full local browser execution remains blocked by sandbox `EPERM` binding `127.0.0.1:4173`. Subsequent security-signal regression additions bring the current Vitest result to 13 files / 82 tests, as recorded above. The populated-route changes and later security display-limit fix both have hosted evidence on code head `75edab266f1fbc65d7fdacb55abca49f25125e35`; the doc-only commit still needs its own hosted checks.

The current P2 review finding identified that security signals beyond the
`limits.risks` display cap could disappear from the pulse count,
recommendations, and route. The implementation now classifies the full
validated commit-risk set for counts, recommendations, and routes while
keeping the rendered risk-card list bounded. The Security panel shows at most
three details and reports how many additional signals were omitted. Regression
coverage verifies a hidden security signal remains counted and actionable
without expanding the displayed list. The fix and its regression coverage are
captured in three commits:
`a07e113` (`fix(ui): preserve security signals beyond display limit`),
`f58cb06` (`fix(ui): cap rendered security signal details`), and `685ba9f`
(`test(ui): cover hidden security recommendations`). These commits are on
code head `75edab266f1fbc65d7fdacb55abca49f25125e35`; hosted evidence for that
head is recorded above. Bead `BI-5u3.1.9` remains open pending successful hosted
checks for this documentation update and resolution of the review thread.
