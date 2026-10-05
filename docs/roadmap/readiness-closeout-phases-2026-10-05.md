# Readiness Closeout Phases — 2026-10-05

## Objective

Finish the current readiness remediation through the protected PR flow. Close
the weighted-retention review finding, preserve the fail-closed RustSec gate,
prove DuckDB remains prebuilt-only, validate the supported native platforms,
and update the release evidence from terminal results. Do not call the roadmap
complete while a required check, advisory refresh, or platform acceptance is
missing or still running.

This closeout plan supplements
[`rustsec-zero-exception-remediation-plan.md`](rustsec-zero-exception-remediation-plan.md),
which retains the original investigation and implementation history.

## Phase sequence and exit gates

| Phase | Work | Validation and exit gate | Current state |
|---|---|---|---|
| 0 — Establish the live baseline | Record `main`, branch, PR, head SHA, review threads, and check status. Refresh remote state before making a terminal claim. | PR base/head and current checks are obtained from GitHub; unresolved review comments are enumerated. If GitHub is unreachable, record the last successful snapshot and leave remote status unverified. | Partially complete. PR #112 was last confirmed OPEN/MERGEABLE at remote head `ea440e4`; current local changes are uncommitted on `fix/weighted-rollup-aggregation`. Hosted run `37294465326` later terminated **cancelled** after its workspace test stalled. No current-head hosted run exists for these local changes. |
| 1 — Correct weighted retention | Keep rollup `metric_sum` and `sample_count` through aggregate metrics and committer scoring; combine them with live samples using total sum / total count. | Run the focused regression, complete storage suite, and serial full workspace suite; run formatting and warning-denied Clippy. Hosted `rust-workspace-tests` must terminate green on the same head. | Local gates now pass: focused weighted regression, storage suite 21/21, full serial analytics workspace 269 tests across 63 suites, formatting, and all-target/all-feature Clippy with warnings denied. The retention test now drops the ingestion sender after its first completed promotion; this cleared the local full-suite stall. Hosted run `37294465326` was cancelled on the prior remote head, so same-head hosted confirmation remains open. |
| 2 — Clear security and dependency gates | Keep all 17 original advisory records accounted for, with the 15 authorized closures evidenced and the remaining affected packages removed from supported lockfiles. Keep the registry and audit ignore list empty. Preserve the RustSec gate. | Run governance and unfiltered `cargo audit --deny warnings` for both supported lockfiles against a freshly fetched database. Record database revision and report; hosted `rust-audit` and governance checks must pass on this PR head. | The authorized 15 closures are documented; the two remaining affected dependency paths have been removed from both supported lockfiles. Governance passes with 0 registry entries and 0 ignores. Fresh audits passed for both lockfiles against RustSec DB revision `ef6173cbc5c50ec8166f9a5b28f07834144373ee` (1,290 advisories; DB commit time 2026-10-03T10:14:03+02:00): 517 app dependencies and 82 migration-tool dependencies, `--deny warnings`. Hosted checks still need a run on the current branch changes. |
| 3 — Preserve DuckDB binary-only packaging | Retain the SHA-256-verified official shared library and prohibit `bundled`, `bundled-cmake`, and any other source-build feature. | Run the DuckDB feature guard and provisioner contracts; inspect all supported package jobs and installed runtime evidence for Linux, macOS, and Windows. | DuckDB feature guard passed locally and packaging on the last remote head passed for Linux, macOS, and Windows. Same-head hosted package and installed-runtime evidence remains outstanding. Keep DuckDB as a verified prebuilt binary; no source build is allowed. |
| 4 — Validate UI and native acceptance | Validate the pinned pnpm UI lane and installed shell behavior: cold/warm URL delivery, saved-state restart, Linux notification and dependency-floor checks, plus platform packaging. | Required UI, package, registration, URL-dispatch, notification, dependency-floor, and aggregate CI checks terminate green at one PR head. Record any behavior the scripts do not exercise as open acceptance. | Local UI execution with pinned pnpm `12.9.1` is blocked by the registry signature lookup/network failure; prior hosted `ui-quality` passed on `ea440e4`, not current local changes. Local macOS cold/warm URL delivery, saved-state restart, tray/window flow, and notification request passed; AppKit activation returned false. Current-head hosted UI/platform results and direct physical tray/menu evidence remain open. |
| 5 — Resolve review comments and keep complex work tracked | Keep the weighted-retention PR #108 comment linked to its fix. Confirm other findings are fixed or have concrete roadmap acceptance criteria (including BI-060 operation draining and atomic persistence). | Refresh PR #108/#112 review threads; resolve only findings whose implementation and required validation are complete. Confirm deferred work has owner-independent scope, acceptance criteria, and a test plan in the roadmap/bead tracker. | PR #108 identity thread `discussion_r4182304432` is resolved after `scans_repositories_and_persists_sanitized_metrics_without_a_host_runtime` passed 1/1; it verifies unique legacy rows rebind to the discovered stable identity while ambiguous basenames remain separate. The complex shutdown/persistence thread `discussion_r4182304443` is resolved with its operation-draining and atomic-write acceptance criteria tracked in BI-060. The weighted-retention thread remains unresolved pending current-head hosted validation. The exported admin storage services now require the configured token secret before admin authorization; admin-secret guard (1/1), admin-service (8/8), admin-ingestion guard (2/2), command compatibility (5/5), and scoring-audit (1/1) tests pass locally. The PR #108 security thread remains unresolved pending hosted validation of the new fix. |
| 6 — Reconcile evidence and release decision | Synchronize roadmap, test plan, BOM, publish checklist, codemap, and dated decision record to actual results. | Run roadmap/publish contracts and `git diff --check`; only mark hosted/platform checks complete with terminal evidence. PR #112 must have a green aggregate and no unresolved blocking review comments before it is considered ready for protected merge. | Roadmap/publish contract suite passed 10/10; formatting, governance, DuckDB source-build guard, desktop dependency guard, and `git diff --check` pass. The worktree contains uncommitted code and doc changes. Do not publish or merge until pinned-pnpm/UI and same-head hosted required checks terminate green. |

## Execution log — 2026-10-05

### Current progress update

- The last hosted PR #112 workspace job (`37294465326`, job `111712579979`)
  was cancelled after `async_ingestion_engine_applies_retention_before_promotion`
  exceeded 60 seconds. The same test also stalled in a local full-workspace
  run. The test now drops its ingestion sender after the first completed
  promotion to prevent subsequent interval promotions during read assertions.
  The focused regression and storage suite passed, then the full serial
  analytics workspace passed **269 tests across 63 suites**. Treat this as a
  validated local fix; hosted confirmation on the updated commit is still
  required.
- A fresh RustSec database refresh succeeded in an isolated writable location.
  Both supported lockfiles pass unfiltered `cargo audit --deny warnings`:
  517 app dependencies and 82 migration-tool dependencies, with zero findings.
  The database revision is `ef6173cbc5c50ec8166f9a5b28f07834144373ee`.
- The new admin-secret guard test passes 1/1; admin-service 8/8,
  admin-ingestion guard 2/2, command compatibility 5/5, and scoring-audit 1/1
  also pass with explicit test-only secrets. Warning-denied all-target,
  all-feature workspace Clippy, formatting, the security-governance checker,
  DuckDB source-build guard, desktop dependency guard, roadmap contracts
  (10/10), and `git diff --check` pass locally.
- Pinned-pnpm UI checks could not start because pnpm `12.9.1` could not verify
  its registry signature while `registry.npmjs.org` was unreachable. Prior
  hosted UI/platform successes are from remote head `ea440e4` and do not validate
  the local uncommitted state.
- Local branch `fix/weighted-rollup-aggregation` still tracks remote PR #112 at
  `ea440e4`; the admin-secret and documentation changes are uncommitted. No
  readiness or merge claim is made for the current worktree.

### Completed in this execution pass

- Confirmed a clean worktree at `ea440e450ce5ba5f34e0178d6bc0e1c63e4c1882`,
  tracking `origin/fix/weighted-rollup-aggregation`; local `origin/main` is
  `eb83be9da64057dc71838b33edc01a9a2769b0fa`.
- Re-ran the focused
  `aggregates_and_scores_weighted_rollups_with_live_samples` integration test:
  **1 passed**.
- Re-ran `scripts/test-roadmap-doc-contracts.sh` through `bash`:
  **10 tests passed** across its four test groups.
- Re-ran `scripts/check-security-advisory-exceptions.py`: **0 registry
  entries, 0 audit ignores, no overdue dates**.
- Re-ran the DuckDB source-build feature guard and desktop dependency guard:
  both passed; the latter excludes GTK, GLib, Wry, Tauri, and
  `proc-macro-error` from supported desktop workspace targets.

### Still required

- Obtain pinned-pnpm UI check results. The local attempt could not access npm's
  registry signature endpoint; use the hosted PR lane after pushing the changes.
- Push the reviewed changes to the existing PR #112 branch after local gates
  pass, then require terminal-green aggregate, security, UI, and platform checks
  on that exact commit. Resolve review threads only after the relevant evidence
  is available.
- Refresh hosted Linux, macOS, and Windows package/lifecycle results. Keep
  AppKit activation and direct physical tray/menu evidence explicit if CI does
  not cover them.
- Reconcile the publish checklist, BOM, test plan, codemap, and dated decision
  record with exact terminal evidence. Do not merge outside the protected PR
  flow.

## Reproducible validation commands

```sh
rtk cargo test --locked --offline --manifest-path src-tauri/Cargo.toml \
  --test storage_duallayer_tests \
  aggregates_and_scores_weighted_rollups_with_live_samples -- --exact
rtk proxy bash scripts/test-roadmap-doc-contracts.sh
rtk proxy python3 scripts/check-security-advisory-exceptions.py
rtk proxy bash scripts/check-duckdb-features.sh
rtk proxy bash scripts/check-desktop-shell-dependencies.sh
rtk cargo audit --db /private/tmp/rocinante-rustsec-advisory-db \
  --file src-tauri/Cargo.lock --deny warnings
rtk cargo audit --db /private/tmp/rocinante-rustsec-advisory-db \
  --file tools/sled-migration/Cargo.lock --deny warnings
```
