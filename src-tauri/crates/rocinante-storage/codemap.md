# rocinante-storage

## Responsibility

Host-neutral admin/service and persistence layer shared by the Tauri app and
the GTK-free native shell. It owns SQLite ingestion, DuckDB analytics, scoring
operations, and the companion admin command bridge. Legacy Sled reading is
isolated in `tools/sled-migration`; this crate never opens or links Sled.

## Modules and contracts

- `src/storage.rs` contains the SQLite ingestion engine, shared DuckDB
  lifecycle/snapshot management, and `BaselineStore` adapter. A durable
  pending marker makes analytics snapshot publication retry after a committed
  promotion, including idle cycles. It refuses startup while Sled markers
  exist unless the migration completion marker is valid.
- `src/admin.rs` contains shared authorization, storage, scoring, and risk
  command services. `execute_admin_bridge_command` dispatches the nine
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
- `tools/sled-migration` owns the isolated legacy reader, its separate lockfile,
  format/lint/test CI lane, and dependency graph audit. Run it before upgrading
  a store with Sled `conf`/`db` markers; it preserves the original files and
  writes the verified records to SQLite.
