# src-tauri/

## Responsibility
Backend runtime package for the Tauri application. Owns command handlers,
auth validation, storage coordination, scoring, telemetry, and analysis plugin
modules.

## Design
Layered command architecture:
- `main.rs` bootstraps Tauri and registers the command surface.
- `admin.rs` enforces authorization and storage-route policy.
- `storage.rs` owns the dual-layer persistence engine.
- `scoring.rs`, `team_policies.rs`, and `telemetry.rs` encapsulate domain
  rules and data shaping.
- `plugins/` hosts isolated analysis and sanitization implementations.

## Flow
1. UI invokes a Tauri command.
2. `main.rs` routes to `admin.rs`.
3. `admin.rs` decodes the principal, checks role/routing constraints, and
   opens the storage backend.
4. `storage.rs` persists or queries the appropriate store tier.
5. Results return through the command handler to the frontend bridge.

## Integration
- Consumed by `ui/src/tauri-admin.ts` via the desktop invoke bridge.
- Depends on `Cargo.toml` dependencies, `src-tauri/src/plugins/`, and the
  `tests/` suite for policy and storage validation.
