use repo_analyzer_core::auth::issue_test_token;
use repo_analyzer_core::storage::DualLayerStore;
use repo_analyzer_core::types::PrCandidate;
use tempfile::tempdir;

#[test]
fn tauri_command_evaluate_pr_risk_uses_the_default_schema() {
    let token = issue_test_token("alice", &["admin"], 3600);
    let candidate = PrCandidate {
        pr_id: "pr-21".to_string(),
        repo_name: "repo-a".to_string(),
        author: "alice".to_string(),
        release: "v1.0.0".to_string(),
        file_risk: 0.92,
        author_velocity: 0.35,
        approval_fidelity: 0.45,
    };

    let evaluation = repo_analyzer_core::tauri_commands::evaluate_pr_risk(
        token.to_string(),
        candidate,
    )
    .expect("evaluate pr risk through command surface");

    assert_eq!(evaluation.schema_version, "risk-v1");
    assert_eq!(evaluation.pr_id, "pr-21");
}

#[test]
fn tauri_command_rejects_non_admin_pr_risk_evaluation() {
    let token = issue_test_token("bob", &["reader"], 3600);
    let candidate = PrCandidate {
        pr_id: "pr-22".to_string(),
        repo_name: "repo-a".to_string(),
        author: "bob".to_string(),
        release: "v1.0.0".to_string(),
        file_risk: 0.2,
        author_velocity: 0.2,
        approval_fidelity: 0.2,
    };

    let err = repo_analyzer_core::tauri_commands::evaluate_pr_risk(token.to_string(), candidate)
        .expect_err("non-admin rejected through command surface");

    assert!(err.contains("permission denied"));
}

#[test]
fn tauri_command_release_baseline_roundtrips() {
    let dir = tempdir().expect("tmp");
    let kv = dir.path().join("kv");
    let col = dir.path().join("analytics.duckdb");
    let token = issue_test_token("alice", &["admin"], 3600);
    let store = DualLayerStore::open(
        kv.to_str().expect("kv path"),
        col.to_str().expect("col path"),
    )
    .expect("open");
    store
        .reseed_release_baseline("repo-a", 9.75)
        .expect("seed baseline");

    let seeded = repo_analyzer_core::tauri_commands::query_release_baseline(
        token.to_string(),
        kv.to_str().expect("kv path").to_string(),
        col.to_str().expect("col path").to_string(),
        "repo-a".to_string(),
    )
    .expect("query baseline");
    assert_eq!(seeded, Some(9.75));

    let reseeded = repo_analyzer_core::tauri_commands::reseed_release_baseline(
        token.to_string(),
        kv.to_str().expect("kv path").to_string(),
        col.to_str().expect("col path").to_string(),
        "repo-a".to_string(),
        11.25,
    )
    .expect("reseed baseline");
    assert_eq!(reseeded, 11.25);
}

#[test]
fn tauri_command_release_baseline_rejects_non_admin() {
    let dir = tempdir().expect("tmp");
    let kv = dir.path().join("kv");
    let col = dir.path().join("analytics.duckdb");
    let token = issue_test_token("bob", &["reader"], 3600);

    let err = repo_analyzer_core::tauri_commands::reseed_release_baseline(
        token.to_string(),
        kv.to_str().expect("kv path").to_string(),
        col.to_str().expect("col path").to_string(),
        "repo-a".to_string(),
        13.5,
    )
    .expect_err("non-admin rejected");

    assert!(err.contains("permission denied"));
}
