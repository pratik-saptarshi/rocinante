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
| 0 — Establish the live baseline | Record `main`, branch, PR, head SHA, review threads, and check status. Refresh remote state before making a terminal claim. | PR base/head and current checks are obtained from GitHub; unresolved review comments are enumerated. If GitHub is unreachable, record the last successful snapshot and leave remote status unverified. | GitHub App refresh on 2026-10-05 confirmed PR #112 OPEN/MERGEABLE at `496e1ec`, based on `main` `eb83be9`, with zero unresolved review threads. The last full green aggregate remains run `37311344183` on `27b9e21`. Newer CI run `37317927760` reached terminal failure in the workspace/core test shards on a stale documentation-string assertion; a follow-up fix passes locally and requires hosted rerun. Security run `37317927662` and Dependency Review run `37317927667` passed. The latest recorded Dependabot query found zero alerts; it was not refreshed in this pass. |
| 1 — Correct weighted retention | Keep rollup `metric_sum` and `sample_count` through aggregate metrics and committer scoring; combine them with live samples using total sum / total count. | Run the focused regression, complete storage suite, and serial full workspace suite; run formatting and warning-denied Clippy. Hosted `rust-workspace-tests` must terminate green on the same head. | The weighted-retention regression, storage suite (21/21), full serial workspace (269 tests across 63 suites), formatting, and warning-denied all-target/all-feature Clippy pass locally. On `496e1ec`, the hosted workspace/core shards failed only in the stale security-governance documentation contract; the assertion now normalizes whitespace and checks the current checklist evidence. The full workspace and security-advisory suite pass locally after the correction; hosted confirmation on the follow-up head is pending. |
| 2 — Clear security and dependency gates | Keep all 17 original advisory records accounted for, with the 15 authorized closures evidenced and the remaining affected packages removed from supported lockfiles. Keep the registry and audit ignore list empty. Preserve the RustSec gate. | Run governance and unfiltered `cargo audit --deny warnings` for both supported lockfiles against a freshly fetched database. Record database revision and report; hosted `rust-audit` and governance checks must pass on this PR head. | The authorized 15 closures are documented; the two remaining affected dependency paths are absent from both supported lockfiles. Governance passes with 0 registry entries and 0 ignores. Security run `37317927662` on `496e1ec` fetched the RustSec database, loaded 1,290 advisories, scanned both supported lockfiles (517 app and 82 migration-tool dependencies), and passed with no findings; CodeQL and secret scan also passed. Current upstream `RustSec/advisory-db` HEAD is `ef6173cbc5c50ec8166f9a5b28f07834144373ee` (2026-10-03); the workflow log confirms a fresh fetch but does not print the downloaded SHA. A local refresh attempt failed, while cached no-fetch scans passed. The security workflow must rerun on the follow-up head. |
| 3 — Preserve DuckDB binary-only packaging | Retain the SHA-256-verified official shared library and prohibit `bundled`, `bundled-cmake`, and any other source-build feature. | Run the DuckDB feature guard and provisioner contracts; inspect all supported package jobs and installed runtime evidence for Linux, macOS, and Windows. | The local DuckDB feature guard and provisioner contracts pass. On `27b9e21`, the Linux, macOS, and Windows native-shell package jobs passed, including runtime/loader checks; DuckDB remains a checksum-verified prebuilt and no source build is allowed. |
| 4 — Validate UI and native acceptance | Validate the pinned pnpm UI lane and installed shell behavior: cold/warm URL delivery, saved-state restart, Linux notification and dependency-floor checks, plus platform packaging. | Required UI, package, registration, URL-dispatch, notification, dependency-floor, and aggregate CI checks terminate green at one PR head. Record any behavior the scripts do not exercise as open acceptance. | On `496e1ec`, hosted `ui-quality` passed with pinned pnpm `12.9.1`; Linux/macOS/Windows package, registration, and URL lifecycle checks passed, including visible Linux notifications. Physical tray-menu delivery and foreground restoration remain open; opt-in manual verification is `ROCINANTE_ACCEPTANCE_MANUAL_TRAY=1 bash scripts/test-macos-url-dispatch.sh` and still needs an interactive run. |
| 5 — Resolve review comments and keep complex work tracked | Keep the weighted-retention PR #108 comment linked to its fix. Confirm other findings are fixed or have concrete roadmap acceptance criteria (including BI-060 operation draining and atomic persistence). | Refresh PR #108/#112 review threads; resolve only findings whose implementation and required validation are complete. Confirm deferred work has owner-independent scope, acceptance criteria, and a test plan in the roadmap/bead tracker. | The weighted-retention and admin-secret threads were replied to with validation evidence and resolved; identity and shutdown/persistence follow-ups are resolved or tracked under BI-060. GitHub App refresh on PR #112 returned zero review threads. The PR is open and mergeable at `496e1ec`; current follow-up checks remain required. |
| 6 — Reconcile evidence and release decision | Synchronize roadmap, test plan, BOM, publish checklist, codemap, and dated decision record to actual results. | Run roadmap/publish contracts and `git diff --check`; only mark hosted/platform checks complete with terminal evidence. PR #112 must have a green aggregate and no unresolved blocking review comments before it is considered ready for protected merge. | On `496e1ec`, hosted UI, governance, quality/Clippy/fmt, package/lifecycle, RustSec/CodeQL/secret scan, and Dependency Review passed, but aggregate `test` failed on the stale documentation assertion. The assertion fix passes locally: full workspace 269 tests across 63 suites, advisory suite 5/5, warning-denied all-target/all-feature Clippy, formatting, roadmap contracts 10/10, governance, DuckDB/dependency guards, and `git diff --check`. A hosted rerun is required on the follow-up head. Physical tray-menu and foreground-activation evidence remains open. |

## Execution log — 2026-10-05

### Last fully green aggregate and newer hosted validation

- The last fully green PR aggregate remains run `37311344183` on head
  `27b9e21`: 24 checks passed, none failed, and one configured coverage check
  was informationally skipped. Security run `37311344136` and Dependency
  Review run `37311344143` also passed.
- GitHub App refresh confirmed PR #112 OPEN/MERGEABLE at `496e1ec`, based on
  `main` `eb83be9`; the review-thread query returned no unresolved threads.
  CI run `37317927760` completed with a failed `test` gate because the core
  and full-workspace shards hit the same stale exact-string documentation
  assertion. UI quality, all platform packaging/registration/URL jobs,
  governance, Rust quality, Clippy, and storage tests passed; coverage was
  skipped as configured.
- Security run `37317927662` completed successfully: `rust-audit` fetched the
  RustSec database and loaded 1,290 advisories for both supported lockfiles,
  while CodeQL and secret scan passed. Dependency Review run `37317927667`
  passed. The newest code/test correction passes locally and requires a new
  hosted run on the follow-up head.
- A current GitHub API read confirms the upstream RustSec advisory database
  HEAD is `ef6173cbc5c50ec8166f9a5b28f07834144373ee`, dated 2026-10-03. The
  hosted run logs prove a database fetch and 1,290-advisory load but do not
  print the fetched commit SHA, so the run-specific SHA is not directly
  evidenced.
- A fresh `rtk cargo audit` database fetch also failed to connect. Both lockfiles
  passed cached `--no-fetch --deny warnings` scans against database revision
  `ef6173cbc5c50ec8166f9a5b28f07834144373ee` (1,290 advisories); this does not
  replace a future database refresh.

### Current local closeout validation — correction after `496e1ec`

- The repeated hosted failure was a documentation-contract mismatch: the
  assertion expected a Dependabot phrase without normalizing Markdown
  whitespace and pinned an older hosted head. The test now normalizes
  whitespace and checks the current last-green evidence at `27b9e21`, plus
  the checklist's refresh instruction.
- The full serial Rust workspace passes locally (269 tests across 63 suites),
  including the security-advisory suite (5/5). Rust formatting and warning-
  denied all-target/all-feature Clippy pass locally. The previous manual-tray
  contract (1/1), roadmap contracts (10/10), governance (0 entries/0 ignores),
  DuckDB prebuilt-only guard, desktop dependency-floor guard, and diff check
  also pass.
- A local RustSec database refresh failed, but Security run `37317927662`
  successfully refreshed the database on head `496e1ec`. Both supported
  lockfiles passed hosted `--deny warnings` scans, loading 1,290 advisories
  (517 app and 82 migration-tool dependencies). Cached local `--no-fetch`
  scans also pass against revision
  `ef6173cbc5c50ec8166f9a5b28f07834144373ee` (1,290 advisories; 517 app and
  82 migration-tool dependencies); Cargo emitted a crates.io index lock
  warning but returned success with no audit findings. The hosted audit is
  fresh evidence for `496e1ec`; the security workflow must rerun on the
  follow-up head.
- The manual tray mode has not been run on an interactive macOS desktop. A
  person still needs to select Show and Quit and record the resulting
  frontmost/process-exit evidence.

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
