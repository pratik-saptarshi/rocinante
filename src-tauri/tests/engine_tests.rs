use repo_analyzer_core::engine::Pipeline;
use repo_analyzer_core::types::RepoTarget;
use std::fs;
use tempfile::tempdir;

#[test]
fn pipeline_runs_all_beads() {
    let tmp = tempdir().expect("tempdir");
    fs::write(
        tmp.path().join("main.rs"),
        "fn main(){ if true { println!(\"ok\"); } }",
    )
    .expect("write file");

    let repo = RepoTarget {
        name: "demo".to_string(),
        path: tmp.path().to_string_lossy().to_string(),
    };

    let pipeline = Pipeline::default();
    let record = pipeline.analyze_repo(repo, "v0.1.0").expect("analyze");

    assert!(record.metrics.iter().any(|m| m.plugin == "code_quality"));
    assert!(record.metrics.iter().any(|m| m.plugin == "complexity"));
    assert!(record
        .metrics
        .iter()
        .any(|m| m.plugin == "contribution_velocity"));
    assert!(record.metrics.iter().any(|m| m.plugin == "pr_approval"));
}
