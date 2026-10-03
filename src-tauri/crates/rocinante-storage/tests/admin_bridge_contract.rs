use rocinante_storage::ADMIN_BRIDGE_COMMANDS;

#[test]
fn admin_bridge_command_names_match_the_companion_contract() {
    assert_eq!(
        ADMIN_BRIDGE_COMMANDS,
        [
            "ingest_event",
            "promote_lifecycle",
            "query_aggregates",
            "committer_scores",
            "rank_prs",
            "query_release_baseline",
            "reseed_release_baseline",
            "update_scoring_weights",
        ]
    );
}

#[test]
fn unsupported_admin_command_is_rejected_before_auth_or_storage_access() {
    let error = rocinante_storage::execute_admin_bridge_command(
        "delete_everything",
        "",
        serde_json::json!({}),
        "/invalid/kv",
        "/invalid/columnar",
        "/invalid/weights",
        "/invalid/audit",
    )
    .expect_err("unknown command should be rejected");

    assert!(error.to_string().contains("unknown admin command"));
}

#[test]
fn companion_admin_bridge_payloads_use_shared_admin_services() {
    std::env::set_var(
        "RUNICIPAL_TOKEN_SECRET",
        "admin-bridge-test-secret-with-at-least-32-bytes",
    );
    std::env::set_var("ROCINANTE_BADGER_SIDECAR_ENDPOINT", "inproc://test-sidecar");
    let token = rocinante_storage::auth::issue_test_token("alice", &["admin"], 3600);
    let reader = rocinante_storage::auth::issue_test_token("bob", &["reader"], 3600);
    let directory = tempfile::tempdir().expect("temporary store directory");
    let kv = directory.path().join("kv");
    let columnar = directory.path().join("analytics.duckdb");
    let weights = directory.path().join("scoring-weights.json");
    let audit = directory.path().join("scoring-audit.jsonl");
    let path = |value: &std::path::Path| value.to_str().expect("UTF-8 temporary path").to_string();
    let kv_path = path(&kv);
    let columnar_path = path(&columnar);
    let weights_path = path(&weights);
    let audit_path = path(&audit);

    let result = rocinante_storage::execute_admin_bridge_command(
        "ingest_event",
        &token,
        serde_json::json!({"event":{"commit_id":"ui-bridge-001","repo_name":"repo-a","release":"v1.0.0","committer":"ui","telemetry":[{"plugin":"ui","metric_key":"bridge_probe","metric_value":1,"details":"admin bridge probe"}]}}),
        &kv_path,
        &columnar_path,
        &weights_path,
        &audit_path,
    )
    .expect("authorized ingestion");
    assert_eq!(result, serde_json::Value::Null);

    let result = rocinante_storage::execute_admin_bridge_command(
        "promote_lifecycle",
        &token,
        serde_json::json!({}),
        &kv_path,
        &columnar_path,
        &weights_path,
        &audit_path,
    )
    .expect("authorized promotion");
    assert_eq!(result, serde_json::json!(1));

    let result = rocinante_storage::execute_admin_bridge_command(
        "query_aggregates",
        &token,
        serde_json::json!({"name":"repo-a","release":"v1.0.0"}),
        &kv_path,
        &columnar_path,
        &weights_path,
        &audit_path,
    )
    .expect("authorized aggregate query");
    assert!(!result.as_array().expect("aggregate array").is_empty());

    let result = rocinante_storage::execute_admin_bridge_command(
        "committer_scores",
        &token,
        serde_json::json!({"name":"repo-a","release":"v1.0.0"}),
        &kv_path,
        &columnar_path,
        &weights_path,
        &audit_path,
    )
    .expect("authorized committer score query");
    assert!(!result.as_array().expect("score array").is_empty());

    let result = rocinante_storage::execute_admin_bridge_command(
        "rank_prs",
        &token,
        serde_json::json!({"prs":[{"pr_id":"pr-001","repo_name":"repo-a","author":"ui","release":"v1.0.0","file_risk":0.4,"author_velocity":0.6,"approval_fidelity":0.9,"files":[{"path":"src/ui-bridge.ts","risk":0.72}],"circuit_breaker_triggered":true}]}),
        &kv_path,
        &columnar_path,
        &weights_path,
        &audit_path,
    )
    .expect("authorized PR ranking");
    assert_eq!(result.as_array().expect("ranking array").len(), 1);

    let result = rocinante_storage::execute_admin_bridge_command(
        "update_scoring_weights",
        &token,
        serde_json::json!({"weights":{"version":"v1","complexity_weight":0.3,"coverage_weight":0.25,"churn_weight":0.2,"pipeline_weight":0.25,"pr_file_risk_weight":0.5,"pr_velocity_weight":0.2,"pr_approval_weight":0.3}}),
        &kv_path,
        &columnar_path,
        &weights_path,
        &audit_path,
    )
    .expect("authorized weights update");
    assert_eq!(result, serde_json::Value::Null);

    let result = rocinante_storage::execute_admin_bridge_command(
        "reseed_release_baseline",
        &token,
        serde_json::json!({"repoName":"repo-a","baselineComplexity":12.5}),
        &kv_path,
        &columnar_path,
        &weights_path,
        &audit_path,
    )
    .expect("authorized reseed");
    assert_eq!(result, serde_json::json!(12.5));

    let result = rocinante_storage::execute_admin_bridge_command(
        "query_release_baseline",
        &token,
        serde_json::json!({"repoName":"repo-a"}),
        &kv_path,
        &columnar_path,
        &weights_path,
        &audit_path,
    )
    .expect("authorized query");
    assert_eq!(result, serde_json::json!(12.5));

    let error = rocinante_storage::execute_admin_bridge_command(
        "query_release_baseline",
        &reader,
        serde_json::json!({"repoName":"repo-a"}),
        &kv_path,
        &columnar_path,
        &weights_path,
        &audit_path,
    )
    .expect_err("reader must be denied");
    assert!(error.to_string().contains("permission denied"));
}
