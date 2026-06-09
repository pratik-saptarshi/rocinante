use repo_analyzer_core::storage::{
    AnalyticsQueryMode, AnalyticsSnapshot, AsyncIngestionEngine, DualLayerStore,
    IngestionBackendConfig, IngestionBackendKind, RetentionPolicy, StorageRoute,
};
use repo_analyzer_core::types::{AdminQuery, CommitIngestionEvent, TelemetryPoint};
use serde_json;
use sled;
use std::fs;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tempfile::tempdir;

fn enqueue_with_backpressure(
    engine: &AsyncIngestionEngine,
    evt: CommitIngestionEvent,
) {
    for attempt in 0..120 {
        if let Ok(()) = engine.enqueue(evt.clone()) {
            return;
        }
        if attempt > 80 {
            panic!("buffer enqueue failed after retries");
        }
        thread::sleep(Duration::from_millis(2));
    }
    panic!("buffer enqueue failed after retries");
}

fn now_ts_for_test() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

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

fn sample_event_with_release(id: &str, release: &str) -> CommitIngestionEvent {
    CommitIngestionEvent {
        commit_id: id.to_string(),
        repo_name: "repo-a".to_string(),
        release: release.to_string(),
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
fn rejects_ingest_route_for_ingestion_command_layer() {
    let dir = tempdir().expect("tmp");
    let kv = dir.path().join("kv");
    let col = dir.path().join("analytics.duckdb");
    let store = DualLayerStore::open(
        kv.to_str().expect("kv path"),
        col.to_str().expect("col path"),
    )
    .expect("open");

    let backend = IngestionBackendConfig {
        kind: IngestionBackendKind::BadgerSidecar,
        strict_badger_required: true,
        endpoint: Some("inproc://badger".to_string()),
    };

    let err = store
        .ingest_commit_event_with_backend_on_route(
            &sample_event("misroute"),
            StorageRoute::Analytics,
            &backend,
        )
        .expect_err("cross-route ingest should fail");
    assert!(err
        .to_string()
        .contains("Ingestion writes must route exclusively to BadgerDB"));
}

#[test]
fn rejects_analytics_route_for_query_command_layer() {
    let dir = tempdir().expect("tmp");
    let kv = dir.path().join("kv");
    let col = dir.path().join("analytics.duckdb");
    let store = DualLayerStore::open(
        kv.to_str().expect("kv path"),
        col.to_str().expect("col path"),
    )
    .expect("open");

    let err = store
        .aggregate_by_query_on_route(
            StorageRoute::Ingestion,
            &AdminQuery {
                name: Some("repo-a".to_string()),
                release: None,
            },
        )
        .expect_err("cross-route query should fail");
    assert!(err
        .to_string()
        .contains("Analytics queries must route exclusively to DuckDB"));
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
    assert!(engine.queue_depth() <= 2);
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

    let mut queue_depth = 2usize;
    for _ in 0..20 {
        queue_depth = engine.queue_depth();
        if queue_depth == 0 {
            break;
        }
        thread::sleep(Duration::from_millis(100));
    }
    assert_eq!(queue_depth, 0);
}

#[test]
fn async_ingestion_engine_applies_retention_before_promotion() {
    let dir = tempdir().expect("tmp");
    let kv = dir.path().join("kv");
    let col = dir.path().join("analytics.duckdb");
    let expired = sample_event_with_release("legacy", "legacy-retention");
    let old_payload = serde_json::to_vec(&expired).expect("serialize legacy event");
    let old_key = format!("evt:{}:aa:legacy", now_ts_for_test().saturating_sub(120));
    {
        let db = sled::open(&kv).expect("open kv");
        db.insert(old_key.as_bytes(), old_payload)
            .expect("insert legacy event");
        db.flush().expect("flush legacy event");
    }

    let store = DualLayerStore::open(
        kv.to_str().expect("kv path"),
        col.to_str().expect("col path"),
    )
    .expect("open");

    let engine = AsyncIngestionEngine::start_with_store(
        store.clone(),
        16,
        50,
        Some(RetentionPolicy { raw_ttl_secs: 50 }),
    )
    .expect("start");

    enqueue_with_backpressure(
        &engine,
        sample_event_with_release("active", "active-retention"),
    );

    let mut promoted = false;
    for _ in 0..20 {
        if engine.promotion_count() > 0 {
            promoted = true;
            break;
        }
        thread::sleep(Duration::from_millis(50));
    }
    assert!(promoted);

    let legacy_hits = store
        .aggregate_by_query(&AdminQuery {
            name: Some("repo-a".to_string()),
            release: Some("legacy-retention".to_string()),
        })
        .expect("legacy query");
    let active_hits = store
        .aggregate_by_query(&AdminQuery {
            name: Some("repo-a".to_string()),
            release: Some("active-retention".to_string()),
        })
        .expect("active query");
    assert!(legacy_hits.is_empty());
    assert!(!active_hits.is_empty());
}

#[test]
fn query_aggregates_stable_while_promotion_runs_via_immutable_snapshot() {
    let dir = tempdir().expect("tmp");
    let kv = dir.path().join("kv");
    let col = dir.path().join("analytics.duckdb");

    let store = DualLayerStore::open(
        kv.to_str().expect("kv path"),
        col.to_str().expect("col path"),
    )
    .expect("open");
    store
        .ingest_commit_event(&sample_event_with_release("anchor", "baseline"))
        .expect("seed baseline");
    store.promote_to_columnar().expect("bootstrap promotion");

    let snapshot_path = dir.path().join("analytics.snapshot.readonly.duckdb");
    fs::copy(
        col.to_str().expect("col path"),
        snapshot_path.to_str().expect("snapshot path"),
    )
    .expect("snapshot copy");
    let snapshot = AnalyticsSnapshot::new(snapshot_path.to_str().expect("snapshot path"), 42);
    let engine = AsyncIngestionEngine::start_with_store(
        store.clone(),
        256,
        40,
        None,
    )
    .expect("start");

    for idx in 0..120 {
        enqueue_with_backpressure(
            &engine,
            sample_event_with_release(&format!("stream-{idx}"), "promotion"),
        );
    }

    let mut promotion_seen = false;
    for _ in 0..60 {
        let points = store
            .aggregate_by_query_with_snapshot(
                &AdminQuery {
                    name: Some("repo-a".to_string()),
                    release: Some("baseline".to_string()),
                },
                &snapshot,
                AnalyticsQueryMode::ReadOnly,
            )
            .expect("snapshot query");
        assert_eq!(points.len(), 1);

        if engine.promotion_count() > 0 {
            promotion_seen = true;
            break;
        }
        thread::sleep(Duration::from_millis(25));
    }

    assert!(promotion_seen);
}

#[test]
fn dual_layer_store_rejects_duplicate_open_for_same_kv_path() {
    let dir = tempdir().expect("tmp");
    let kv = dir.path().join("kv");
    let col = dir.path().join("analytics.duckdb");

    let store = DualLayerStore::open(
        kv.to_str().expect("kv path"),
        col.to_str().expect("col path"),
    )
    .expect("open");

    let open_again_err = DualLayerStore::open(
        kv.to_str().expect("kv path"),
        col.to_str().expect("col path"),
    );
    assert!(open_again_err.is_err(), "expected lock ownership rejection");
    assert!(open_again_err
        .err()
        .expect("expected lock ownership rejection")
        .to_string()
        .contains("already owned by another writer"));

    drop(store);

    DualLayerStore::open(
        kv.to_str().expect("kv path"),
        col.to_str().expect("col path"),
    )
    .expect("open after release");
}

#[test]
fn aggregate_queries_remain_non_empty_during_promotion_handoff() {
    let dir = tempdir().expect("tmp");
    let kv = dir.path().join("kv");
    let col = dir.path().join("analytics.duckdb");

    let store = DualLayerStore::open(
        kv.to_str().expect("kv path"),
        col.to_str().expect("col path"),
    )
    .expect("open");
    store
        .ingest_commit_event(&sample_event("anchor"))
        .expect("seed anchor");
    store.promote_to_columnar().expect("bootstrap promotion");

    let engine = AsyncIngestionEngine::start_with_store(
        store.clone(),
        256,
        40,
        None,
    )
    .expect("start");

    for idx in 0..80 {
        enqueue_with_backpressure(&engine, sample_event(&format!("stream-{idx}")));
    }

    for _ in 0..120 {
        let points = store
            .aggregate_by_query(&AdminQuery {
                name: Some("repo-a".to_string()),
                release: Some("v1.0.0".to_string()),
            })
            .expect("query");
        assert!(!points.is_empty(), "aggregate visibility must stay non-empty during handoff");

        if engine.promotion_count() > 0 {
            break;
        }
        thread::sleep(Duration::from_millis(25));
    }
}
