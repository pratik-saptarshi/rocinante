use repo_analyzer_core::admin;
use repo_analyzer_core::auth::issue_test_token;
use repo_analyzer_core::risk_contract::{PrRiskDecision, PrRiskSchema};
use repo_analyzer_core::storage::{IngestionBackendConfig, IngestionBackendKind};
use repo_analyzer_core::types::{
    AdminQuery, CommitIngestionEvent, PrCandidate, ScoringWeights, TelemetryPoint,
};
use tempfile::tempdir;

fn sample_event(id: &str) -> CommitIngestionEvent {
    CommitIngestionEvent {
        commit_id: id.to_string(),
        repo_name: "repo-a".to_string(),
        release: "v1.0.0".to_string(),
        committer: "alice".to_string(),
        telemetry: vec![
            TelemetryPoint {
                plugin: "complexity".to_string(),
                metric_key: "estimated_cyclomatic_complexity".to_string(),
                metric_value: 9.0,
                details: "ok".to_string(),
            },
            TelemetryPoint {
                plugin: "coverage".to_string(),
                metric_key: "coverage_delta".to_string(),
                metric_value: 3.0,
                details: "ok".to_string(),
            },
            TelemetryPoint {
                plugin: "churn".to_string(),
                metric_key: "churn_efficiency".to_string(),
                metric_value: 0.7,
                details: "ok".to_string(),
            },
            TelemetryPoint {
                plugin: "ci".to_string(),
                metric_key: "pipeline_success".to_string(),
                metric_value: 1.0,
                details: "ok".to_string(),
            },
        ],
    }
}

#[test]
fn admin_services_roundtrip_happy_path() {
    let dir = tempdir().expect("tmp");
    let kv = dir.path().join("kv");
    let col = dir.path().join("analytics.duckdb");
    let weights = dir.path().join("weights.json");
    let audit = dir.path().join("audit.jsonl");
    let backend = IngestionBackendConfig {
        kind: IngestionBackendKind::BadgerSidecar,
        strict_badger_required: true,
        endpoint: Some("inproc://badger".to_string()),
    };

    admin::ingest_event(
        &issue_test_token("alice", &["admin"], 3600),
        kv.to_str().expect("kv"),
        col.to_str().expect("col"),
        sample_event("c1"),
        &backend,
    )
    .expect("ingest");

    let promoted = admin::promote_lifecycle(
        &issue_test_token("alice", &["admin"], 3600),
        kv.to_str().expect("kv"),
        col.to_str().expect("col"),
    )
    .expect("promote");
    assert_eq!(promoted.promoted_events, 1);

    let aggregates = admin::query_aggregates(
        &issue_test_token("alice", &["admin"], 3600),
        kv.to_str().expect("kv"),
        col.to_str().expect("col"),
        AdminQuery {
            name: Some("repo-a".to_string()),
            release: Some("v1".to_string()),
        },
    )
    .expect("aggregates");
    assert!(!aggregates.is_empty());

    let scores = admin::committer_scores(
        &issue_test_token("alice", &["admin"], 3600),
        kv.to_str().expect("kv"),
        col.to_str().expect("col"),
        AdminQuery {
            name: Some("repo-a".to_string()),
            release: None,
        },
        weights.to_str().expect("weights"),
    )
    .expect("scores");
    assert!(!scores.is_empty());

    let prs = vec![PrCandidate {
        pr_id: "pr-1".to_string(),
        repo_name: "repo-a".to_string(),
        author: "alice".to_string(),
        release: "v1.0.0".to_string(),
        file_risk: 0.8,
        author_velocity: 0.5,
        approval_fidelity: 0.6,
    }];
    let ranked = admin::rank_prs(
        &issue_test_token("alice", &["admin"], 3600),
        kv.to_str().expect("kv"),
        col.to_str().expect("col"),
        prs,
        weights.to_str().expect("weights"),
    )
    .expect("ranked");
    assert_eq!(ranked.len(), 1);

    let new_weights = ScoringWeights {
        version: "v-next".to_string(),
        ..ScoringWeights::default()
    };
    admin::update_scoring_weights(
        &issue_test_token("alice", &["admin"], 3600),
        weights.to_str().expect("weights"),
        audit.to_str().expect("audit"),
        new_weights,
    )
    .expect("update weights");
}

#[test]
fn admin_services_reject_non_admin() {
    let dir = tempdir().expect("tmp");
    let kv = dir.path().join("kv");
    let col = dir.path().join("analytics.duckdb");
    let backend = IngestionBackendConfig {
        kind: IngestionBackendKind::BadgerSidecar,
        strict_badger_required: true,
        endpoint: Some("inproc://badger".to_string()),
    };

    let err = admin::ingest_event(
        &issue_test_token("bob", &["reader"], 3600),
        kv.to_str().expect("kv"),
        col.to_str().expect("col"),
        sample_event("c2"),
        &backend,
    )
    .expect_err("permission denied");
    assert!(err.to_string().contains("permission denied"));
}

#[test]
fn admin_services_analytics_commands_require_analytics_route() {
    let dir = tempdir().expect("tmp");
    let kv = dir.path().join("kv");
    let col = dir.path().join("analytics.duckdb");
    let weights = dir.path().join("weights.json");
    let backend = IngestionBackendConfig {
        kind: IngestionBackendKind::BadgerSidecar,
        strict_badger_required: true,
        endpoint: Some("inproc://badger".to_string()),
    };
    let token = issue_test_token("alice", &["admin"], 3600);

    admin::ingest_event(
        &token,
        kv.to_str().expect("kv"),
        col.to_str().expect("col"),
        sample_event("c3"),
        &backend,
    )
    .expect("ingest");

    admin::promote_lifecycle(&token, kv.to_str().expect("kv"), col.to_str().expect("col"))
        .expect("promote");

    let aggregates = admin::query_aggregates(
        &token,
        kv.to_str().expect("kv"),
        col.to_str().expect("col"),
        AdminQuery {
            name: Some("repo-a".to_string()),
            release: None,
        },
    )
    .expect("query aggregates");
    assert!(!aggregates.is_empty());

    let scores = admin::committer_scores(
        &token,
        kv.to_str().expect("kv"),
        col.to_str().expect("col"),
        AdminQuery {
            name: Some("repo-a".to_string()),
            release: None,
        },
        weights.to_str().expect("weights"),
    )
    .expect("committer scores");
    assert!(!scores.is_empty());

    let ranked = admin::rank_prs(
        &token,
        kv.to_str().expect("kv"),
        col.to_str().expect("col"),
        vec![PrCandidate {
            pr_id: "pr-1".to_string(),
            repo_name: "repo-a".to_string(),
            author: "alice".to_string(),
            release: "v1.0.0".to_string(),
            file_risk: 0.8,
            author_velocity: 0.5,
            approval_fidelity: 0.6,
        }],
        weights.to_str().expect("weights"),
    )
    .expect("rank prs");
    assert_eq!(ranked.len(), 1);
}

#[test]
fn admin_services_evaluate_pr_risk_using_the_default_contract() {
    let token = issue_test_token("alice", &["admin"], 3600);
    let candidate = PrCandidate {
        pr_id: "pr-9".to_string(),
        repo_name: "repo-a".to_string(),
        author: "alice".to_string(),
        release: "v1.0.0".to_string(),
        file_risk: 0.92,
        author_velocity: 0.35,
        approval_fidelity: 0.45,
    };

    let evaluation = admin::evaluate_pr_risk(&token, candidate).expect("evaluate pr risk");

    assert_eq!(evaluation.schema_version, "risk-v1");
    assert_eq!(evaluation.decision, PrRiskDecision::Block);
    assert!(evaluation
        .reason_codes
        .iter()
        .any(|reason| reason.starts_with("file_risk=")));
}

#[test]
fn admin_services_evaluate_pr_risk_with_injected_schema() {
    let token = issue_test_token("alice", &["admin"], 3600);
    let candidate = PrCandidate {
        pr_id: "pr-10".to_string(),
        repo_name: "repo-a".to_string(),
        author: "alice".to_string(),
        release: "v1.0.0".to_string(),
        file_risk: 0.92,
        author_velocity: 0.35,
        approval_fidelity: 0.45,
    };
    let schema = PrRiskSchema {
        version: "risk-custom".to_string(),
        file_risk_weight: 0.0,
        velocity_weight: 0.0,
        approval_weight: 0.0,
        review_threshold: 0.10,
        block_threshold: 0.20,
    };

    let evaluation =
        admin::evaluate_pr_risk_with_schema(&token, candidate, schema).expect("evaluate pr risk");

    assert_eq!(evaluation.schema_version, "risk-custom");
    assert_eq!(evaluation.decision, PrRiskDecision::Allow);
}

#[test]
fn admin_services_reject_non_admin_pr_risk_evaluation() {
    let token = issue_test_token("bob", &["reader"], 3600);
    let candidate = PrCandidate {
        pr_id: "pr-11".to_string(),
        repo_name: "repo-a".to_string(),
        author: "bob".to_string(),
        release: "v1.0.0".to_string(),
        file_risk: 0.4,
        author_velocity: 0.4,
        approval_fidelity: 0.4,
    };

    let err = admin::evaluate_pr_risk(&token, candidate).expect_err("non-admin rejected");

    assert!(matches!(
        err,
        repo_analyzer_core::errors::AnalyzerError::PermissionDenied(_)
    ));
}
