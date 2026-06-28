use repo_analyzer_core::fix_proposal::{
    evaluate_fix_proposal, FixProposalContract, FixProposalSubmission,
};

fn sample_submission(
    proposal_id: &str,
    issue_ids: Vec<&str>,
    retry_count: u32,
    context: Vec<&str>,
) -> FixProposalSubmission {
    FixProposalSubmission {
        proposal_id: proposal_id.to_string(),
        issue_ids: issue_ids.into_iter().map(str::to_string).collect(),
        remediation: "narrow the fix to the smallest viable change".to_string(),
        retry_count,
        context: context.into_iter().map(str::to_string).collect(),
    }
}

#[test]
fn default_fix_proposal_contract_is_versioned_and_capped() {
    let contract = FixProposalContract::default();

    assert_eq!(contract.version, "fix-proposal-v1");
    assert_eq!(contract.max_retries, 2);
}

#[test]
fn minimal_fix_proposal_requires_one_problem() {
    let contract = FixProposalContract::default();
    let evaluation = evaluate_fix_proposal(
        &sample_submission("fix-17", vec!["issue-a", "issue-b"], 0, vec!["trace-1"]),
        &contract,
    );

    assert_eq!(evaluation.schema_version, contract.version);
    assert_eq!(evaluation.issue_count, 2);
    assert!(!evaluation.one_problem);
    assert!(!evaluation.full_context_required);
    assert_eq!(evaluation.escalation_context, Vec::<String>::new());
    assert!(evaluation
        .reason_codes
        .iter()
        .any(|reason| reason == "one_problem_required"));
}

#[test]
fn reaching_retry_cap_forces_escalation_with_full_context() {
    let contract = FixProposalContract::default();
    let evaluation = evaluate_fix_proposal(
        &sample_submission(
            "fix-18",
            vec!["issue-a"],
            contract.max_retries,
            vec!["trace-1", "trace-2"],
        ),
        &contract,
    );

    assert_eq!(evaluation.proposal_id, "fix-18");
    assert_eq!(evaluation.issue_count, 1);
    assert!(evaluation.one_problem);
    assert!(evaluation.retry_cap_reached);
    assert!(evaluation.full_context_required);
    assert_eq!(
        evaluation.escalation_context,
        vec!["trace-1".to_string(), "trace-2".to_string()]
    );
    assert!(evaluation
        .reason_codes
        .iter()
        .any(|reason| reason == "retry_cap_reached"));
}
