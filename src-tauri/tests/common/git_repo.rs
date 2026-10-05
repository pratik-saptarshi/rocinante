use std::fs;
use std::path::Path;
use std::process::Command;

pub fn initialize_tagged_repository(repo: &Path, tag: &str) {
    run_git(repo, &["init", "--quiet"]);
    run_git(repo, &["config", "user.name", "Test User"]);
    run_git(repo, &["config", "user.email", "test@example.com"]);

    fs::create_dir(repo.join("test-hooks")).expect("create empty Git hooks directory");
    run_git(repo, &["config", "commit.gpgsign", "false"]);
    run_git(repo, &["config", "core.hooksPath", "test-hooks"]);
    run_git(repo, &["add", "--all"]);
    run_git(repo, &["commit", "--quiet", "-m", "initial fixture"]);
    run_git(repo, &["tag", tag]);
}

fn run_git(repo: &Path, args: &[&str]) {
    let output = Command::new("git")
        .args(args)
        .current_dir(repo)
        .output()
        .expect("run git");
    assert!(
        output.status.success(),
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}
