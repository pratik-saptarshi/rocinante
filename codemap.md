# Repository Atlas: rocinante

## Project Responsibility
Rocinante is a cross-language planning and execution workspace for AI quality
checking, dashboarding, and loop-engineering controls. The repository combines
a Rust/Tauri backend, a React/Vite UI, and documentation-led backlog and
roadmap artifacts.

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
- `src-tauri/crates/rocinante-storage/src/lib.rs`: shared Sled/DuckDB persistence
  and admin-authorized release-baseline operations used by Tauri and the shell.
- `src-tauri/crates/rocinante-desktop-shell/src/main.rs`: GTK-free native shell
  entry point using eframe/winit, with native UI work tracked by BI-049.
- `src-tauri/src/ci_gate.rs`: PR-risk CI comment formatter and merge-block
  decision helper.
- `src-tauri/src/incident_feedback.rs`: incident and annotation feedback ledger
  with cache invalidation and auditable risk raises.
- `.github/workflows/ci.yml`: CI contract for Rust formatting, linting,
  checking, full-workspace crate tests, current advisory exception review-date enforcement,
  Linux URI/notification acceptance plus Windows and macOS URL/restart acceptance jobs,
  required `rust-workspace-tests` and `ui-quality` gates, and informational
  backend Rust coverage via `rust-coverage`. The `ci-workflow-parse` job also
  runs the standard-library-only roadmap and publish-doc contract tests before
  the Tauri test lanes.
- `ui/src/App.tsx`: dashboard shell and admin bridge consumer.
- `ui/src/admin-bridge-panel.tsx`: extracted command-bridge control block.
- `ui/package.json`: `pnpm@12.8.2` UI manifest and test/build entry points.
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

## Directory Map

| Directory | Responsibility Summary | Notes |
|---|---|---|
| `src-tauri/src/` | Backend service layer, command facade, storage boundaries, auth, scoring, telemetry, risk contracts, budget/fix-proposal/triage/verifier/convergence contracts, app support, baseline adapters, and the bulk-import telemetry surface tracked by the F-033 planning slice. | Tauri commands should stay thin and delegate into service/storage layers. `app_support.rs` owns the shared app builder and state. Storage opens keep the process-level ownership guard and tolerate transient sled close/reopen lock lag. |
| `src-tauri/crates/rocinante-storage/` | Host-neutral admin/scoring services, Sled/DuckDB storage, and release-baseline authorization adapter. | Tauri re-exports its public storage API; both hosts use shared path defaults, writer locking, and snapshot refresh behavior. |
| `src-tauri/crates/rocinante-desktop-shell/` | GTK-free native window, persisted navigation, keyboard routes, repository-folder selection, asynchronous scans, exact saved-metric reload, scan completion notifications, release-baseline query/reseed controls, a repository-metric dashboard view model, and ports of companion insight scoring, quality snapshot and audience-focus views, plus static sample accessibility, SEO, Drupal security, and performance panels, built with eframe/winit and serde_json. | The Dashboard summarizes repository, plugin, and metric data without inferring synthetic risk claims; sample insight and audit panel data are labeled; custom JSON can be applied/reset, and SEO scope/performance data selectors are present. React companion parity is implemented for all admin bridge commands; Alerts and Settings are placeholders and no fallback deferral is approved. The tray supports Show/Quit and hide-on-window-close. Linux URI metadata and per-user installation are implemented, with cold/warm/restart and notification-display acceptance wired into Ubuntu CI. macOS has a URL-scheme bundle and retained `NSAppleEventManager` `kAEGetURL` handler; its local acceptance installs a temporary bundle through the per-user installer and routes cold/warm URI-only Launch Services opens. Windows has current-user protocol registration and an icon-bearing Start Menu shortcut; its installer and cold/warm/restart URL acceptance are wired to a `windows-latest` CI job. A bounded warm-instance inbox and second-process forwarding test are implemented. macOS cold/warm URI, first-frame visible startup, minimized-window visible restoration, close-to-tray, Show restoration, Quit, native notification request, and saved-state restart acceptance passed locally on Darwin on 2026-09-30; macOS activation is requested through `NSApplication` and `NSRunningApplication`; the acceptance helper now measures frontmost state and strict mode reported visible=true/frontmost=false in this automation session, so foreground activation remains unproven. Hosted lifecycle validation remains open. Shared window, bundle, launcher, and protocol icons are packaged and contract-tested; visual launch checks remain open. The installed Darwin acceptance exercises Show/Quit through the same action handler used by tray callbacks; physical menu-click delivery and Linux/Windows tray runtime remain unverified, along with Linux notification hosted results, visible macOS/Windows notification delivery, hosted URL results, and the unresolved file-import decision. The native macOS notification request returned success locally, but visibility was not observed. Login autostart is out of scope unless separately approved. Tauri remains the production entry point. |
| `src-tauri/crates/rocinante-analysis/` | Host-independent auth, repository discovery/analysis, telemetry persistence, and shared database-path configuration. | Owns the shared source modules; the Tauri crate re-exports these modules for its existing command layer. |
| `src-tauri/tests/` | Backend regression coverage for PR risk contracts, CI-gate comment contracts, publish-gate documentation contracts, incident-feedback contracts, storage behavior, admin-only flows, and registered-handler integration. | Tests should protect command wiring, release-gate docs, and storage invariants. |
| `ui/src/` | Frontend dashboard, bridge adapters, explainability panels, and quality-pulse rendering. | UI state should flow through the bridge adapters rather than direct runtime assumptions. |
| `ui/e2e/` | Browser-level smoke coverage for the Tauri bridge and user-visible flows. | Keeps the Playwright surface separate from unit tests. |
| `docs/` | Feature backlog, roadmap, test plan, publish-readiness checklist, and bead tracker artifacts. | This is the source of truth for phase sequencing and backlog accounting. |
| `scripts/` | Repo automation and local operational helpers. | Prefer existing scripts over ad hoc shell snippets; include dependency-floor checks for security posture. |

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
    admin-authorized scans and exact saved-metric queries, and `rocinante-storage`
    for Sled/DuckDB administration and release-baseline operations; both hosts resolve the
    same app-data telemetry DB. It parses cold-launch
    `rocinante://repository/open?path=...` arguments and uses an exclusive-lock
    per-user inbox to forward validated links and activation requests from second
    launches. Linux and macOS per-user installers register the URI handler, and a
    Windows PowerShell installer writes the current-user protocol key. The native
    Dashboard summarizes real saved metrics. Linux URI and notification display
    acceptance runs under Xvfb and D-Bus; Windows and macOS URI/restart acceptance
    is also wired into CI. Hosted runtime results, visible macOS/Windows notification
    delivery, remaining dashboard parity, physical tray-menu click delivery, Linux/Windows tray runtime validation, visual icon launch validation, and full lifecycle parity remain in BI-049; Darwin acceptance now covers close-to-tray, Show restoration, and Quit through the shared action handler. `AXIsProcessTrusted` returned false for the current automation process, so physical menu clicks require a separate Accessibility-authorized interactive run. File-selection scope is unresolved; F-027/F-033 backend telemetry import does not define a desktop chooser flow.

## Governance and Execution Snapshot

- Active bead slices: `BI-046` and `BI-049`,
  and governance doc updates in `docs/feature-list.html`, `docs/bill-of-materials.html`,
  and `docs/publish-readiness-checklist.html`.
- CI recovery slices `BI-053`, `BI-054`, `BI-055`, `BI-056`, and `BI-057` are complete; the latest
  green remote run is `28987645462`.
- Active security slice: `RT-RC-001` (GTK/GLib dependency floor) and five
  unrelated Dependabot alerts open on `main` and recorded in the release checklist. A fresh
  live query on 2026-09-30 confirmed the tracked esbuild alert is closed.
- The latest hosted Rust audit failed on 2026-09-28 with vulnerable `rustls`
  and `rkyv` versions plus yanked `chacha20`; the working lockfile remediation
  also resolves the open `serde_with` alert to 3.21.0 and passes the documented
  root `cargo audit --file src-tauri/Cargo.lock --deny warnings`; hosted validation
  is pending, and the live alert remains open on `main` until the lockfile change merges.
- Current local signal: CI includes a top-level `test` aggregate gate for branch
  protection, and the local UI lockfile resolves `esbuild` at `0.28.1`; the Node
  floor script passes under a temporary Node 22 runtime. The working UI lockfile also resolves
  Vitest, `@vitest/mocker`, and `@vitest/coverage-v8` to 4.1.11 and PostCSS to 8.5.23; all five unrelated alert fixes
  are pending merge. UI tests and coverage pass (62 tests), the production build passes, and pnpm audit reports no known vulnerabilities. The merged CI lane contract materializes delta
  lanes from scope outputs with PR run `28987645462`.
- Sync signal: local `main` is aligned with fetched `origin/main` at `4c28d9f`
  (2026-09-30); the previous eight-commit local history is preserved at
  `backup/main-pre-reconcile-20260930`. The TruffleHog pin matches upstream at
  `v3.95.9`. BI-047 merged on PR #85. BI-048 core
  extraction is complete; BI-049 has authenticated scans, saved-metric reload,
  GTK-free tray Show/Quit, and close-to-hide policy, with broader parity still open; BI-046 review dates remain overdue.
  PR #59 patched esbuild; a fresh live query confirmed its alert is closed, while the five unrelated alerts remain open on `main` pending working-tree fixes merging.
- Publish status: blocked by open security advisory exceptions and unresolved host-
  migration + release-gating parity tasks.

## Design Patterns

- Command facade for Tauri invocation.
- Service layer separation between command wrappers and storage logic.
- Adapter boundary for the UI bridge.
- Dual-layer persistence for ingest and analytics responsibilities.
- Contract-driven testing for admin workflows and roadmap-backed behavior.
