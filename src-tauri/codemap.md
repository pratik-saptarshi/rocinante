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

- Workspace formatting, all-target checks, and test-target compilation pass
  locally; full workspace tests are being rerun after contract updates.
- The dependency guard requires GTK, GLib, Wry, Tauri runtime, and tracked
  advisory packages to be absent from supported workspace graphs.
- CI defines required Linux, macOS, and Windows native-shell package and
  lifecycle checks. Hosted evidence on this Tauri-retirement worktree is
  pending.
- DuckDB source-build features remain prohibited; CI stages official
  checksum-verified binaries before building or packaging.
