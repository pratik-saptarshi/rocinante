#![cfg(feature = "analytics")]

use repo_analyzer_core::storage::{DualLayerStore, IngestionBackendConfig, IngestionBackendKind};
use repo_analyzer_core::types::{CommitIngestionEvent, TelemetryPoint};
use rusqlite::Connection;
use std::fs;
use std::path::Path;

use tempfile::tempdir;

fn sample_event(id: &str) -> CommitIngestionEvent {
    CommitIngestionEvent {
        commit_id: id.to_string(),
        repo_name: "repo-a".to_string(),
        release: "v1.0.0".to_string(),
        committer: "alice".to_string(),
        telemetry: vec![TelemetryPoint {
            plugin: "complexity".to_string(),
            metric_key: "estimated_cyclomatic_complexity".to_string(),
            metric_value: 8.0,
            details: "ok".to_string(),
        }],
    }
}

fn pending_event_count(kv_path: &Path) -> i64 {
    fs::create_dir_all(kv_path).expect("create SQLite ingestion directory");
    let connection = Connection::open(kv_path.join("ingestion.sqlite3"))
        .expect("open SQLite ingestion database");
    connection
        .execute_batch(
            "CREATE TABLE IF NOT EXISTS ingestion_kv (
                key BLOB PRIMARY KEY NOT NULL,
                value BLOB NOT NULL
            ) WITHOUT ROWID;",
        )
        .expect("create ingestion table");
    connection
        .query_row(
            "SELECT COUNT(*) FROM ingestion_kv WHERE key >= ?1 AND key < ?2",
            rusqlite::params![b"evt:".as_slice(), b"evu:".as_slice()],
            |row| row.get(0),
        )
        .expect("count pending events")
}

#[test]
fn badger_sidecar_inproc_transport_ingests_event() {
    let dir = tempdir().expect("tmp");
    let kv = dir.path().join("kv");
    let col = dir.path().join("analytics.duckdb");
    let store =
        DualLayerStore::open(kv.to_str().expect("kv"), col.to_str().expect("col")).expect("open");

    let backend = IngestionBackendConfig {
        kind: IngestionBackendKind::BadgerSidecar,
        strict_badger_required: true,
        endpoint: Some("inproc://badger".to_string()),
    };

    store
        .ingest_commit_event_with_backend(&sample_event("c1"), &backend)
        .expect("ingest");

    let stats = store.promote_to_columnar().expect("promote");
    assert_eq!(stats.promoted_events, 1);
}

#[test]
fn badger_sidecar_unix_transport_errors_if_socket_unavailable() {
    let dir = tempdir().expect("tmp");
    let kv = dir.path().join("kv");
    let col = dir.path().join("analytics.duckdb");
    let store =
        DualLayerStore::open(kv.to_str().expect("kv"), col.to_str().expect("col")).expect("open");

    let backend = IngestionBackendConfig {
        kind: IngestionBackendKind::BadgerSidecar,
        strict_badger_required: true,
        endpoint: Some("unix:///tmp/does-not-exist-badger.sock".to_string()),
    };

    let err = store
        .ingest_commit_event_with_backend(&sample_event("c2"), &backend)
        .expect_err("expected transport failure");
    assert!(err.to_string().contains("badger sidecar transport failed"));
}

#[test]
fn badger_sidecar_rejects_unsupported_endpoint_scheme() {
    let dir = tempdir().expect("tmp");
    let kv = dir.path().join("kv");
    let col = dir.path().join("analytics.duckdb");
    let store =
        DualLayerStore::open(kv.to_str().expect("kv"), col.to_str().expect("col")).expect("open");

    let backend = IngestionBackendConfig {
        kind: IngestionBackendKind::BadgerSidecar,
        strict_badger_required: true,
        endpoint: Some("tcp://127.0.0.1:9000".to_string()),
    };

    let err = store
        .ingest_commit_event_with_backend(&sample_event("c3"), &backend)
        .expect_err("expected unsupported scheme");
    assert!(err
        .to_string()
        .contains("Badger sidecar endpoint must start with inproc:// or unix://"));
}

#[test]
fn badger_sidecar_transport_failure_does_not_persist_raw_event() {
    let dir = tempdir().expect("tmp");
    let kv = dir.path().join("kv");
    let col = dir.path().join("analytics.duckdb");
    let before_count = pending_event_count(&kv);

    let res = {
        let store = DualLayerStore::open(kv.to_str().expect("kv"), col.to_str().expect("col"))
            .expect("open");
        let backend = IngestionBackendConfig {
            kind: IngestionBackendKind::BadgerSidecar,
            strict_badger_required: true,
            endpoint: Some("unix:///tmp/does-not-exist-badger.sock".to_string()),
        };
        store.ingest_commit_event_with_backend(&sample_event("c5"), &backend)
    };
    let err = match res {
        Ok(()) => panic!("expected transport failure"),
        Err(err) => err,
    };
    assert!(err.to_string().contains("badger sidecar transport failed"));

    let after_count = pending_event_count(&kv);
    assert_eq!(before_count, after_count);
}

#[test]
fn badger_sidecar_inproc_transport_trims_endpoint_whitespace() {
    let dir = tempdir().expect("tmp");
    let kv = dir.path().join("kv");
    let col = dir.path().join("analytics.duckdb");
    let store =
        DualLayerStore::open(kv.to_str().expect("kv"), col.to_str().expect("col")).expect("open");
    let backend = IngestionBackendConfig {
        kind: IngestionBackendKind::BadgerSidecar,
        strict_badger_required: true,
        endpoint: Some("  inproc://badger  ".to_string()),
    };
    assert!(store
        .ingest_commit_event_with_backend(&sample_event("c4"), &backend)
        .is_ok());
}
