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
use plugins::sanitizer::scrub_text;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::Duration;
use telemetry::{TelemetryImportSummary, TelemetryStore};
use types::{AdminQuery, AnalysisMetric, RepoTarget};

pub fn default_telemetry_db_path() -> PathBuf {
    if let Some(path) = std::env::var_os("ROCINANTE_TELEMETRY_DB") {
        let path = PathBuf::from(path);
        return if path.is_absolute() {
            path
        } else {
            std::env::current_dir()
                .unwrap_or_else(|_| PathBuf::from("."))
                .join(path)
        };
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

/// Return the per-user data directory shared by telemetry and application stores.
pub fn default_application_data_dir() -> PathBuf {
    default_telemetry_db_path()
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")))
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
    if repositories.is_empty() {
        return Err(AnalyzerError::Io(format!(
            "no Git repositories found under {}",
            root.display()
        )));
    }
    let mut names = repositories
        .into_iter()
        .map(|repository| repository.name)
        .collect::<Vec<_>>();
    names.extend(legacy_repository_names(root));
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
    if repositories.is_empty() {
        return Err(AnalyzerError::Io(format!(
            "no Git repositories found under {}",
            root.display()
        )));
    }
    let pipeline = Pipeline::default();
    let mut records = Vec::with_capacity(repositories.len());

    for repository in repositories {
        records.push(pipeline.analyze_repo(repository, release)?);
    }

    let mut names = records
        .iter()
        .map(|record| record.repo_name.clone())
        .collect::<Vec<_>>();
    names.extend(legacy_repository_names(root));
    names.sort_unstable();
    names.dedup();
    let store = TelemetryStore::open(db_path)?;
    let summary = store.insert_records(&records, release)?;
    Ok((summary, names, store))
}

fn uniquely_named_repositories(root: &Path) -> Vec<RepoTarget> {
    let mut repositories = discover_repositories_path(root);
    for repository in &mut repositories {
        repository.name = stable_repository_name(&repository.path, &repository.name);
    }
    repositories
}

fn legacy_repository_names(root: &Path) -> Vec<String> {
    let mut repositories = discover_repositories_path(root);
    disambiguate_duplicate_names(root, &mut repositories);
    repositories
        .into_iter()
        .map(|repository| repository.name)
        .collect()
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

fn stable_repository_name(path: &Path, display_name: &str) -> String {
    let stable_path = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
    let encoded_path = path_identity(&stable_path);
    let digest = Sha256::digest(encoded_path.as_bytes());
    let mut stable_suffix = String::with_capacity(32);
    for byte in digest.iter().take(16) {
        stable_suffix.push(char::from(b'a' + (byte >> 4)));
        stable_suffix.push(char::from(b'a' + (byte & 0x0f)));
    }
    // A local path hash stays stable across scan-root changes without
    // persisting the absolute repository path in telemetry.
    format!("{} {stable_suffix}", scrub_text(display_name))
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

#[cfg(test)]
mod repository_identity_tests {
    use super::stable_repository_name;

    #[cfg(unix)]
    #[test]
    fn repository_identity_is_unique_stable_and_hides_non_utf8_paths() {
        use std::os::unix::ffi::OsStringExt;

        let root = tempfile::tempdir().expect("workspace root");
        let paths = [0xfe, 0xff]
            .into_iter()
            .map(|byte| {
                let parent =
                    std::ffi::OsString::from_vec(vec![b'g', b'r', b'o', b'u', b'p', b'-', byte]);
                root.path().join(parent).join("shared")
            })
            .collect::<Vec<_>>();
        let escaped_looking_path = root.path().join("group-%FE").join("shared");
        let paths = paths
            .into_iter()
            .chain(std::iter::once(escaped_looking_path))
            .collect::<Vec<_>>();
        let names = paths
            .iter()
            .map(|path| stable_repository_name(path, "shared"))
            .collect::<Vec<_>>();
        assert_eq!(
            names.iter().collect::<std::collections::HashSet<_>>().len(),
            paths.len()
        );
        assert!(names.iter().all(|name| name.starts_with("shared ")));
        assert!(names.iter().all(|name| !name.contains('%')));
        assert_eq!(
            stable_repository_name(&paths[0], "shared"),
            names[0],
            "the same repository must keep its identity when selected from a different scan root"
        );
    }
}
