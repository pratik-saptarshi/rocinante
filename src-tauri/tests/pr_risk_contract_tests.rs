use repo_analyzer_core::risk_contract::{evaluate_pr_risk, PrRiskDecision, PrRiskSchema};
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
        release: "v2.1.0".to_string(),
        file_risk,
        author_velocity,
        approval_fidelity,
    }
}

#[test]
fn default_pr_risk_schema_is_versioned() {
    let schema = PrRiskSchema::default();

    assert_eq!(schema.version, "risk-v1");
    assert_eq!(schema.block_threshold, 0.60);
    assert_eq!(schema.review_threshold, 0.35);
}

#[test]
fn high_risk_pr_is_blocked_by_the_contract() {
    let schema = PrRiskSchema::default();
    let evaluation = evaluate_pr_risk(&sample_candidate("pr-17", 0.8, 0.5, 0.6), &schema);

    assert_eq!(evaluation.schema_version, schema.version);
    assert_eq!(evaluation.decision, PrRiskDecision::Block);
    assert!((evaluation.risk_score - 0.62).abs() < 1e-12);
    assert_eq!(
        evaluation.reason_codes,
        vec![
            "file_risk=0.80".to_string(),
            "velocity_penalty=0.50".to_string(),
            "approval_penalty=0.40".to_string(),
        ],
    );
}

#[test]
fn low_risk_pr_is_allowed_by_the_contract() {
    let schema = PrRiskSchema::default();
    let evaluation = evaluate_pr_risk(&sample_candidate("pr-18", 0.1, 0.9, 0.95), &schema);

    assert_eq!(evaluation.decision, PrRiskDecision::Allow);
    assert!((evaluation.risk_score - 0.085).abs() < 1e-12);
    assert!(evaluation
        .reason_codes
        .iter()
        .all(|reason| reason.contains('=')));
}
