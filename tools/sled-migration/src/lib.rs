//! One-time, copy-based migration of legacy Sled ingestion data to SQLite.

use fs2::FileExt;
use rocinante_storage::ingestion_schema::{
    has_completed_legacy_sled_migration, initialize_ingestion_schema, LEGACY_SLED_MIGRATION_ID,
};
use rusqlite::{params, Connection, OptionalExtension, Transaction, TransactionBehavior};
use sha2::{Digest, Sha256};
use sled::{Db, Tree};
use std::error::Error;
use std::fs::{self, File, OpenOptions};
use std::io;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use tempfile::Builder;

type MigrationResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

const SQLITE_NAME: &str = "ingestion.sqlite3";
const STAGING_NAME: &str = ".ingestion.sqlite3.sled-migration-staging";
const SLED_DEFAULT_TREE_NAME: &[u8] = b"__sled__default";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MigrationReport {
    pub tree_count: u64,
    pub record_count: u64,
    pub source_sha256: String,
    pub already_migrated: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ScanSummary {
    tree_count: u64,
    record_count: u64,
    source_sha256: [u8; 32],
}

struct TreeSnapshot {
    kind: i64,
    name: Vec<u8>,
    tree: Tree,
}

/// Migrate the Sled store under `root` into `root/ingestion.sqlite3`.
///
/// The storage lock used by the desktop application is acquired first. Only
/// Sled's `conf` and `db` entries are copied into a temporary directory; Sled
/// opens and recovers that copy, while the original store remains untouched.
pub fn migrate_legacy_sled_store(root: impl AsRef<Path>) -> MigrationResult<MigrationReport> {
    migrate_with_interrupt_after(root.as_ref(), None)
}

fn migrate_with_interrupt_after(
    root: &Path,
    interrupt_after_records: Option<u64>,
) -> MigrationResult<MigrationReport> {
    if !root.is_dir() || !(root.join("conf").exists() || root.join("db").exists()) {
        return Err(invalid_input(format!(
            "{} does not contain a legacy Sled store (expected conf or db)",
            root.display()
        )));
    }

    let _lock = acquire_storage_lock(root)?;
    let parent = root
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    let copy_parent = Builder::new()
        .prefix(".rocinante-sled-reader-")
        .tempdir_in(parent)?;
    let copy_root = copy_parent.path().join("store");
    fs::create_dir(&copy_root)?;
    copy_sled_entry(root, &copy_root, "conf")?;
    copy_sled_entry(root, &copy_root, "db")?;

    let db = sled::open(&copy_root)?;
    db.flush()?;
    let expected = walk_store(&db, |_, _| Ok(()), |_, _, _, _| Ok(()))?;

    let target = root.join(SQLITE_NAME);
    let created_staging = !target.exists();
    let working_path = if created_staging {
        let staging = root.join(STAGING_NAME);
        remove_sqlite_file_family(&staging)?;
        OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&staging)?;
        staging
    } else {
        target.clone()
    };

    let result = apply_snapshot(
        &db,
        &working_path,
        expected,
        interrupt_after_records,
        created_staging,
    );

    match result {
        Ok(already_migrated) => {
            if created_staging {
                fs::rename(&working_path, &target)?;
            }
            Ok(MigrationReport {
                tree_count: expected.tree_count,
                record_count: expected.record_count,
                source_sha256: hex_digest(&expected.source_sha256),
                already_migrated,
            })
        }
        Err(error) => {
            if created_staging {
                let _ = remove_sqlite_file_family(&working_path);
            }
            Err(error)
        }
    }
}

fn apply_snapshot(
    db: &Db,
    sqlite_path: &Path,
    expected: ScanSummary,
    interrupt_after_records: Option<u64>,
    staging: bool,
) -> MigrationResult<bool> {
    let tree_count_sql = i64::try_from(expected.tree_count)
        .map_err(|_| invalid_data("Sled tree count exceeds SQLite's integer range"))?;
    let record_count_sql = i64::try_from(expected.record_count)
        .map_err(|_| invalid_data("Sled record count exceeds SQLite's integer range"))?;
    let mut connection = Connection::open(sqlite_path)?;
    connection.busy_timeout(std::time::Duration::from_secs(10))?;
    if staging {
        connection.pragma_update(None, "journal_mode", "DELETE")?;
    }

    let transaction = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;
    initialize_ingestion_schema(&transaction)?;

    if let Some((digest, tree_count, record_count)) = migration_marker(&transaction)? {
        if digest.as_slice() != expected.source_sha256.as_slice()
            || tree_count != tree_count_sql
            || record_count != record_count_sql
        {
            return Err(invalid_data(
                "the Sled store differs from the source recorded by a completed migration; refusing to merge it",
            ));
        }
        if !has_completed_legacy_sled_migration(&transaction)? {
            return Err(invalid_data(
                "the existing Sled migration marker is malformed",
            ));
        }
        check_integrity(&transaction)?;
        transaction.commit()?;
        return Ok(true);
    }

    let mut inserted_count = 0_u64;
    let actual = walk_store(
        db,
        |kind, name| {
            transaction.execute(
                "INSERT OR IGNORE INTO ingestion_trees (tree_kind, tree_name) VALUES (?1, ?2)",
                params![kind, name],
            )?;
            Ok(())
        },
        |kind, name, key, value| {
            let inserted = transaction.execute(
                "INSERT OR IGNORE INTO ingestion_kv
                 (tree_kind, tree_name, key, value) VALUES (?1, ?2, ?3, ?4)",
                params![kind, name, key, value],
            )?;
            if inserted == 0 {
                let existing: Vec<u8> = transaction.query_row(
                    "SELECT value FROM ingestion_kv
                     WHERE tree_kind = ?1 AND tree_name = ?2 AND key = ?3",
                    params![kind, name, key],
                    |row| row.get(0),
                )?;
                if existing.as_slice() != value {
                    return Err(invalid_data(format!(
                        "conflicting SQLite value at tree kind {kind}, tree {name:?}, key {key:?}"
                    )));
                }
            }
            inserted_count = inserted_count
                .checked_add(1)
                .ok_or_else(|| invalid_data("Sled record count overflow"))?;
            if interrupt_after_records == Some(inserted_count) {
                return Err(io::Error::new(
                    io::ErrorKind::Interrupted,
                    "injected interruption before migration commit",
                )
                .into());
            }
            Ok(())
        },
    )?;

    if actual != expected || inserted_count != expected.record_count {
        return Err(invalid_data(
            "the copied Sled store changed while it was being migrated",
        ));
    }

    transaction.execute(
        "INSERT INTO rocinante_migration_metadata
         (migration_id, source_sha256, tree_count, record_count, completed)
         VALUES (?1, ?2, ?3, ?4, 1)",
        params![
            LEGACY_SLED_MIGRATION_ID,
            expected.source_sha256.as_slice(),
            tree_count_sql,
            record_count_sql
        ],
    )?;
    check_integrity(&transaction)?;
    transaction.commit()?;
    check_integrity(&connection)?;
    Ok(false)
}

fn migration_marker(transaction: &Transaction<'_>) -> MigrationResult<Option<(Vec<u8>, i64, i64)>> {
    transaction
        .query_row(
            "SELECT source_sha256, tree_count, record_count
             FROM rocinante_migration_metadata WHERE migration_id = ?1",
            params![LEGACY_SLED_MIGRATION_ID],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
        )
        .optional()
        .map_err(Into::into)
}

fn walk_store(
    db: &Db,
    mut visit_tree: impl FnMut(i64, &[u8]) -> MigrationResult<()>,
    mut visit_record: impl FnMut(i64, &[u8], &[u8], &[u8]) -> MigrationResult<()>,
) -> MigrationResult<ScanSummary> {
    let mut names = db
        .tree_names()
        .into_iter()
        .map(|name| name.to_vec())
        .filter(|name| name.as_slice() != SLED_DEFAULT_TREE_NAME)
        .collect::<Vec<_>>();
    names.sort();
    names.dedup();

    let mut trees = Vec::with_capacity(names.len() + 1);
    trees.push(TreeSnapshot {
        kind: 0,
        name: Vec::new(),
        tree: db.deref().clone(),
    });
    for name in names {
        trees.push(TreeSnapshot {
            kind: 1,
            tree: db.open_tree(&name)?,
            name,
        });
    }

    let mut hasher = Sha256::new();
    let mut record_count = 0_u64;
    for snapshot in &trees {
        hasher.update(b"T");
        hasher.update([snapshot.kind as u8]);
        hash_field(&mut hasher, &snapshot.name)?;
        visit_tree(snapshot.kind, &snapshot.name)?;
        for entry in snapshot.tree.iter() {
            let (key, value) = entry?;
            hasher.update(b"R");
            hash_field(&mut hasher, &key)?;
            hash_field(&mut hasher, &value)?;
            visit_record(snapshot.kind, &snapshot.name, &key, &value)?;
            record_count = record_count
                .checked_add(1)
                .ok_or_else(|| invalid_data("Sled record count overflow"))?;
        }
    }

    let tree_count =
        u64::try_from(trees.len()).map_err(|_| invalid_data("Sled tree count overflow"))?;
    Ok(ScanSummary {
        tree_count,
        record_count,
        source_sha256: hasher.finalize().into(),
    })
}

fn hash_field(hasher: &mut Sha256, value: &[u8]) -> MigrationResult<()> {
    let length =
        u64::try_from(value.len()).map_err(|_| invalid_data("Sled key/value length overflow"))?;
    hasher.update(length.to_be_bytes());
    hasher.update(value);
    Ok(())
}

fn acquire_storage_lock(root: &Path) -> MigrationResult<File> {
    let file_name = root
        .file_name()
        .and_then(|name| name.to_str())
        .map(|name| format!("{name}.lock"))
        .unwrap_or_else(|| "kv.lock".to_string());
    let lock_path = root.with_file_name(file_name);
    let lock = OpenOptions::new()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .open(&lock_path)?;
    lock.try_lock_exclusive().map_err(|error| {
        io::Error::new(
            io::ErrorKind::WouldBlock,
            format!("storage path is in use; stop Rocinante before migration: {error}"),
        )
    })?;
    Ok(lock)
}

fn copy_sled_entry(source_root: &Path, target_root: &Path, name: &str) -> MigrationResult<()> {
    let source = source_root.join(name);
    if !source.exists() {
        return Ok(());
    }
    copy_path(&source, &target_root.join(name))
}

fn copy_path(source: &Path, target: &Path) -> MigrationResult<()> {
    let metadata = fs::symlink_metadata(source)?;
    if metadata.file_type().is_symlink() {
        return Err(invalid_data(format!(
            "refusing to follow symlink in Sled store: {}",
            source.display()
        )));
    }
    if metadata.is_dir() {
        fs::create_dir(target)?;
        for entry in fs::read_dir(source)? {
            let entry = entry?;
            copy_path(&entry.path(), &target.join(entry.file_name()))?;
        }
    } else if metadata.is_file() {
        fs::copy(source, target)?;
        make_copy_file_writable(target)?;
    } else {
        return Err(invalid_data(format!(
            "unsupported file type in Sled store: {}",
            source.display()
        )));
    }
    Ok(())
}

fn make_copy_file_writable(path: &Path) -> io::Result<()> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut permissions = fs::metadata(path)?.permissions();
        permissions.set_mode(permissions.mode() | 0o200);
        fs::set_permissions(path, permissions)?;
    }
    #[cfg(not(unix))]
    {
        let mut permissions = fs::metadata(path)?.permissions();
        if permissions.readonly() {
            #[allow(clippy::permissions_set_readonly_false)]
            permissions.set_readonly(false);
            fs::set_permissions(path, permissions)?;
        }
    }
    Ok(())
}

fn check_integrity(connection: &Connection) -> MigrationResult<()> {
    let result: String = connection.query_row("PRAGMA integrity_check", [], |row| row.get(0))?;
    if result != "ok" {
        return Err(invalid_data(format!(
            "SQLite integrity check failed: {result}"
        )));
    }
    Ok(())
}

fn remove_sqlite_file_family(path: &Path) -> io::Result<()> {
    for suffix in ["", "-wal", "-shm", "-journal"] {
        let file = if suffix.is_empty() {
            path.to_path_buf()
        } else {
            let mut file = path.as_os_str().to_os_string();
            file.push(suffix);
            PathBuf::from(file)
        };
        match fs::remove_file(file) {
            Ok(()) => {}
            Err(error) if error.kind() == io::ErrorKind::NotFound => {}
            Err(error) => return Err(error),
        }
    }
    Ok(())
}

fn invalid_input(message: impl Into<String>) -> Box<dyn Error + Send + Sync> {
    io::Error::new(io::ErrorKind::InvalidInput, message.into()).into()
}

fn invalid_data(message: impl Into<String>) -> Box<dyn Error + Send + Sync> {
    io::Error::new(io::ErrorKind::InvalidData, message.into()).into()
}

fn hex_digest(digest: &[u8; 32]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(64);
    for byte in digest {
        output.push(HEX[(byte >> 4) as usize] as char);
        output.push(HEX[(byte & 0x0f) as usize] as char);
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::params;
    use std::fs;
    use tempfile::tempdir;

    fn create_legacy_store(root: &Path) -> Db {
        let db = sled::open(root).expect("create legacy Sled store");
        db.insert(&[0x00, 0xff][..], &[0xff, 0x00][..])
            .expect("insert binary default-tree record");
        db.insert(b"same-key", b"default")
            .expect("insert default-tree record");
        let first = db.open_tree(b"named").expect("open named tree");
        first
            .insert(b"same-key", b"named")
            .expect("insert named-tree record");
        first
            .insert(&[0xff, 0x00][..], &[0x00, 0xff][..])
            .expect("insert binary named-tree record");
        db.open_tree(b"empty").expect("create empty named tree");
        db.open_tree([] as [u8; 0])
            .expect("create empty-name named tree");
        db.flush().expect("flush legacy Sled store");
        db
    }

    fn digest_file(path: &Path) -> Vec<u8> {
        fs::read(path).expect("read file for digest")
    }

    fn snapshot_entry(path: &Path, root: &Path, files: &mut Vec<(PathBuf, Vec<u8>)>) {
        let metadata = fs::symlink_metadata(path).expect("inspect legacy source");
        if metadata.is_dir() {
            files.push((path.strip_prefix(root).unwrap().to_path_buf(), Vec::new()));
            for entry in fs::read_dir(path).expect("list legacy directory") {
                snapshot_entry(&entry.expect("read legacy entry").path(), root, files);
            }
        } else {
            files.push((
                path.strip_prefix(root).unwrap().to_path_buf(),
                digest_file(path),
            ));
        }
    }

    fn snapshot_legacy_store(root: &Path) -> Vec<(PathBuf, Vec<u8>)> {
        let mut files = Vec::new();
        for name in ["conf", "db"] {
            let path = root.join(name);
            if path.exists() {
                snapshot_entry(&path, root, &mut files);
            }
        }
        files.sort_by(|left, right| left.0.cmp(&right.0));
        files
    }

    fn rows(connection: &Connection, kind: i64, name: &[u8]) -> Vec<(Vec<u8>, Vec<u8>)> {
        let mut statement = connection
            .prepare(
                "SELECT key, value FROM ingestion_kv
                 WHERE tree_kind = ?1 AND tree_name = ?2 ORDER BY key",
            )
            .expect("prepare rows query");
        statement
            .query_map(params![kind, name], |row| Ok((row.get(0)?, row.get(1)?)))
            .expect("query rows")
            .collect::<rusqlite::Result<Vec<_>>>()
            .expect("collect rows")
    }

    #[test]
    fn migrates_binary_records_and_preserves_default_named_and_empty_trees() {
        let root = tempdir().expect("create migration root");
        let db = create_legacy_store(&root.path().join("legacy"));
        db.flush().expect("flush fixture store");
        drop(db);
        let legacy = root.path().join("legacy");
        let original_files = snapshot_legacy_store(&legacy);

        let report = migrate_legacy_sled_store(&legacy).expect("migrate store");
        assert_eq!(report.tree_count, 4, "default plus three named trees");
        assert_eq!(report.record_count, 4);
        assert!(!report.already_migrated);
        assert_eq!(
            snapshot_legacy_store(&legacy),
            original_files,
            "source Sled files remain unchanged"
        );

        let connection = Connection::open(legacy.join(SQLITE_NAME)).expect("open migrated DB");
        assert_eq!(rows(&connection, 0, b"").len(), 2);
        assert_eq!(rows(&connection, 1, b"named").len(), 2);
        assert!(rows(&connection, 1, b"empty").is_empty());
        assert!(rows(&connection, 1, b"").is_empty());
        let tree_count: i64 = connection
            .query_row("SELECT count(*) FROM ingestion_trees", [], |row| row.get(0))
            .expect("count preserved trees");
        assert_eq!(tree_count, 4);
        assert!(has_completed_legacy_sled_migration(&connection).expect("read marker"));
        check_integrity(&connection).expect("check migrated DB");
        drop(connection);

        rocinante_storage::storage::DualLayerStore::open(
            legacy.to_str().expect("legacy storage path"),
            root.path()
                .join("analytics.duckdb")
                .to_str()
                .expect("analytics path"),
        )
        .expect("application accepts the completed migration marker");
    }

    #[test]
    fn migration_is_idempotent_and_rejects_changed_source() {
        let root = tempdir().expect("create migration root");
        let legacy = root.path().join("legacy");
        let db = create_legacy_store(&legacy);
        drop(db);

        let first = migrate_legacy_sled_store(&legacy).expect("first migration");
        let second = migrate_legacy_sled_store(&legacy).expect("retry migration");
        assert!(!first.already_migrated);
        assert!(second.already_migrated);
        assert_eq!(first.source_sha256, second.source_sha256);

        let db = sled::open(&legacy).expect("open original source");
        db.insert(b"new-source-record", b"later")
            .expect("change original source");
        db.flush().expect("flush source change");
        drop(db);
        let error = migrate_legacy_sled_store(&legacy).expect_err("reject changed source");
        assert!(error.to_string().contains("differs from the source"));
    }

    #[test]
    fn interruption_rolls_back_and_retry_completes() {
        let root = tempdir().expect("create migration root");
        let legacy = root.path().join("legacy");
        let db = create_legacy_store(&legacy);
        drop(db);

        let error = migrate_with_interrupt_after(&legacy, Some(1)).expect_err("interrupt import");
        assert_eq!(
            error.to_string(),
            "injected interruption before migration commit"
        );
        assert!(!legacy.join(SQLITE_NAME).exists());
        assert!(legacy.join("conf").exists());

        let report = migrate_legacy_sled_store(&legacy).expect("retry migration");
        assert_eq!(report.record_count, 4);
        assert!(legacy.join(SQLITE_NAME).exists());
    }

    #[test]
    fn conflicting_target_data_is_not_overwritten_or_marked_complete() {
        let root = tempdir().expect("create migration root");
        let legacy = root.path().join("legacy");
        let db = create_legacy_store(&legacy);
        drop(db);
        let target = legacy.join(SQLITE_NAME);
        let connection = Connection::open(&target).expect("create conflicting target");
        connection
            .execute_batch(
                "CREATE TABLE ingestion_kv (
                    key BLOB PRIMARY KEY NOT NULL,
                    value BLOB NOT NULL
                 ) WITHOUT ROWID;",
            )
            .expect("create v1 target schema");
        connection
            .execute(
                "INSERT INTO ingestion_kv (key, value) VALUES (?1, ?2)",
                params![b"same-key".as_slice(), b"different".as_slice()],
            )
            .expect("insert conflicting target value");
        drop(connection);
        let before = digest_file(&target);

        let error = migrate_legacy_sled_store(&legacy).expect_err("reject conflict");
        assert!(error.to_string().contains("conflicting SQLite value"));
        assert_eq!(
            digest_file(&target),
            before,
            "transaction rolls back target"
        );
        let connection = Connection::open(target).expect("reopen conflicted DB");
        assert!(!has_completed_legacy_sled_migration(&connection).expect("read migration marker"));
    }

    #[test]
    fn legacy_store_lock_prevents_concurrent_migration() {
        let root = tempdir().expect("create migration root");
        let legacy = root.path().join("legacy");
        let db = create_legacy_store(&legacy);
        drop(db);
        let lock_path = root.path().join("legacy.lock");
        let lock = OpenOptions::new()
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .open(lock_path)
            .expect("open application lock");
        lock.try_lock_exclusive().expect("lock store");

        let error = migrate_legacy_sled_store(&legacy).expect_err("reject active owner");
        assert!(error.to_string().contains("stop Rocinante"));
    }

    #[test]
    fn corrupt_store_is_refused_without_changing_source() {
        let root = tempdir().expect("create migration root");
        let legacy = root.path().join("legacy");
        fs::create_dir_all(legacy.join("db")).expect("create broken Sled directory");
        fs::write(legacy.join("conf"), b"not a valid Sled config").expect("write bad config");
        fs::write(
            legacy.join("db").join("corrupt"),
            b"not a valid Sled page cache",
        )
        .expect("write bad page cache");
        let original_conf = digest_file(&legacy.join("conf"));

        assert!(migrate_legacy_sled_store(&legacy).is_err());
        assert_eq!(digest_file(&legacy.join("conf")), original_conf);
        assert!(!legacy.join(SQLITE_NAME).exists());
    }

    #[test]
    fn empty_store_and_duckdb_history_are_preserved() {
        let root = tempdir().expect("create migration root");
        let legacy = root.path().join("legacy");
        let db = sled::open(&legacy).expect("create empty store");
        db.open_tree(b"empty-tree").expect("create empty tree");
        db.flush().expect("flush empty store");
        drop(db);
        let duckdb_path = legacy.join("telemetry.duckdb");
        fs::write(&duckdb_path, b"existing DuckDB history sentinel")
            .expect("create DuckDB history sentinel");
        let history_before = digest_file(&duckdb_path);

        let report = migrate_legacy_sled_store(&legacy).expect("migrate empty store");
        assert_eq!(report.tree_count, 2);
        assert_eq!(report.record_count, 0);
        assert_eq!(digest_file(&duckdb_path), history_before);
        let connection = Connection::open(legacy.join(SQLITE_NAME)).expect("open migrated DB");
        let named_tree: bool = connection
            .query_row(
                "SELECT EXISTS (
                SELECT 1 FROM ingestion_trees WHERE tree_kind = 1 AND tree_name = ?1
            )",
                params![b"empty-tree".as_slice()],
                |row| row.get(0),
            )
            .expect("find empty tree");
        assert!(named_tree);
    }
}
