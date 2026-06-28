use std::fs;
use std::path::PathBuf;

use repo_analyzer_core::ci_gate::build_ci_gate_comment;
use repo_analyzer_core::risk_contract::{evaluate_pr_risk, PrRiskSchema};
use repo_analyzer_core::types::PrCandidate;

fn sample_candidate(
    pr_id: &str,
    file_risk: f64,
    author_velocity: f64,
    approval_fidelity: f64,
) -> PrCandidate {
    PrCandidate {
        pr_id: pr_id.to_string(),
        repo_name: "repo-a".to_string(),
        author: "alice".to_string(),
        release: "v1.0.0".to_string(),
        file_risk,
        author_velocity,
        approval_fidelity,
        ..Default::default()
    }
}

#[test]
fn ci_gate_comment_blocks_high_risk_prs_with_a_stable_summary() {
    let schema = PrRiskSchema::default();
    let evaluation = evaluate_pr_risk(&sample_candidate("pr-31", 0.9, 0.35, 0.45), &schema);
    let comment = build_ci_gate_comment(&evaluation);

    assert!(comment.should_block_merge);
    assert!(comment.summary.contains("block merge"));
    assert!(comment.summary.contains("security-review"));
    assert!(comment.body.contains("Decision: Block merge"));
    assert!(comment.body.contains("Review requirement: security-review"));
    assert!(comment.body.contains("Reason codes:"));
    assert!(comment.body.contains("file_risk=0.90"));
}

#[test]
fn ci_gate_comment_allows_low_risk_prs_with_a_stable_summary() {
    let schema = PrRiskSchema::default();
    let evaluation = evaluate_pr_risk(&sample_candidate("pr-32", 0.1, 0.9, 0.95), &schema);
    let comment = build_ci_gate_comment(&evaluation);

    assert!(!comment.should_block_merge);
    assert!(comment.summary.contains("allow merge"));
    assert!(comment.summary.contains("none"));
    assert!(comment.body.contains("Decision: Allow merge"));
    assert!(comment.body.contains("Review requirement: none"));
}

#[test]
fn ci_gate_comment_round_trips_through_json() {
    let schema = PrRiskSchema::default();
    let evaluation = evaluate_pr_risk(&sample_candidate("pr-33", 0.8, 0.5, 0.6), &schema);
    let comment = build_ci_gate_comment(&evaluation);

    let raw = serde_json::to_string(&comment).expect("serialize gate comment");
    let round_trip = serde_json::from_str(&raw).expect("deserialize gate comment");

    assert_eq!(comment, round_trip);
}

#[test]
fn ci_workflow_includes_the_ci_gate_contract_step() {
    let workflow_path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.github/workflows/ci.yml");
    let workflow = fs::read_to_string(workflow_path).expect("read ci workflow");

    assert!(workflow.contains("CI gate contract"));
    assert!(
        workflow.contains("cargo test --manifest-path src-tauri/Cargo.toml --test ci_gate_tests")
    );
}
