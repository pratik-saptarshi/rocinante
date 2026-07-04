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
        files: vec![],
        circuit_breaker_triggered: false,
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
    assert!(workflow
        .contains("cargo test --locked --manifest-path src-tauri/Cargo.toml --test ci_gate_tests"));
}

#[test]
fn repo_pins_the_rust_toolchain_to_a_specific_stable_release() {
    let toolchain_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../rust-toolchain.toml");
    let toolchain = fs::read_to_string(toolchain_path).expect("read rust toolchain");
    let manifest_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
    let manifest = fs::read_to_string(manifest_path).expect("read cargo manifest");

    assert!(toolchain.contains("channel = \"1.96.1\""));
    assert!(toolchain.contains("profile = \"minimal\""));
    assert!(toolchain.contains("\"clippy\""));
    assert!(toolchain.contains("\"rustfmt\""));
    assert!(manifest.contains("rust-version = \"1.96.1\""));
}

#[test]
fn ci_workflow_uses_the_pinned_toolchain_and_locked_rust_commands() {
    let workflow_path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.github/workflows/ci.yml");
    let workflow = fs::read_to_string(workflow_path).expect("read ci workflow");
    let manifest_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
    let manifest = fs::read_to_string(manifest_path).expect("read cargo manifest");

    assert!(workflow.contains("dtolnay/rust-toolchain@1.96.1"));
    assert!(workflow.contains("components: clippy, rustfmt"));
    assert!(workflow.contains("cargo clippy --locked --manifest-path src-tauri/Cargo.toml"));
    assert!(workflow.contains("cargo check --locked --manifest-path src-tauri/Cargo.toml --bins"));
    assert!(
        workflow.contains("cargo test --locked --manifest-path src-tauri/Cargo.toml --lib --tests")
    );
    assert!(manifest.contains("test = false"));
}

#[test]
fn security_workflow_uses_the_same_pinned_toolchain_for_rust_analysis() {
    let workflow_path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.github/workflows/security.yml");
    let workflow = fs::read_to_string(workflow_path).expect("read security workflow");
    let audit_config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.cargo/audit.toml");
    let audit_config = fs::read_to_string(audit_config_path).expect("read audit config");

    assert!(workflow.contains("dtolnay/rust-toolchain@1.96.1"));
    assert!(workflow.contains("components: clippy, rustfmt"));
    assert!(workflow.contains("taiki-e/install-action@cargo-audit"));
    assert!(workflow.contains("cargo audit --file src-tauri/Cargo.lock --deny warnings"));
    assert!(audit_config.contains("RUSTSEC-2024-0411"));
    assert!(audit_config.contains("RUSTSEC-2024-0429"));
    assert!(audit_config.contains("RUSTSEC-2025-0100"));
}
