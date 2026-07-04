# src-tauri/tests/

## Responsibility
Backend regression and contract-validation suite for auth, storage, scoring,
admin commands, and plugin behavior.

## Design
Tests are organized by subsystem and scenario:
- command-layer and service-level checks for admin handlers
- storage and promotion invariants for the dual-layer engine
- scoring and policy correctness checks
- plugin and sanitizer edge-case coverage

## Flow
1. Arrange fixtures and sample payloads.
2. Call the target backend function or command path.
3. Assert state transitions, routing constraints, and emitted values.
4. Repeat against regression scenarios to guard the release surface.

## Integration
- Executed through `cargo test --manifest-path src-tauri/Cargo.toml`.
- Validates the modules documented in `src-tauri/src/codemap.md`.
