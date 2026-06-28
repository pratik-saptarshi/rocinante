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
    pub tier: String,
    pub review_requirement: String,
    pub risk_score: f64,
    pub decision: PrRiskDecision,
    pub reason_codes: Vec<String>,
}

pub fn evaluate_pr_risk(candidate: &PrCandidate, schema: &PrRiskSchema) -> PrRiskEvaluation {
    let file_risk = candidate.file_risk.clamp(0.0, 1.0);
    let author_velocity = candidate.author_velocity.clamp(0.0, 1.0);
    let approval_fidelity = candidate.approval_fidelity.clamp(0.0, 1.0);
    let velocity_penalty = 1.0 - author_velocity;
    let approval_penalty = 1.0 - approval_fidelity;
    let risk_score = (file_risk * schema.file_risk_weight)
        + (velocity_penalty * schema.velocity_weight)
        + (approval_penalty * schema.approval_weight);
    let decision = if risk_score >= schema.block_threshold {
        PrRiskDecision::Block
    } else if risk_score >= schema.review_threshold {
        PrRiskDecision::Review
    } else {
        PrRiskDecision::Allow
    };
    let (tier, review_requirement) = match decision {
        PrRiskDecision::Allow => ("low".to_string(), "none".to_string()),
        PrRiskDecision::Review => ("medium".to_string(), "human-review".to_string()),
        PrRiskDecision::Block => ("high".to_string(), "security-review".to_string()),
    };

    PrRiskEvaluation {
        schema_version: schema.version.clone(),
        pr_id: candidate.pr_id.clone(),
        repo_name: candidate.repo_name.clone(),
        author: candidate.author.clone(),
        release: candidate.release.clone(),
        tier,
        review_requirement,
        risk_score,
        decision,
        reason_codes: vec![
            format!("file_risk={:.2}", file_risk),
            format!("velocity_penalty={:.2}", velocity_penalty),
            format!("approval_penalty={:.2}", approval_penalty),
        ],
    }
}
