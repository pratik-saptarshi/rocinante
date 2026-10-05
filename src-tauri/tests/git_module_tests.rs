use repo_analyzer_core::git::{
    changed_files_since_tag, changed_files_since_tag_path_optional, discover_repositories,
    git_stdout,
};
use std::fs;
use std::process::Command;
use tempfile::tempdir;

#[test]
fn discovers_nested_git_repositories() {
    let dir = tempdir().expect("tmp");
    let repo = dir.path().join("repo-a");
    fs::create_dir_all(&repo).expect("mkdir");
    Command::new("git")
        .arg("init")
        .current_dir(&repo)
        .output()
        .expect("git init");

    let repos = discover_repositories(dir.path().to_str().expect("path"));
    assert!(repos.iter().any(|r| r.name == "repo-a"));
}

#[test]
fn git_stdout_returns_error_for_non_repo() {
    let dir = tempdir().expect("tmp");
    let err = git_stdout(dir.path().to_str().expect("path"), &["status"])
        .expect_err("expected git error");
    assert!(err.to_string().contains("git error"));
}

#[test]
fn changed_files_since_tag_returns_empty_on_empty_release() {
    let dir = tempdir().expect("tmp");
    let files = changed_files_since_tag(dir.path().to_str().expect("path"), "").expect("ok");
    assert!(files.is_empty());
}

#[test]
fn changed_files_since_missing_tag_is_distinguishable_from_full_scan() {
    let dir = tempdir().expect("tmp");
    let init = Command::new("git")
        .args(["init", "--quiet"])
        .current_dir(dir.path())
        .output()
        .expect("git init");
    assert!(init.status.success(), "git init failed: {:?}", init.stderr);

    let optional_files = changed_files_since_tag_path_optional(dir.path(), "release-not-here")
        .expect("checking a missing release ref");
    assert!(
        optional_files.is_none(),
        "missing release refs are not full scans"
    );

    let error = changed_files_since_tag(dir.path().to_str().expect("path"), "release-not-here")
        .expect_err("the strict API should reject a missing release ref");
    assert!(error.to_string().contains("release-not-here"));
}
