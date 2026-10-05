use rocinante_core::auth_claims::PrincipalClaims;
use rocinante_core::authorization::principal_is_admin;
use rocinante_core::baseline::{
    query_release_baseline, reseed_release_baseline, score_baseline_components, BaselineScoreInput,
    ReleaseBaselineError, ReleaseBaselineRepository,
};
use rocinante_core::budget_guard::{BudgetDecision, BudgetGuard, StopReason};
use rocinante_core::fix_proposal::{
    evaluate_fix_proposal, FixProposalContract, FixProposalSubmission,
};
use rocinante_core::risk_contract::{evaluate_pr_risk, PrCandidate, PrRiskDecision, PrRiskSchema};
use rocinante_core::triage::{build_triage_report, TriageFinding, TriageInput};
use rocinante_core::types::{Principal, ScoringWeights};
use std::cell::RefCell;

#[derive(Default)]
struct MemoryBaselines(RefCell<Vec<(String, f64)>>);

impl ReleaseBaselineRepository for MemoryBaselines {
    type Error = ();

    fn read_release_baseline(&self, repo_name: &str) -> Result<Option<f64>, Self::Error> {
        Ok(self
            .0
            .borrow()
            .iter()
            .find(|(name, _)| name == repo_name)
            .map(|(_, value)| *value))
    }

    fn reseed_release_baseline(
        &self,
        repo_name: &str,
        baseline_complexity: f64,
    ) -> Result<f64, Self::Error> {
        let mut values = self.0.borrow_mut();
        if let Some((_, value)) = values.iter_mut().find(|(name, _)| name == repo_name) {
            *value = baseline_complexity;
        } else {
            values.push((repo_name.to_string(), baseline_complexity));
        }
        Ok(baseline_complexity)
    }
}
use rocinante_core::verifier::{review_fix_attempt, VerifierInput};

#[test]
fn host_agnostic_baseline_scoring_preserves_delta_and_default_component_rules() {
    let components = score_baseline_components(
        BaselineScoreInput {
            complexity: Some(30.0),
            baseline_complexity: Some(20.0),
            coverage_delta: None,
            churn_efficiency: Some(1.5),
            pipeline_success: None,
        },
        &ScoringWeights::default(),
    );

    assert!((components.complexity - (30.0 / 11.0)).abs() < 1e-12);
    assert_eq!(components.coverage, 12.5);
    assert_eq!(components.churn, 20.0);
    assert_eq!(components.pipeline, 12.5);
}

#[test]
fn host_agnostic_release_baseline_service_authorizes_and_uses_repository_contract() {
    let repository = MemoryBaselines::default();
    let reader = Principal {
        user: "bob".to_string(),
        roles: vec!["reader".to_string()],
    };
    assert_eq!(
        reseed_release_baseline(&reader, &repository, "repo-a", 12.5),
        Err(ReleaseBaselineError::PermissionDenied("bob".to_string()))
    );
    assert_eq!(
        repository.read_release_baseline("repo-a").expect("read"),
        None,
        "denied reseed must not reach the host repository"
    );

    let admin = Principal {
        user: "alice".to_string(),
        roles: vec!["admin".to_string()],
    };
    assert_eq!(
        reseed_release_baseline(&admin, &repository, "repo-a", 12.5),
        Ok(12.5)
    );
    assert_eq!(
        query_release_baseline(&admin, &repository, "repo-a"),
        Ok(Some(12.5))
    );
}

#[test]
fn host_agnostic_authorization_requires_the_explicit_admin_role() {
    let admin = Principal {
        user: "alice".to_string(),
        roles: vec!["reader".to_string(), "admin".to_string()],
    };
    let reader = Principal {
        user: "bob".to_string(),
        roles: vec!["reader".to_string()],
    };

    assert!(principal_is_admin(&admin));
    assert!(!principal_is_admin(&reader));
}

#[test]
fn host_agnostic_claim_policy_checks_identity_context_and_expiry() {
    let claims = PrincipalClaims {
        user: "alice".to_string(),
        roles: vec!["admin".to_string()],
        iss: "rocinante-console".to_string(),
        aud: "repo-analyzer".to_string(),
        exp: 101,
    };
    let principal = claims
        .clone()
        .into_principal_if_valid(100, "rocinante-console", "repo-analyzer")
        .expect("valid claims");
    assert_eq!(principal.user, "alice");
    assert!(principal_is_admin(&principal));

    assert!(claims
        .clone()
        .into_principal_if_valid(101, "rocinante-console", "repo-analyzer")
        .is_none());
    assert!(claims
        .clone()
        .into_principal_if_valid(100, "other-issuer", "repo-analyzer")
        .is_none());
    assert!(claims
        .clone()
        .into_principal_if_valid(100, "rocinante-console", "other-audience")
        .is_none());

    let mut blank_user = claims;
    blank_user.user = "  ".to_string();
    assert!(blank_user
        .into_principal_if_valid(100, "rocinante-console", "repo-analyzer")
        .is_none());
}

#[test]
fn host_agnostic_budget_guard_preserves_pause_and_limit_boundaries() {
    let guard = BudgetGuard;

    assert_eq!(guard.evaluate(79, false), BudgetDecision::Continue);
    assert_eq!(guard.evaluate(80, false), BudgetDecision::ReportOnly);
    assert_eq!(
        guard.evaluate(100, false),
        BudgetDecision::Stop(StopReason::BudgetExceeded)
    );
    assert_eq!(
        guard.evaluate(1, true),
        BudgetDecision::Stop(StopReason::Paused)
    );
}

#[test]
fn host_agnostic_triage_keeps_thresholds_order_and_state_updates() {
    let report = build_triage_report(TriageInput {
        report_only: true,
        findings: vec![
            TriageFinding {
                title: "watch-low".into(),
                score: 0.40,
                details: "w".into(),
            },
            TriageFinding {
                title: "urgent".into(),
                score: 0.90,
                details: "h".into(),
            },
            TriageFinding {
                title: "noise".into(),
                score: 0.20,
                details: "n".into(),
            },
        ],
        state_updates: vec!["phase remains active".into()],
    });

    assert_eq!(report.high_priority[0].title, "urgent");
    assert_eq!(report.watch[0].title, "watch-low");
    assert_eq!(report.noise[0].title, "noise");
    assert!(report.body.starts_with("Report-only mode\n"));
    assert!(report.body.contains("- phase remains active\n"));
}

#[test]
fn host_agnostic_verifier_requires_one_issue_and_test_evidence() {
    let rejected = review_fix_attempt(VerifierInput {
        issues: vec!["one issue".into()],
        test_evidence: Vec::new(),
        attempts: 0,
        attempt_cap: 2,
    });
    assert!(!rejected.can_approve);

    let approved = review_fix_attempt(VerifierInput {
        issues: vec!["one issue".into()],
        test_evidence: vec!["focused test passed".into()],
        attempts: 1,
        attempt_cap: 2,
    });
    assert!(approved.can_approve);
}

#[test]
fn host_agnostic_fix_proposal_escalates_with_full_context_at_retry_cap() {
    let submission = FixProposalSubmission {
        proposal_id: "fix-7".into(),
        issue_ids: vec!["BI-048".into()],
        remediation: "extract contract".into(),
        retry_count: 2,
        context: vec!["prior failure".into()],
    };
    let evaluation = evaluate_fix_proposal(&submission, &FixProposalContract::default());

    assert!(evaluation.one_problem);
    assert!(evaluation.retry_cap_reached);
    assert!(evaluation.full_context_required);
    assert_eq!(evaluation.escalation_context, vec!["prior failure"]);
}

#[test]
fn host_agnostic_risk_contract_clamps_signals_and_preserves_candidate_identity() {
    let candidate = PrCandidate {
        pr_id: "pr-42".to_string(),
        repo_name: "acme/service".to_string(),
        author: "alice".to_string(),
        release: "2026.09".to_string(),
        file_risk: 2.0,
        author_velocity: -1.0,
        approval_fidelity: 0.0,
        files: Vec::new(),
        circuit_breaker_triggered: false,
    };

    let evaluation = evaluate_pr_risk(&candidate, &PrRiskSchema::default());

    assert_eq!(evaluation.decision, PrRiskDecision::Block);
    assert_eq!(evaluation.risk_score, 1.0);
    assert_eq!(evaluation.pr_id, "pr-42");
    assert_eq!(evaluation.repo_name, "acme/service");
    assert_eq!(evaluation.schema_version, "risk-v1");
}
