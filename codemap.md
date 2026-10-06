# Repository Atlas: rocinante

## Current Architecture Status (2026-10-06)

The supported desktop host is the GTK-free eframe/winit shell; Tauri runtime/build dependencies and GTK/GLib are removed from supported manifests. The former 11 command names and payload shapes remain host-neutral compatibility metadata. DuckDB remains a checksum-verified prebuilt and is never compiled from source.

PR #112 merged into main as `035c290662e2a4c79e247de6036886ea90e7bf60`. Its final head `bb2d1700e4ae0c1663df140336f2f2ededbd0963` passed CI (`37417428474`), Security (`37417428663`), and Dependency Review (`37417428598`). Five P2 inline review threads remain unresolved after merge; a focused documentation follow-up is tracking their closeout. Current shutdown source uses the shared `quit_explicitly` helper: it synchronously saves shell state and flushes storage, removes the tray icon, then calls `std::process::exit(0)`; Linux and Windows use eframe viewport close, and ordinary close-to-tray still hides the window. The manual Show attempt timed out after earlier code head `2fac448`, before the current hard-exit candidate; it did not validate current physical Show/Quit or foreground behavior. Keep those acceptance items open until a person observes the callbacks and visible restoration. DuckDB remains a checksum-verified prebuilt and is never compiled from source.

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
  runtime, required `rust-workspace-tests`, `ui-quality`, and `ui-playwright`
  aggregate gates, and informational
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
| `src-tauri/crates/rocinante-desktop-shell/` | GTK-free native window, navigation, repository selection, analysis, saved metrics, notifications, release-baseline controls, and sample insight/reference panels using eframe/winit. | Earlier Linux/macOS/Windows package and URL results passed on `2fac448`; the current `quit_explicitly` helper synchronously saves state, flushes storage, removes the tray icon, and directly exits on macOS. The scripted Linux/macOS/Windows URL and package jobs passed on predecessor head `d598d59`; physical macOS tray clicks and foreground activation remain unverified; physical macOS tray clicks and foreground activation remain unverified. |
| `src-tauri/crates/rocinante-analysis/` | Host-independent auth, repository discovery/analysis, telemetry persistence, and shared database-path configuration. | Owns shared modules used by native-shell and Rust service callers. |
| `src-tauri/tests/` | Backend regression coverage for PR-risk, CI-gate, publish-doc, incident-feedback, storage, authorization, and command-compatibility contracts. | Tests protect shared-service and release-gate invariants; there is no registered Tauri handler suite. |
| `ui/src/` | Frontend dashboard, bridge adapters, explainability panels, and quality-pulse rendering. | UI state should flow through the bridge adapters rather than direct runtime assumptions. |
| `ui/e2e/` | Browser-level smoke coverage for the React/Vite preview and user-visible flows. | Covers telemetry import, empty/partial payloads, malformed JSON recovery, and last-good dashboard checks. Hosted Playwright validation is required; local execution may be blocked when the sandbox denies the configured web-server bind. |
| `docs/` | Feature backlog, roadmap, test plan, publish-readiness checklist, and bead tracker artifacts. | This is the source of truth for phase sequencing and backlog accounting. |
| `scripts/` | Repo automation, local operational helpers, and verified native-library provisioning. | Prefer existing scripts over ad hoc shell snippets; keep CI scope, dependency-floor, advisory, and DuckDB source-build guards contract-tested. |

## Data and Control Flow

1. Roadmap documents and the bead tracker define phase order, acceptance criteria, and release blockers.
2. The eframe/winit desktop shell calls authenticated scan and admin services in `rocinante-analysis`, `rocinante-storage`, and `rocinante-core`.
3. `src-tauri/src/command_compat.rs` records the retired Tauri command names and wire shapes. It does not register IPC; `lib.rs` re-exports its module at the former Rust path for compatibility.
4. Native scan and admin operations require the configured `RUNICIPAL_TOKEN_SECRET` and enforce role checks at service boundaries.
5. SQLite WAL handles ingestion, DuckDB handles analytics through checksum-verified official prebuilt libraries, and legacy Sled stores require the isolated audited migrator before upgrade.
6. CI scope detection routes Rust manifest, lockfile, workflow, and configuration changes to Rust validation. The aggregate gate now requires the Linux/macOS/Windows native-shell package matrix.
7. The React/Vite UI is a browser preview surface; it is not a production desktop transport. Imported payloads do not seed omitted collections with demo records; samples are limited to the initial/reset sample view. Pinned pnpm, typecheck, unit, and production-build checks run in `ui-quality`; browser tests run in `ui-playwright`, which the aggregate gate requires to succeed.
8. Roadmap, governance, package, and platform contracts are tested locally and in CI. Historical hosted Tauri results do not validate the current native-host retirement worktree.

## Governance and Execution Snapshot

- PR #108 merged at `eb83be9`; PR #112 merged at `035c290`. Five review threads on PR #112 remain open for evidence/documentation reconciliation.
- Final PR #112 head `bb2d170` passed aggregate CI `37417428474`, Security `37417428663` (RustSec, CodeQL, and secret scan), and Dependency Review `37417428598`. The earlier failed run `37404464369` on `d598d59` is historical and was superseded by these green checks.
- The final source uses `quit_explicitly`: synchronously save shell state, flush storage, drop the tray icon, and directly exit on macOS. Linux/Windows close the viewport; ordinary close-to-tray hides the window.
- The recorded physical Show attempt timed out after `2fac448`, before the current hard-exit source was validated. Physical macOS Show/Quit clicks, frontmost restoration, and saved-state restart remain unverified and are release acceptance gates.
- The 15 withdrawn/absent RustSec records have evidence-backed closure; the exception registry and Cargo audit ignore list are empty.
- pnpm `12.9.1` is pinned. DuckDB uses only the checksum-verified prebuilt binary and is never compiled from source.

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
