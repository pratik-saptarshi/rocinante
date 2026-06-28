use repo_analyzer_core::telemetry::TelemetryStore;
use repo_analyzer_core::types::{AdminQuery, AnalysisMetric, AnalysisRecord};
use tempfile::NamedTempFile;

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
        AnalysisRecord {
            repo_name: "repo-x".to_string(),
            release: "v1.2.3".to_string(),
            metrics: vec![AnalysisMetric {
                plugin: "complexity".to_string(),
                key: "estimated_cyclomatic_complexity".to_string(),
                value: 12.0,
                details: "demo-a".to_string(),
            }],
        },
        AnalysisRecord {
            repo_name: "repo-y".to_string(),
            release: "v1.2.3".to_string(),
            metrics: vec![
                AnalysisMetric {
                    plugin: "parser".to_string(),
                    key: "ast_nodes".to_string(),
                    value: 8.0,
                    details: "demo-b".to_string(),
                },
                AnalysisMetric {
                    plugin: "velocity".to_string(),
                    key: "churn_window".to_string(),
                    value: 4.0,
                    details: "demo-c".to_string(),
                },
            ],
        },
    ];

    let inserted = store.insert_records(&records).expect("bulk insert records");

    assert_eq!(inserted, 3);

    let results = store
        .query(&AdminQuery {
            name: Some("repo-".to_string()),
            release: Some("v1.2".to_string()),
        })
        .expect("query");
    assert_eq!(results.len(), 3);
}

#[test]
fn bulk_import_empty_batch_is_noop() {
    let f = NamedTempFile::new().expect("temp db");
    let db_path = f.path().to_string_lossy().to_string();
    let store = TelemetryStore::open(&db_path).expect("open db");

    let inserted = store.insert_records(&[]).expect("bulk insert records");
    assert_eq!(inserted, 0);

    let results = store
        .query(&AdminQuery {
            name: Some("repo".to_string()),
            release: Some("v1".to_string()),
        })
        .expect("query");
    assert!(results.is_empty());
}
