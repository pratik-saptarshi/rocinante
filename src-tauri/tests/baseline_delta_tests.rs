#![cfg(feature = "analytics")]

use repo_analyzer_core::scoring::load_or_init_weights;
use repo_analyzer_core::storage::DualLayerStore;
use repo_analyzer_core::types::{AdminQuery, CommitIngestionEvent, TelemetryPoint};
use tempfile::tempdir;

#[test]
fn committer_score_uses_baseline_complexity_delta() {
    let dir = tempdir().expect("tmp");
    let kv = dir.path().join("kv");
    let col = dir.path().join("analytics.duckdb");
    let weights_file = dir.path().join("weights.json");
    let store = DualLayerStore::open(
        kv.to_str().expect("kv path"),
        col.to_str().expect("col path"),
    )
    .expect("open");

    // Baseline seed event
    store
        .ingest_commit_event(&CommitIngestionEvent {
            commit_id: "b1".to_string(),
            repo_name: "repo-b".to_string(),
            release: "v1".to_string(),
            committer: "alice".to_string(),
            telemetry: vec![
                TelemetryPoint {
                    plugin: "complexity".to_string(),
                    metric_key: "estimated_cyclomatic_complexity".to_string(),
                    metric_value: 20.0,
                    details: "base".to_string(),
                },
                TelemetryPoint {
                    plugin: "coverage".to_string(),
                    metric_key: "coverage_delta".to_string(),
                    metric_value: 2.0,
                    details: "base".to_string(),
                },
                TelemetryPoint {
                    plugin: "churn".to_string(),
                    metric_key: "churn_efficiency".to_string(),
                    metric_value: 0.8,
                    details: "base".to_string(),
                },
                TelemetryPoint {
                    plugin: "ci".to_string(),
                    metric_key: "pipeline_success".to_string(),
                    metric_value: 1.0,
                    details: "base".to_string(),
                },
            ],
        })
        .expect("ingest baseline");

    // Improved complexity event
    store
        .ingest_commit_event(&CommitIngestionEvent {
            commit_id: "b2".to_string(),
            repo_name: "repo-b".to_string(),
            release: "v2".to_string(),
            committer: "alice".to_string(),
            telemetry: vec![
                TelemetryPoint {
                    plugin: "complexity".to_string(),
                    metric_key: "estimated_cyclomatic_complexity".to_string(),
                    metric_value: 10.0,
                    details: "delta".to_string(),
                },
                TelemetryPoint {
                    plugin: "coverage".to_string(),
                    metric_key: "coverage_delta".to_string(),
                    metric_value: 4.0,
                    details: "delta".to_string(),
                },
                TelemetryPoint {
                    plugin: "churn".to_string(),
                    metric_key: "churn_efficiency".to_string(),
                    metric_value: 0.9,
                    details: "delta".to_string(),
                },
                TelemetryPoint {
                    plugin: "ci".to_string(),
                    metric_key: "pipeline_success".to_string(),
                    metric_value: 1.0,
                    details: "delta".to_string(),
                },
            ],
        })
        .expect("ingest delta");

    store.promote_to_columnar().expect("promote");
    let weights =
        load_or_init_weights(weights_file.to_str().expect("weights path")).expect("weights");
    let scores = store
        .compute_committer_scores(
            &AdminQuery {
                name: Some("repo-b".to_string()),
                release: None,
            },
            &weights,
        )
        .expect("scores");
    assert!(!scores.is_empty());
    assert!(scores[0].complexity_component > 0.0);
}
