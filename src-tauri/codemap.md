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

- Local Rust formatting passes with `rtk cargo fmt --manifest-path src-tauri/Cargo.toml --all -- --check`; the isolated Sled migrator formatting check also passes.
- The desktop-shell suite passes 52 tests across 9 suites. The full serial workspace passes 270 tests across 64 suites, and warning-denied workspace Clippy passes locally.
- Hosted CI on source head `fe5b2eb` failed macOS URL dispatch. Its log records `quit_control_received=true`, `quit_action_started=true`, and a resident process after the tray icon was dropped and the viewport was closed.
- The current candidate routes tray Quit, Command-Q, and in-app Quit through one helper. It synchronously saves and flushes state, drops the tray icon, then uses `std::process::exit(0)` on macOS; Linux and Windows retain eframe viewport close. Ordinary close-to-tray still hides the window. This deliberately skips macOS destructors and shutdown hooks. Same-head hosted CI, Security, Dependency Review, and packaged lifecycle remain pending; prior green results do not validate this candidate.
- The latest local installed lifecycle attempt was inconclusive: Launch Services refused the temporary bundle (`-10822`). A separate standalone executable attempt aborted before startup because dyld could not load `@rpath/libduckdb.dylib` (`no LC_RPATH`); it did not exercise the app UI. A later direct execution of the signed bundle binary also aborted in AppKit during `NSApplication::sharedApplication` from `macos_url::register`, before a window appeared. That shell-launched attempt bypassed Launch Services inside this sandbox and does not validate a user-installed app. Do not mark packaged macOS acceptance complete.
- The manual Show attempt after `2fac448` timed out at the Show prompt without observing its callback. Physical Show/Quit and foreground acceptance remain open.
- DuckDB source-build features remain prohibited; CI stages official checksum-verified binaries before building or packaging.
