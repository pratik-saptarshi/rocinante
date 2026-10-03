# rocinante-storage

## Responsibility

Host-neutral admin/service and persistence layer shared by the Tauri app and
the GTK-free native shell. It owns the Sled/DuckDB store, scoring operations,
and the companion admin command bridge.

## Modules and contracts

- `src/storage.rs` contains the storage engine, Sled ownership lock, DuckDB
  lifecycle/snapshot management, and `BaselineStore` adapter.
- `src/admin.rs` contains shared authorization, storage, scoring, and risk
  command services. `execute_admin_bridge_command` dispatches the eight
  companion command names using editable JSON payloads.
- `src/scoring.rs` owns score configuration persistence and audit logging.
- `src/lib.rs` exposes the same default storage paths to both hosts and applies
  configured-secret, JWT, and admin-role checks before bridge operations.
- Tauri's `src/storage.rs` is a compatibility re-export; its admin baseline
  commands delegate to this crate. The native shell uses the same API directly.

## Validation

- The crate test `shared_baseline_adapter_roundtrips` validates baseline storage
  on the shared adapter.
- Tauri integration tests continue to cover authorized baseline commands and
  rejection of non-admin principals.
