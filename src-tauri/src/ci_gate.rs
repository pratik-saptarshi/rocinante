use crate::risk_contract::{PrRiskDecision, PrRiskEvaluation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CiGateComment {
    pub schema_version: String,
    pub pr_id: String,
    pub repo_name: String,
    pub decision: String,
    pub review_requirement: String,
    pub should_block_merge: bool,
    pub summary: String,
    pub body: String,
}

pub fn build_ci_gate_comment(evaluation: &PrRiskEvaluation) -> CiGateComment {
    let decision = match evaluation.decision {
        PrRiskDecision::Allow => "Allow merge",
        PrRiskDecision::Review => "Review required",
        PrRiskDecision::Block => "Block merge",
    };
    let should_block_merge = matches!(evaluation.decision, PrRiskDecision::Block);
    let summary = format!(
        "{} | {} | {} | {:.2}",
        evaluation.schema_version,
        decision.to_lowercase(),
        evaluation.review_requirement,
        evaluation.risk_score
    );
    let body = format!(
        "## PR Risk Gate\n- Schema: {}\n- PR: {}/{}\n- Decision: {}\n- Tier: {}\n- Review requirement: {}\n- Risk score: {:.2}\n- Reason codes:\n  - {}",
        evaluation.schema_version,
        evaluation.repo_name,
        evaluation.pr_id,
        decision,
        evaluation.tier,
        evaluation.review_requirement,
        evaluation.risk_score,
        evaluation.reason_codes.join("\n  - ")
    );

    CiGateComment {
        schema_version: evaluation.schema_version.clone(),
        pr_id: evaluation.pr_id.clone(),
        repo_name: evaluation.repo_name.clone(),
        decision: decision.to_string(),
        review_requirement: evaluation.review_requirement.clone(),
        should_block_merge,
        summary,
        body,
    }
}
