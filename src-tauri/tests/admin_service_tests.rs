use repo_analyzer_core::admin;
use repo_analyzer_core::storage::{IngestionBackendConfig, IngestionBackendKind};
use repo_analyzer_core::types::{AdminQuery, CommitIngestionEvent, PrCandidate, ScoringWeights, TelemetryPoint};
use tempfile::tempdir;

fn sample_event(id: &str) -> CommitIngestionEvent {
    CommitIngestionEvent {
        commit_id: id.to_string(),
        repo_name: "repo-a".to_string(),
        release: "v1.0.0".to_string(),
        committer: "alice".to_string(),
        telemetry: vec![
            TelemetryPoint { plugin: "complexity".to_string(), metric_key: "estimated_cyclomatic_complexity".to_string(), metric_value: 9.0, details: "ok".to_string() },
            TelemetryPoint { plugin: "coverage".to_string(), metric_key: "coverage_delta".to_string(), metric_value: 3.0, details: "ok".to_string() },
            TelemetryPoint { plugin: "churn".to_string(), metric_key: "churn_efficiency".to_string(), metric_value: 0.7, details: "ok".to_string() },
            TelemetryPoint { plugin: "ci".to_string(), metric_key: "pipeline_success".to_string(), metric_value: 1.0, details: "ok".to_string() },
        ],
    }
}

#[test]
fn admin_services_roundtrip_happy_path() {
    let dir = tempdir().expect("tmp");
    let kv = dir.path().join("kv");
    let col = dir.path().join("analytics.duckdb");
    let weights = dir.path().join("weights.json");
    let audit = dir.path().join("audit.jsonl");
    let backend = IngestionBackendConfig {
        kind: IngestionBackendKind::BadgerSidecar,
        strict_badger_required: false,
        endpoint: Some("inproc://badger".to_string()),
    };

    admin::ingest_event(
        "alice:admin",
        kv.to_str().expect("kv"),
        col.to_str().expect("col"),
        sample_event("c1"),
        &backend,
    )
    .expect("ingest");

    let promoted = admin::promote_lifecycle(
        "alice:admin",
        kv.to_str().expect("kv"),
        col.to_str().expect("col"),
    )
    .expect("promote");
    assert_eq!(promoted.promoted_events, 1);

    let aggregates = admin::query_aggregates(
        "alice:admin",
        kv.to_str().expect("kv"),
        col.to_str().expect("col"),
        AdminQuery { name: Some("repo-a".to_string()), release: Some("v1".to_string()) },
    )
    .expect("aggregates");
    assert!(!aggregates.is_empty());

    let scores = admin::committer_scores(
        "alice:admin",
        kv.to_str().expect("kv"),
        col.to_str().expect("col"),
        AdminQuery { name: Some("repo-a".to_string()), release: None },
        weights.to_str().expect("weights"),
    )
    .expect("scores");
    assert!(!scores.is_empty());

    let prs = vec![PrCandidate {
        pr_id: "pr-1".to_string(),
        repo_name: "repo-a".to_string(),
        author: "alice".to_string(),
        release: "v1.0.0".to_string(),
        file_risk: 0.8,
        author_velocity: 0.5,
        approval_fidelity: 0.6,
    }];
    let ranked = admin::rank_prs(
        "alice:admin",
        kv.to_str().expect("kv"),
        col.to_str().expect("col"),
        prs,
        weights.to_str().expect("weights"),
    )
    .expect("ranked");
    assert_eq!(ranked.len(), 1);

    let mut new_weights = ScoringWeights::default();
    new_weights.version = "v-next".to_string();
    admin::update_scoring_weights(
        "alice:admin",
        weights.to_str().expect("weights"),
        audit.to_str().expect("audit"),
        new_weights,
    )
    .expect("update weights");
}

#[test]
fn admin_services_reject_non_admin() {
    let dir = tempdir().expect("tmp");
    let kv = dir.path().join("kv");
    let col = dir.path().join("analytics.duckdb");
    let backend = IngestionBackendConfig {
        kind: IngestionBackendKind::BadgerSidecar,
        strict_badger_required: false,
        endpoint: Some("inproc://badger".to_string()),
    };

    let err = admin::ingest_event(
        "bob:reader",
        kv.to_str().expect("kv"),
        col.to_str().expect("col"),
        sample_event("c2"),
        &backend,
    )
    .expect_err("permission denied");
    assert!(err.to_string().contains("permission denied"));
}
