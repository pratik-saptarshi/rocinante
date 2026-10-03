pub mod auth;
pub mod engine;
pub mod errors;
pub mod git;
pub mod plugins;
pub mod telemetry;
pub mod types;

use engine::Pipeline;
use errors::AnalyzerError;
use git::discover_repositories_path;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::Duration;
use telemetry::{TelemetryImportSummary, TelemetryStore};
use types::{AdminQuery, AnalysisMetric, RepoTarget};

pub fn default_telemetry_db_path() -> PathBuf {
    if let Some(path) = std::env::var_os("ROCINANTE_TELEMETRY_DB") {
        return PathBuf::from(path);
    }

    #[cfg(target_os = "macos")]
    if let Some(home) = std::env::var_os("HOME") {
        return PathBuf::from(home)
            .join("Library/Application Support/Rocinante")
            .join("telemetry.db");
    }

    #[cfg(target_os = "windows")]
    if let Some(app_data) = std::env::var_os("APPDATA") {
        return PathBuf::from(app_data)
            .join("Rocinante")
            .join("telemetry.db");
    }

    #[cfg(target_os = "linux")]
    if let Some(data_home) = std::env::var_os("XDG_DATA_HOME") {
        return PathBuf::from(data_home)
            .join("rocinante")
            .join("telemetry.db");
    }

    #[cfg(any(
        target_os = "linux",
        not(any(target_os = "macos", target_os = "windows"))
    ))]
    if let Some(home) = std::env::var_os("HOME") {
        return PathBuf::from(home)
            .join(".local/share/rocinante")
            .join("telemetry.db");
    }

    std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join("telemetry.db")
}

pub fn resolve_telemetry_db_path() -> Result<PathBuf, AnalyzerError> {
    let path = default_telemetry_db_path();
    let parent = path
        .parent()
        .ok_or_else(|| AnalyzerError::Io("telemetry database has no parent directory".into()))?;
    std::fs::create_dir_all(parent)?;

    let legacy_path = std::env::current_dir()?.join("telemetry.db");
    let mut lock_path = path.as_os_str().to_os_string();
    lock_path.push(".migration.lock");
    let lock = std::fs::OpenOptions::new()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .open(PathBuf::from(lock_path))?;
    fs2::FileExt::lock_exclusive(&lock)?;
    if path != legacy_path && !path.exists() && legacy_path.is_file() {
        let temporary_path = path.with_extension("db.migrating");
        let source = rusqlite::Connection::open(&legacy_path)?;
        let mut destination = rusqlite::Connection::open(&temporary_path)?;
        {
            let backup = rusqlite::backup::Backup::new(&source, &mut destination)?;
            backup.run_to_completion(64, Duration::from_millis(10), None)?;
        }
        drop(destination);
        drop(source);
        match std::fs::rename(&temporary_path, &path) {
            Ok(()) => {}
            Err(_) if path.exists() => {
                let _ = std::fs::remove_file(&temporary_path);
            }
            Err(error) => return Err(error.into()),
        }
    }
    Ok(path)
}

pub fn run_scan(
    token: &str,
    root: &Path,
    release: &str,
    db_path: &Path,
) -> Result<TelemetryImportSummary, AnalyzerError> {
    scan_and_persist(token, root, release, db_path).map(|(summary, _, _)| summary)
}

pub fn run_scan_with_metrics(
    token: &str,
    root: &Path,
    release: &str,
    db_path: &Path,
) -> Result<(TelemetryImportSummary, Vec<types::RepositoryMetric>), AnalyzerError> {
    let (summary, names, store) = scan_and_persist(token, root, release, db_path)?;
    let metrics = store.query_repositories(&names, release)?;
    Ok((summary, metrics))
}

pub fn query_repository_metrics(
    token: &str,
    root: &Path,
    release: &str,
    db_path: &Path,
) -> Result<Vec<types::RepositoryMetric>, AnalyzerError> {
    auth::require_configured_token_secret()?;
    let principal = auth::decode_principal(token)?;
    auth::require_admin(&principal)?;
    let repositories = uniquely_named_repositories(root);
    let mut names = repositories
        .into_iter()
        .map(|repository| repository.name)
        .collect::<Vec<_>>();
    names.sort_unstable();
    names.dedup();
    TelemetryStore::open(db_path)?.query_repositories(&names, release)
}

fn scan_and_persist(
    token: &str,
    root: &Path,
    release: &str,
    db_path: &Path,
) -> Result<(TelemetryImportSummary, Vec<String>, TelemetryStore), AnalyzerError> {
    auth::require_configured_token_secret()?;
    let principal = auth::decode_principal(token)?;
    auth::require_admin(&principal)?;
    let repositories = uniquely_named_repositories(root);
    let pipeline = Pipeline::default();
    let mut records = Vec::with_capacity(repositories.len());

    for repository in repositories {
        records.push(pipeline.analyze_repo(repository, release)?);
    }

    let mut names = records
        .iter()
        .map(|record| record.repo_name.clone())
        .collect::<Vec<_>>();
    names.sort_unstable();
    names.dedup();
    let store = TelemetryStore::open(db_path)?;
    let summary = store.insert_records(&records, release)?;
    Ok((summary, names, store))
}

fn uniquely_named_repositories(root: &Path) -> Vec<RepoTarget> {
    let mut repositories = discover_repositories_path(root);
    disambiguate_duplicate_names(root, &mut repositories);
    repositories
}

fn disambiguate_duplicate_names(root: &Path, repositories: &mut [RepoTarget]) {
    let mut name_counts = HashMap::<String, usize>::new();
    for repository in repositories.iter() {
        *name_counts.entry(repository.name.clone()).or_default() += 1;
    }
    for repository in repositories.iter_mut() {
        if name_counts
            .get(&repository.name)
            .copied()
            .unwrap_or_default()
            > 1
        {
            let mut relative_path = repository
                .path
                .strip_prefix(root)
                .unwrap_or(&repository.path)
                .to_path_buf();
            if relative_path.as_os_str().is_empty() || relative_path == Path::new(".") {
                relative_path = Path::new(".").join(&repository.name);
            }
            repository.name = path_identity(&relative_path);
        }
    }
}

#[cfg(unix)]
fn path_identity(path: &Path) -> String {
    use std::os::unix::ffi::OsStrExt;

    encode_path_bytes(path.as_os_str().as_bytes(), b"/\\ -_.~")
}

#[cfg(windows)]
fn path_identity(path: &Path) -> String {
    use std::os::windows::ffi::OsStrExt;

    let mut identity = String::new();
    for unit in path.as_os_str().encode_wide() {
        if unit <= 0x7f && (unit as u8).is_ascii_alphanumeric()
            || matches!(unit, 0x2f | 0x5c | 0x20 | 0x2d | 0x5f | 0x2e | 0x7e)
        {
            identity.push(char::from_u32(u32::from(unit)).expect("ASCII path unit"));
        } else {
            identity.push_str(&format!("%u{unit:04X}"));
        }
    }
    identity
}

#[cfg(not(any(unix, windows)))]
fn path_identity(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

#[cfg(unix)]
fn encode_path_bytes(bytes: &[u8], safe_ascii: &[u8]) -> String {
    let mut identity = String::with_capacity(bytes.len());
    for byte in bytes {
        if byte.is_ascii_alphanumeric() || safe_ascii.contains(byte) {
            identity.push(char::from(*byte));
        } else {
            identity.push_str(&format!("%{byte:02X}"));
        }
    }
    identity
}

#[cfg(test)]
mod repository_identity_tests {
    use super::disambiguate_duplicate_names;
    use crate::types::RepoTarget;

    #[cfg(unix)]
    #[test]
    fn duplicate_repository_names_preserve_non_utf8_path_identity() {
        use std::os::unix::ffi::OsStringExt;

        let root = tempfile::tempdir().expect("workspace root");
        let mut repositories = [0xfe, 0xff]
            .into_iter()
            .map(|byte| {
                let parent =
                    std::ffi::OsString::from_vec(vec![b'g', b'r', b'o', b'u', b'p', b'-', byte]);
                RepoTarget {
                    name: "shared".into(),
                    path: root.path().join(parent).join("shared"),
                }
            })
            .collect::<Vec<_>>();
        repositories.push(RepoTarget {
            name: "shared".into(),
            path: root.path().join("group-%FE").join("shared"),
        });
        disambiguate_duplicate_names(root.path(), &mut repositories);
        let mut names = repositories
            .into_iter()
            .map(|repository| repository.name)
            .collect::<Vec<_>>();
        names.sort_unstable();
        assert_eq!(
            names,
            ["group-%25FE/shared", "group-%FE/shared", "group-%FF/shared"]
        );
    }
}

pub fn query_metrics(
    token: &str,
    query: &AdminQuery,
    db_path: &Path,
) -> Result<Vec<AnalysisMetric>, AnalyzerError> {
    auth::require_configured_token_secret()?;
    let principal = auth::decode_principal(token)?;
    auth::require_admin(&principal)?;
    TelemetryStore::open(db_path)?.query(query)
}
