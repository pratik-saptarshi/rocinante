use repo_analyzer_core::storage::{StorageOperation, StorageRoute};

#[test]
fn rejects_ingestion_write_on_analytics_route() {
    let err = StorageRoute::Analytics
        .enforce(StorageOperation::IngestWrite)
        .expect_err("expected route violation");
    assert!(err
        .to_string()
        .contains("Ingestion writes must route exclusively to BadgerDB"));
}

#[test]
fn rejects_analytics_query_on_ingestion_route() {
    let err = StorageRoute::Ingestion
        .enforce(StorageOperation::AnalyticsQuery)
        .expect_err("expected route violation");
    assert!(err
        .to_string()
        .contains("Analytics queries must route exclusively to DuckDB"));
}

#[test]
fn allows_correct_route_operation_pairs() {
    assert!(StorageRoute::Ingestion
        .enforce(StorageOperation::IngestWrite)
        .is_ok());
    assert!(StorageRoute::Analytics
        .enforce(StorageOperation::AnalyticsQuery)
        .is_ok());
}
