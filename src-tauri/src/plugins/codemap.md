# src-tauri/src/plugins/

## Responsibility
Analysis plugin subsystem for sanitization, parser caching, complexity proxy
metrics, PR approval telemetry, and velocity heuristics.

## Design
Plugins are organized as deterministic helpers rather than runtime-loaded
extensions. Each module focuses on one concern:
- `sanitizer.rs`: pattern-based redaction and metric scrubbing.
- `parser.rs`: language detection, digesting, and incremental cache support.
- `complexity.rs`: complexity estimation heuristics.
- `pr_approval.rs`: approval-fidelity scoring utilities.
- `velocity.rs`: commit/churn velocity helpers.
- `code_quality.rs`: lint-proxy quality signals.

## Flow
1. Backend pipeline selects plugin helpers based on analysis mode.
2. Sanitizer runs before metrics are persisted or surfaced.
3. Parser and heuristic modules convert source and commit metadata into
   normalized metrics.
4. Resulting plugin outputs feed the storage and scoring layers.

## Integration
- Imported by `src-tauri/src/engine.rs`, `storage.rs`, and `admin.rs`.
- Covered by plugin-specific tests under `src-tauri/tests/`.
