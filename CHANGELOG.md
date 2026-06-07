# Changelog

## 0.2.0 - 2026-06-07

### Added
- BDD/TDD roadmap completion coverage for strict Badger sidecar conformance,
  sharded ingestion keys, retention jobs, immutable analytics snapshots,
  signed scoring policies, team policy profiles, AST metrics, observability,
  enterprise provider guards, directory role mapping, bulk import dedupe, and
  admin UI maturity controls.
- Deeper integration coverage for Unix sidecar transport, promoted DuckDB
  snapshot materialization, and raw retention pruning after promotion.
- CI coverage gate using `cargo llvm-cov` with an 85% core line coverage floor.

### Changed
- Scoring policy signatures now use SHA-256 hex digests for tamper evidence.
- Strict Badger production mode rejects `inproc://` fallback endpoints; existing
  in-process tests now explicitly model dev-mode sidecar simulation.

### Notes
- CI coverage excludes `src-tauri/src/main.rs` because it is a Tauri bootstrap
  shell with runtime wiring and no practical unit surface. Core logic remains
  covered through `repo_analyzer_core` modules and integration tests.
