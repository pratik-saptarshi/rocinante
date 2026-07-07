#![cfg(feature = "duckdb-analytics")]

use repo_analyzer_core::storage::{AnalyticsQueryMode, AnalyticsSnapshot};

#[test]
fn snapshot_mode_is_read_only() {
    let snapshot = AnalyticsSnapshot::new("analytics.duckdb", 42);
    assert!(snapshot.enforce_mode(AnalyticsQueryMode::ReadOnly).is_ok());
}

#[test]
fn mutable_mode_is_rejected_for_analytics_queries() {
    let snapshot = AnalyticsSnapshot::new("analytics.duckdb", 42);
    let err = snapshot
        .enforce_mode(AnalyticsQueryMode::Mutable)
        .expect_err("expected mode violation");
    assert!(err
        .to_string()
        .contains("Analytics queries must execute against read-only snapshots"));
}

#[test]
fn snapshot_descriptor_is_immutable_and_versioned() {
    let snapshot = AnalyticsSnapshot::new("analytics.duckdb", 7);
    assert_eq!(snapshot.snapshot_id, 7);
    assert!(snapshot.path.ends_with("analytics.duckdb"));
    assert!(snapshot.immutable);
}
