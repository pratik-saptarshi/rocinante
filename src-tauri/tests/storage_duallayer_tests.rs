#![cfg(feature = "analytics")]

use duckdb::params;
use duckdb::Connection;
use repo_analyzer_core::storage::{
    AnalyticsQueryMode, AnalyticsSnapshot, AsyncIngestionEngine, DualLayerStore,
    IngestionBackendConfig, IngestionBackendKind, RetentionPolicy, StorageRoute,
};
use repo_analyzer_core::types::{AdminQuery, CommitIngestionEvent, ScoringWeights, TelemetryPoint};
use rusqlite::Connection as SqliteConnection;
use std::fs;
use std::path::Path;
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tempfile::tempdir;

fn enqueue_with_backpressure(engine: &AsyncIngestionEngine, evt: CommitIngestionEvent) {
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

fn write_raw_event(kv_path: &Path, key: &[u8], payload: &[u8]) {
    fs::create_dir_all(kv_path).expect("create SQLite ingestion directory");
    let connection = SqliteConnection::open(kv_path.join("ingestion.sqlite3"))
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
        .execute(
            "INSERT OR REPLACE INTO ingestion_kv (key, value) VALUES (?1, ?2)",
            rusqlite::params![key, payload],
        )
        .expect("insert raw event");
}

fn snapshot_files_for(database_path: &Path) -> Vec<std::path::PathBuf> {
    let file_name = database_path
        .file_name()
        .expect("database file name")
        .to_string_lossy();
    let prefix = format!("{file_name}.snapshot-");
    let parent = database_path.parent().expect("database parent");
    fs::read_dir(parent)
        .expect("read snapshot directory")
        .filter_map(Result::ok)
        .filter_map(|entry| {
            entry
                .file_name()
                .to_string_lossy()
                .starts_with(&prefix)
                .then_some(entry.path())
        })
        .collect()
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

fn sample_event_for_repo_release(
    repo_name: &str,
    commit_id: &str,
    release: &str,
) -> CommitIngestionEvent {
    CommitIngestionEvent {
        commit_id: commit_id.to_string(),
        repo_name: repo_name.to_string(),
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
fn empty_promotions_skip_snapshots_and_real_promotions_replace_old_snapshots() {
    let dir = tempdir().expect("tmp");
    let kv = dir.path().join("kv");
    let col = dir.path().join("analytics.duckdb");
    let store = DualLayerStore::open(
        kv.to_str().expect("kv path"),
        col.to_str().expect("col path"),
    )
    .expect("open");

    for _ in 0..3 {
        assert_eq!(
            store
                .promote_to_columnar()
                .expect("empty promotion")
                .promoted_events,
            0
        );
    }
    assert!(snapshot_files_for(&col).is_empty());

    store
        .ingest_commit_event(&sample_event("snapshot-one"))
        .expect("ingest first event");
    assert_eq!(
        store
            .promote_to_columnar()
            .expect("promote first event")
            .promoted_events,
        1
    );
    let first_snapshot = snapshot_files_for(&col);
    assert_eq!(first_snapshot.len(), 1);

    store.promote_to_columnar().expect("idle promotion");
    assert_eq!(snapshot_files_for(&col), first_snapshot);

    store
        .ingest_commit_event(&sample_event("snapshot-two"))
        .expect("ingest second event");
    store.promote_to_columnar().expect("promote second event");
    let current_snapshots = snapshot_files_for(&col);
    assert_eq!(current_snapshots.len(), 1);
    assert_ne!(current_snapshots[0], first_snapshot[0]);
    assert!(!first_snapshot[0].exists());

    drop(store);
    let _reopened = DualLayerStore::open(
        kv.to_str().expect("kv path"),
        col.to_str().expect("col path"),
    )
    .expect("reopen store");
    assert!(snapshot_files_for(&col).is_empty());
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

    let db = SqliteConnection::open(kv.join("ingestion.sqlite3")).expect("open kv");
    let mut statement = db
        .prepare("SELECT key FROM ingestion_kv WHERE key >= ?1 AND key < ?2 ORDER BY key")
        .expect("prepare raw key query");
    let keys = statement
        .query_map(
            rusqlite::params![b"evt:".as_slice(), b"evu:".as_slice()],
            |row| row.get::<_, Vec<u8>>(0),
        )
        .expect("query raw keys")
        .collect::<Result<Vec<_>, _>>()
        .expect("read raw keys");
    assert_eq!(keys.len(), 1, "only one raw event expected");
    let key = keys[0].clone();

    let key = String::from_utf8(key).expect("utf-8 key");
    let parts: Vec<&str> = key.split(':').collect();
    assert_eq!(parts.len(), 4);
    assert_eq!(parts[0], "evt");
    assert!(parts[1].chars().all(|c| c.is_ascii_digit()));
    assert!(parts[2].parse::<u8>().expect("numeric shard") < 16);
    assert_eq!(parts[3].len(), 64, "event identity is a SHA-256 digest");
    assert!(parts[3].chars().all(|c| c.is_ascii_hexdigit()));
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
    write_raw_event(&kv, old_key.as_bytes(), &payload);
    write_raw_event(&kv, fresh_key.as_bytes(), &payload);

    let store = DualLayerStore::open(
        kv.to_str().expect("kv path"),
        col.to_str().expect("col path"),
    )
    .expect("open");

    let policy = RetentionPolicy {
        raw_ttl_secs: 50,
        ..RetentionPolicy::default()
    };
    let stats = store
        .promote_to_columnar_with_retention(&policy, 200)
        .expect("promote with retention");
    assert_eq!(stats.pruned_events, 1);
    assert_eq!(stats.promoted_events, 1);
}

#[test]
fn prunes_old_releases_and_preserves_queryability_by_rollup() {
    let dir = tempdir().expect("tmp");
    let kv = dir.path().join("kv");
    let col = dir.path().join("analytics.duckdb");

    let store = DualLayerStore::open(
        kv.to_str().expect("kv path"),
        col.to_str().expect("col path"),
    )
    .expect("open");

    for id in ["r2019", "r2020", "r2021"] {
        store
            .ingest_commit_event(&sample_event_with_release(id, id))
            .expect("ingest legacy");
    }

    let policy = RetentionPolicy {
        raw_ttl_secs: 3600,
        max_release_partitions: Some(2),
    };
    let stats = store
        .promote_to_columnar_with_retention(&policy, now_ts_for_test())
        .expect("promote with retention");
    assert_eq!(stats.promoted_events, 3);

    let legacy_aggregate = store
        .aggregate_by_query(&AdminQuery {
            name: Some("repo-a".to_string()),
            release: Some("r2019".to_string()),
        })
        .expect("legacy aggregate from rollup");
    assert!(!legacy_aggregate.is_empty());

    for release in ["r2020", "r2021"] {
        let points = store
            .aggregate_by_query(&AdminQuery {
                name: Some("repo-a".to_string()),
                release: Some(release.to_string()),
            })
            .expect("recent aggregate");
        assert!(!points.is_empty());
    }

    let conn = Connection::open(col.to_str().expect("col path")).expect("open analytics");
    let raw_stale_count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM telemetry_history WHERE release = ?1",
            params!["r2019"],
            |row| row.get(0),
        )
        .expect("count stale raw");
    let rollup_stale_count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM telemetry_history_rollup WHERE release = ?1",
            params!["r2019"],
            |row| row.get(0),
        )
        .expect("count stale rollup");

    assert_eq!(raw_stale_count, 0);
    assert!(rollup_stale_count > 0);
}

#[test]
fn aggregates_and_scores_weighted_rollups_with_live_samples() {
    let dir = tempdir().expect("tmp");
    let kv = dir.path().join("kv");
    let col = dir.path().join("analytics.duckdb");
    let store = DualLayerStore::open(
        kv.to_str().expect("kv path"),
        col.to_str().expect("col path"),
    )
    .expect("open");

    for (commit_id, release, metric_value) in [
        ("old-1", "r2019", 5.0),
        ("old-2", "r2019", 15.0),
        ("kept-1", "r2020", 0.0),
        ("kept-2", "r2021", 1.0),
    ] {
        let mut event = sample_event_with_release(commit_id, release);
        event.telemetry[0].metric_value = metric_value;
        store.ingest_commit_event(&event).expect("ingest event");
    }

    let retention = RetentionPolicy {
        raw_ttl_secs: 3600,
        max_release_partitions: Some(2),
    };
    let stats = store
        .promote_to_columnar_with_retention(&retention, now_ts_for_test())
        .expect("promote with retention");
    assert_eq!(stats.promoted_events, 4);

    let mut new_event = sample_event_with_release("new-r2019", "r2019");
    new_event.telemetry[0].metric_value = 25.0;
    store
        .ingest_commit_event(&new_event)
        .expect("ingest live sample for rolled-up release");
    store.promote_to_columnar().expect("promote live sample");

    let query = AdminQuery {
        name: Some("repo-a".to_string()),
        release: Some("r2019".to_string()),
    };
    let points = store.aggregate_by_query(&query).expect("aggregate metrics");
    let complexity = points
        .iter()
        .find(|point| point.metric_key == "estimated_cyclomatic_complexity")
        .expect("complexity metric");
    assert!((complexity.metric_value - 15.0).abs() < 1e-9);

    let scores = store
        .compute_committer_scores(
            &query,
            &ScoringWeights {
                version: "weighted-rollup-test".to_string(),
                complexity_weight: 1.0,
                coverage_weight: 0.0,
                churn_weight: 0.0,
                pipeline_weight: 0.0,
                pr_file_risk_weight: 0.0,
                pr_velocity_weight: 0.0,
                pr_approval_weight: 0.0,
            },
        )
        .expect("score committer");
    assert_eq!(scores.len(), 1);
    let conn = Connection::open(col.to_str().expect("col path")).expect("open analytics");
    let baseline_complexity: f64 = conn
        .query_row(
            "SELECT baseline_complexity FROM repo_baseline WHERE repo_name = ?1",
            params!["repo-a"],
            |row| row.get(0),
        )
        .expect("read baseline");
    assert!(baseline_complexity < 15.0);
    let expected_complexity_component = 100.0 / (1.0 + (15.0 - baseline_complexity).max(0.0));
    assert!(
        (scores[0].complexity_component - expected_complexity_component).abs() < 1e-9,
        "expected weighted complexity score {expected_complexity_component}, got {:?}",
        scores[0]
    );
}

#[test]
fn prunes_release_partitions_per_repo_without_cross_repo_drift() {
    let dir = tempdir().expect("tmp");
    let kv = dir.path().join("kv");
    let col = dir.path().join("analytics.duckdb");

    let store = DualLayerStore::open(
        kv.to_str().expect("kv path"),
        col.to_str().expect("col path"),
    )
    .expect("open");

    for (repo, releases) in [
        ("repo-a", ["r2019", "r2020", "r2021"]),
        ("repo-b", ["r2018", "r2019", "r2020"]),
    ] {
        for release in releases {
            store
                .ingest_commit_event(&sample_event_for_repo_release(
                    repo,
                    &format!("{repo}-{release}"),
                    release,
                ))
                .expect("ingest historical release");
        }
    }

    let policy = RetentionPolicy {
        raw_ttl_secs: 3600,
        max_release_partitions: Some(2),
    };
    let stats = store
        .promote_to_columnar_with_retention(&policy, now_ts_for_test())
        .expect("promote with retention");
    assert_eq!(stats.promoted_events, 6);

    let conn = Connection::open(col.to_str().expect("col path")).expect("open analytics");

    for (repo, stale_release, kept_release) in
        [("repo-a", "r2019", "r2021"), ("repo-b", "r2018", "r2020")]
    {
        let stale_raw_count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM telemetry_history WHERE repo_name = ?1 AND release = ?2",
                params![repo, stale_release],
                |row| row.get(0),
            )
            .expect("count stale raw");
        let stale_rollup_count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM telemetry_history_rollup WHERE repo_name = ?1 AND release = ?2",
                params![repo, stale_release],
                |row| row.get(0),
            )
            .expect("count stale rollup");
        let kept_raw_count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM telemetry_history WHERE repo_name = ?1 AND release = ?2",
                params![repo, kept_release],
                |row| row.get(0),
            )
            .expect("count kept raw");

        assert_eq!(stale_raw_count, 0);
        assert!(stale_rollup_count > 0);
        assert!(kept_raw_count > 0);
    }
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
fn async_idle_promotion_timeouts_do_not_create_snapshot_files() {
    let dir = tempdir().expect("tmp");
    let kv = dir.path().join("kv");
    let col = dir.path().join("analytics.duckdb");
    let engine = AsyncIngestionEngine::start_with_interval(
        kv.to_str().expect("kv path"),
        col.to_str().expect("col path"),
        2,
        5,
    )
    .expect("start async engine");

    let deadline = Instant::now() + Duration::from_secs(2);
    while engine.promotion_count() < 3 && Instant::now() < deadline {
        thread::sleep(Duration::from_millis(2));
    }
    assert!(
        engine.promotion_count() >= 3,
        "idle promotion timeouts did not run"
    );
    assert!(snapshot_files_for(&col).is_empty());
}

#[test]
fn async_ingestion_reports_background_storage_errors() {
    let dir = tempdir().expect("tmp");
    let kv = dir.path().join("kv");
    let col = dir.path().join("analytics.duckdb");
    let store = DualLayerStore::open(
        kv.to_str().expect("kv path"),
        col.to_str().expect("col path"),
    )
    .expect("open");
    let engine =
        AsyncIngestionEngine::start_with_store(store, 2, 5, None).expect("start async engine");
    let mut invalid_event = sample_event("invalid-float");
    invalid_event.telemetry[0].metric_value = f64::NAN;
    engine
        .enqueue(invalid_event)
        .expect("enqueue event for worker");

    let deadline = Instant::now() + Duration::from_secs(2);
    while engine.metrics().background_error_count == 0 && Instant::now() < deadline {
        thread::sleep(Duration::from_millis(2));
    }
    let metrics = engine.metrics();
    assert!(metrics.background_error_count > 0);
    assert!(metrics
        .last_background_error
        .as_deref()
        .is_some_and(|message| !message.is_empty()));
    assert_eq!(metrics.queue_depth, 0);
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
        write_raw_event(&kv, old_key.as_bytes(), &old_payload);
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
        Some(RetentionPolicy {
            raw_ttl_secs: 50,
            ..RetentionPolicy::default()
        }),
    )
    .expect("start");

    enqueue_with_backpressure(
        &engine,
        sample_event_with_release("active", "active-retention"),
    );

    let promotion_deadline = Instant::now() + Duration::from_secs(10);
    while engine.promotion_count() == 0 && Instant::now() < promotion_deadline {
        thread::sleep(Duration::from_millis(50));
    }
    assert!(
        engine.promotion_count() > 0,
        "background ingestion did not complete a promotion before the timeout"
    );
    // Stop the periodic worker before querying the published snapshot so later
    // interval promotions cannot contend with these read assertions.
    drop(engine);

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
    let engine =
        AsyncIngestionEngine::start_with_store(store.clone(), 256, 40, None).expect("start");

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

    let engine =
        AsyncIngestionEngine::start_with_store(store.clone(), 256, 40, None).expect("start");

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
        assert!(
            !points.is_empty(),
            "aggregate visibility must stay non-empty during handoff"
        );

        if engine.promotion_count() > 0 {
            break;
        }
        thread::sleep(Duration::from_millis(25));
    }
}

#[test]
fn async_ingestion_engine_tracks_enqueue_rejections_under_burst_pressure() {
    let dir = tempdir().expect("tmp");
    let kv = dir.path().join("kv");
    let col = dir.path().join("analytics.duckdb");
    let engine = AsyncIngestionEngine::start_with_interval(
        kv.to_str().expect("kv path"),
        col.to_str().expect("col path"),
        1,
        500,
    )
    .expect("start");

    let mut observed_rejections = 0usize;
    for idx in 0..400 {
        if engine
            .enqueue(sample_event(&format!("burst-{idx}")))
            .is_err()
        {
            observed_rejections += 1;
        }
    }

    assert!(
        observed_rejections > 0,
        "expected at least one enqueue rejection under burst pressure"
    );

    let mut drained = false;
    for _ in 0..200 {
        if engine.queue_depth() == 0 {
            drained = true;
            break;
        }
        thread::sleep(Duration::from_millis(25));
    }

    assert!(drained, "queue should drain");

    let metrics = engine.metrics();
    assert!(metrics.enqueue_rejections >= observed_rejections);
    assert!(metrics.max_queue_depth >= 1);
}

#[test]
fn async_ingestion_engine_tracks_queue_lag_and_promotion_throughput() {
    let dir = tempdir().expect("tmp");
    let kv = dir.path().join("kv");
    let col = dir.path().join("analytics.duckdb");
    let engine = AsyncIngestionEngine::start_with_interval(
        kv.to_str().expect("kv path"),
        col.to_str().expect("col path"),
        4,
        1000,
    )
    .expect("start");

    let attempts = 120usize;
    let mut observed_rejections = 0usize;
    let mut observed_accepts = 0usize;

    for idx in 0..attempts {
        if engine.enqueue(sample_event(&format!("slo-{idx}"))).is_ok() {
            observed_accepts += 1;
        } else {
            observed_rejections += 1;
        }
    }

    assert!(observed_accepts > 0);
    assert_eq!(observed_accepts + observed_rejections, attempts);

    thread::sleep(Duration::from_millis(40));
    assert!(engine.queue_depth() <= 4);
    assert!(engine.max_queue_depth() >= 1);

    let mut observed_promotion = false;
    for _ in 0..120 {
        if engine.promotion_count() > 0 {
            observed_promotion = true;
            break;
        }
        thread::sleep(Duration::from_millis(100));
    }

    assert!(
        observed_promotion,
        "promotion worker should run under queue pressure"
    );

    let mut drained = false;
    for _ in 0..240 {
        if engine.queue_depth() == 0 {
            drained = true;
            break;
        }
        thread::sleep(Duration::from_millis(100));
    }

    assert!(drained, "queue should drain after pressure burst");

    let metrics = engine.metrics();
    assert!(metrics.max_queue_depth >= 1);
    assert_eq!(metrics.enqueue_rejections, observed_rejections);
    assert!(metrics.max_queue_lag_ms > 0);
}

#[test]
fn committer_score_read_uses_published_snapshot_when_live_db_is_unavailable() {
    let dir = tempdir().expect("tmp");
    let kv = dir.path().join("kv");
    let col = dir.path().join("analytics.duckdb");
    let store = DualLayerStore::open(
        kv.to_str().expect("kv path"),
        col.to_str().expect("col path"),
    )
    .expect("open");

    store
        .ingest_commit_event(&sample_event("snapshot-score"))
        .expect("seed event");
    store.promote_to_columnar().expect("promote");

    let query = AdminQuery {
        name: Some("repo-a".to_string()),
        release: Some("v1.0.0".to_string()),
    };
    let weights = ScoringWeights::default();

    let initial_scores = store
        .compute_committer_scores(&query, &weights)
        .expect("compute before corruption");
    assert_eq!(initial_scores.len(), 1);

    fs::remove_file(&col).expect("remove live analytics db");

    let resilient_scores = store
        .compute_committer_scores(&query, &weights)
        .expect("compute from published snapshot");
    assert_eq!(resilient_scores.len(), 1);
}

#[test]
fn reseeding_baselines_refreshes_published_snapshot_for_committer_scores() {
    let dir = tempdir().expect("tmp");
    let kv = dir.path().join("kv");
    let col = dir.path().join("analytics.duckdb");
    let store = DualLayerStore::open(
        kv.to_str().expect("kv path"),
        col.to_str().expect("col path"),
    )
    .expect("open");

    store
        .ingest_commit_event(&sample_event_for_repo_release(
            "repo-a",
            "snapshot-refresh",
            "v1.0.0",
        ))
        .expect("ingest");
    store.promote_to_columnar().expect("promote");

    let query = AdminQuery {
        name: Some("repo-a".to_string()),
        release: Some("v1.0.0".to_string()),
    };
    let weights = ScoringWeights::default();

    let initial_scores = store
        .compute_committer_scores(&query, &weights)
        .expect("compute before reseed");
    assert_eq!(initial_scores.len(), 1);
    let initial_score = initial_scores[0].score;

    store
        .reseed_release_baseline("repo-a", 6.0)
        .expect("reseed baseline");

    let refreshed_scores = store
        .compute_committer_scores(&query, &weights)
        .expect("compute after reseed");
    assert_eq!(refreshed_scores.len(), 1);
    assert!(refreshed_scores[0].score < initial_score);
}
