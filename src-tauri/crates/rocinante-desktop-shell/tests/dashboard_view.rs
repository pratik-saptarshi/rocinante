#![cfg(feature = "native-ui")]

use rocinante_analysis::telemetry::TelemetryImportSummary;
use rocinante_analysis::types::RepositoryMetric;
use rocinante_desktop_shell::dashboard::{format_analysis_summary, DashboardViewModel};

fn metric(repo: &str, plugin: &str, key: &str, value: f64, details: &str) -> RepositoryMetric {
    RepositoryMetric {
        repo_name: repo.to_owned(),
        release: "v1.0".to_owned(),
        plugin: plugin.to_owned(),
        key: key.to_owned(),
        value,
        details: details.to_owned(),
    }
}

#[test]
fn dashboard_view_model_represents_empty_metric_data() {
    let view = DashboardViewModel::from_metrics(&[]);

    assert_eq!(view.repository_count, 0);
    assert_eq!(view.metric_count, 0);
    assert!(view.plugins.is_empty());
    assert!(view.rows.is_empty());
}

#[test]
fn dashboard_groups_plugins_and_sorts_rows_deterministically() {
    let input = [
        metric("repo-b", "zeta", "last", 3.0, "last details"),
        metric("repo-a", "alpha", "second", 2.0, "second details"),
        metric("repo-a", "alpha", "first", 1.0, "first details"),
    ];
    let view = DashboardViewModel::from_metrics(&input);

    assert_eq!(view.repository_count, 2);
    assert_eq!(view.metric_count, 3);
    assert_eq!(
        view.plugins
            .iter()
            .map(|plugin| (plugin.name.as_str(), plugin.metric_count))
            .collect::<Vec<_>>(),
        vec![("alpha", 2), ("zeta", 1)]
    );
    assert_eq!(
        view.rows
            .iter()
            .map(|row| (
                row.plugin.as_str(),
                row.repo_name.as_str(),
                row.key.as_str()
            ))
            .collect::<Vec<_>>(),
        vec![
            ("alpha", "repo-a", "first"),
            ("alpha", "repo-a", "second"),
            ("zeta", "repo-b", "last"),
        ]
    );
    assert_eq!(view.rows[0].details, "first details");
    assert_eq!(view.rows[0].value, 1.0);
}

#[test]
fn dashboard_view_model_does_not_infer_risk_from_generic_metric_values() {
    let input = [metric(
        "repo-a",
        "complexity",
        "score",
        999.0,
        "unclassified",
    )];
    let view = DashboardViewModel::from_metrics(&input);

    assert_eq!(view.rows[0].value, 999.0);
    assert_eq!(view.rows[0].details, "unclassified");
    assert_eq!(view.metric_count, 1);
}

#[test]
fn analysis_summary_distinguishes_new_rows_from_total_stored_metrics() {
    let summary = TelemetryImportSummary {
        source: "scan".to_owned(),
        records_processed: 2,
        rows_inserted: 0,
        duplicate_source_keys: 18,
    };

    assert_eq!(
        format_analysis_summary(&summary),
        "2 repositories analyzed; 0 new metric rows added; 18 duplicate metric keys skipped."
    );
}
