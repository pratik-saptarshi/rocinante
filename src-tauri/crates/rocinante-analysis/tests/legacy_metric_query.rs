use rocinante_analysis::telemetry::TelemetryStore;
use rocinante_analysis::types::{AdminQuery, AnalysisMetric, AnalysisRecord};

fn record(repo_name: &str, release: &str, value: f64) -> AnalysisRecord {
    AnalysisRecord {
        repo_name: repo_name.to_string(),
        release: release.to_string(),
        metrics: vec![AnalysisMetric {
            plugin: "complexity".to_string(),
            key: "score".to_string(),
            value,
            details: "query compatibility fixture".to_string(),
        }],
    }
}

#[test]
fn query_suppresses_legacy_rows_only_when_a_stable_same_release_exists() {
    let directory = tempfile::tempdir().expect("temporary directory");
    let mut store =
        TelemetryStore::open(directory.path().join("telemetry.db")).expect("telemetry store");
    store
        .replace_records(
            &[
                record("foo", "v1", 1.0),
                record("foo", "v2", 2.0),
                record("foo abcdefghijklmnopabcdefghijklmnop", "v2", 3.0),
            ],
            "legacy-query-fixture",
        )
        .expect("seed telemetry rows");

    let all_releases = store
        .query(&AdminQuery {
            name: Some("foo".to_string()),
            release: None,
        })
        .expect("query all matching releases");
    let mut values = all_releases
        .iter()
        .map(|metric| metric.value)
        .collect::<Vec<_>>();
    values.sort_by(f64::total_cmp);
    assert_eq!(values, vec![1.0, 3.0]);

    let old_release = store
        .query(&AdminQuery {
            name: Some("foo".to_string()),
            release: Some("v1".to_string()),
        })
        .expect("query older release");
    assert_eq!(old_release.len(), 1);
    assert_eq!(old_release[0].value, 1.0);
}

#[test]
fn query_keeps_distinct_stable_repositories_with_the_same_basename() {
    let directory = tempfile::tempdir().expect("temporary directory");
    let mut store =
        TelemetryStore::open(directory.path().join("telemetry.db")).expect("telemetry store");
    store
        .replace_records(
            &[
                record("foo abcdefghijklmnopabcdefghijklmnop", "v2", 3.0),
                record("foo pppppppppppppppppppppppppppppppp", "v2", 3.0),
            ],
            "stable-query-fixture",
        )
        .expect("seed telemetry rows");

    let metrics = store
        .query(&AdminQuery {
            name: Some("foo".to_string()),
            release: None,
        })
        .expect("query stable repositories");
    assert_eq!(metrics.len(), 2);
}
