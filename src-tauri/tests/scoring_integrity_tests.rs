use repo_analyzer_core::scoring::load_or_init_weights;
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
fn weights_are_persisted_as_signed_envelopes() {
    let dir = tempdir().expect("tmp");
    let weights = dir.path().join("weights.json");

    let loaded = load_or_init_weights(weights.to_str().expect("weights path")).expect("weights");
    assert_eq!(
        serialized_weights(&loaded),
        serialized_weights(&ScoringWeights::default())
    );

    let raw = fs::read_to_string(&weights).expect("read weights");
    let envelope: SignedWeightsEnvelope = serde_json::from_str(&raw).expect("envelope");
    let digest = signature_for_weights(&envelope.weights);

    assert_eq!(
        serialized_weights(&envelope.weights),
        serialized_weights(&loaded)
    );
    assert_eq!(envelope.signature, digest);
}

#[test]
fn tampered_weights_fail_integrity_check() {
    let dir = tempdir().expect("tmp");
    let weights = dir.path().join("weights.json");

    let _ = load_or_init_weights(weights.to_str().expect("weights path")).expect("weights");
    let raw = fs::read_to_string(&weights).expect("read weights");
    let mut envelope: SignedWeightsEnvelope = serde_json::from_str(&raw).expect("envelope");
    envelope.weights.version = "v9".to_string();

    fs::write(
        &weights,
        serde_json::to_string_pretty(&envelope).expect("tampered json"),
    )
    .expect("write tampered");

    let err = load_or_init_weights(weights.to_str().expect("weights path"))
        .expect_err("tamper must fail");
    assert!(err.to_string().contains("integrity"));
}

fn signature_for_weights(weights: &ScoringWeights) -> String {
    let payload = serde_json::to_vec(weights).expect("weights json");
    let mut hasher = Sha256::new();
    hasher.update(payload);
    format!("{:x}", hasher.finalize())
}

fn serialized_weights(weights: &ScoringWeights) -> serde_json::Value {
    serde_json::to_value(weights).expect("weights value")
}
