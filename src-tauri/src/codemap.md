# src-tauri/src/

## Responsibility
Core backend modules for command execution, storage, policy enforcement,
repository discovery, scoring, telemetry, and error modeling.

## Design
The folder is organized by bounded concern:
- `admin.rs`: service layer for command handlers.
- `auth.rs`: principal decoding and RBAC checks.
- `storage.rs`: dual-layer persistence and query routing.
- `engine.rs`: pipeline orchestration for repository analysis.
- `scoring.rs` and `team_policies.rs`: deterministic ranking policy.
- `git.rs` / `git_providers.rs`: repository discovery and provider adapters.
- `plugins/`: analysis helpers, parser cache, and sanitizer rules.
- `types.rs` and `errors.rs`: shared data contracts and error enums.

## Flow
1. `main.rs` constructs application state and command arguments.
2. `admin.rs` validates token and storage route before any mutation/query.
3. `storage.rs` persists events to the correct tier and promotes snapshots.
4. `engine.rs` and `plugins/` generate analysis inputs for scoring modules.
5. Results are marshaled back through typed structs in `types.rs`.

## Integration
- Called by the Tauri runtime and exercised by the backend test suite.
- Shared across the whole backend via module imports from `lib.rs`.
