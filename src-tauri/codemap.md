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

- Final PR #112 source head `bb2d1700e4ae0c1663df140336f2f2ededbd0963` passed aggregate CI run `37417428474`, Security run `37417428663`, and Dependency Review run `37417428598`. These checks cover the current hard-exit source; earlier failure on `d598d59` and green checks on `2fac448` are historical.
- The macOS explicit quit helper synchronously persists shell state and flushes storage, drops the tray icon, then calls `std::process::exit(0)`; Linux and Windows retain eframe viewport close. Ordinary close-to-tray still hides the window.
- The manual Show attempt after `2fac448` timed out without a witnessed callback and predates the current hard-exit source. Physical Show/Quit, foreground restoration, and current installed-app stability remain unverified.
- Earlier Launch Services, dyld, and AppKit failures occurred in sandboxed or direct-launch paths and did not validate a user-installed application. Capture fresh launch logs and crash evidence if the current installed app does not start cleanly.
- DuckDB source-build features remain prohibited; CI stages the official checksum-verified prebuilt library before packaging.
