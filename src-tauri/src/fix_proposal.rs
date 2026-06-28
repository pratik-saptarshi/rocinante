use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FixProposalContract {
    pub version: String,
    pub max_retries: u32,
}

impl Default for FixProposalContract {
    fn default() -> Self {
        Self {
            version: "fix-proposal-v1".to_string(),
            max_retries: 2,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FixProposalSubmission {
    pub proposal_id: String,
    pub issue_ids: Vec<String>,
    pub remediation: String,
    pub retry_count: u32,
    pub context: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FixProposalEvaluation {
    pub schema_version: String,
    pub proposal_id: String,
    pub issue_count: usize,
    pub retry_count: u32,
    pub one_problem: bool,
    pub retry_cap_reached: bool,
    pub full_context_required: bool,
    pub escalation_context: Vec<String>,
    pub reason_codes: Vec<String>,
}

pub fn evaluate_fix_proposal(
    submission: &FixProposalSubmission,
    contract: &FixProposalContract,
) -> FixProposalEvaluation {
    let issue_count = submission.issue_ids.len();
    let one_problem = issue_count == 1;
    let retry_cap_reached = submission.retry_count >= contract.max_retries;
    let full_context_required = retry_cap_reached;
    let escalation_context = if full_context_required {
        submission.context.clone()
    } else {
        Vec::new()
    };
    let mut reason_codes = vec![
        format!("issue_count={issue_count}"),
        format!("retry_count={}", submission.retry_count),
        format!("retry_cap={}", contract.max_retries),
    ];

    if !one_problem {
        reason_codes.push("one_problem_required".to_string());
    }

    if retry_cap_reached {
        reason_codes.push("retry_cap_reached".to_string());
    }

    if full_context_required {
        reason_codes.push("full_context_required".to_string());
    }

    FixProposalEvaluation {
        schema_version: contract.version.clone(),
        proposal_id: submission.proposal_id.clone(),
        issue_count,
        retry_count: submission.retry_count,
        one_problem,
        retry_cap_reached,
        full_context_required,
        escalation_context,
        reason_codes,
    }
}
