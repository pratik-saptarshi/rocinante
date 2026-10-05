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
| 0 — Establish the live baseline | Record `main`, branch, PR, head SHA, review threads, and check status. Refresh remote state before making a terminal claim. | PR base/head and current checks are obtained from GitHub; unresolved review comments are enumerated. If GitHub is unreachable, record the last successful snapshot and leave remote status unverified. | Refreshed GitHub state confirms PR #112 OPEN/MERGEABLE at documentation head `2fac448550540f555f0fc9b9cdd19c4c1e27adce` on `fix/weighted-rollup-aggregation`, based on `main` `eb83be9da64057dc71838b33edc01a9a2769b0fa`. CI `37346608465`, Security `37346608399`, and Dependency Review `37346608411` all completed successfully. All four PR review threads are resolved. |
| 1 — Correct weighted retention | Keep rollup `metric_sum` and `sample_count` through aggregate metrics and committer scoring; combine them with live samples using total sum / total count. | Run the focused regression, complete storage suite, and serial full workspace suite; run formatting and warning-denied Clippy. Hosted `rust-workspace-tests` must terminate green on the same head. | Retention promotion merges prior rollup sums/counts with newly stale raw samples. The regression forces a second retention cycle and verifies sum 45/count 3, average 15, and the weighted score. The full serial workspace passes 270 tests across 64 suites; formatting and warning-denied all-target/all-feature Clippy pass locally. CI `37346608465` passed workspace tests and Clippy on `2fac448`. |
| 2 — Clear security and dependency gates | Keep all 17 original advisory records accounted for, with the 15 authorized closures evidenced and the remaining affected packages removed from supported lockfiles. Keep the registry and audit ignore list empty. Preserve the RustSec gate. | Run governance and unfiltered `cargo audit --deny warnings` for both supported lockfiles against a freshly fetched database. Record database revision and report; hosted `rust-audit` and governance checks must pass on this PR head. | The 15 user-authorized closures are documented; both affected dependency paths are absent from supported lockfiles. Governance passed on `2fac448`; the registry and audit ignore list remain empty. Security `37346608399` passed RustSec audit, CodeQL, and secret scan. The hosted audit result does not expose a database SHA. |
| 3 — Preserve DuckDB binary-only packaging | Retain the SHA-256-verified official shared library and prohibit `bundled`, `bundled-cmake`, and any other source-build feature. | Run the DuckDB feature guard and provisioner contracts; inspect all supported package jobs and installed runtime evidence for Linux, macOS, and Windows. | Local DuckDB feature and provisioner contracts pass. CI `37346608465` passed Linux, macOS, and Windows package jobs using the verified prebuilt runtime. DuckDB remains a checksum-verified prebuilt and no source build is allowed. |
| 4 — Validate UI and native acceptance | Validate the pinned pnpm UI lane and installed shell behavior: cold/warm URL delivery, saved-state restart, Linux notification and dependency-floor checks, plus platform packaging. | Required UI, package, registration, URL-dispatch, notification, dependency-floor, and aggregate CI checks terminate green at one PR head. Record any behavior the scripts do not exercise as open acceptance. | CI `37346608465` passed UI typecheck, unit tests, production build, Linux/macOS/Windows packaging and lifecycle checks, Windows registration, Linux notification delivery, and dependency-floor checks. Hosted cold/warm dispatch passed; the most recent manual macOS run after code head `2fac448` timed out without observing the physical Show callback. Physical tray-menu delivery, foreground restoration, Quit, and saved-state restart remain open. |
| 5 — Resolve review comments and keep complex work tracked | Fix weighted-retention data loss and configured-secret bypasses. Keep multi-operation shutdown/persistence concerns scoped under BI-060. | Refresh PR #112 review threads; resolve only findings whose implementation and required validation are complete. Confirm deferred work has owner-independent scope, acceptance criteria, and a test plan in the roadmap/bead tracker. | The weighted-retention and configured-secret findings are fixed and tested. The additional Quit-witness finding is fixed on `ce5a1e1`: the tray callback writes a Quit marker, and manual acceptance requires it before accepting process exit. All four review threads, including the scoped codemap finding, are resolved after local contracts and hosted checks passed. BI-060 operation draining and atomic persistence remain tracked separately. |
| 6 — Reconcile evidence and release decision | Synchronize roadmap, test plan, BOM, publish checklist, codemap, and dated decision record to actual results. | Run roadmap/publish contracts and `git diff --check`; only mark hosted/platform checks complete with terminal evidence. PR #112 must have a green aggregate and no unresolved blocking review comments before it is considered ready for protected merge. | Local full workspace (270 tests/64 suites), Clippy, formatting, roadmap/publish contracts (10/10), governance (0 entries/0 ignores), DuckDB and desktop dependency guards/contracts, CI-scope and Dependabot-checker contracts, provisioner contract (9 tests), and `git diff --check` pass. The latest documentation-only roadmap contract run passed 10/10 and `rtk git diff --check` passed. CI `37346608465`, Security `37346608399`, and Dependency Review `37346608411` are green on `2fac448`; all four review threads are resolved. Phase 8 physical tray-menu and foreground-activation evidence remains open; coverage was skipped as informational. |

## Remaining execution phases

| Phase | Work | Validation and exit gate | Current state |
|---|---|---|---|
| 7 — Reconcile current evidence | Update current-state roadmap, test plan, security roadmaps, README, codemap, BOM, publish checklist, and dated decision summary to PR documentation head `2fac448` and its successful hosted runs. Preserve older run data as history. | Run `bash scripts/test-roadmap-doc-contracts.sh`, inspect current summaries for stale-head references, and require `rtk git diff --check` to pass. | Complete locally. Current summaries cite `2fac448` and runs `37346608465`, `37346608399`, and `37346608411`; older heads and runs remain as dated history. All 10 roadmap/publish contract tests and `rtk git diff --check` pass. Hosted validation of this documentation refresh follows the push. |
| 8 — Complete interactive macOS acceptance | Run the opt-in installed-app check on this interactive Mac and physically choose Show and Quit from the app menu. Require frontmost state after Show and successful process exit after Quit. | Run with `ROCINANTE_ACCEPTANCE_MANUAL_TRAY=1` and leave strict cold-launch frontmost mode unset; manual mode checks frontmost after the physical Show action. Require a zero exit and the script's success witness. | Still open. The strict cold-launch attempt stopped before tray prompts (`visible=true`, `frontmost=false`). The most recent manual-mode run after code head `2fac448` installed and registered the bundle, passed cold URL, notification, and close-to-tray setup, then timed out after 60 seconds without observing `show_action_started=true`. Warm URL, Show foreground, Quit, and restart remain unverified. No physical Show/Quit acceptance is claimed. |
| 9 — Final closeout audit | Record the interactive result, rerun document contracts and diff checks, push the existing PR branch, and refresh all hosted checks at its new head. | Require green aggregate CI, Security, and Dependency Review at one head, no unresolved blocking comments, and all required roadmap acceptance evidence. | On documentation head `2fac448`, CI `37346608465`, Security `37346608399`, and Dependency Review `37346608411` passed with all four review threads resolved. The latest manual Show attempt timed out. Documentation contracts (10/10) and diff checks are being rerun before pushing this refresh to the existing PR branch; replacement hosted checks must complete. Phase 8 remains open, so final readiness is not claimed. |

## Execution log — 2026-10-05

### Hosted validation and review resolution — `3748d5b`

This is the prior validation snapshot. The current PR head and replacement
check runs are recorded in the 2026-10-05 refresh below.

PR #112 remains OPEN/MERGEABLE at `3748d5b` on `main` `eb83be9`. CI run
`37325510392` passed the aggregate, Rust tests/lint, UI quality, all Linux,
macOS, and Windows package and lifecycle jobs, and governance. Security run
`37325510228` passed RustSec audit, CodeQL, and secret scan; Dependency Review
run `37325510232` passed. The Rust coverage job was skipped as configured.
Both review threads were replied to with the local regression evidence and
resolved after these same-head checks completed. The physical macOS tray-menu
Show/Quit click and foreground activation still require an interactive run.

### Historical hosted validation refresh — `e9d6d3a`

The live PR read confirms PR #112 is OPEN/MERGEABLE at `e9d6d3ab6d398d6c9c786e7b7108d9d926d0b1bb` on `main` `eb83be9`. CI `37328034504`, Security
`37328034651`, and Dependency Review `37328034502` all completed successfully.
CI includes the aggregate gate, Rust tests/lint, UI quality, Linux/macOS/Windows
package and native lifecycle jobs, plus security governance. Security includes
RustSec audit, CodeQL, and secret scan. The informational Rust coverage job was
skipped as configured. Both blocking PR review threads are resolved. The
physical macOS tray Show/Quit clicks and foreground activation remain open;
phase 8 is the remaining interactive acceptance step.

The manual-mode attempt ran with Launch Services access and reached the
physical Show prompt after installing the temporary bundle and completing its
cold URL, notification, and close-to-tray setup. The script then timed out
without observing the Show action; the result is not a pass. Warm URI delivery,
Show foreground state, Quit, and restart were therefore not exercised. A
previous run with strict cold-launch frontmost mode stopped earlier at
`visible=true`, `frontmost=false`, so that flag is intentionally omitted from
the manual retry.

Hosted validation of the documentation refresh began on commit `a1b8e7e`.
Dependency Review run `37333311656` passed. CI run `37333311678` failed the
core and full-workspace test shards because the security-advisory contract
still expected the previous `3748d5b` checklist line; all other completed CI
jobs passed, including formatting, Clippy, governance, UI, package, and
lifecycle jobs. The stale assertion has been updated locally, and the focused
security-advisory suite passes 5/5. Security run `37333311614` has successful
RustSec audit and secret-scan jobs; CodeQL was still in progress at the last
poll. Replacement hosted runs are required after pushing the assertion fix.

Follow-up CI run `37334243657` on `7e5bd2a` passed the full/core/storage Rust
test jobs, Clippy, UI quality, security governance, and all package and URL
lifecycle jobs. The aggregate failed only because `rust-lint (fmt)` found the
new dependency-review assertion should be formatted on one line. Local
`cargo fmt --manifest-path src-tauri/Cargo.toml --all -- --check` reproduced
and then passed after the formatting correction. Security run `37334243778`
has successful RustSec audit and secret-scan jobs; CodeQL is still running.
Dependency Review run `37334243714` passed. The formatted test and evidence
refresh were pushed in `c88062d`; its replacement run is recorded below.

### Follow-up hosted validation and macOS retry — `c88062d`

CI run `37335883596` passed workspace tests, security governance, UI quality,
native package jobs, and URL lifecycle jobs before the subsequent evidence-
only push cancelled it while `rust-quality-gates` was still running Linux
dependency installation and the CI gate contract. The full lint gate was not
reached on this attempt. Security run `37335883407` passed RustSec audit,
CodeQL, and secret scan. Dependency Review run `37335883632` passed.

A repeated manual macOS run rebuilt the app, confirmed the DuckDB 1.5.6
prebuilt, installed and registered the temporary app, passed cold URL,
notification, and close-to-tray setup, then timed out after 60 seconds without
observing `show_action_started=true`. The Show action, warm URL, foreground
state, Quit action, and restart were not verified.

### Previous complete hosted validation — `6555587`

On head `65555870a22ddade2cae3592e97eb8d0b7446654`, CI run `37339337822`,
Security run `37339337692`, and Dependency Review run `37339337795` all
completed successfully. CI passed Rust formatting, Clippy, core/storage/full
workspace tests, security governance, UI quality, and Linux/macOS/Windows
package and lifecycle jobs. Security passed RustSec audit, CodeQL, and secret
scan. The informational coverage job was skipped as configured. PR #112
remains OPEN/MERGEABLE at this evidence head with both review threads resolved.
The interactive macOS Show/Quit attempt still timed out at the Show prompt, so
Phase 8 remains open and the roadmap is not complete.

### Review findings after green checks on `b3f0c83`

GitHub refreshed PR #112 as OPEN/MERGEABLE at `b3f0c83` on `main` `eb83be9`.
CI run `37321514028`, Security run `37321514047`, and Dependency Review run
`37321514294` all succeeded. The refreshed thread list then exposed two
unresolved findings that were not present in the prior zero-thread snapshot.

- The retention regression had promoted the new sample without retention, so
  it did not exercise a second stale-release rollup. The retention transaction
  now merges existing `metric_sum`/`sample_count` rows with new stale raw
  samples before replacing the rollup. The regression promotes again through
  retention and verifies the combined sum 45, count 3, average 15, and score.
- All four public baseline compatibility functions now share authorization
  that requires a configured secret before decoding the principal. A separate
  integration test signs a valid admin token with the known fallback secret,
  verifies path-based calls do not open stores, and verifies with-store calls
  cannot read or mutate the baseline.

The focused regressions pass. The full serial Rust workspace passes 270 tests
across 64 suites, formatting passes, warning-denied all-target/all-feature
Clippy passes, and `git diff --check` passes. These fixes are local and have not
yet received hosted checks; keep both review threads open until the follow-up
commit has green required checks. Physical macOS tray-menu and foreground
acceptance also remain open.

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

- The live PR refresh confirmed `2fac448` and green CI, Security, and
  Dependency Review runs, with no unresolved review threads. After this
  documentation update is pushed, refresh the checks again on its new head;
  do not treat the `2fac448` results as proof for a later commit.
- Run `ROCINANTE_ACCEPTANCE_MANUAL_TRAY=1 bash scripts/test-macos-url-dispatch.sh`
  on an interactive macOS desktop when a person can choose Show and Quit
  during the prompts. The most recent manual run after code head `2fac448`
  timed out without observing the Show callback; warm URL, foreground, Quit,
  and restart are still unverified. The
  script requires a real Show callback and Quit-action witness, a frontmost
  window, and process exit after Quit. Keep AppKit's `accepted=false` result
  alongside the observed frontmost state.
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

### Previous hosted closeout and manual acceptance — `ce5a1e1` (2026-10-05)

GitHub reports PR #112 OPEN/MERGEABLE at
`ce5a1e1a6c8fbc67f0077e08f7e43805c0b26abc`, based on `main`
`eb83be9da64057dc71838b33edc01a9a2769b0fa`. CI run `37341472090`, Security
run `37341472140`, and Dependency Review run `37341472151` all completed
successfully. CI passed workspace tests, Rust format/Clippy, UI typecheck/unit
tests/production build, advisory governance, Linux/macOS/Windows package jobs,
Linux and macOS URL lifecycle checks, Windows registration/lifecycle, Linux
notifications, and DuckDB/dependency-floor contracts. Security passed RustSec
audit, CodeQL, and secret scan. Dependency Review passed. Rust coverage remains
skipped as configured.

The Quit-witness review finding was fixed on this head. The tray Quit callback
records `quit_action_started=true`; the manual script requires that witness
before it accepts the app's exit. Its contract and feature checks passed, and
the review thread was resolved after all three hosted workflows were green.
The two earlier functional review findings also remain resolved. The PR has no
unresolved inline review threads.

Two further interactive macOS attempts installed the signed app and passed
cold URL delivery, notification request, and close-to-tray setup, but neither
observed `show_action_started=true` during its 60-second prompt. Together with
the two earlier timeouts, this leaves physical Show, foreground restoration,
warm URL delivery, Quit, and saved-state restart unverified. The attempt is
not a pass; Phase 8 and final release readiness remain open. This documentation
refresh still needs its contract/diff checks, push to the existing PR branch,
and new same-head hosted check runs.

### Latest hosted status and interactive attempt — `2fac448` (2026-10-05)

GitHub confirms PR #112 OPEN/MERGEABLE at documentation head
`2fac448550540f555f0fc9b9cdd19c4c1e27adce`, based on `main`
`eb83be9da64057dc71838b33edc01a9a2769b0fa`. CI run `37346608465`, Security
run `37346608399`, and Dependency Review run `37346608411` completed
successfully. CI passed the aggregate, Rust quality and workspace tests, UI
quality, governance, Linux/macOS/Windows package and lifecycle checks, Linux
notification delivery, and dependency-floor checks. Security passed RustSec,
CodeQL, and secret scan. All four inline review threads are resolved.

The latest local manual attempt verified the DuckDB 1.5.6 prebuilt, installed
and registered the temporary app, passed cold URL, notification, and
close-to-tray setup, then reached the physical Show prompt. The 60-second wait
ended without observing `show_action_started=true`. The Show action was not
verified in this run; warm URL, foreground state, Quit, and saved-state restart
remain unverified. Phase 8 and release readiness remain open. The accompanying
documentation refresh records these results; its local contracts and hosted
checks must be recorded separately from the green `2fac448` runs.
