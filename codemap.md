# Repository Atlas: rocinante

## Current Architecture Status (2026-10-05)

The supported desktop host on the current remediation worktree is the
GTK-free eframe/winit shell. The Tauri binary, runtime/build dependencies,
bootstrap, and installer configs are removed locally. The former 11 Tauri
command names and wire shapes are retained as host-neutral compatibility
metadata in `src-tauri/src/command_compat.rs`; they do not register IPC.
DuckDB remains a checksum-verified prebuilt shared library and is never
compiled from source. PR #108 is merged; follow-up branch
`fix/weighted-rollup-aggregation` is the active PR #112 review path. The stale
test assertion was corrected in commit `8285ee6`, followed by readiness-doc
syncs on the same branch. The latest recorded hosted validation is for PR head
`27b9e21`: CI run `37311344183` completed with 24 passed checks, no failures,
and one configured coverage skip. Security run `37311344136` and Dependency
Review run `37311344143` passed. UI, Rust quality/tests, aggregate,
Linux/macOS/Windows packages and URL lifecycle also passed. At the last
successful GitHub refresh, PR #112 was OPEN/CLEAN, its review threads were all
resolved, and the Dependabot alert query returned zero open alerts. A new live
refresh on 2026-10-05 could not connect to GitHub, so current remote state is
not freshly verified. Manual physical tray-menu clicks and macOS foreground
activation remain unverified.

## Project Responsibility
Rocinante is a cross-language planning and execution workspace for AI quality
checking, dashboarding, and loop-engineering controls. The repository combines
a shared Rust service workspace, a React/Vite preview UI, and a GTK-free native
desktop host.

## System Entry Points
- `src-tauri/crates/rocinante-desktop-shell/src/main.rs`: native desktop entry point.
- `src-tauri/src/lib.rs`: backend crate surface and shared module exports.
- `src-tauri/src/command_compat.rs`: host-neutral inventory of the retired
  command names and request/response shapes, plus shared service facades.
- `src-tauri/src/budget_guard.rs`: budget guard and kill-switch contract for
  report-only and stop behavior.
- `src-tauri/src/fix_proposal.rs`: minimal-fix and escalation contract for
  one-problem remediation loops.
- `src-tauri/src/roadmap_coherence.rs`: stage-three convergence validator for
  phase gates, test mapping, and unlabeled-task detection.
- `src-tauri/src/triage.rs`: report-only triage formatter for high-priority,
  watch, noise, and state-updates sections.
- `src-tauri/src/verifier.rs`: reject-by-default verifier contract for
  one-problem diffs and evidence-backed approval.
- `src-tauri/crates/rocinante-core/src/lib.rs`: host-agnostic Rust contracts
  for risk evaluation, budget guards, fix proposals, triage, verification,
  authorization policy, baseline scoring, and shared types.
- `src-tauri/crates/rocinante-analysis/src/lib.rs`: host-agnostic repository
  scan entry point, admin authorization, and shared telemetry database path
  resolution used by the native shell and Rust services.
- `src-tauri/crates/rocinante-analysis/src/auth.rs`: shared JWT validation and role checks; admin
  operations require a configured 32-byte secret.
- `src-tauri/crates/rocinante-storage/src/lib.rs`: shared SQLite WAL/DuckDB
  persistence and admin-authorized release-baseline operations used by the shell.
  DuckDB is dynamically linked from the checksum-verified prebuilt library for
  the current target. SQLite ingestion now uses transactional writes, ordered
  prefix scans, replay receipts, and a legacy-Sled startup guard. The isolated
  audited reader in `tools/sled-migration` preserves Sled stores while copying
  their tree/key/value data into SQLite; its own lockfile is audited in CI.
- `src-tauri/crates/rocinante-desktop-shell/src/main.rs`: GTK-free native shell
  entry point using eframe/winit; navigation state is also persisted as JSON in
  the per-user app-data directory for cross-process restart restoration, with
  broader host, data-migration, and operation-locking work tracked by BI-056 through BI-061.
- `src-tauri/src/ci_gate.rs`: PR-risk CI comment formatter and merge-block
  decision helper.
- `src-tauri/src/incident_feedback.rs`: incident and annotation feedback ledger
  with cache invalidation and auditable risk raises.
- `.github/workflows/ci.yml`: CI contract for Rust formatting, linting,
  checking, full-workspace crate tests, current advisory exception review-date enforcement,
  Linux URI/notification acceptance plus Windows and macOS URL/restart acceptance jobs,
  required Linux/macOS/Windows native-shell package checks for the prebuilt DuckDB
  runtime, required `rust-workspace-tests` and `ui-quality` gates, and informational
  backend Rust coverage via `rust-coverage`. Rust build lanes stage DuckDB
  releases before Cargo, and installed-app acceptance covers app-relative
  `.so`, bundled `.dylib`, and colocated `.dll` loading. The `ci-workflow-parse` job also
  runs the standard-library-only roadmap and publish-doc contract tests before
  the workspace test lanes.
- `ui/src/App.tsx`: dashboard shell and admin bridge consumer.
- `ui/src/admin-bridge-panel.tsx`: extracted command-bridge control block.
- `ui/package.json`: `pnpm@12.9.1` UI manifest and test/build entry points.
- `docs/feature-list.html`: feature backlog with acceptance criteria and bead linkage.
- `docs/product-roadmap.html`: stage ordering and release-gate sequencing.
- `docs/roadmap/bead-issue-tracker.html`: execution ledger for active bead issues.
- `docs/bill-of-materials.html`: release inventory, dependency surface, and
  backend Rust coverage artifact inventory.
- `docs/publish-readiness-checklist.html`: publish gate checklist, including
  security, validation, and coverage artifact criteria.
- `docs/roadmap/gtk-free-host-migration-plan.html`: phase-gated GTK-free host
  migration plan with TDD and parity checkpoints.
- `docs/roadmap/desktop-parity-matrix.html`: phase-0 parity matrix and host
  decision record with must-have/should-have/can-defer classification.
- `docs/roadmap/dependabot-esbuild-remediation-plan.html`: completed esbuild
  remediation record and current unrelated Dependabot alert snapshot.
- `docs/roadmap/security-advisory-exceptions.json`: machine-readable registry
  for time-boxed audit ignores and host-floor governance.
- `scripts/dependency-floor-proof.sh`: dependency-floor proof command for GTK
  and GLib transitive-path validation.
- `scripts/check-desktop-shell-dependencies.sh`: Linux-target native-shell graph
  gate that excludes GTK/GLib/Tauri/Wry and runs in the Linux acceptance job.
- `scripts/test-check-desktop-shell-dependencies.sh`: contract coverage for the
  native-shell dependency gate's clean and forbidden-package cases.
- `scripts/test-roadmap-doc-contracts.sh`: directly compiles and runs the
  std-only host-decision, publish-readiness, and roadmap-coherence Rust tests
  with the pinned toolchain before the slower root Tauri test targets.
- `scripts/check-security-advisory-exceptions.py`: release-time validator for
  exception metadata and review-date expiry; it fails closed on overdue reviews.
- `scripts/test-linux-url-dispatch.sh`: Ubuntu acceptance for installed cold/warm
  URI delivery, notification display, and saved-state process restart.
- `scripts/test-macos-url-dispatch.sh` and `scripts/test-windows-url-dispatch.ps1`:
  installed cold URI dispatch, warm secondary-process delivery, and new-process
  saved-state acceptance. Windows uses the registered URI handler for cold
  launch and starts the installed binary with the URI for warm forwarding.
- `scripts/macos-window-state.m`: macOS acceptance witness for window visibility,
  frontmost state, AppKit activation policy, and AppKit active state.
- `scripts/generate-desktop-icons.sh` and `.swift`: regenerate the shared native
  window, macOS bundle, Linux launcher, and Windows shortcut/URI-handler icon assets.
- `scripts/check-esbuild-lock.mjs`: enforces local lockfile dependency-floor for
  `esbuild >= 0.28.1` in CI and local verification.
- `scripts/provision_duckdb.py` and `scripts/duckdb-prebuilt-artifacts.json`:
  stage official DuckDB shared-library releases after SHA-256 verification;
  source-build features are rejected by `scripts/check-duckdb-features.sh`.
- `scripts/test_native_shell_packaging.py`: contract-test the required
  Linux/macOS/Windows native-shell package matrix and prebuilt DuckDB placement.
- `scripts/detect-ci-scope.sh`: routes Rust manifest, lockfile, and workflow
  changes to the Rust workspace lanes; `scripts/test-ci-scope-contract.sh`
  covers those path classifications.

## Directory Map

| Directory | Responsibility Summary | Notes |
|---|---|---|
| `src-tauri/src/` | Shared backend services and host-neutral compatibility facades for auth, storage, scoring, telemetry, risk, budget, fix-proposal, triage, verifier, convergence, and baseline operations. | No Tauri bootstrap remains. `command_compat.rs` records the retired public command shapes and keeps the old Rust module path as a compatibility re-export. |
| `src-tauri/crates/rocinante-storage/` | Host-neutral admin/scoring services, SQLite WAL ingestion, DuckDB analytics, and release-baseline authorization adapter. | The native shell calls this crate directly. Analytics and scoring keep legacy working-directory defaults until BI-058 provides a data-preserving migration. The separate analysis `telemetry.db` uses per-user data with legacy-file migration. DuckDB links only to the official staged prebuilt; the isolated `tools/sled-migration` utility handles legacy Sled data. |
| `src-tauri/crates/rocinante-desktop-shell/` | Supported GTK-free native window, navigation, repository selection, analysis, saved metrics, notifications, release-baseline controls, and sample insight/reference panels using eframe/winit. | Linux, macOS, and Windows package and URL lifecycle checks passed in CI run `37311344183` on PR head `27b9e21`; visible Linux notification delivery also passed. Physical macOS tray-menu clicks and foreground activation remain unverified; opt-in manual validation is `ROCINANTE_ACCEPTANCE_MANUAL_TRAY=1 bash scripts/test-macos-url-dispatch.sh`. Visible macOS/Windows notifications and Linux/Windows tray runtime remain parity gaps. |
| `src-tauri/crates/rocinante-analysis/` | Host-independent auth, repository discovery/analysis, telemetry persistence, and shared database-path configuration. | Owns shared modules used by native-shell and Rust service callers. |
| `src-tauri/tests/` | Backend regression coverage for PR-risk, CI-gate, publish-doc, incident-feedback, storage, authorization, and command-compatibility contracts. | Tests protect shared-service and release-gate invariants; there is no registered Tauri handler suite. |
| `ui/src/` | Frontend dashboard, bridge adapters, explainability panels, and quality-pulse rendering. | UI state should flow through the bridge adapters rather than direct runtime assumptions. |
| `ui/e2e/` | Browser-level smoke coverage for the React/Vite preview and user-visible flows. | Keeps the Playwright surface separate from unit tests. |
| `docs/` | Feature backlog, roadmap, test plan, publish-readiness checklist, and bead tracker artifacts. | This is the source of truth for phase sequencing and backlog accounting. |
| `scripts/` | Repo automation, local operational helpers, and verified native-library provisioning. | Prefer existing scripts over ad hoc shell snippets; keep CI scope, dependency-floor, advisory, and DuckDB source-build guards contract-tested. |

## Data and Control Flow

1. Roadmap documents and the bead tracker define phase order, acceptance criteria, and release blockers.
2. The eframe/winit desktop shell calls authenticated scan and admin services in `rocinante-analysis`, `rocinante-storage`, and `rocinante-core`.
3. `src-tauri/src/command_compat.rs` records the retired Tauri command names and wire shapes. It does not register IPC; `lib.rs` re-exports its module at the former Rust path for compatibility.
4. Native scan and admin operations require the configured `RUNICIPAL_TOKEN_SECRET` and enforce role checks at service boundaries.
5. SQLite WAL handles ingestion, DuckDB handles analytics through checksum-verified official prebuilt libraries, and legacy Sled stores require the isolated audited migrator before upgrade.
6. CI scope detection routes Rust manifest, lockfile, workflow, and configuration changes to Rust validation. The aggregate gate now requires the Linux/macOS/Windows native-shell package matrix.
7. The React/Vite UI is a browser preview surface; it is not a production desktop transport. Its pinned pnpm, typecheck, unit, and production-build checks remain in the UI quality lane.
8. Roadmap, governance, package, and platform contracts are tested locally and in CI. Historical hosted Tauri results do not validate the current native-host retirement worktree.

## Governance and Execution Snapshot

- PR #108 is merged into current `origin/main` at `eb83be9`. PR #112 carries the weighted-retention correction on `fix/weighted-rollup-aggregation`; the full serial workspace suite, storage suite (21/21), formatting, warning-denied Clippy, and roadmap/publish contracts pass locally. Hosted CI run `37311344183`, Security run `37311344136`, and Dependency Review run `37311344143` passed on source head `27b9e21`; the formerly unresolved PR #108 threads were resolved after the relevant evidence passed.
- Phase 4C removes the Tauri executable, runtime/build dependencies, bootstrap, and installer configuration. The native package and URL lifecycle jobs passed on Linux/macOS/Windows in run `37311344183`. Direct physical tray-menu validation remains open.
- The 15 withdrawn or absent advisory records have evidence-backed closures, and the two affected package paths have been removed. The registry and audit ignore list are empty; both supported lockfiles pass the last refreshed audit and the cached `--no-fetch` recheck. A new database fetch failed during this pass because GitHub could not be reached. Hosted governance and RustSec audit passed on `27b9e21`; the last live Dependabot alert query returned zero open alerts.
- On the latest local code, full-workspace tests pass serially, the storage suite passes 21/21, formatting passes, and all-target/all-feature Clippy passes with warnings denied. Earlier worktree evidence also covers `cargo check --offline --workspace --all-targets` and test-target compilation.
- Rust formatting, CI scope, DuckDB prebuilt-only, native packaging, dependency-guard, governance, and roadmap contracts have local pass evidence. The hosted aggregate and required checks passed on `27b9e21`; one informational coverage job was skipped. A live status refresh now fails to connect to GitHub, so the last successful remote snapshot is retained rather than presented as newly verified.
- pnpm `12.9.1` is pinned and was verified as the latest stable release on 2026-10-05. Hosted `ui-quality` passed with the pin, including typecheck, UI tests, and production build; local registry signature lookup remains unavailable.
- The installed macOS acceptance passes cold/warm URL delivery, saved-state restart, close-to-tray/Show/Quit handling, visible restore, and notification request. AppKit returned `accepted=false`, and physical tray-menu clicks were not exercised; foreground activation remains unverified.

## Design Patterns

The current host routes retained operations through shared service crates and
uses a required native-shell package matrix. Stored repository metrics use a
sanitized label plus stable local path hash. Reads use legacy basename or
tree-relative aliases only when the stable identity has no matching rows for
the requested release, avoiding duplicate metrics during upgrade. The analysis
`telemetry.db` migrates to per-user storage; analytics and scoring defaults
retain their legacy working-directory paths until BI-058 defines and validates
migration, backup, conflict, and rollback behavior.

**Historical PR #108 Tauri packaging fixes (superseded by Phase 4C):** the
former Tauri runtime bundle matrix was routed through the aggregate gate, the
checksum-verified DuckDB binary was staged before Tauri resource validation,
and the macOS app was signed after loader rewrites. Those Tauri packaging steps
were retired with the Tauri host; the native-shell package matrix now owns
release packaging validation.

- Host-neutral compatibility record for retired command payloads.
- Service layer separation between desktop controls and storage logic.
- Browser preview adapter kept outside the native runtime boundary.
- Dual-layer persistence for ingest and analytics responsibilities.
- Contract-driven testing for admin workflows and roadmap-backed behavior.
