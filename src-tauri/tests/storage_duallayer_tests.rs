use repo_analyzer_core::storage::DualLayerStore;
use repo_analyzer_core::types::{AdminQuery, CommitIngestionEvent, TelemetryPoint};
use tempfile::tempdir;

#[test]
fn promotes_events_and_reads_aggregates() {
    let dir = tempdir().expect("tmp");
    let kv = dir.path().join("kv");
    let col = dir.path().join("analytics.duckdb");
    let store = DualLayerStore::open(
        kv.to_str().expect("kv path"),
        col.to_str().expect("col path"),
    )
    .expect("open");

    store
        .ingest_commit_event(&CommitIngestionEvent {
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
        })
        .expect("ingest");

    let stats = store.promote_to_columnar().expect("promote");
    assert_eq!(stats.promoted_events, 1);

    let points = store
        .aggregate_by_query(&AdminQuery {
            name: Some("repo-a".to_string()),
            release: Some("v1.0".to_string()),
        })
        .expect("query");

    assert_eq!(points.len(), 1);
    assert_eq!(points[0].metric_key, "estimated_cyclomatic_complexity");
}
