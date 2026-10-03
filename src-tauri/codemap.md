# src-tauri/

## Responsibility
Backend runtime package for the Tauri application. Owns command handlers,
Tauri app state, storage coordination, scoring, and host-specific adapters.
The `rocinante-analysis` crate owns shared auth, Git, telemetry, and repository
analysis modules used by both Tauri and the native shell. `rocinante-storage`
owns the shared Sled/DuckDB persistence adapter and release-baseline operations.

## Design
Layered command architecture:
- `main.rs` bootstraps Tauri and registers the command surface.
- `admin.rs` re-exports the shared admin service API.
- `storage.rs` re-exports the host-neutral `rocinante-storage` implementation.
- `crates/rocinante-desktop-shell/src/dashboard_insights.rs` ports the React
  companion's deterministic insight score and JSON payload contract for the
  GTK-free native Dashboard, including quality snapshot and stakeholder focus views.
- `crates/rocinante-desktop-shell/src/dashboard_reference.rs` provides static
  sample accessibility, SEO, Drupal security, and performance panels and their
  selectors for the GTK-free Dashboard.
- `scoring.rs` and `storage.rs` re-export the shared service and persistence
  modules; `team_policies.rs` remains Tauri-side.
- `crates/rocinante-analysis/` owns repository scanning, token validation,
  telemetry persistence, and analysis plugins.
- `crates/rocinante-storage/` owns admin/scoring services, the dual-layer
  Sled/DuckDB store, baseline persistence, and command dispatch; Tauri and the
  native shell call the same implementation.

## Flow
1. UI invokes a Tauri command.
2. `main.rs` routes to `admin.rs`.
3. `admin.rs` decodes the principal, checks role/routing constraints, and
   opens the storage backend.
4. `storage.rs` persists or queries the appropriate store tier.
5. Results return through the command handler to the frontend bridge.

## Integration
- Consumed by `ui/src/tauri-admin.ts` via the desktop invoke bridge.
- Depends on `Cargo.toml`, `crates/rocinante-analysis/`, and the `tests/` suite
  for policy and storage validation.
