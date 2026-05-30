use crate::errors::AnalyzerError;
use crate::plugins::sanitizer::{scrub_metric, scrub_record_strings, scrub_text};
use crate::types::{AdminQuery, AnalysisMetric, AnalysisRecord};
use rusqlite::{params, Connection};

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
              details TEXT NOT NULL
            );
            ",
        )?;
        Ok(Self { conn })
    }

    pub fn insert_record(&self, record: &AnalysisRecord) -> Result<(), AnalyzerError> {
        let (repo_name, release) = scrub_record_strings(&record.repo_name, &record.release);
        for metric in &record.metrics {
            let mut m = metric.clone();
            scrub_metric(&mut m);
            self.conn.execute(
                "INSERT INTO telemetry (repo_name, release, plugin, metric_key, metric_value, details)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![
                    repo_name,
                    release,
                    m.plugin,
                    m.key,
                    m.value,
                    m.details
                ],
            )?;
        }
        Ok(())
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
