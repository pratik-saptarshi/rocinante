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
        ..Default::default()
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

#[test]
fn out_of_range_pr_risk_inputs_are_clamped_before_scoring() {
    let schema = PrRiskSchema::default();
    let evaluation = evaluate_pr_risk(&sample_candidate("pr-19", 1.4, 2.0, -0.2), &schema);

    assert_eq!(evaluation.decision, PrRiskDecision::Block);
    assert!((evaluation.risk_score - 0.8).abs() < 1e-12);
    assert_eq!(
        evaluation.reason_codes,
        vec![
            "file_risk=1.00".to_string(),
            "velocity_penalty=0.00".to_string(),
            "approval_penalty=1.00".to_string(),
        ]
    );
}

#[test]
fn pr_risk_contract_is_stable_for_same_input() {
    let schema = PrRiskSchema::default();
    let candidate = sample_candidate("pr-20", 0.7, 0.5, 0.6);

    let first = evaluate_pr_risk(&candidate, &schema);
    let second = evaluate_pr_risk(&candidate, &schema);

    assert_eq!(first.schema_version, second.schema_version);
    assert_eq!(first.tier, second.tier);
    assert_eq!(first.review_requirement, second.review_requirement);
    assert_eq!(first.risk_score, second.risk_score);
    assert_eq!(first.decision, second.decision);
    assert_eq!(first.reason_codes, second.reason_codes);
}

#[test]
fn pr_risk_contract_maps_thresholds_to_expected_tier_and_review_requirement() {
    let schema = PrRiskSchema::default();

    let allow = evaluate_pr_risk(&sample_candidate("pr-21", 0.69, 1.0, 1.0), &schema);
    assert_eq!(allow.tier, "low");
    assert_eq!(allow.review_requirement, "none");

    let review = evaluate_pr_risk(&sample_candidate("pr-22", 0.70, 1.0, 1.0), &schema);
    assert_eq!(review.tier, "medium");
    assert_eq!(review.review_requirement, "human-review");

    let block = evaluate_pr_risk(&sample_candidate("pr-23", 0.60, 1.0, 0.0), &schema);
    assert_eq!(block.tier, "high");
    assert_eq!(block.review_requirement, "security-review");
}

#[test]
fn pr_risk_contract_round_trips_through_json() {
    let schema = PrRiskSchema::default();
    let evaluation = evaluate_pr_risk(&sample_candidate("pr-24", 0.7, 0.5, 0.6), &schema);

    let raw = serde_json::to_string(&evaluation).expect("serialize evaluation");
    let round_trip: repo_analyzer_core::risk_contract::PrRiskEvaluation =
        serde_json::from_str(&raw).expect("deserialize evaluation");

    assert_eq!(evaluation, round_trip);
}
