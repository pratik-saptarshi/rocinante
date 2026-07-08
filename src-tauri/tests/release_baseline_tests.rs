#![cfg(feature = "analytics")]

use repo_analyzer_core::admin;
use repo_analyzer_core::auth::issue_test_token;
use repo_analyzer_core::storage::DualLayerStore;
use tempfile::tempdir;

#[test]
fn release_baseline_roundtrips_through_storage() {
    let dir = tempdir().expect("tmp");
    let kv = dir.path().join("kv");
    let col = dir.path().join("analytics.duckdb");
    let store = DualLayerStore::open(
        kv.to_str().expect("kv path"),
        col.to_str().expect("col path"),
    )
    .expect("open");

    assert_eq!(store.read_release_baseline("repo-a").expect("read"), None);

    let seeded = store
        .reseed_release_baseline("repo-a", 18.5)
        .expect("reseed");
    assert_eq!(seeded, 18.5);
    assert_eq!(
        store.read_release_baseline("repo-a").expect("read"),
        Some(18.5)
    );

    let reseeded = store
        .reseed_release_baseline("repo-a", 12.25)
        .expect("reseed again");
    assert_eq!(reseeded, 12.25);
    assert_eq!(
        store.read_release_baseline("repo-a").expect("read"),
        Some(12.25)
    );
}

#[test]
fn admin_release_baseline_roundtrips_and_requires_admin() {
    let dir = tempdir().expect("tmp");
    let kv = dir.path().join("kv");
    let col = dir.path().join("analytics.duckdb");
    let admin_token = issue_test_token("alice", &["admin"], 3600);
    let reader_token = issue_test_token("bob", &["reader"], 3600);

    let seeded = admin::reseed_release_baseline(
        &admin_token,
        kv.to_str().expect("kv path"),
        col.to_str().expect("col path"),
        "repo-a",
        14.75,
    )
    .expect("reseed");
    assert_eq!(seeded, 14.75);

    let query = admin::query_release_baseline(
        &admin_token,
        kv.to_str().expect("kv path"),
        col.to_str().expect("col path"),
        "repo-a",
    )
    .expect("query");
    assert_eq!(query, Some(14.75));

    let err = admin::query_release_baseline(
        &reader_token,
        kv.to_str().expect("kv path"),
        col.to_str().expect("col path"),
        "repo-a",
    )
    .expect_err("permission denied");
    assert!(err.to_string().contains("permission denied"));
}
