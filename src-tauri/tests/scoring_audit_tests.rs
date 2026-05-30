use repo_analyzer_core::admin::update_scoring_weights;
use repo_analyzer_core::types::ScoringWeights;
use std::fs;
use tempfile::tempdir;

#[test]
fn weight_updates_are_audited_and_admin_only() {
    let dir = tempdir().expect("tmp");
    let weights = dir.path().join("weights.json");
    let audit = dir.path().join("audit.log");

    let denied = update_scoring_weights(
        "bob:reader",
        weights.to_str().expect("weights path"),
        audit.to_str().expect("audit path"),
        ScoringWeights::default(),
    );
    assert!(denied.is_err());

    let mut w = ScoringWeights::default();
    w.version = "v2".to_string();
    update_scoring_weights(
        "alice:admin",
        weights.to_str().expect("weights path"),
        audit.to_str().expect("audit path"),
        w,
    )
    .expect("admin update");

    let log = fs::read_to_string(audit).expect("read audit");
    assert!(log.contains("alice"));
    assert!(log.contains("v2"));
}
