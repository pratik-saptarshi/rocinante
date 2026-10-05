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
| 0 — Establish the live baseline | Record `main`, branch, PR, head SHA, review threads, and check status. Refresh remote state before making a terminal claim. | PR base/head and current checks are obtained from GitHub; unresolved review comments are enumerated. If GitHub is unreachable, record the last successful snapshot and leave remote status unverified. | At the last successful GitHub refresh on 2026-10-05, PR #112 was OPEN/CLEAN at `27b9e21`; CI run `37311344183` had 24 passed, 0 failed, and 1 informational coverage skip; Security run `37311344136` and Dependency Review run `37311344143` passed. The PR #112 review-thread query returned zero unresolved threads, and the live Dependabot query returned zero open alerts. A fresh API check in the current pass could not connect, so these remain last-known remote facts, not a new live verification. |
| 1 — Correct weighted retention | Keep rollup `metric_sum` and `sample_count` through aggregate metrics and committer scoring; combine them with live samples using total sum / total count. | Run the focused regression, complete storage suite, and serial full workspace suite; run formatting and warning-denied Clippy. Hosted `rust-workspace-tests` must terminate green on the same head. | The weighted-retention regression, storage suite (21/21), full serial workspace (269 tests across 63 suites), formatting, and warning-denied all-target/all-feature Clippy pass locally. On `27b9e21`, hosted full-workspace/core/storage test shards, Rust quality, and aggregate `test` passed in CI run `37311344183`. |
| 2 — Clear security and dependency gates | Keep all 17 original advisory records accounted for, with the 15 authorized closures evidenced and the remaining affected packages removed from supported lockfiles. Keep the registry and audit ignore list empty. Preserve the RustSec gate. | Run governance and unfiltered `cargo audit --deny warnings` for both supported lockfiles against a freshly fetched database. Record database revision and report; hosted `rust-audit` and governance checks must pass on this PR head. | The authorized 15 closures are documented; the two remaining affected dependency paths are absent from both supported lockfiles. Governance passes with 0 registry entries and 0 ignores. The last successful fresh audits passed both lockfiles on RustSec revision `ef6173cbc5c50ec8166f9a5b28f07834144373ee` (1,290 advisories; 517 app and 82 migration-tool dependencies). This pass attempted a database refresh through `rtk cargo audit`, but GitHub fetch failed; `--no-fetch` scans of both lockfiles still pass against that cached revision. Hosted governance, RustSec audit, CodeQL, and secret scan passed on `27b9e21`; Dependency Review passed as well. |
| 3 — Preserve DuckDB binary-only packaging | Retain the SHA-256-verified official shared library and prohibit `bundled`, `bundled-cmake`, and any other source-build feature. | Run the DuckDB feature guard and provisioner contracts; inspect all supported package jobs and installed runtime evidence for Linux, macOS, and Windows. | The local DuckDB feature guard and provisioner contracts pass. On `27b9e21`, the Linux, macOS, and Windows native-shell package jobs passed, including runtime/loader checks; DuckDB remains a checksum-verified prebuilt and no source build is allowed. |
| 4 — Validate UI and native acceptance | Validate the pinned pnpm UI lane and installed shell behavior: cold/warm URL delivery, saved-state restart, Linux notification and dependency-floor checks, plus platform packaging. | Required UI, package, registration, URL-dispatch, notification, dependency-floor, and aggregate CI checks terminate green at one PR head. Record any behavior the scripts do not exercise as open acceptance. | Hosted `ui-quality` passed on `27b9e21` using pinned pnpm `12.9.1`; Linux/macOS URL delivery, Windows registration, all package jobs, and Linux visible notification acceptance passed. The local installed macOS acceptance passed cold/warm delivery, close-to-tray/Show/Quit handling, notification request, visible restore, and saved-state restart. AppKit returned `accepted=false`; physical menu delivery and frontmost restoration remain open. Opt-in manual verification is `ROCINANTE_ACCEPTANCE_MANUAL_TRAY=1 bash scripts/test-macos-url-dispatch.sh`; it waits for actual Show and Quit selections, checks frontmost state after Show, and verifies process exit. |
| 5 — Resolve review comments and keep complex work tracked | Keep the weighted-retention PR #108 comment linked to its fix. Confirm other findings are fixed or have concrete roadmap acceptance criteria (including BI-060 operation draining and atomic persistence). | Refresh PR #108/#112 review threads; resolve only findings whose implementation and required validation are complete. Confirm deferred work has owner-independent scope, acceptance criteria, and a test plan in the roadmap/bead tracker. | The weighted-retention and admin-secret threads were replied to with validation evidence and resolved; identity and shutdown/persistence follow-ups are resolved or tracked under BI-060. The last live GraphQL refresh returned zero unresolved PR #112 review threads. Refreshing GitHub during this pass failed, so current thread state is not newly verified. |
| 6 — Reconcile evidence and release decision | Synchronize roadmap, test plan, BOM, publish checklist, codemap, and dated decision record to actual results. | Run roadmap/publish contracts and `git diff --check`; only mark hosted/platform checks complete with terminal evidence. PR #112 must have a green aggregate and no unresolved blocking review comments before it is considered ready for protected merge. | Local roadmap/publish contracts passed 10/10, the targeted security suite passed 5/5, Rust formatting and `git diff --check` passed; the latest hosted CI run `37311344183` on `27b9e21` passed 24 checks with no failures and one informational coverage skip. Security and Dependency Review also passed. Direct physical tray-menu and foreground-activation evidence remains open. Current GitHub state could not be refreshed in this pass. |

## Execution log — 2026-10-05

### Latest recorded hosted validation

- The last successful PR status refresh recorded PR #112 OPEN/CLEAN at
  `27b9e21`. CI run `37311344183` completed with 24 passed checks, 0 failed,
  and 1 configured informational coverage skip. Security run `37311344136`
  passed RustSec audit, CodeQL, and secret scan; Dependency Review run
  `37311344143` passed. Hosted UI, Rust workspace and quality, aggregate,
  Linux/macOS/Windows package jobs, URL lifecycle, Linux visible notification,
  and the dependency/governance gates were green. The review-thread query
  returned zero unresolved threads and Dependabot returned zero open alerts.
- This pass could not refresh GitHub: `gh` and the web fetch failed to connect
  to GitHub. Preserve the snapshot above as last-known evidence and leave the
  live PR/check state unverified until connectivity returns.
- A fresh `rtk cargo audit` database fetch also failed to connect. Both lockfiles
  passed cached `--no-fetch --deny warnings` scans against database revision
  `ef6173cbc5c50ec8166f9a5b28f07834144373ee` (1,290 advisories); this does not
  replace a future database refresh.

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

### Local validation completed in the prior reconciliation pass

- Confirmed PR #112 remains on `fix/weighted-rollup-aggregation`, based on
  `origin/main` `eb83be9`; the last hosted source head at that point was
  `ff367c4`. Reviewed the readiness sync and corrected the roadmap to distinguish the
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

- Refresh PR #112 and required-check state when GitHub is reachable; do not
  treat a previous successful snapshot as proof of current remote status.
- Run `ROCINANTE_ACCEPTANCE_MANUAL_TRAY=1 bash scripts/test-macos-url-dispatch.sh`
  in an interactive macOS terminal. Physically select Show and Quit when
  prompted, then record the complete terminal result. The script does not
  synthesize input; it requires a real Show callback, a frontmost window, and
  process exit after Quit. Keep AppKit's `accepted=false` result alongside the
  observed frontmost state.
- Do not merge outside the protected PR flow.

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
