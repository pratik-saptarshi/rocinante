use crate::errors::AnalyzerError;
use crate::types::{CommitterScore, PrRanking, ScoringWeights};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

pub const ALGO_VERSION: &str = "score-script-1.0.0";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoringAuditEntry {
    pub ts: i64,
    pub actor: String,
    pub old_version: String,
    pub new_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SignedScoringWeights {
    weights: ScoringWeights,
    signature: String,
}
pub fn load_or_init_weights(path: &str) -> Result<ScoringWeights, AnalyzerError> {
    if !Path::new(path).exists() {
        let defaults = ScoringWeights::default();
        persist_weights(path, &defaults)?;
        return Ok(defaults);
    }
    let raw = fs::read_to_string(path)?;
    if let Ok(envelope) = serde_json::from_str::<SignedScoringWeights>(&raw) {
        let expected = signature_for_weights(&envelope.weights)?;
        if envelope.signature != expected {
            return Err(AnalyzerError::Integrity(
                "scoring config signature mismatch".to_string(),
            ));
        }
        return Ok(envelope.weights);
    }
    let parsed = serde_json::from_str::<ScoringWeights>(&raw)
        .map_err(|e| AnalyzerError::Db(e.to_string()))?;
    Ok(parsed)
}

pub fn persist_weights(path: &str, weights: &ScoringWeights) -> Result<(), AnalyzerError> {
    let raw = serde_json::to_string_pretty(&SignedScoringWeights {
        weights: weights.clone(),
        signature: signature_for_weights(weights)?,
    })
    .map_err(|e| AnalyzerError::Db(e.to_string()))?;
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

fn signature_for_weights(weights: &ScoringWeights) -> Result<String, AnalyzerError> {
    let payload = serde_json::to_vec(weights).map_err(|e| AnalyzerError::Db(e.to_string()))?;
    let mut hasher = Sha256::new();
    hasher.update(payload);
    let digest = hasher.finalize();
    Ok(digest.iter().map(|byte| format!("{:02x}", byte)).collect())
}
