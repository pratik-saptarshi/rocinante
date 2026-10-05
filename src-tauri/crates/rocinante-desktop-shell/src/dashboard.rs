use rocinante_analysis::telemetry::TelemetryImportSummary;
use rocinante_analysis::types::RepositoryMetric;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DashboardPluginCount {
    pub name: String,
    pub metric_count: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DashboardMetricRow {
    pub repo_name: String,
    pub release: String,
    pub plugin: String,
    pub key: String,
    pub value: f64,
    pub details: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DashboardViewModel {
    pub repository_count: usize,
    pub metric_count: usize,
    pub plugins: Vec<DashboardPluginCount>,
    pub rows: Vec<DashboardMetricRow>,
}

pub fn format_analysis_summary(summary: &TelemetryImportSummary) -> String {
    format!(
        "{} repositories analyzed; {} new metric rows added; {} duplicate metric keys skipped.",
        summary.records_processed, summary.rows_inserted, summary.duplicate_source_keys
    )
}

impl DashboardViewModel {
    pub fn from_metrics(metrics: &[RepositoryMetric]) -> Self {
        let mut repositories = BTreeSet::new();
        let mut plugins = BTreeMap::<&str, usize>::new();
        let mut rows = metrics
            .iter()
            .map(|metric| {
                repositories.insert(metric.repo_name.as_str());
                *plugins.entry(&metric.plugin).or_default() += 1;
                DashboardMetricRow {
                    repo_name: metric.repo_name.clone(),
                    release: metric.release.clone(),
                    plugin: metric.plugin.clone(),
                    key: metric.key.clone(),
                    value: metric.value,
                    details: metric.details.clone(),
                }
            })
            .collect::<Vec<_>>();
        rows.sort_by(|left, right| {
            (&left.plugin, &left.repo_name, &left.release, &left.key).cmp(&(
                &right.plugin,
                &right.repo_name,
                &right.release,
                &right.key,
            ))
        });

        Self {
            repository_count: repositories.len(),
            metric_count: rows.len(),
            plugins: plugins
                .into_iter()
                .map(|(name, metric_count)| DashboardPluginCount {
                    name: name.to_owned(),
                    metric_count,
                })
                .collect(),
            rows,
        }
    }
}
