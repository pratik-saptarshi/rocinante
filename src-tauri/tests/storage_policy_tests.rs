use repo_analyzer_core::storage::{IngestionEngine, RetentionPolicy, StorageProfile};

#[test]
fn rejects_workload_mixing_when_profile_is_invalid() {
    let profile = StorageProfile {
        ingestion_engine: IngestionEngine::Sled,
        analytics_engine: "duckdb".to_string(),
    };

    let err = profile.validate().expect_err("expected invalid profile");
    assert!(err.to_string().contains("BadgerDB"));
}

#[test]
fn accepts_badger_duckdb_profile() {
    let profile = StorageProfile::strict_badger_duckdb();
    assert!(profile.validate().is_ok());
}

#[test]
fn prunes_raw_events_after_ttl_window() {
    let policy = RetentionPolicy { raw_ttl_secs: 60 };
    assert!(policy.is_raw_event_expired(100, 161));
    assert!(!policy.is_raw_event_expired(100, 159));
}
