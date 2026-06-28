use crate::errors::AnalyzerError;
use crate::plugins::sanitizer::{scrub_metric, scrub_record_strings, scrub_text};
use crate::types::{AdminQuery, AnalysisMetric, AnalysisRecord};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TelemetryImportSummary {
    pub source: String,
    pub records_processed: usize,
    pub rows_inserted: usize,
    pub duplicate_source_keys: usize,
}

pub struct TelemetryStore {
    conn: Connection,
}

impl TelemetryStore {
    pub fn open(path: &str) -> Result<Self, AnalyzerError> {
        let conn = Connection::open(path)?;
        conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS telemetry (
              id INTEGER PRIMARY KEY,
              repo_name TEXT NOT NULL,
              release TEXT NOT NULL,
              plugin TEXT NOT NULL,
              metric_key TEXT NOT NULL,
              metric_value REAL NOT NULL,
              details TEXT NOT NULL,
              UNIQUE(repo_name, release, plugin, metric_key)
            );
            CREATE TABLE IF NOT EXISTS telemetry_import_summary (
              id INTEGER PRIMARY KEY,
              source TEXT NOT NULL,
              records_processed INTEGER NOT NULL,
              rows_inserted INTEGER NOT NULL,
              duplicate_source_keys INTEGER NOT NULL
            );
            ",
        )?;
        Ok(Self { conn })
    }

    pub fn insert_record(&self, record: &AnalysisRecord) -> Result<(), AnalyzerError> {
        self.insert_records(std::slice::from_ref(record), "single-record")
            .map(|_| ())
    }

    pub fn insert_records(
        &self,
        records: &[AnalysisRecord],
        source: &str,
    ) -> Result<TelemetryImportSummary, AnalyzerError> {
        let mut rows_inserted = 0usize;
        let mut rows_attempted = 0usize;

        for record in records {
            let (repo_name, release) = scrub_record_strings(&record.repo_name, &record.release);
            for metric in &record.metrics {
                rows_attempted += 1;
                let mut m = metric.clone();
                scrub_metric(&mut m);
                rows_inserted += self
                    .conn
                    .execute(
                        "INSERT OR IGNORE INTO telemetry (repo_name, release, plugin, metric_key, metric_value, details)
                         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                        params![repo_name, release, m.plugin, m.key, m.value, m.details],
                    )?;
            }
        }

        let summary = TelemetryImportSummary {
            source: source.to_string(),
            records_processed: records.len(),
            rows_inserted,
            duplicate_source_keys: rows_attempted.saturating_sub(rows_inserted),
        };
        let source_string = summary.source.clone();
        self.conn.execute(
            "INSERT INTO telemetry_import_summary (source, records_processed, rows_inserted, duplicate_source_keys)
             VALUES (?1, ?2, ?3, ?4)",
            params![
                source_string,
                summary.records_processed as i64,
                summary.rows_inserted as i64,
                summary.duplicate_source_keys as i64
            ],
        )?;
        Ok(summary)
    }

    pub fn query_import_summaries(&self) -> Result<Vec<TelemetryImportSummary>, AnalyzerError> {
        let mut stmt = self.conn.prepare(
            "SELECT source, records_processed, rows_inserted, duplicate_source_keys
             FROM telemetry_import_summary
             ORDER BY id",
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(TelemetryImportSummary {
                source: row.get(0)?,
                records_processed: row.get::<_, i64>(1)? as usize,
                rows_inserted: row.get::<_, i64>(2)? as usize,
                duplicate_source_keys: row.get::<_, i64>(3)? as usize,
            })
        })?;

        let mut out = Vec::new();
        for row in rows {
            out.push(row?);
        }
        Ok(out)
    }

    pub fn query(&self, query: &AdminQuery) -> Result<Vec<AnalysisMetric>, AnalyzerError> {
        let name = scrub_text(&query.name.clone().unwrap_or_default());
        let release = scrub_text(&query.release.clone().unwrap_or_default());

        let mut stmt = self.conn.prepare(
            "SELECT plugin, metric_key, metric_value, details
             FROM telemetry
             WHERE repo_name LIKE '%' || ?1 || '%'
               AND release LIKE '%' || ?2 || '%'",
        )?;

        let rows = stmt.query_map(params![name, release], |row| {
            Ok(AnalysisMetric {
                plugin: row.get(0)?,
                key: row.get(1)?,
                value: row.get(2)?,
                details: row.get(3)?,
            })
        })?;

        let mut out = Vec::new();
        for row in rows {
            let mut metric = row?;
            scrub_metric(&mut metric);
            out.push(metric);
        }
        Ok(out)
    }
}
