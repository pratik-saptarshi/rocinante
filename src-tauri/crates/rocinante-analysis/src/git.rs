use crate::errors::AnalyzerError;
use crate::types::RepoTarget;
use std::path::{Path, PathBuf};
use std::process::Command;
use walkdir::WalkDir;

pub fn discover_repositories(root: &str) -> Vec<RepoTarget> {
    discover_repositories_path(Path::new(root))
}

pub fn discover_repositories_path(root: &Path) -> Vec<RepoTarget> {
    let mut repos = Vec::new();
    for entry in WalkDir::new(root).follow_links(false).into_iter().flatten() {
        if entry.file_name() == ".git"
            && (entry.file_type().is_dir() || entry.file_type().is_file())
        {
            if let Some(repo_path) = entry.path().parent() {
                let name = repo_path
                    .file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_else(|| "unknown".to_string());
                repos.push(RepoTarget {
                    name,
                    path: PathBuf::from(repo_path),
                });
            }
        }
    }
    repos
}

pub fn git_stdout(repo_path: &str, args: &[&str]) -> Result<String, AnalyzerError> {
    git_stdout_path(Path::new(repo_path), args)
}

pub fn git_stdout_path(repo_path: &Path, args: &[&str]) -> Result<String, AnalyzerError> {
    let output = Command::new("git")
        .args(args)
        .current_dir(repo_path)
        .output()
        .map_err(|e| AnalyzerError::Git(e.to_string()))?;

    if !output.status.success() {
        return Err(AnalyzerError::Git(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

pub fn changed_files_since_tag(
    repo_path: &str,
    release: &str,
) -> Result<Vec<String>, AnalyzerError> {
    changed_files_since_tag_path(Path::new(repo_path), release)
}

pub fn changed_files_since_tag_path(
    repo_path: &Path,
    release: &str,
) -> Result<Vec<String>, AnalyzerError> {
    if release.is_empty() {
        return Ok(Vec::new());
    }
    let diff = git_stdout_path(repo_path, &["diff", "--name-only", release, "HEAD"])?;
    Ok(diff
        .lines()
        .map(str::trim)
        .filter(|v| !v.is_empty())
        .map(ToString::to_string)
        .collect())
}
