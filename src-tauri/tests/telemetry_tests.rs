use repo_analyzer_core::admin;
use repo_analyzer_core::telemetry::TelemetryStore;
use repo_analyzer_core::types::{AdminQuery, AnalysisMetric, AnalysisRecord};
use std::fs;
use tempfile::NamedTempFile;

fn sample_record(repo_name: &str, release: &str, key: &str, value: f64) -> AnalysisRecord {
    AnalysisRecord {
        repo_name: repo_name.to_string(),
        release: release.to_string(),
        metrics: vec![AnalysisMetric {
            plugin: "complexity".to_string(),
            key: key.to_string(),
            value,
            details: "demo".to_string(),
        }],
    }
}

#[test]
fn inserts_and_queries_by_release() {
    let f = NamedTempFile::new().expect("temp db");
    let db_path = f.path().to_string_lossy().to_string();
    let store = TelemetryStore::open(&db_path).expect("open db");

    let record = AnalysisRecord {
        repo_name: "repo-x".to_string(),
        release: "v1.2.3".to_string(),
        metrics: vec![AnalysisMetric {
            plugin: "complexity".to_string(),
            key: "estimated_cyclomatic_complexity".to_string(),
            value: 12.0,
            details: "demo".to_string(),
        }],
    };

    store.insert_record(&record).expect("insert");
    let results = store
        .query(&AdminQuery {
            name: Some("repo-x".to_string()),
            release: Some("v1.2".to_string()),
        })
        .expect("query");

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].plugin, "complexity");
}

#[test]
fn bulk_imports_multiple_records_and_queries_all_rows() {
    let f = NamedTempFile::new().expect("temp db");
    let db_path = f.path().to_string_lossy().to_string();
    let store = TelemetryStore::open(&db_path).expect("open db");

    let records = vec![
        sample_record("repo-x", "v1.2.3", "estimated_cyclomatic_complexity", 12.0),
        sample_record("repo-y", "v1.2.3", "estimated_cyclomatic_complexity", 14.0),
    ];

    let summary = store
        .insert_records(&records, "batch-1")
        .expect("bulk insert records");
    assert_eq!(summary.source, "batch-1");
    assert_eq!(summary.records_processed, 2);
    assert_eq!(summary.rows_inserted, 2);
    assert_eq!(summary.duplicate_source_keys, 0);

    let summaries = store.query_import_summaries().expect("query summaries");
    assert_eq!(summaries.len(), 1);
    assert_eq!(summaries[0].source, "batch-1");
    assert_eq!(summaries[0].rows_inserted, 2);

    let results = store
        .query(&AdminQuery {
            name: None,
            release: Some("v1.2.3".to_string()),
        })
        .expect("query");
    assert_eq!(results.len(), 2);
}

#[test]
fn bulk_import_dedupes_duplicate_source_keys() {
    let f = NamedTempFile::new().expect("temp db");
    let db_path = f.path().to_string_lossy().to_string();
    let store = TelemetryStore::open(&db_path).expect("open db");

    let record = sample_record("repo-x", "v1.2.3", "estimated_cyclomatic_complexity", 12.0);
    let summary = store
        .insert_records(&[record.clone(), record], "batch-dup")
        .expect("bulk insert records");

    assert_eq!(summary.records_processed, 2);
    assert_eq!(summary.rows_inserted, 1);
    assert_eq!(summary.duplicate_source_keys, 1);

    let results = store
        .query(&AdminQuery {
            name: Some("repo-x".to_string()),
            release: Some("v1.2.3".to_string()),
        })
        .expect("query");
    assert_eq!(results.len(), 1);
}

#[test]
fn scan_replacement_updates_values_and_removes_stale_metrics_atomically() {
    let f = NamedTempFile::new().expect("temp db");
    let db_path = f.path().to_string_lossy().to_string();
    let mut store = TelemetryStore::open(&db_path).expect("open db");

    let mut first_snapshot = sample_record("repo-x", "v1.2.3", "current", 12.0);
    first_snapshot.metrics.push(AnalysisMetric {
        plugin: "complexity".to_string(),
        key: "stale_metric".to_string(),
        value: 3.0,
        details: "obsolete".to_string(),
    });
    assert_eq!(
        store
            .replace_records(&[first_snapshot], "scan")
            .expect("persist initial snapshot")
            .rows_inserted,
        2
    );

    let mut current_snapshot = sample_record("repo-x", "v1.2.3", "current", 24.0);
    current_snapshot.metrics[0].details = "updated".to_string();
    let summary = store
        .replace_records(&[current_snapshot.clone()], "scan")
        .expect("replace previous snapshot");
    assert_eq!(summary.rows_inserted, 1);
    assert_eq!(summary.duplicate_source_keys, 0);

    let metrics = store
        .query(&AdminQuery {
            name: Some("repo-x".to_string()),
            release: Some("v1.2.3".to_string()),
        })
        .expect("query replaced snapshot");
    assert_eq!(metrics.len(), 1);
    assert_eq!(metrics[0].key, "current");
    assert_eq!(metrics[0].value, 24.0);
    assert_eq!(metrics[0].details, "updated");

    let unchanged = store
        .replace_records(&[current_snapshot], "scan")
        .expect("replace with an unchanged snapshot");
    assert_eq!(unchanged.rows_inserted, 0);
    assert_eq!(unchanged.duplicate_source_keys, 1);
}

#[test]
fn run_scan_surfaces_import_summary_for_backend_boundary() {
    std::env::set_var(
        "RUNICIPAL_TOKEN_SECRET",
        "test-secret-for-telemetry-scan-32-bytes",
    );
    let root = tempfile::tempdir().expect("root");
    let repo = root.path().join("repo-x");
    fs::create_dir_all(repo.join(".git")).expect("git dir");
    fs::create_dir_all(repo.join("src")).expect("src dir");
    fs::write(repo.join("src/lib.rs"), "pub fn example() {}\n").expect("file");

    let db = NamedTempFile::new().expect("temp db");
    let token = repo_analyzer_core::auth::issue_test_token("scan-admin", &["admin"], 300);
    let summary = admin::run_scan(&token, root.path().to_str().expect("root"), "", db.path())
        .expect("run scan");

    assert_eq!(summary.source, "");
    assert_eq!(summary.records_processed, 1);
    assert!(summary.rows_inserted >= 1);
}
