//! SQLite schema shared by the application and the one-time legacy-store migrator.

use rusqlite::{params, Connection};
use std::collections::HashMap;

pub const LEGACY_SLED_MIGRATION_ID: &str = "legacy-sled-v1";

#[derive(Debug)]
struct ColumnInfo {
    column_type: String,
    not_null: bool,
    primary_key_order: i64,
}

/// Ensure the current schema without committing the caller's transaction.
pub fn initialize_ingestion_schema(connection: &Connection) -> rusqlite::Result<()> {
    connection.execute_batch(
        "CREATE TABLE IF NOT EXISTS ingestion_trees (
            tree_kind INTEGER NOT NULL CHECK (tree_kind IN (0, 1)),
            tree_name BLOB NOT NULL,
            PRIMARY KEY (tree_kind, tree_name)
        ) WITHOUT ROWID;
        CREATE TABLE IF NOT EXISTS rocinante_migration_metadata (
            migration_id TEXT PRIMARY KEY NOT NULL,
            source_sha256 BLOB NOT NULL,
            tree_count INTEGER NOT NULL,
            record_count INTEGER NOT NULL,
            completed INTEGER NOT NULL CHECK (completed = 1)
        ) WITHOUT ROWID;",
    )?;
    validate_tree_table(connection)?;

    let table_exists: bool = connection.query_row(
        "SELECT EXISTS (
            SELECT 1 FROM sqlite_master
            WHERE type = 'table' AND name = 'ingestion_kv'
        )",
        [],
        |row| row.get(0),
    )?;

    if !table_exists {
        create_ingestion_kv_table(connection, "ingestion_kv")?;
    } else {
        let columns = read_columns(connection, "ingestion_kv")?;
        let is_v2 = columns.len() == 4
            && ["tree_kind", "tree_name", "key", "value"]
                .iter()
                .all(|column| columns.contains_key(*column));

        if is_v2 {
            validate_v2_kv_table(&columns)?;
        } else {
            if columns.len() != 2 || !columns.contains_key("key") || !columns.contains_key("value")
            {
                return Err(rusqlite::Error::InvalidColumnName(
                    "unsupported ingestion_kv schema; expected key/value or tree_kind/tree_name/key/value"
                        .to_string(),
                ));
            }
            validate_v1_kv_table(&columns)?;

            create_ingestion_kv_table(connection, "ingestion_kv_v2")?;
            connection.execute(
                "INSERT INTO ingestion_kv_v2 (tree_kind, tree_name, key, value)
                 SELECT 0, X'', key, value FROM ingestion_kv",
                [],
            )?;
            connection.execute_batch(
                "DROP TABLE ingestion_kv;
                 ALTER TABLE ingestion_kv_v2 RENAME TO ingestion_kv;",
            )?;
        }
    }
    validate_v2_kv_table(&read_columns(connection, "ingestion_kv")?)?;

    connection.execute(
        "INSERT OR IGNORE INTO ingestion_trees (tree_kind, tree_name) VALUES (0, X'')",
        [],
    )?;
    connection.execute(
        "INSERT OR IGNORE INTO ingestion_trees (tree_kind, tree_name)
         SELECT DISTINCT tree_kind, tree_name FROM ingestion_kv",
        [],
    )?;
    Ok(())
}

fn read_columns(
    connection: &Connection,
    table_name: &str,
) -> rusqlite::Result<HashMap<String, ColumnInfo>> {
    let mut statement = connection.prepare(&format!("PRAGMA table_info({table_name})"))?;
    let columns = statement
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(1)?,
                ColumnInfo {
                    column_type: row.get(2)?,
                    not_null: row.get::<_, i64>(3)? != 0,
                    primary_key_order: row.get(5)?,
                },
            ))
        })?
        .collect();
    columns
}

fn validate_tree_table(connection: &Connection) -> rusqlite::Result<()> {
    let columns = read_columns(connection, "ingestion_trees")?;
    if columns.len() != 2
        || !column_matches(&columns, "tree_kind", "INTEGER", true, 1)
        || !column_matches(&columns, "tree_name", "BLOB", true, 2)
    {
        return Err(rusqlite::Error::InvalidColumnName(
            "unsupported ingestion_trees schema".to_string(),
        ));
    }
    Ok(())
}

fn validate_v1_kv_table(columns: &HashMap<String, ColumnInfo>) -> rusqlite::Result<()> {
    if !column_matches(columns, "key", "BLOB", true, 1)
        || !column_matches(columns, "value", "BLOB", true, 0)
    {
        return Err(rusqlite::Error::InvalidColumnName(
            "unsupported key/value ingestion_kv schema".to_string(),
        ));
    }
    Ok(())
}

fn validate_v2_kv_table(columns: &HashMap<String, ColumnInfo>) -> rusqlite::Result<()> {
    if columns.len() != 4
        || !column_matches(columns, "tree_kind", "INTEGER", true, 1)
        || !column_matches(columns, "tree_name", "BLOB", true, 2)
        || !column_matches(columns, "key", "BLOB", true, 3)
        || !column_matches(columns, "value", "BLOB", true, 0)
    {
        return Err(rusqlite::Error::InvalidColumnName(
            "unsupported tree/key/value ingestion_kv schema".to_string(),
        ));
    }
    Ok(())
}

fn column_matches(
    columns: &HashMap<String, ColumnInfo>,
    name: &str,
    column_type: &str,
    not_null: bool,
    primary_key_order: i64,
) -> bool {
    columns.get(name).is_some_and(|column| {
        column.column_type.eq_ignore_ascii_case(column_type)
            && column.not_null == not_null
            && column.primary_key_order == primary_key_order
    })
}

fn create_ingestion_kv_table(connection: &Connection, table_name: &str) -> rusqlite::Result<()> {
    let statement = match table_name {
        "ingestion_kv" => {
            "CREATE TABLE ingestion_kv (
                tree_kind INTEGER NOT NULL CHECK (tree_kind IN (0, 1)),
                tree_name BLOB NOT NULL,
                key BLOB NOT NULL,
                value BLOB NOT NULL,
                PRIMARY KEY (tree_kind, tree_name, key)
            ) WITHOUT ROWID;"
        }
        "ingestion_kv_v2" => {
            "CREATE TABLE ingestion_kv_v2 (
                tree_kind INTEGER NOT NULL CHECK (tree_kind IN (0, 1)),
                tree_name BLOB NOT NULL,
                key BLOB NOT NULL,
                value BLOB NOT NULL,
                PRIMARY KEY (tree_kind, tree_name, key)
            ) WITHOUT ROWID;"
        }
        _ => {
            return Err(rusqlite::Error::InvalidParameterName(
                table_name.to_string(),
            ))
        }
    };
    connection.execute_batch(statement)
}

/// Return whether a completed Sled migration marker is present and well-formed.
pub fn has_completed_legacy_sled_migration(connection: &Connection) -> rusqlite::Result<bool> {
    let table_exists: bool = connection.query_row(
        "SELECT EXISTS (
            SELECT 1 FROM sqlite_master
            WHERE type = 'table' AND name = 'rocinante_migration_metadata'
        )",
        [],
        |row| row.get(0),
    )?;
    if !table_exists {
        return Ok(false);
    }

    let marker = connection.query_row(
        "SELECT source_sha256, tree_count, record_count, completed
         FROM rocinante_migration_metadata WHERE migration_id = ?1",
        params![LEGACY_SLED_MIGRATION_ID],
        |row| {
            Ok((
                row.get::<_, Vec<u8>>(0)?,
                row.get::<_, i64>(1)?,
                row.get::<_, i64>(2)?,
                row.get::<_, i64>(3)?,
            ))
        },
    );

    match marker {
        Ok((digest, trees, records, completed)) => {
            Ok(digest.len() == 32 && trees > 0 && records >= 0 && completed == 1)
        }
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(false),
        Err(error) => Err(error),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        has_completed_legacy_sled_migration, initialize_ingestion_schema, LEGACY_SLED_MIGRATION_ID,
    };
    use rusqlite::{params, Connection, TransactionBehavior};

    #[test]
    fn upgrades_v1_rows_and_allows_same_key_in_distinct_trees() {
        let mut connection = Connection::open_in_memory().expect("open SQLite memory database");
        connection
            .execute_batch(
                "CREATE TABLE ingestion_kv (
                    key BLOB PRIMARY KEY NOT NULL,
                    value BLOB NOT NULL
                ) WITHOUT ROWID;
                INSERT INTO ingestion_kv (key, value) VALUES (X'00FF', X'FF00');",
            )
            .expect("create v1 schema and binary row");

        let transaction = connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .expect("start schema transaction");
        initialize_ingestion_schema(&transaction).expect("upgrade schema");
        transaction.commit().expect("commit schema upgrade");

        let default_value: Vec<u8> = connection
            .query_row(
                "SELECT value FROM ingestion_kv
                 WHERE tree_kind = 0 AND tree_name = X'' AND key = X'00FF'",
                [],
                |row| row.get(0),
            )
            .expect("read preserved default-tree value");
        assert_eq!(default_value, [0xff, 0x00]);
        connection
            .execute(
                "INSERT INTO ingestion_kv (tree_kind, tree_name, key, value)
                 VALUES (1, ?1, X'00FF', X'0102')",
                params![b"named".as_slice()],
            )
            .expect("same binary key is valid in another tree");
        let count: i64 = connection
            .query_row("SELECT count(*) FROM ingestion_kv", [], |row| row.get(0))
            .expect("count default and named rows");
        assert_eq!(count, 2);
    }

    #[test]
    fn only_well_formed_completed_migration_markers_open_legacy_stores() {
        let mut connection = Connection::open_in_memory().expect("open SQLite memory database");
        assert!(!has_completed_legacy_sled_migration(&connection).expect("missing marker"));
        let transaction = connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .expect("start schema transaction");
        initialize_ingestion_schema(&transaction).expect("create schema");
        transaction
            .execute(
                "INSERT INTO rocinante_migration_metadata
                 (migration_id, source_sha256, tree_count, record_count, completed)
                 VALUES (?1, ?2, 1, 0, 1)",
                params![LEGACY_SLED_MIGRATION_ID, vec![0_u8; 32]],
            )
            .expect("insert complete marker");
        transaction.commit().expect("commit marker");
        assert!(has_completed_legacy_sled_migration(&connection).expect("valid marker"));
    }

    #[test]
    fn refuses_a_key_value_table_without_its_legacy_primary_key() {
        let mut connection = Connection::open_in_memory().expect("open SQLite memory database");
        connection
            .execute_batch(
                "CREATE TABLE ingestion_kv (
                    key BLOB NOT NULL,
                    value BLOB NOT NULL
                );",
            )
            .expect("create malformed v1 schema");
        let transaction = connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .expect("start schema transaction");

        let error = initialize_ingestion_schema(&transaction)
            .expect_err("reject schema without a unique key");
        assert!(error.to_string().contains("unsupported key/value"));
        drop(transaction);
        let tables: i64 = connection
            .query_row(
                "SELECT count(*) FROM sqlite_master WHERE type = 'table' AND name = 'ingestion_trees'",
                [],
                |row| row.get(0),
            )
            .expect("check rollback");
        assert_eq!(tables, 0, "schema creation rolls back with the rejection");
    }
}
