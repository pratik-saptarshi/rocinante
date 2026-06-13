use repo_analyzer_core::scoring::{load_or_init_weights, persist_weights};
use repo_analyzer_core::types::ScoringWeights;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use tempfile::tempdir;

#[derive(Debug, Deserialize, Serialize)]
struct SignedWeightsEnvelope {
    weights: ScoringWeights,
    signature: String,
}

#[test]
fn persists_and_loads_signed_scoring_config() {
    let dir = tempdir().expect("tmp");
    let weights_path = dir.path().join("weights.json");
    let weights = ScoringWeights::default();

    persist_weights(weights_path.to_str().expect("path"), &weights).expect("persist");

    let raw = fs::read_to_string(&weights_path).expect("read");
    let stored: SignedWeightsEnvelope = serde_json::from_str(&raw).expect("json");
    assert_eq!(stored.signature, signature_for_weights(&stored.weights));
    assert_eq!(stored.weights.version, weights.version);

    let loaded = load_or_init_weights(weights_path.to_str().expect("path")).expect("load");
    assert_eq!(loaded.version, weights.version);
}

#[test]
fn rejects_tampered_scoring_config_envelope() {
    let dir = tempdir().expect("tmp");
    let weights_path = dir.path().join("weights.json");
    let tampered = SignedWeightsEnvelope {
        weights: ScoringWeights {
            version: "v1".to_string(),
            complexity_weight: 0.30,
            coverage_weight: 0.25,
            churn_weight: 0.20,
            pipeline_weight: 0.25,
            pr_file_risk_weight: 0.50,
            pr_velocity_weight: 0.20,
            pr_approval_weight: 0.30,
        },
        signature: "invalid".to_string(),
    };

    std::fs::write(
        &weights_path,
        serde_json::to_string_pretty(&tampered).expect("json"),
    )
    .expect("write");

    let err = load_or_init_weights(weights_path.to_str().expect("path")).expect_err("tamper");
    assert!(err.to_string().contains("integrity"));
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

fn signature_for_weights(weights: &ScoringWeights) -> String {
    let payload = serde_json::to_vec(weights).expect("weights json");
    let mut hasher = Sha256::new();
    hasher.update(payload);
    format!("{:x}", hasher.finalize())
}
