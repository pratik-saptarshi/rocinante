#![cfg(feature = "analytics")]

#[test]
fn linked_duckdb_engine_matches_the_verified_release_pin() {
    let connection =
        duckdb::Connection::open_in_memory().expect("open the linked prebuilt DuckDB engine");
    let version: String = connection
        .query_row("SELECT version()", [], |row| row.get(0))
        .expect("query the linked DuckDB engine version");

    assert_eq!(version.trim_start_matches('v'), "1.5.6");
}
