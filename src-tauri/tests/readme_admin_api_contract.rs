use repo_analyzer_core::admin;
use repo_analyzer_core::auth::issue_test_token;
use repo_analyzer_core::risk_contract::PrCandidate;
use repo_analyzer_core::storage::{IngestionBackendConfig, IngestionBackendKind};
use repo_analyzer_core::types::{AdminQuery, CommitIngestionEvent, ScoringWeights, TelemetryPoint};
use std::path::Path;

fn documented_admin_api_calls_compile(
    repository_root: &Path,
    telemetry_db: &Path,
    ingestion_path: &str,
    analytics_path: &str,
    weights_path: &str,
    audit_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let token = issue_test_token("auditor", &["admin"], 900);
    let backend = IngestionBackendConfig {
        kind: IngestionBackendKind::BadgerSidecar,
        strict_badger_required: true,
        endpoint: Some("inproc://readme-contract".to_owned()),
    };

    let _scan_summary = admin::run_scan(
        &token,
        repository_root.to_str().expect("UTF-8 test path"),
        "release-contract",
        telemetry_db,
    )?;
    let event = CommitIngestionEvent {
        commit_id: "readme-contract-commit".to_owned(),
        repo_name: "readme-contract-repo".to_owned(),
        release: "release-contract".to_owned(),
        committer: "auditor".to_owned(),
        telemetry: vec![TelemetryPoint {
            plugin: "readme-contract".to_owned(),
            metric_key: "example".to_owned(),
            metric_value: 1.0,
            details: "README API example compile contract".to_owned(),
        }],
    };
    admin::ingest_event(&token, ingestion_path, analytics_path, event, &backend)?;
    admin::promote_lifecycle(&token, ingestion_path, analytics_path)?;

    let query = AdminQuery {
        name: Some("readme-contract-repo".to_owned()),
        release: Some("release-contract".to_owned()),
    };
    let _aggregates = admin::query_aggregates(&token, ingestion_path, analytics_path, query)?;
    let _scores = admin::committer_scores(
        &token,
        ingestion_path,
        analytics_path,
        AdminQuery {
            name: None,
            release: Some("release-contract".to_owned()),
        },
        weights_path,
    )?;
    let _ranked = admin::rank_prs(
        &token,
        ingestion_path,
        analytics_path,
        vec![PrCandidate {
            pr_id: "pr-contract".to_owned(),
            repo_name: "readme-contract-repo".to_owned(),
            author: "auditor".to_owned(),
            release: "release-contract".to_owned(),
            file_risk: 0.4,
            author_velocity: 0.6,
            approval_fidelity: 0.9,
            ..PrCandidate::default()
        }],
        weights_path,
    )?;
    admin::update_scoring_weights(&token, weights_path, audit_path, ScoringWeights::default())?;

    Ok(())
}

#[test]
fn readme_admin_calls_match_the_public_rust_api() {
    type DocumentedAdminApiSignature =
        fn(&Path, &Path, &str, &str, &str, &str) -> Result<(), Box<dyn std::error::Error>>;

    let _documented_example_signature: DocumentedAdminApiSignature =
        documented_admin_api_calls_compile;
}
