use repo_analyzer_core::scoring::{load_or_init_weights, persist_weights};
use repo_analyzer_core::types::ScoringWeights;
use serde_json::json;
use tempfile::tempdir;

#[test]
fn persists_and_loads_signed_scoring_config() {
    let dir = tempdir().expect("tmp");
    let weights_path = dir.path().join("weights.json");
    let weights = ScoringWeights::default();

    persist_weights(weights_path.to_str().expect("path"), &weights).expect("persist");

    let raw = std::fs::read_to_string(&weights_path).expect("read");
    let stored: serde_json::Value = serde_json::from_str(&raw).expect("json");
    assert!(stored.get("weights").is_some());
    assert!(stored.get("sha256").is_some());
    assert!(stored.get("signature").is_some());

    let loaded = load_or_init_weights(weights_path.to_str().expect("path")).expect("load");
    assert_eq!(loaded.version, weights.version);
}

#[test]
fn rejects_tampered_scoring_config_envelope() {
    let dir = tempdir().expect("tmp");
    let weights_path = dir.path().join("weights.json");
    let tampered = json!({
        "weights": {
            "version": "v1",
            "complexity_weight": 0.30,
            "coverage_weight": 0.25,
            "churn_weight": 0.20,
            "pipeline_weight": 0.25,
            "pr_file_risk_weight": 0.50,
            "pr_velocity_weight": 0.20,
            "pr_approval_weight": 0.30
        },
        "sha256": "0000000000000000000000000000000000000000000000000000000000000000",
        "signature": "invalid"
    });

    std::fs::write(&weights_path, serde_json::to_string_pretty(&tampered).expect("json")).expect("write");

    let err = load_or_init_weights(weights_path.to_str().expect("path")).expect_err("tamper");
    assert!(err.to_string().contains("scoring config"));
}

#[test]
fn keeps_legacy_plain_weights_compatible() {
    let dir = tempdir().expect("tmp");
    let weights_path = dir.path().join("weights.json");
    let legacy = ScoringWeights::default();
    std::fs::write(
        &weights_path,
        serde_json::to_string_pretty(&legacy).expect("json"),
    )
    .expect("write");

    let loaded = load_or_init_weights(weights_path.to_str().expect("path")).expect("load");
    assert_eq!(loaded.version, legacy.version);
}
