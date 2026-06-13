use repo_analyzer_core::engine::Pipeline;
use repo_analyzer_core::plugins::parser::ParserPlugin;
use repo_analyzer_core::plugins::BeadPlugin;
use repo_analyzer_core::types::{AnalysisInput, RepoTarget};
use std::fs;
use tempfile::tempdir;

fn parser_metric_value(metrics: &[repo_analyzer_core::types::AnalysisMetric], key: &str) -> f64 {
    metrics
        .iter()
        .find(|metric| metric.key == key)
        .expect("metric present")
        .value
}

#[test]
fn parser_plugin_tracks_language_counts_and_incremental_cache_hits() {
    let tmp = tempdir().expect("tempdir");
    fs::create_dir_all(tmp.path().join("src")).expect("src dir");
    fs::create_dir_all(tmp.path().join("web")).expect("web dir");
    fs::write(
        tmp.path().join("src/main.rs"),
        "fn main() { if true { println!(\"ok\"); } }",
    )
    .expect("write rust file");
    fs::write(
        tmp.path().join("web/app.ts"),
        "export const answer = () => true;",
    )
    .expect("write ts file");

    let repo = RepoTarget {
        name: "demo".to_string(),
        path: tmp.path().to_string_lossy().to_string(),
    };
    let input = AnalysisInput {
        repo,
        changed_files: Vec::new(),
    };
    let plugin = ParserPlugin::new();

    let first = plugin.run(&input).expect("first parse");
    assert_eq!(parser_metric_value(&first, "ast_cache_hits"), 0.0);
    assert_eq!(parser_metric_value(&first, "ast_cache_misses"), 2.0);
    assert_eq!(parser_metric_value(&first, "language_rust_files"), 1.0);
    assert_eq!(parser_metric_value(&first, "language_typescript_files"), 1.0);
    assert!(parser_metric_value(&first, "ast_node_estimate") > 0.0);

    let second = plugin.run(&input).expect("second parse");
    assert_eq!(parser_metric_value(&second, "ast_cache_hits"), 2.0);
    assert_eq!(parser_metric_value(&second, "ast_cache_misses"), 0.0);
    assert_eq!(parser_metric_value(&second, "language_rust_files"), 1.0);
    assert_eq!(parser_metric_value(&second, "language_typescript_files"), 1.0);

    fs::write(
        tmp.path().join("web/app.ts"),
        "export const answer = () => false; export const mode = 'dark';",
    )
    .expect("rewrite ts file");

    let third = plugin.run(&input).expect("third parse");
    assert_eq!(parser_metric_value(&third, "ast_cache_hits"), 1.0);
    assert_eq!(parser_metric_value(&third, "ast_cache_misses"), 1.0);
    assert_eq!(parser_metric_value(&third, "language_rust_files"), 1.0);
    assert_eq!(parser_metric_value(&third, "language_typescript_files"), 1.0);
}

#[test]
fn pipeline_default_exposes_parser_metrics() {
    let tmp = tempdir().expect("tempdir");
    fs::write(
        tmp.path().join("main.rs"),
        "fn main() { if true { println!(\"ok\"); } }",
    )
    .expect("write file");

    let repo = RepoTarget {
        name: "demo".to_string(),
        path: tmp.path().to_string_lossy().to_string(),
    };
    let pipeline = Pipeline::default();
    let record = pipeline.analyze_repo(repo, "v0.1.0").expect("analyze");

    assert!(record.metrics.iter().any(|m| m.plugin == "parser"));
    assert!(record.metrics.iter().any(|m| m.key == "ast_cache_hits"));
    assert!(record.metrics.iter().any(|m| m.key == "language_rust_files"));
}
