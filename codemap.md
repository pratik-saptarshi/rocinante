# Repository Atlas: rocinante

## Project Responsibility
Rocinante is a cross-language planning and execution workspace for AI quality
checking, dashboarding, and loop-engineering controls. The repository combines
a shared Rust service workspace, a React/Vite UI, current Tauri integration,
and an in-progress GTK-free native desktop host.

## System Entry Points
- `src-tauri/src/main.rs`: Tauri command registration and application state wiring.
- `src-tauri/src/lib.rs`: backend crate surface and shared module exports.
- `src-tauri/src/tauri_commands.rs`: token-checked backend facade for admin,
  risk, and release-baseline operations.
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
  resolution used by both desktop hosts.
- `src-tauri/crates/rocinante-analysis/src/auth.rs`: shared JWT validation and role checks; the production
  Tauri host and repository scan service require a configured 32-byte secret.
- `src-tauri/crates/rocinante-storage/src/lib.rs`: shared SQLite WAL/DuckDB
  persistence and admin-authorized release-baseline operations used by Tauri and the shell.
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
  Linux/macOS/Windows Tauri installer packaging checks for the prebuilt DuckDB
  runtime, required `rust-workspace-tests` and `ui-quality` gates, and informational
  backend Rust coverage via `rust-coverage`. Rust build lanes stage DuckDB
  releases before Cargo, and installed-app acceptance covers app-relative
  `.so`, bundled `.dylib`, and colocated `.dll` loading. The `ci-workflow-parse` job also
  runs the standard-library-only roadmap and publish-doc contract tests before
  the Tauri test lanes.
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
- `scripts/prepare-tauri-duckdb-bundle.py` and
  `src-tauri/tauri.conf.json`: stage the verified prebuilt DuckDB runtime into
  the Tauri installer resource directory and set an app-relative loader path;
  CI builds and inspects Linux, macOS, and Windows packages.
- `scripts/detect-ci-scope.sh`: routes Rust manifest, lockfile, and workflow
  changes to the Rust workspace lanes; `scripts/test-ci-scope-contract.sh`
  covers those path classifications.

## Directory Map

| Directory | Responsibility Summary | Notes |
|---|---|---|
| `src-tauri/src/` | Backend service layer, command facade, storage boundaries, auth, scoring, telemetry, risk contracts, budget/fix-proposal/triage/verifier/convergence contracts, app support, baseline adapters, and the bulk-import telemetry surface tracked by the F-033 planning slice. | Tauri commands should stay thin and delegate into service/storage layers. `app_support.rs` owns the shared app builder and state. Storage opens retain the process-level ownership guard and use SQLite busy-timeout handling. |
| `src-tauri/crates/rocinante-storage/` | Host-neutral admin/scoring services, SQLite WAL ingestion, DuckDB analytics, and release-baseline authorization adapter. | Tauri re-exports its public storage API; both hosts use the same legacy working-directory defaults for analytics and scoring until BI-058 implements data-preserving migration. The separate analysis `telemetry.db` uses the per-user data directory with a legacy-file migration. DuckDB links only through the staged official prebuilt shared library. The isolated `tools/sled-migration` utility must run before upgrading an installation with legacy Sled data. |
| `src-tauri/crates/rocinante-desktop-shell/` | GTK-free native window, persisted navigation, keyboard routes, repository-folder selection, asynchronous scans, exact saved-metric reload, scan completion notifications, release-baseline query/reseed controls, a repository-metric dashboard view model, and ports of companion insight scoring, quality snapshot and audience-focus views, plus static sample accessibility, SEO, Drupal security, and performance panels, built with eframe/winit and serde_json. | The Dashboard summarizes repository, plugin, and metric data without inferring synthetic risk claims; sample insight and audit panel data are labeled; custom JSON can be applied/reset, and SEO scope/performance data selectors are present. React companion parity is implemented for all admin bridge commands; Alerts and Settings are placeholders and no fallback deferral is approved. The tray supports Show/Quit and hide-on-window-close. Linux URI metadata and per-user installation are implemented, with cold/warm/restart and notification-display acceptance wired into Ubuntu CI. macOS has a URL-scheme bundle and retained `NSAppleEventManager` `kAEGetURL` handler; its local acceptance installs a temporary bundle through the per-user installer and routes cold/warm URI-only Launch Services opens. Windows has current-user protocol registration and an icon-bearing Start Menu shortcut; its installer and cold/warm/restart URL acceptance are wired to a `windows-latest` CI job. A bounded warm-instance inbox and second-process forwarding test are implemented. Hosted cold/warm URL delivery and saved-state restart acceptance passed on Linux, macOS, and Windows in CI run `37215719759`, including visible Linux notifications. macOS first-frame visibility, minimized-window restoration, close-to-tray, Show/Quit handler behavior, notification request, and saved-state restart also passed locally on Darwin on 2026-09-30; macOS foreground activation remains unproven because strict mode measured `visible=true/frontmost=false`. Shared window, bundle, launcher, and protocol icons are packaged and contract-tested; visual icon checks remain open. Physical tray-menu clicks and Linux/Windows tray runtime remain unverified, as do visible macOS/Windows notification delivery and the file-import decision. The native macOS notification request returned success locally, but visibility was not observed. Login autostart is out of scope unless separately approved. Tauri remains the production entry point. |
| `src-tauri/crates/rocinante-analysis/` | Host-independent auth, repository discovery/analysis, telemetry persistence, and shared database-path configuration. | Owns the shared source modules; the Tauri crate re-exports these modules for its existing command layer. |
| `src-tauri/tests/` | Backend regression coverage for PR risk contracts, CI-gate comment contracts, publish-gate documentation contracts, incident-feedback contracts, storage behavior, admin-only flows, and registered-handler integration. | Tests should protect command wiring, release-gate docs, and storage invariants. |
| `ui/src/` | Frontend dashboard, bridge adapters, explainability panels, and quality-pulse rendering. | UI state should flow through the bridge adapters rather than direct runtime assumptions. |
| `ui/e2e/` | Browser-level smoke coverage for the Tauri bridge and user-visible flows. | Keeps the Playwright surface separate from unit tests. |
| `docs/` | Feature backlog, roadmap, test plan, publish-readiness checklist, and bead tracker artifacts. | This is the source of truth for phase sequencing and backlog accounting. |
| `scripts/` | Repo automation, local operational helpers, and verified native-library provisioning. | Prefer existing scripts over ad hoc shell snippets; keep CI scope, dependency-floor, advisory, and DuckDB source-build guards contract-tested. |

## Data and Control Flow

1. Roadmap docs define the active phase and bead backlog.
2. The Tauri backend exposes admin commands through `src-tauri/src/main.rs`
   and the service helpers in `src-tauri/src/admin.rs`.
3. CI uses a deterministic scope gate in `.github/workflows/ci.yml` to skip
   Rust-heavy test and lint lanes when only docs/metadata/non-Rust paths are
   changed.
4. The Tauri app builder in `src-tauri/src/app_support.rs` owns the registered
   handler table and shared `AppState` wiring.
5. Storage and auth layers validate access before mutating persistence or
   reading protected state; release baseline operations flow through the
   baseline adapter rather than directly through the broader store.
   The Tauri app refuses startup when `RUNICIPAL_TOKEN_SECRET` is missing or
   shorter than 32 bytes, and the native scan service applies the same check.
6. Budget guard loops enforce report-only and kill-switch behavior before
   broader automation continues.
7. Triage loops enforce report-only formatting with high-priority, watch, noise,
   and state-updates sections.
8. Verifier loops enforce reject-by-default behavior and require evidence before
   approval.
9. Stage 3 convergence now has an explicit roadmap-coherence validator so
   release-gate collapse only happens when test mappings and phase gates are present.
10. Fix-proposal loops enforce one-problem remediation and retry caps before
   escalation with full context.
11. The GTK-free migration plan adds a phase-gated host path for parity,
    core extraction, native shell MVP, fallback containment, and dependency
    removal without reintroducing GTK/GLib.
12. The Dependabot remediation plan records the `esbuild` floor and alert-check evidence and the
    remaining open alert snapshot that blocks release.
13. The React UI invokes the bridge through `ui/src/tauri-admin.ts`, the
    extracted command bridge shell in `ui/src/admin-bridge-panel.tsx`, and
    renders results in `ui/src/App.tsx`.
14. Unit, integration, and e2e tests validate the command facade, the bridge
    seam, and the browser-visible behavior.
15. The Tauri adapter delegates extracted domain behavior to `rocinante-core`;
    release-baseline persistence is shared through `rocinante-storage`, while `rocinante-analysis`
    owns the shared auth, analysis, Git, and telemetry modules re-exported by
    the Tauri adapter.
16. The GTK-free shell uses eframe/winit and calls `rocinante-analysis` for
    admin-authorized scans and saved-metric queries, and `rocinante-storage`
    for SQLite WAL ingestion, DuckDB analytics, and release-baseline operations.
    DuckDB is dynamically linked from a checksum-verified official binary;
    installed packages use an app-relative Linux path, a macOS Frameworks
    path, or a Windows-adjacent DLL. The shell parses cold-launch
    `rocinante://repository/open?path=...` arguments and forwards links and
    activation requests through a per-user inbox. The macOS installed
    acceptance passes cold/warm Launch Services URLs, tray actions, notification
    request, and saved-state restart locally. Linux acceptance also exercises
    visible notification delivery under Xvfb/D-Bus. Hosted run `37198628846`
    passes Linux/macOS/Windows URL and saved-state restart acceptance, including
    Linux visible notifications and packaged DuckDB loading. SQLite ingestion,
    legacy Sled migration and Tauri/GTK retirement are phase-gated in the
    RustSec remediation plan.

## Governance and Execution Snapshot

- PR #108 branch `fix/rocinante-readiness-remediation` remains relevant and open. Its latest hosted code-validation head is `1be093a`; all 47 inline review threads are resolved. CI run [37235903810](https://github.com/pratik-saptarshi/rocinante/actions/runs/37235903810) passed Rust, UI, lifecycle, package, and contract lanes. At that head the governance registry still contained 17 overdue entries, so `security-exception-governance` and aggregate `test` failed. Security run [37235903812](https://github.com/pratik-saptarshi/rocinante/actions/runs/37235903812) and Dependency Review [37235903871](https://github.com/pratik-saptarshi/rocinante/actions/runs/37235903871) passed.
- A local, user-authorized update removes 15 advisory IDs from both `.cargo/audit.toml` and `docs/roadmap/security-advisory-exceptions.json`: eight records were withdrawn upstream and seven affected packages are absent from both supported lockfiles. Evidence is in `docs/roadmap/rustsec-exception-closure-evidence-2026-10-04.md`. Hosted validation of this update is pending.
- The two current app warnings remain in both registries: `RUSTSEC-2024-0370` (`proc-macro-error 1.0.4`) and `RUSTSEC-2024-0429` (`glib 0.18.5`). Their existing review dates are overdue. The fail-closed governance check correctly remains red until security-owner dispositions are supplied; no acceptance or date renewal was inferred.
- Local advisory contract coverage now computes the overdue-count message from the registry size; the checker contract and roadmap-doc contracts pass. The configured audit ignore list and registry contain the same two live IDs. The unfiltered app audit has zero vulnerability findings and exactly two warnings; the isolated migrator audit has no findings or warnings.
- DuckDB remains an official checksum-verified prebuilt. CI stages it before Tauri resource validation, the source-build feature guard rejects compilation from source, and the macOS package check rejects any remaining `duckdb-download` RPATH.
- Legacy Sled migration uses the separate audited workspace under `tools/sled-migration`; it copies `conf`, `db`, and `blobs/` while holding the storage lock. The 1 MiB blob-backed regression verifies migrated bytes and unchanged source files. Hosted workspace tests, core/storage shards, formatting, Clippy, UI quality, platform lifecycle, and all three Tauri package checks passed on `1be093a` in run `37235903810`.
- The pinned UI tool is pnpm 12.9.1. The latest completed hosted run passed typecheck, all 62 unit tests, and production build.
- Historical status: BI-047 merged on PR #85; the CI recovery and lane slices passed PR run 28987645462. These results do not establish readiness for PR #108.
- Historical status: BI-047 merged on PR #85; the CI recovery and lane slices passed PR run 28987645462. Those results do not establish readiness for PR #108.

## Design Patterns

Recent PR #108 review fixes also route the Tauri runtime bundle matrix through
the aggregate gate, route Rust toolchain and Cargo config edits to Rust checks,
stage the checksum-verified DuckDB binary before Tauri resource validation,
sign the macOS app after loader rewrites, and key stored repository metrics
with a sanitized label plus stable local path hash. Reads use legacy basename
or tree-relative aliases only when the stable identity has no matching rows
for the requested release, avoiding duplicate metrics during upgrade. The
analysis `telemetry.db` migrates to per-user storage; analytics and scoring
defaults retain their legacy working-directory paths until BI-058 defines and
validates migration, backup, conflict, and rollback behavior.

- Command facade for Tauri invocation.
- Service layer separation between command wrappers and storage logic.
- Adapter boundary for the UI bridge.
- Dual-layer persistence for ingest and analytics responsibilities.
- Contract-driven testing for admin workflows and roadmap-backed behavior.
