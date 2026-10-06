#![cfg(feature = "analytics")]

use repo_analyzer_core::admin;
use repo_analyzer_core::auth::issue_test_token as issue_token_with_test_secret;
use repo_analyzer_core::storage::{IngestionBackendConfig, IngestionBackendKind};
use repo_analyzer_core::types::{CommitIngestionEvent, TelemetryPoint};
use std::sync::OnceLock;
use tempfile::tempdir;

fn issue_test_token(user: &str, roles: &[&str], ttl_seconds: i64) -> String {
    static CONFIGURED_SECRET: OnceLock<()> = OnceLock::new();
    CONFIGURED_SECRET.get_or_init(|| {
        std::env::set_var(
            "RUNICIPAL_TOKEN_SECRET",
            "rocinante-admin-ingestion-tests-secret-32-bytes",
        );
    });
    issue_token_with_test_secret(user, roles, ttl_seconds)
}

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
fn blocks_removed_sled_ingestion_with_migration_hint() {
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

    assert!(err.to_string().contains("configure the SQLite WAL backend"));
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
