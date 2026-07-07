#![cfg(feature = "duckdb-analytics")]

use repo_analyzer_core::admin;
use repo_analyzer_core::auth::issue_test_token;
use repo_analyzer_core::storage::{IngestionBackendConfig, IngestionBackendKind};
use repo_analyzer_core::types::{CommitIngestionEvent, TelemetryPoint};
use tempfile::tempdir;

fn sample_event() -> CommitIngestionEvent {
    CommitIngestionEvent {
        commit_id: "c1".to_string(),
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

#[test]
fn blocks_ingestion_when_strict_mode_not_badger() {
    let dir = tempdir().expect("tmp");
    let kv = dir.path().join("kv");
    let col = dir.path().join("analytics.duckdb");

    let backend = IngestionBackendConfig {
        kind: IngestionBackendKind::SledTransitional,
        strict_badger_required: true,
        endpoint: None,
    };

    let err = admin::ingest_event(
        &issue_test_token("alice", &["admin"], 3600),
        kv.to_str().expect("kv"),
        col.to_str().expect("col"),
        sample_event(),
        &backend,
    )
    .expect_err("strict mode should block");

    assert!(err
        .to_string()
        .contains("Badger sidecar backend is required in strict mode"));
}

#[test]
fn allows_ingestion_when_strict_mode_badger_sidecar() {
    let dir = tempdir().expect("tmp");
    let kv = dir.path().join("kv");
    let col = dir.path().join("analytics.duckdb");

    let backend = IngestionBackendConfig {
        kind: IngestionBackendKind::BadgerSidecar,
        strict_badger_required: true,
        endpoint: Some("inproc://badger".to_string()),
    };

    assert!(admin::ingest_event(
        &issue_test_token("alice", &["admin"], 3600),
        kv.to_str().expect("kv"),
        col.to_str().expect("col"),
        sample_event(),
        &backend,
    )
    .is_ok());
}
