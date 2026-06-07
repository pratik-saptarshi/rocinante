use repo_analyzer_core::storage::{
    AsyncIngestionEngine, AnalyticsQueryMode, AnalyticsSnapshot, DualLayerStore, RetentionPolicy,
};
use repo_analyzer_core::types::{AdminQuery, CommitIngestionEvent, TelemetryPoint};
use serde_json;
use sled;
use tempfile::tempdir;
use std::thread;
use std::time::Duration;

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

#[test]
fn stores_raw_events_with_sharded_keys() {
    let dir = tempdir().expect("tmp");
    let kv = dir.path().join("kv");
    let col = dir.path().join("analytics.duckdb");
    {
        let store = DualLayerStore::open(
            kv.to_str().expect("kv path"),
            col.to_str().expect("col path"),
        )
        .expect("open");

        store
            .ingest_commit_event(&CommitIngestionEvent {
                commit_id: "deadbeefcafec0de".to_string(),
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
    }

    let db = sled::open(&kv).expect("open kv");
    let mut keys = db.scan_prefix("evt:").filter_map(|e| e.ok());
    let key = keys.next().expect("raw key").0.to_vec();
    assert!(keys.next().is_none(), "only one raw event expected");

    let key = String::from_utf8(key).expect("utf-8 key");
    let parts: Vec<&str> = key.split(':').collect();
    assert_eq!(parts.len(), 4);
    assert_eq!(parts[0], "evt");
    assert!(parts[1].chars().all(|c| c.is_ascii_digit()));
    assert_eq!(parts[3], "deadbeefcafec0de");
    assert_eq!(parts[2].len(), 2);
}

#[test]
fn enforces_mutable_mode_rejection_for_analytics_snapshots() {
    let dir = tempdir().expect("tmp");
    let kv = dir.path().join("kv");
    let col = dir.path().join("analytics.duckdb");
    let store = DualLayerStore::open(
        kv.to_str().expect("kv path"),
        col.to_str().expect("col path"),
    )
    .expect("open");

    let snapshot = AnalyticsSnapshot::new(col.to_str().expect("col path"), 42);
    let err = store
        .aggregate_by_query_with_snapshot(
            &AdminQuery {
                name: Some("repo-a".to_string()),
                release: None,
            },
            &snapshot,
            AnalyticsQueryMode::Mutable,
        )
        .expect_err("expected immutable snapshot enforcement");
    assert!(err.to_string().contains("read-only snapshots"));
}

#[test]
fn prunes_expired_raw_events_and_counts_pruned_events() {
    let dir = tempdir().expect("tmp");
    let kv = dir.path().join("kv");
    let col = dir.path().join("analytics.duckdb");
    let db = sled::open(&kv).expect("open kv");
    let event = CommitIngestionEvent {
        commit_id: "same".to_string(),
        repo_name: "repo-b".to_string(),
        release: "v1.0.0".to_string(),
        committer: "alice".to_string(),
        telemetry: vec![TelemetryPoint {
            plugin: "complexity".to_string(),
            metric_key: "estimated_cyclomatic_complexity".to_string(),
            metric_value: 10.0,
            details: "baseline".to_string(),
        }],
    };
    let payload = serde_json::to_vec(&event).expect("serialize");
    let old_key = format!("evt:{}:aa:old", 100);
    let fresh_key = format!("evt:{}:ab:fresh", 180);
    db.insert(old_key.as_bytes(), payload.clone())
        .expect("insert old");
    db.insert(fresh_key.as_bytes(), payload)
        .expect("insert fresh");
    drop(db);

    let store = DualLayerStore::open(
        kv.to_str().expect("kv path"),
        col.to_str().expect("col path"),
    )
    .expect("open");

    let policy = RetentionPolicy { raw_ttl_secs: 50 };
    let stats = store
        .promote_to_columnar_with_retention(&policy, 200)
        .expect("promote with retention");
    assert_eq!(stats.pruned_events, 1);
    assert_eq!(stats.promoted_events, 1);
}

#[test]
fn async_ingestion_engine_queues_events_before_promotion() {
    let dir = tempdir().expect("tmp");
    let kv = dir.path().join("kv");
    let col = dir.path().join("analytics.duckdb");

    let engine = AsyncIngestionEngine::start_with_interval(
        kv.to_str().expect("kv path"),
        col.to_str().expect("col path"),
        16,
        400,
    )
    .expect("start");

    engine
        .enqueue(CommitIngestionEvent {
            commit_id: "burst-a".to_string(),
            repo_name: "repo-a".to_string(),
            release: "v1.0.0".to_string(),
            committer: "alice".to_string(),
            telemetry: vec![],
        })
        .expect("enqueue burst-a");
    engine
        .enqueue(CommitIngestionEvent {
            commit_id: "burst-b".to_string(),
            repo_name: "repo-a".to_string(),
            release: "v1.0.0".to_string(),
            committer: "alice".to_string(),
            telemetry: vec![],
        })
        .expect("enqueue burst-b");

    thread::sleep(Duration::from_millis(120));
    let snapshot = sled::open(&kv).expect("open kv");
    assert_eq!(snapshot.scan_prefix("evt:").count(), 2);
    assert_eq!(engine.promotion_count(), 0);

    thread::sleep(Duration::from_millis(500));
    let mut promoted = false;
    for _ in 0..10 {
        if engine.promotion_count() > 0 {
            promoted = true;
            break;
        }
        thread::sleep(Duration::from_millis(100));
    }
    assert!(promoted);
    assert_eq!(engine.max_queue_depth(), 2);

    let mut raw_count = 2usize;
    for _ in 0..20 {
        let db = sled::open(&kv).expect("open kv");
        raw_count = db.scan_prefix("evt:").count();
        if raw_count == 0 {
            break;
        }
        thread::sleep(Duration::from_millis(100));
    }
    assert_eq!(raw_count, 0);
}
