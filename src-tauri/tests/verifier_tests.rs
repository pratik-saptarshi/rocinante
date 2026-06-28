use repo_analyzer_core::verifier::{review_fix_attempt, VerifierInput};

#[test]
fn verifier_defaults_to_reject_without_test_evidence() {
    let report = review_fix_attempt(VerifierInput {
        issues: vec!["panic in triage formatter".to_string()],
        test_evidence: vec![],
        attempts: 1,
        attempt_cap: 3,
    });

    assert_eq!(report.decision, "reject");
    assert!(report.reason.contains("test evidence required"));
    assert!(!report.can_approve);
}

#[test]
fn verifier_approves_only_when_test_evidence_is_present_for_one_problem_diff() {
    let report = review_fix_attempt(VerifierInput {
        issues: vec!["panic in triage formatter".to_string()],
        test_evidence: vec!["cargo test --test triage_tests".to_string()],
        attempts: 2,
        attempt_cap: 3,
    });

    assert_eq!(report.decision, "approve");
    assert!(report.can_approve);
    assert!(report.reason.contains("one-problem diff"));
    assert!(report.reason.contains("test evidence"));
}

#[test]
fn verifier_escalates_when_attempt_cap_is_reached() {
    let report = review_fix_attempt(VerifierInput {
        issues: vec!["panic in triage formatter".to_string()],
        test_evidence: vec!["cargo test --test triage_tests".to_string()],
        attempts: 3,
        attempt_cap: 3,
    });

    assert_eq!(report.decision, "escalate");
    assert!(report.reason.contains("attempt cap"));
    assert!(!report.can_approve);
}
