use repo_analyzer_core::incident_feedback::RiskFeedbackStore;
use repo_analyzer_core::risk_contract::{evaluate_pr_risk, PrRiskDecision, PrRiskSchema};
use repo_analyzer_core::types::PrCandidate;
use tempfile::tempdir;

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
fn incidents_raise_risk_and_are_auditable() {
    let schema = PrRiskSchema::default();
    let base = evaluate_pr_risk(&sample_candidate("pr-41", 0.1, 0.9, 0.95), &schema);
    let mut store = RiskFeedbackStore::default();

    let event = store.record_incident("carol", "pr-41", "production defect", 0.30);
    let adjusted = store.apply_to(&base);

    assert!(adjusted.risk_score > base.risk_score);
    assert_eq!(adjusted.decision, PrRiskDecision::Review);
    assert_eq!(adjusted.cache_revision, 1);
    assert_eq!(adjusted.audit_log.len(), 1);
    assert_eq!(adjusted.audit_log[0], event);
    assert_eq!(adjusted.audit_log[0].actor, "carol");
    assert_eq!(adjusted.audit_log[0].subject, "pr-41");
    assert_eq!(adjusted.audit_log[0].risk_delta, 0.30);
}

#[test]
fn new_annotations_invalidate_cached_assessments() {
    let schema = PrRiskSchema::default();
    let base = evaluate_pr_risk(&sample_candidate("pr-42", 0.1, 0.9, 0.95), &schema);
    let mut store = RiskFeedbackStore::default();
    let cached_revision = store.current_revision();

    store.record_annotation("dan", "pr-42", "review notes", 0.15);

    assert!(!store.is_cache_valid(cached_revision));

    let adjusted = store.apply_to(&base);
    assert!(adjusted.risk_score > base.risk_score);
    assert_eq!(adjusted.cache_revision, store.current_revision());
    assert_eq!(adjusted.audit_log[0].kind, "annotation");
}

#[test]
fn repeated_feedback_increments_revision_and_keeps_history() {
    let mut store = RiskFeedbackStore::default();

    store.record_incident("carol", "pr-43", "production defect", 0.20);
    store.record_annotation("dana", "pr-43", "triage notes", 0.10);

    assert_eq!(store.current_revision(), 2);
    assert_eq!(store.audit_log().len(), 2);
    assert_eq!(store.audit_log()[0].revision, 1);
    assert_eq!(store.audit_log()[1].revision, 2);
}

#[test]
fn file_backed_feedback_store_reloads_audit_log_and_revision() {
    let dir = tempdir().expect("tmp");
    let path = dir.path().join("feedback.jsonl");

    {
        let mut store = RiskFeedbackStore::open(&path).expect("open");
        store.record_incident("erin", "pr-44", "production defect", 0.20);
        store.record_annotation("frank", "pr-44", "follow-up", 0.10);
        assert_eq!(store.current_revision(), 2);
    }

    let store = RiskFeedbackStore::open(&path).expect("reopen");
    assert_eq!(store.current_revision(), 2);
    assert_eq!(store.audit_log().len(), 2);
    assert_eq!(store.audit_log()[0].subject, "pr-44");
    assert_eq!(store.audit_log()[1].kind, "annotation");

    let schema = PrRiskSchema::default();
    let base = evaluate_pr_risk(&sample_candidate("pr-44", 0.1, 0.9, 0.95), &schema);
    let adjusted = store.apply_to(&base);
    assert!(adjusted.risk_score > base.risk_score);
    assert_eq!(adjusted.cache_revision, 2);
}
