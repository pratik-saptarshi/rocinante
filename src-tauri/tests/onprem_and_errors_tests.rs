use repo_analyzer_core::errors::AnalyzerError;
use repo_analyzer_core::onprem::{
    ActiveDirectoryProvider, DirectoryProvider, InternalGitProvider, LocalGitProvider,
    StaticDirectoryLookup,
};
use std::collections::HashMap;
use std::sync::Arc;

#[test]
fn local_git_provider_builds_internal_url() {
    let p = LocalGitProvider;
    let url = p.repo_url("repo-a").expect("url");
    assert_eq!(url, "ssh://git.internal/repo-a.git");
}

#[test]
fn active_directory_provider_returns_group_membership() {
    let mut memberships = HashMap::new();
    memberships.insert("alice".to_string(), vec!["engineering".to_string()]);
    let source = Arc::new(StaticDirectoryLookup::new(memberships));
    let mut aliases = HashMap::new();
    aliases.insert("eng".to_string(), "engineering".to_string());

    let d = ActiveDirectoryProvider::with_group_aliases(source.clone(), aliases);

    assert!(d.is_in_group(" Alice ", "eng").expect("group check"));
    assert!(d
        .is_in_group("alice", "engineering")
        .expect("cached group check"));
    assert_eq!(source.lookup_count(), 1);
    assert_eq!(d.cache_size(), 1);
}

#[test]
fn active_directory_provider_denies_blank_or_invalid_inputs() {
    let source = Arc::new(StaticDirectoryLookup::new(HashMap::new()));
    let d = ActiveDirectoryProvider::new(source);

    let blank_user = d.is_in_group("   ", "engineering").expect_err("blank user");
    assert!(blank_user.to_string().contains("blank user"));

    let invalid_group = d
        .is_in_group("alice", "eng\u{0007}")
        .expect_err("control char group");
    assert!(invalid_group.to_string().contains("invalid group"));

    assert!(!d
        .is_in_group("alice", "engineering")
        .expect("unknown group"));
}

#[test]
fn active_directory_provider_rejects_alias_cycles() {
    let source = Arc::new(StaticDirectoryLookup::new(HashMap::new()));
    let mut aliases = HashMap::new();
    aliases.insert("eng".to_string(), "engineering".to_string());
    aliases.insert("engineering".to_string(), "eng".to_string());

    let d = ActiveDirectoryProvider::with_group_aliases(source, aliases);
    let err = d.is_in_group("alice", "eng").expect_err("alias cycle");
    assert!(err.to_string().contains("alias cycle"));
}

#[test]
fn io_and_db_errors_map_to_analyzer_error() {
    let io_err = std::io::Error::other("boom");
    let e: AnalyzerError = io_err.into();
    assert!(e.to_string().contains("io error"));

    let db_err = rusqlite::Error::InvalidPath("bad".into());
    let e2: AnalyzerError = db_err.into();
    assert!(e2.to_string().contains("db error"));
}
