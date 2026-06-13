use crate::errors::AnalyzerError;
use crate::types::{CommitterScore, PrRanking, ScoringWeights};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

type HmacSha256 = Hmac<Sha256>;

pub const ALGO_VERSION: &str = "score-script-1.0.0";
const DEFAULT_CONFIG_SECRET: &str = "dev-scoring-config-key";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoringAuditEntry {
    pub ts: i64,
    pub actor: String,
    pub old_version: String,
    pub new_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ScoringConfigEnvelope {
    weights: ScoringWeights,
    sha256: String,
    signature: String,
}

fn config_secret() -> String {
    std::env::var("ROCINANTE_SCORING_CONFIG_SECRET")
        .unwrap_or_else(|_| DEFAULT_CONFIG_SECRET.to_string())
}

fn sha256_hex(raw: &str) -> String {
    let digest = Sha256::digest(raw.as_bytes());
    digest.iter().map(|byte| format!("{:02x}", byte)).collect()
}

fn sign_hash(hash: &str) -> Result<String, AnalyzerError> {
    let mut mac = HmacSha256::new_from_slice(config_secret().as_bytes())
        .map_err(|e| AnalyzerError::Db(e.to_string()))?;
    mac.update(hash.as_bytes());
    Ok(URL_SAFE_NO_PAD.encode(mac.finalize().into_bytes()))
}

fn envelope_for(weights: &ScoringWeights) -> Result<ScoringConfigEnvelope, AnalyzerError> {
    let raw = serde_json::to_string(weights).map_err(|e| AnalyzerError::Db(e.to_string()))?;
    let sha256 = sha256_hex(&raw);
    let signature = sign_hash(&sha256)?;
    Ok(ScoringConfigEnvelope {
        weights: weights.clone(),
        sha256,
        signature,
    })
}

fn verify_envelope(envelope: &ScoringConfigEnvelope) -> Result<(), AnalyzerError> {
    let raw =
        serde_json::to_string(&envelope.weights).map_err(|e| AnalyzerError::Db(e.to_string()))?;
    let expected_hash = sha256_hex(&raw);
    if envelope.sha256 != expected_hash {
        return Err(AnalyzerError::Db(
            "scoring config hash mismatch".to_string(),
        ));
    }
    if sign_hash(&envelope.sha256)? != envelope.signature {
        return Err(AnalyzerError::Db(
            "scoring config signature mismatch".to_string(),
        ));
    }
    Ok(())
}

pub fn load_or_init_weights(path: &str) -> Result<ScoringWeights, AnalyzerError> {
    if !Path::new(path).exists() {
        let defaults = ScoringWeights::default();
        persist_weights(path, &defaults)?;
        return Ok(defaults);
    }
    let raw = fs::read_to_string(path)?;
    if let Ok(envelope) = serde_json::from_str::<ScoringConfigEnvelope>(&raw) {
        verify_envelope(&envelope)?;
        return Ok(envelope.weights);
    }
    let parsed = serde_json::from_str::<ScoringWeights>(&raw)
        .map_err(|e| AnalyzerError::Db(e.to_string()))?;
    Ok(parsed)
}

pub fn persist_weights(path: &str, weights: &ScoringWeights) -> Result<(), AnalyzerError> {
    let envelope = envelope_for(weights)?;
    let raw =
        serde_json::to_string_pretty(&envelope).map_err(|e| AnalyzerError::Db(e.to_string()))?;
    fs::write(path, raw)?;
    Ok(())
}

pub fn append_audit_log(path: &str, entry: &ScoringAuditEntry) -> Result<(), AnalyzerError> {
    let mut f = OpenOptions::new().create(true).append(true).open(path)?;
    let line = serde_json::to_string(entry).map_err(|e| AnalyzerError::Db(e.to_string()))?;
    f.write_all(line.as_bytes())?;
    f.write_all(b"\n")?;
    Ok(())
}

pub fn update_weights_with_audit(
    weights_path: &str,
    audit_path: &str,
    actor: &str,
    new_weights: ScoringWeights,
) -> Result<(), AnalyzerError> {
    let old = load_or_init_weights(weights_path)?;
    persist_weights(weights_path, &new_weights)?;
    append_audit_log(
        audit_path,
        &ScoringAuditEntry {
            ts: now_ts(),
            actor: actor.to_string(),
            old_version: old.version,
            new_version: new_weights.version.clone(),
        },
    )
}

pub fn normalize_scores(mut rows: Vec<CommitterScore>) -> Vec<CommitterScore> {
    if rows.is_empty() {
        return rows;
    }
    let max = rows.iter().map(|r| r.score).fold(f64::MIN, f64::max);
    let min = rows.iter().map(|r| r.score).fold(f64::MAX, f64::min);
    if (max - min).abs() < f64::EPSILON {
        return rows;
    }
    for row in &mut rows {
        row.score = ((row.score - min) / (max - min)) * 100.0;
    }
    rows.sort_by(|a, b| b.score.total_cmp(&a.score));
    rows
}

pub fn top_prs(mut rankings: Vec<PrRanking>, n: usize) -> Vec<PrRanking> {
    rankings.sort_by(|a, b| b.rank_score.total_cmp(&a.rank_score));
    rankings.into_iter().take(n).collect()
}

fn now_ts() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}
