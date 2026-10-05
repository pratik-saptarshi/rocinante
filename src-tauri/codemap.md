# src-tauri/

## Responsibility

This directory contains the Rust service workspace and the supported native
desktop host. Despite the historical directory name, the Tauri runtime is
removed from the remediation worktree. `rocinante-core`,
`rocinante-analysis`, and `rocinante-storage` provide host-neutral contracts,
authenticated repository analysis, and persistence/admin services.

## Design

- `crates/rocinante-core/` owns shared domain contracts.
- `crates/rocinante-analysis/` owns authenticated scanning, repository
  discovery, plugins, telemetry, and per-user analysis database paths.
- `crates/rocinante-storage/` owns SQLite WAL ingestion, DuckDB analytics,
  admin operations, and release-baseline persistence.
- `crates/rocinante-desktop-shell/` is the eframe/winit native application.
  It calls shared Rust services directly and owns window, URL, notification,
  and platform packaging behavior.
- `src/` contains remaining service modules and compatibility facades.
  `command_compat.rs` records the 11 public command names and wire shapes
  from the retired host; `lib.rs` re-exports this module at the former
  `tauri_commands` Rust path for source compatibility. No IPC is registered.
- `tools/sled-migration/` is an isolated audited one-time reader for existing
  Sled data. It is not part of the application runtime dependency graph.

## Data Flow

1. The desktop shell authenticates admin actions at the shared service
   boundary and calls analysis/storage APIs directly.
2. Ingestion writes SQLite WAL records; DuckDB handles analytics using an
   official checksum-verified prebuilt shared library.
3. Existing Sled stores must be migrated by the isolated tool before upgrade.
4. The React/Vite UI is a browser preview and validation surface, not a
   production desktop transport.

## Validation

- Workspace formatting, warning-denied all-target/all-feature Clippy, and the
  full serial test suite pass locally (270 tests across 64 suites); the
  security-advisory contract suite passes 5/5 after updating stale
  documentation assertions. The latest targeted documentation checks pass
  both publish/security integration suites (8 tests total), and the roadmap
  contracts pass 10/10.
- The dependency guard requires GTK, GLib, Wry, Tauri runtime, and tracked
  advisory packages to be absent from supported workspace graphs.
- CI defines required Linux, macOS, and Windows native-shell package and
  lifecycle checks. On PR #112 documentation head `2fac448`, CI run
  `37346608465` passed the aggregate, core/storage/full-workspace tests,
  formatting, Clippy, UI checks, security governance, platform packages and
  lifecycle, Windows registration, Linux notifications, and dependency-floor
  contracts. Security run `37346608399` passed RustSec audit, CodeQL, and
  secret scan; Dependency Review run `37346608411` passed. Earlier failures
  on `37317927760` and `496e1ec` were superseded by these green same-head runs.
- The physical macOS tray Show/Quit acceptance remains open. The most recent
  manual run after code head `2fac448` timed out at the Show prompt without
  observing its callback. The validation script also requires a Quit-action
  witness before it accepts process exit.
- DuckDB source-build features remain prohibited; CI stages official
  checksum-verified binaries before building or packaging.
