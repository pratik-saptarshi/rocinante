# Repository Atlas: rocinante

## Project Responsibility

Rocinante is a cross-language planning and execution workspace for AI quality checking, dashboarding, and loop-engineering controls. The repository combines a Rust/Tauri backend, a React/Vite UI, and documentation-led backlog and roadmap artifacts.

## System Entry Points

- `src-tauri/src/main.rs`: Tauri command registration and application state wiring.
- `src-tauri/src/lib.rs`: backend crate surface and shared module exports.
- `ui/src/App.tsx`: dashboard shell and admin bridge consumer.
- `docs/feature-list.html`: feature backlog with acceptance criteria and bead linkage.
- `docs/product-roadmap.html`: stage ordering and release-gate sequencing.
- `docs/roadmap/bead-issue-tracker.html`: execution ledger for active bead issues.

## Directory Map

| Directory | Responsibility Summary | Notes |
|---|---|---|
| `src-tauri/src/` | Backend service layer, command facade, storage boundaries, auth, scoring, telemetry, risk contracts, app support, and baseline adapters. | Tauri commands should stay thin and delegate into service/storage layers. `app_support.rs` owns the shared app builder and state. |
| `src-tauri/tests/` | Backend regression coverage for command contracts, storage behavior, admin-only flows, and registered-handler integration. | Tests should protect command wiring and storage invariants. |
| `ui/src/` | Frontend dashboard, bridge adapters, explainability panels, and quality-pulse rendering. | UI state should flow through the bridge adapters rather than direct runtime assumptions. |
| `ui/e2e/` | Browser-level smoke coverage for the Tauri bridge and user-visible flows. | Keeps the Playwright surface separate from unit tests. |
| `docs/` | Feature backlog, roadmap, test plan, publish-readiness checklist, and bead tracker artifacts. | This is the source of truth for phase sequencing and backlog accounting. |
| `scripts/` | Repo automation and local operational helpers. | Prefer existing scripts over ad hoc shell snippets. |

## Data and Control Flow

1. Roadmap docs define the active phase and bead backlog.
2. The Tauri backend exposes admin commands through `src-tauri/src/main.rs` and the service helpers in `src-tauri/src/admin.rs`.
3. The Tauri app builder in `src-tauri/src/app_support.rs` owns the registered handler table and shared `AppState` wiring.
4. Storage and auth layers validate access before mutating persistence or reading protected state; release baseline operations flow through the baseline adapter rather than directly through the broader store.
5. The React UI invokes the bridge through `ui/src/tauri-admin.ts` and renders the results in `ui/src/App.tsx`.
6. Unit, integration, and e2e tests validate the command facade, the bridge seam, and the browser-visible behavior.

## Design Patterns

- Command facade for Tauri invocation.
- Service layer separation between command wrappers and storage logic.
- Adapter boundary for the UI bridge.
- Dual-layer persistence for ingest and analytics responsibilities.
- Contract-driven testing for admin workflows and roadmap-backed behavior.
