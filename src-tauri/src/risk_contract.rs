use crate::types::PrCandidate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PrRiskDecision {
    Allow,
    Review,
    Block,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PrRiskSchema {
    pub version: String,
    pub file_risk_weight: f64,
    pub velocity_weight: f64,
    pub approval_weight: f64,
    pub review_threshold: f64,
    pub block_threshold: f64,
}

impl Default for PrRiskSchema {
    fn default() -> Self {
        Self {
            version: "risk-v1".to_string(),
            file_risk_weight: 0.50,
            velocity_weight: 0.20,
            approval_weight: 0.30,
            review_threshold: 0.35,
            block_threshold: 0.60,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PrRiskEvaluation {
    pub schema_version: String,
    pub pr_id: String,
    pub repo_name: String,
    pub author: String,
    pub release: String,
    pub risk_score: f64,
    pub decision: PrRiskDecision,
    pub reason_codes: Vec<String>,
}

pub fn evaluate_pr_risk(candidate: &PrCandidate, schema: &PrRiskSchema) -> PrRiskEvaluation {
    let velocity_penalty = 1.0 - candidate.author_velocity;
    let approval_penalty = 1.0 - candidate.approval_fidelity;
    let risk_score = (candidate.file_risk * schema.file_risk_weight)
        + (velocity_penalty * schema.velocity_weight)
        + (approval_penalty * schema.approval_weight);
    let decision = if risk_score >= schema.block_threshold {
        PrRiskDecision::Block
    } else if risk_score >= schema.review_threshold {
        PrRiskDecision::Review
    } else {
        PrRiskDecision::Allow
    };

    PrRiskEvaluation {
        schema_version: schema.version.clone(),
        pr_id: candidate.pr_id.clone(),
        repo_name: candidate.repo_name.clone(),
        author: candidate.author.clone(),
        release: candidate.release.clone(),
        risk_score,
        decision,
        reason_codes: vec![
            format!("file_risk={:.2}", candidate.file_risk),
            format!("velocity_penalty={:.2}", velocity_penalty),
            format!("approval_penalty={:.2}", approval_penalty),
        ],
    }
}
