#![cfg(feature = "analytics")]

use repo_analyzer_core::storage::BaselineStore;
use tempfile::tempdir;

#[test]
fn baseline_store_adapter_roundtrips_through_storage() {
    let dir = tempdir().expect("tmp");
    let kv = dir.path().join("kv");
    let col = dir.path().join("analytics.duckdb");
    let store = BaselineStore::open(
        kv.to_str().expect("kv path"),
        col.to_str().expect("col path"),
    )
    .expect("open adapter");

    assert_eq!(store.read_release_baseline("repo-a").expect("read"), None);

    let seeded = store
        .reseed_release_baseline("repo-a", 16.25)
        .expect("reseed");
    assert_eq!(seeded, 16.25);
    assert_eq!(
        store.read_release_baseline("repo-a").expect("read"),
        Some(16.25)
    );
}
