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
| 0 — Establish the live baseline | Record `main`, branch, PR, head SHA, review threads, and check status. Refresh remote state before making a terminal claim. | PR base/head and current checks are obtained from GitHub; unresolved review comments are enumerated. If GitHub is unreachable, record the last successful snapshot and leave remote status unverified. | PR #112 was OPEN/CLEAN at verified head `ff367c4`. CI run `37305039584` completed with 24 passed checks, no failures, and one configured coverage skip; Security run `37305039580` passed CodeQL, RustSec audit, and secret scan; Dependency Review passed. PR #108 has zero unresolved review threads, the live Dependabot alert query returned zero open alerts, and PR #112 is the only open PR to `main`. These checks validate ff367c4; readiness for any newer PR head depends on terminal-green checks on that exact head. |
| 1 — Correct weighted retention | Keep rollup `metric_sum` and `sample_count` through aggregate metrics and committer scoring; combine them with live samples using total sum / total count. | Run the focused regression, complete storage suite, and serial full workspace suite; run formatting and warning-denied Clippy. Hosted `rust-workspace-tests` must terminate green on the same head. | The weighted-retention regression, storage suite (21/21), full serial workspace (269 tests across 63 suites), formatting, and warning-denied all-target/all-feature Clippy pass locally. On `ff367c4`, hosted `rust-workspace-tests`, core/storage test shards, Rust quality gates, and aggregate `test` all passed in CI run `37305039584`. |
| 2 — Clear security and dependency gates | Keep all 17 original advisory records accounted for, with the 15 authorized closures evidenced and the remaining affected packages removed from supported lockfiles. Keep the registry and audit ignore list empty. Preserve the RustSec gate. | Run governance and unfiltered `cargo audit --deny warnings` for both supported lockfiles against a freshly fetched database. Record database revision and report; hosted `rust-audit` and governance checks must pass on this PR head. | The authorized 15 closures are documented; the two remaining affected dependency paths are absent from both supported lockfiles. Governance passes with 0 registry entries and 0 ignores. Fresh audits passed both lockfiles on RustSec revision `ef6173cbc5c50ec8166f9a5b28f07834144373ee` (1,290 advisories; 517 app and 82 migration-tool dependencies). Hosted governance, RustSec audit, and CodeQL passed on `ff367c4`; Dependency Review passed as well. |
| 3 — Preserve DuckDB binary-only packaging | Retain the SHA-256-verified official shared library and prohibit `bundled`, `bundled-cmake`, and any other source-build feature. | Run the DuckDB feature guard and provisioner contracts; inspect all supported package jobs and installed runtime evidence for Linux, macOS, and Windows. | The local DuckDB feature guard and provisioner contracts pass. On `ff367c4`, the Linux, macOS, and Windows native-shell package jobs all passed, including runtime/loader checks; no source build is allowed. |
| 4 — Validate UI and native acceptance | Validate the pinned pnpm UI lane and installed shell behavior: cold/warm URL delivery, saved-state restart, Linux notification and dependency-floor checks, plus platform packaging. | Required UI, package, registration, URL-dispatch, notification, dependency-floor, and aggregate CI checks terminate green at one PR head. Record any behavior the scripts do not exercise as open acceptance. | Hosted `ui-quality` passed on `ff367c4` using pinned pnpm `12.9.1`; Linux/macOS URL delivery, Windows registration, all package jobs, and Linux visible notification acceptance passed. The local installed macOS acceptance passed cold/warm delivery, close-to-tray/Show/Quit handling, notification request, visible restore, and saved-state restart. AppKit returned `accepted=false`; a physical tray-menu click and foreground activation remain open. |
| 5 — Resolve review comments and keep complex work tracked | Keep the weighted-retention PR #108 comment linked to its fix. Confirm other findings are fixed or have concrete roadmap acceptance criteria (including BI-060 operation draining and atomic persistence). | Refresh PR #108/#112 review threads; resolve only findings whose implementation and required validation are complete. Confirm deferred work has owner-independent scope, acceptance criteria, and a test plan in the roadmap/bead tracker. | PR #108 identity thread `discussion_r4182304432` is resolved after its identity test passed; shutdown/persistence thread `discussion_r4182304443` is resolved with operation draining and atomic-write acceptance in BI-060. After CI run `37305039584` passed the weighted-retention and full workspace tests, the weighted thread `discussion_r4182447911` was replied to with evidence and resolved. The admin-secret thread `discussion_r4182636661` was likewise replied to and resolved after the guard, full workspace, audit, secret scan, and governance checks passed. A live GraphQL refresh reports zero unresolved PR #108 review threads. |
| 6 — Reconcile evidence and release decision | Synchronize roadmap, test plan, BOM, publish checklist, codemap, and dated decision record to actual results. | Run roadmap/publish contracts and `git diff --check`; only mark hosted/platform checks complete with terminal evidence. PR #112 must have a green aggregate and no unresolved blocking review comments before it is considered ready for protected merge. | Local roadmap/publish contracts pass 10/10, the targeted security suite passes 5/5, Rust formatting and `git diff --check` pass. CI run `37305039584` passed 24 checks with no failures and one informational coverage skip; Security run `37305039580` passed, and PR #112 was OPEN/CLEAN at `ff367c4`. Direct physical tray-menu and foreground-activation evidence remains open. Readiness requires terminal-green hosted checks on the latest PR head. |

## Execution log — 2026-10-05

### Earlier progress snapshot — before replacement hosted validation

- The last hosted PR #112 workspace job (`37294465326`, job `111712579979`)
  was cancelled after `async_ingestion_engine_applies_retention_before_promotion`
  exceeded 60 seconds. The same test also stalled in a local full-workspace
  run. The test now drops its ingestion sender after the first completed
  promotion to prevent subsequent interval promotions during read assertions.
  The focused regression and storage suite passed, then the full serial
  analytics workspace passed **269 tests across 63 suites**. Treat this as a
  validated local fix; hosted confirmation on the updated commit is still
  required.
- On remote head `cf9c1d3`, run `37302536069` passed UI quality, Rust
  quality/Clippy/format, RustSec audit, security governance, dependency review,
  all three package jobs, URL dispatch, and Windows registration. Its core and
  full-workspace test failures shared one outdated assertion expecting the
  checklist to say audit refresh was pending. The local assertion now checks
  the recorded audit pass and hosted-pending state; the full local workspace
  passes 269/63. The correction and readiness records are pushed at
  `8285ee6`; replacement hosted status is not available because the GitHub API
  cannot be reached from this environment.
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
- Pinned-pnpm UI checks could not start locally because pnpm `12.9.1` could not
  verify its registry signature while `registry.npmjs.org` was unreachable.
  Hosted `ui-quality` and platform checks passed on `cf9c1d3`; repeat them after
  pushing the corrective contract-test change.
- The local branch is clean and pushed through `8285ee6`. The PR is not ready
  while replacement hosted aggregate and CodeQL results remain unverified.

### Completed in this execution pass

- Confirmed PR #112 remains on `fix/weighted-rollup-aggregation`, based on
  `origin/main` `eb83be9`; its last hosted source head is `ff367c4`. Reviewed
  the pending readiness sync and corrected the roadmap to distinguish the
  completed BI-051 dependency/removal gate from open BI-049/BI-050 parity work.
- Re-ran `scripts/test-roadmap-doc-contracts.sh`: **10 tests passed** across
  its four groups. The focused security-advisory contract suite passed **5/5**.
- Re-ran `scripts/check-security-advisory-exceptions.py`: **0 registry
  entries, 0 audit ignores, no overdue dates**. Refreshed the RustSec database
  through `rtk cargo audit`; both lockfiles passed `--deny warnings` on
  revision `ef6173cbc5c50ec8166f9a5b28f07834144373ee` (1,290 advisories).
- Rust formatting and `git diff --check` passed. DuckDB feature guard and its
  contract passed. The all-target desktop dependency guard and its contract
  passed, excluding GTK, GLib, Wry, Tauri, and `proc-macro-error`.

### Still required

- For each PR #112 update, require terminal-green hosted checks on its exact
  head before closing readiness.
- Obtain an interactive Accessibility-authorized macOS run that physically
  clicks the tray menu and records the resulting Show/Quit behavior. Keep
  AppKit foreground activation separate because the latest request returned
  `accepted=false`.
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
