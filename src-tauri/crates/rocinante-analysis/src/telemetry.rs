use crate::errors::AnalyzerError;
use crate::plugins::sanitizer::{scrub_metric, scrub_record_strings, scrub_text};
use crate::types::{AdminQuery, AnalysisMetric, AnalysisRecord, RepositoryMetric};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::Path;

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
    pub fn open(path: impl AsRef<Path>) -> Result<Self, AnalyzerError> {
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

    /// Replace the saved metric snapshot for each repository and release.
    ///
    /// Unlike `insert_records`, this treats a scan as the current state: changed
    /// metric values are updated and metrics no longer produced by the scan are
    /// removed. The transaction keeps each repository/release replacement
    /// atomic for readers.
    pub fn replace_records(
        &mut self,
        records: &[AnalysisRecord],
        source: &str,
    ) -> Result<TelemetryImportSummary, AnalyzerError> {
        let mut desired =
            HashMap::<(String, String), HashMap<(String, String), (f64, String)>>::new();
        let mut rows_attempted = 0usize;

        for record in records {
            let (repo_name, release) = scrub_record_strings(&record.repo_name, &record.release);
            let metrics = desired.entry((repo_name, release)).or_default();
            for metric in &record.metrics {
                rows_attempted += 1;
                let mut metric = metric.clone();
                scrub_metric(&mut metric);
                metrics.insert((metric.plugin, metric.key), (metric.value, metric.details));
            }
        }

        let transaction = self.conn.transaction()?;
        let mut rows_inserted = 0usize;
        for ((repo_name, release), metrics) in &desired {
            for ((plugin, metric_key), (metric_value, details)) in metrics {
                rows_inserted += transaction.execute(
                    "INSERT INTO telemetry (repo_name, release, plugin, metric_key, metric_value, details)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6)
                     ON CONFLICT(repo_name, release, plugin, metric_key) DO UPDATE SET
                       metric_value = excluded.metric_value,
                       details = excluded.details
                     WHERE telemetry.metric_value IS NOT excluded.metric_value
                        OR telemetry.details IS NOT excluded.details",
                    params![repo_name, release, plugin, metric_key, metric_value, details],
                )?;
            }

            let current_keys = {
                let mut statement = transaction.prepare(
                    "SELECT plugin, metric_key FROM telemetry WHERE repo_name = ?1 AND release = ?2",
                )?;
                let rows = statement.query_map(params![repo_name, release], |row| {
                    Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
                })?;
                rows.collect::<Result<HashSet<_>, _>>()?
            };
            for (plugin, metric_key) in current_keys {
                if !metrics.contains_key(&(plugin.clone(), metric_key.clone())) {
                    transaction.execute(
                        "DELETE FROM telemetry
                         WHERE repo_name = ?1 AND release = ?2 AND plugin = ?3 AND metric_key = ?4",
                        params![repo_name, release, plugin, metric_key],
                    )?;
                }
            }
        }

        let summary = TelemetryImportSummary {
            source: source.to_string(),
            records_processed: records.len(),
            rows_inserted,
            duplicate_source_keys: rows_attempted.saturating_sub(rows_inserted),
        };
        transaction.execute(
            "INSERT INTO telemetry_import_summary (source, records_processed, rows_inserted, duplicate_source_keys)
             VALUES (?1, ?2, ?3, ?4)",
            params![
                summary.source,
                summary.records_processed as i64,
                summary.rows_inserted as i64,
                summary.duplicate_source_keys as i64
            ],
        )?;
        transaction.commit()?;
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

    pub fn query_repositories(
        &self,
        repo_names: &[String],
        release: &str,
    ) -> Result<Vec<RepositoryMetric>, AnalyzerError> {
        let release = scrub_text(release);
        let mut stmt = self.conn.prepare(
            "SELECT repo_name, release, plugin, metric_key, metric_value, details
             FROM telemetry
             WHERE repo_name = ?1 AND (?2 = '' OR release = ?2)
             ORDER BY repo_name, release, plugin, metric_key",
        )?;
        let mut out = Vec::new();
        for name in repo_names {
            let name = scrub_text(name);
            let rows = stmt.query_map(params![name, release], |row| {
                Ok(RepositoryMetric {
                    repo_name: row.get(0)?,
                    release: row.get(1)?,
                    plugin: row.get(2)?,
                    key: row.get(3)?,
                    value: row.get(4)?,
                    details: row.get(5)?,
                })
            })?;
            for row in rows {
                let mut row = row?;
                row.repo_name = scrub_text(&row.repo_name);
                row.release = scrub_text(&row.release);
                let mut metric = AnalysisMetric {
                    plugin: row.plugin.clone(),
                    key: row.key.clone(),
                    value: row.value,
                    details: row.details.clone(),
                };
                scrub_metric(&mut metric);
                row.plugin = metric.plugin;
                row.key = metric.key;
                row.details = metric.details;
                out.push(row);
            }
        }
        Ok(out)
    }
}
