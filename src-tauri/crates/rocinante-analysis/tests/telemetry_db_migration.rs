use rusqlite::Connection;

#[test]
fn migrates_a_live_legacy_database_through_sqlite_backup() {
    let temporary = tempfile::tempdir().expect("temporary migration root");
    let prior_directory = std::env::current_dir().expect("current directory");
    let prior_database_setting = std::env::var_os("ROCINANTE_TELEMETRY_DB");
    std::env::set_current_dir(temporary.path()).expect("switch to isolated current directory");

    let legacy_path = temporary.path().join("telemetry.db");
    let source = Connection::open(&legacy_path).expect("open legacy database");
    source
        .execute_batch(
            "PRAGMA journal_mode=WAL;
             CREATE TABLE migration_probe (value TEXT NOT NULL);
             INSERT INTO migration_probe(value) VALUES ('committed');",
        )
        .expect("write legacy database");

    let destination_path = temporary.path().join("app-data/telemetry.db");
    std::env::set_var("ROCINANTE_TELEMETRY_DB", &destination_path);
    let migrated_path =
        rocinante_analysis::resolve_telemetry_db_path().expect("migrate live legacy database");
    assert_eq!(migrated_path, destination_path);

    let migrated = Connection::open(&migrated_path).expect("open migrated database");
    let value: String = migrated
        .query_row("SELECT value FROM migration_probe", [], |row| row.get(0))
        .expect("read migrated row");
    assert_eq!(value, "committed");

    drop(migrated);
    drop(source);
    if let Some(value) = prior_database_setting {
        std::env::set_var("ROCINANTE_TELEMETRY_DB", value);
    } else {
        std::env::remove_var("ROCINANTE_TELEMETRY_DB");
    }
    std::env::set_current_dir(prior_directory).expect("restore current directory");
}
