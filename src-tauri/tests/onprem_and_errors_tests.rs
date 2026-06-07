use repo_analyzer_core::errors::AnalyzerError;
use repo_analyzer_core::onprem::{
    ActiveDirectoryProvider, DirectoryProvider, InternalGitProvider, LocalGitProvider,
};

#[test]
fn local_git_provider_builds_internal_url() {
    let p = LocalGitProvider;
    let url = p.repo_url("repo-a").expect("url");
    assert_eq!(url, "ssh://git.internal/repo-a.git");
}

#[test]
fn active_directory_provider_returns_group_membership() {
    let d = ActiveDirectoryProvider;
    assert!(d.is_in_group("alice", "engineering").expect("group check"));
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
