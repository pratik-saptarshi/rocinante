#![cfg(feature = "analytics")]

use repo_analyzer_core::auth::issue_test_token;
use repo_analyzer_core::storage::BaselineStore;
use repo_analyzer_core::tauri_commands;
use tempfile::tempdir;

#[test]
fn release_baseline_commands_roundtrip_through_store() {
    let dir = tempdir().expect("tmp");
    let kv = dir.path().join("kv");
    let col = dir.path().join("analytics.duckdb");
    let token = issue_test_token("alice", &["admin"], 3600);
    let store = BaselineStore::open(
        kv.to_str().expect("kv path"),
        col.to_str().expect("col path"),
    )
    .expect("open");

    let seeded = tauri_commands::reseed_release_baseline_with_store(
        token.to_string(),
        store.clone(),
        "repo-a".to_string(),
        19.25,
    )
    .expect("seed baseline through command path");
    assert_eq!(seeded, 19.25);

    let queried = tauri_commands::query_release_baseline_with_store(
        token.to_string(),
        store,
        "repo-a".to_string(),
    )
    .expect("query baseline through command path");
    assert_eq!(queried, Some(19.25));
}
