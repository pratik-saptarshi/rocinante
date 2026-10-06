#![cfg(feature = "analytics")]

use repo_analyzer_core::auth::issue_test_token;
use repo_analyzer_core::storage::BaselineStore;
use tempfile::tempdir;

#[test]
fn baseline_compatibility_surfaces_reject_the_fallback_secret() {
    std::env::remove_var("RUNICIPAL_TOKEN_SECRET");
    let token = issue_test_token("alice", &["admin"], 3600);
    let dir = tempdir().expect("temporary directory");
    let direct_kv = dir.path().join("direct-kv");
    let direct_col = dir.path().join("direct-analytics.duckdb");
    let store = BaselineStore::open(
        direct_kv.to_str().expect("direct kv path"),
        direct_col.to_str().expect("direct analytics path"),
    )
    .expect("open direct baseline store");
    store
        .reseed_release_baseline("repo-a", 4.5)
        .expect("seed baseline directly");

    let with_store_query = repo_analyzer_core::tauri_commands::query_release_baseline_with_store(
        token.clone(),
        store.clone(),
        "repo-a".to_string(),
    )
    .expect_err("with-store query requires configured secret");
    assert!(with_store_query.contains("RUNICIPAL_TOKEN_SECRET"));

    let with_store_reseed = repo_analyzer_core::tauri_commands::reseed_release_baseline_with_store(
        token.clone(),
        store.clone(),
        "repo-a".to_string(),
        9.0,
    )
    .expect_err("with-store reseed requires configured secret");
    assert!(with_store_reseed.contains("RUNICIPAL_TOKEN_SECRET"));
    assert_eq!(
        store
            .read_release_baseline("repo-a")
            .expect("read baseline directly"),
        Some(4.5),
        "rejected reseed must not mutate the baseline"
    );

    let path_kv = dir.path().join("must-not-open-kv");
    let path_col = dir.path().join("must-not-open-analytics.duckdb");
    let path_query = repo_analyzer_core::tauri_commands::query_release_baseline(
        token.clone(),
        path_kv.to_str().expect("path kv").to_string(),
        path_col.to_str().expect("path analytics").to_string(),
        "repo-a".to_string(),
    )
    .expect_err("path query requires configured secret before opening a store");
    assert!(path_query.contains("RUNICIPAL_TOKEN_SECRET"));
    assert!(!path_kv.exists());
    assert!(!path_col.exists());

    let path_reseed = repo_analyzer_core::tauri_commands::reseed_release_baseline(
        token,
        path_kv.to_str().expect("path kv").to_string(),
        path_col.to_str().expect("path analytics").to_string(),
        "repo-a".to_string(),
        12.0,
    )
    .expect_err("path reseed requires configured secret before opening a store");
    assert!(path_reseed.contains("RUNICIPAL_TOKEN_SECRET"));
    assert!(!path_kv.exists());
    assert!(!path_col.exists());
}
