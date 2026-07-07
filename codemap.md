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
- `src-tauri/src/ci_gate.rs`: PR-risk CI comment formatter and merge-block
  decision helper.
- `src-tauri/src/incident_feedback.rs`: incident and annotation feedback ledger
  with cache invalidation and auditable risk raises.
- `.github/workflows/ci.yml`: CI contract for Rust formatting, linting,
  checking, tests, and informational backend Rust coverage via `rust-coverage`.
- `ui/src/App.tsx`: dashboard shell and admin bridge consumer.
- `ui/src/admin-bridge-panel.tsx`: extracted command-bridge control block.
- `ui/package.json`: `pnpm@11.4.0` UI manifest and test/build entry points.
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
- `docs/roadmap/dependabot-esbuild-remediation-plan.html`: targeted remediation
  plan for the current remote Dependabot esbuild alert.
- `docs/roadmap/security-advisory-exceptions.json`: machine-readable registry
  for time-boxed audit ignores and host-floor governance.
- `scripts/dependency-floor-proof.sh`: dependency-floor proof command for GTK
  and GLib transitive-path validation.

## Directory Map

| Directory | Responsibility Summary | Notes |
|---|---|---|
| `src-tauri/src/` | Backend service layer, command facade, storage boundaries, auth, scoring, telemetry, risk contracts, budget/fix-proposal/triage/verifier/convergence contracts, app support, baseline adapters, and the bulk-import telemetry surface tracked by the F-033 planning slice. | Tauri commands should stay thin and delegate into service/storage layers. `app_support.rs` owns the shared app builder and state. Storage opens keep the process-level ownership guard and tolerate transient sled close/reopen lock lag. |
| `src-tauri/tests/` | Backend regression coverage for PR risk contracts, CI-gate comment contracts, publish-gate documentation contracts, incident-feedback contracts, storage behavior, admin-only flows, and registered-handler integration. | Tests should protect command wiring, release-gate docs, and storage invariants. |
| `ui/src/` | Frontend dashboard, bridge adapters, explainability panels, and quality-pulse rendering. | UI state should flow through the bridge adapters rather than direct runtime assumptions. |
| `ui/e2e/` | Browser-level smoke coverage for the Tauri bridge and user-visible flows. | Keeps the Playwright surface separate from unit tests. |
| `docs/` | Feature backlog, roadmap, test plan, publish-readiness checklist, and bead tracker artifacts. | This is the source of truth for phase sequencing and backlog accounting. |
| `scripts/` | Repo automation and local operational helpers. | Prefer existing scripts over ad hoc shell snippets. |

## Data and Control Flow

1. Roadmap docs define the active phase and bead backlog.
2. The Tauri backend exposes admin commands through `src-tauri/src/main.rs`
   and the service helpers in `src-tauri/src/admin.rs`.
3. The Tauri app builder in `src-tauri/src/app_support.rs` owns the registered
   handler table and shared `AppState` wiring.
4. Storage and auth layers validate access before mutating persistence or
   reading protected state; release baseline operations flow through the
   baseline adapter rather than directly through the broader store.
5. Budget guard loops enforce report-only and kill-switch behavior before
   broader automation continues.
6. Triage loops enforce report-only formatting with high-priority, watch, noise,
   and state-updates sections.
7. Verifier loops enforce reject-by-default behavior and require evidence before
   approval.
8. Stage 3 convergence now has an explicit roadmap-coherence validator so
   release-gate collapse only happens when test mappings and phase gates are present.
9. Fix-proposal loops enforce one-problem remediation and retry caps before
   escalation with full context.
10. The GTK-free migration plan adds a phase-gated host path for parity,
    core extraction, native shell MVP, fallback containment, and dependency
    removal without reintroducing GTK/GLib.
11. The Dependabot remediation plan tracks the live `esbuild` alert, lockfile
    upgrade target, and release gate closure for the UI toolchain.
12. The React UI invokes the bridge through `ui/src/tauri-admin.ts`, the
    extracted command bridge shell in `ui/src/admin-bridge-panel.tsx`, and
    renders results in `ui/src/App.tsx`.
13. Unit, integration, and e2e tests validate the command facade, the bridge
    seam, and the browser-visible behavior.

## Design Patterns

- Command facade for Tauri invocation.
- Service layer separation between command wrappers and storage logic.
- Adapter boundary for the UI bridge.
- Dual-layer persistence for ingest and analytics responsibilities.
- Contract-driven testing for admin workflows and roadmap-backed behavior.
