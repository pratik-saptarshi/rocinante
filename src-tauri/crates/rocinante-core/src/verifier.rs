use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VerifierInput {
    pub issues: Vec<String>,
    pub test_evidence: Vec<String>,
    pub attempts: u32,
    pub attempt_cap: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VerifierReport {
    pub decision: String,
    pub can_approve: bool,
    pub reason: String,
}

pub fn review_fix_attempt(input: VerifierInput) -> VerifierReport {
    if input.attempts >= input.attempt_cap {
        return VerifierReport {
            decision: "escalate".to_string(),
            can_approve: false,
            reason: "attempt cap reached; escalate with full context".to_string(),
        };
    }

    if input.issues.len() != 1 {
        return VerifierReport {
            decision: "reject".to_string(),
            can_approve: false,
            reason: "one-problem diff required".to_string(),
        };
    }

    if input.test_evidence.is_empty() {
        return VerifierReport {
            decision: "reject".to_string(),
            can_approve: false,
            reason: "test evidence required to approve".to_string(),
        };
    }

    VerifierReport {
        decision: "approve".to_string(),
        can_approve: true,
        reason: format!(
            "one-problem diff approved with test evidence: {}",
            input.test_evidence.join(", ")
        ),
    }
}
