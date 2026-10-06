#![cfg(feature = "analytics")]

use repo_analyzer_core::admin;
use repo_analyzer_core::auth::{decode_principal, issue_test_token};
use repo_analyzer_core::errors::AnalyzerError;
use repo_analyzer_core::risk_contract::PrRiskSchema;
use repo_analyzer_core::storage::{IngestionBackendConfig, IngestionBackendKind};
use repo_analyzer_core::types::{
    AdminQuery, CommitIngestionEvent, PrCandidate, ScoringWeights, TelemetryPoint,
};
use tempfile::tempdir;

fn assert_secret_required<T>(result: Result<T, AnalyzerError>) {
    match result {
        Err(AnalyzerError::Integrity(message)) => {
            assert!(message.contains("RUNICIPAL_TOKEN_SECRET"));
        }
        Err(error) => panic!("expected configured-secret rejection, got {error}"),
        Ok(_) => panic!("expected configured-secret rejection"),
    }
}

#[test]
fn every_public_admin_service_rejects_the_fallback_secret() {
    std::env::remove_var("RUNICIPAL_TOKEN_SECRET");
    let token = issue_test_token("alice", &["admin"], 3600);
    assert!(decode_principal(&token).is_ok(), "fallback token is valid");

    let dir = tempdir().expect("temporary admin service directory");
    let kv = dir.path().join("kv");
    let columnar = dir.path().join("analytics.duckdb");
    let metrics = dir.path().join("telemetry.db");
    let weights = dir.path().join("weights.json");
    let audit = dir.path().join("audit.jsonl");
    let kv_path = kv.to_str().expect("UTF-8 kv path");
    let columnar_path = columnar.to_str().expect("UTF-8 columnar path");
    let query = AdminQuery {
        name: Some("repo-a".to_string()),
        release: Some("v1".to_string()),
    };
    let backend = IngestionBackendConfig {
        kind: IngestionBackendKind::SqliteWal,
        strict_badger_required: false,
        endpoint: None,
    };
    let event = CommitIngestionEvent {
        commit_id: "secret-guard".to_string(),
        repo_name: "repo-a".to_string(),
        release: "v1".to_string(),
        committer: "alice".to_string(),
        telemetry: vec![TelemetryPoint {
            plugin: "test".to_string(),
            metric_key: "probe".to_string(),
            metric_value: 1.0,
            details: String::new(),
        }],
    };

    assert_secret_required(admin::run_scan(
        &token,
        dir.path().to_str().expect("UTF-8 repository path"),
        "v1",
        &metrics,
    ));
    assert_secret_required(admin::query_metrics(&token, query.clone(), &metrics));
    assert_secret_required(admin::ingest_event(
        &token,
        kv_path,
        columnar_path,
        event,
        &backend,
    ));
    assert_secret_required(admin::promote_lifecycle(&token, kv_path, columnar_path));
    assert_secret_required(admin::query_aggregates(
        &token,
        kv_path,
        columnar_path,
        query.clone(),
    ));
    assert_secret_required(admin::committer_scores(
        &token,
        kv_path,
        columnar_path,
        query,
        weights.to_str().expect("UTF-8 weights path"),
    ));
    assert_secret_required(admin::rank_prs(
        &token,
        kv_path,
        columnar_path,
        vec![PrCandidate::default()],
        weights.to_str().expect("UTF-8 weights path"),
    ));
    assert_secret_required(admin::evaluate_pr_risk(&token, PrCandidate::default()));
    assert_secret_required(admin::evaluate_pr_risk_with_schema(
        &token,
        PrCandidate::default(),
        PrRiskSchema::default(),
    ));
    assert_secret_required(admin::update_scoring_weights(
        &token,
        weights.to_str().expect("UTF-8 weights path"),
        audit.to_str().expect("UTF-8 audit path"),
        ScoringWeights::default(),
    ));
    assert_secret_required(admin::query_release_baseline(
        &token,
        kv_path,
        columnar_path,
        "repo-a",
    ));
    assert_secret_required(admin::reseed_release_baseline(
        &token,
        kv_path,
        columnar_path,
        "repo-a",
        1.0,
    ));
}
