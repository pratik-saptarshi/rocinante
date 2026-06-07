use repo_analyzer_core::types::{CommitIngestionEvent, ScoringWeights};

fn event(id: &str) -> CommitIngestionEvent {
    CommitIngestionEvent {
        commit_id: id.to_string(),
        repo_name: "repo".to_string(),
        release: "v1".to_string(),
        committer: "dev".to_string(),
        telemetry: vec![],
    }
}

#[test]
fn ci_strict_badger_rejects_inproc_fallback_endpoint() {
    use repo_analyzer_core::storage::{IngestionBackendConfig, IngestionBackendKind};

    let backend = IngestionBackendConfig {
        kind: IngestionBackendKind::BadgerSidecar,
        strict_badger_required: true,
        endpoint: Some("inproc://dev-sidecar".to_string()),
    };

    let err = backend.validate().expect_err("strict production mode must reject inproc fallback");
    assert!(err.to_string().contains("inproc fallback is not allowed in strict mode"));
}

#[test]
fn ci_sidecar_request_serializes_commit_event_with_shard_key() {
    use repo_analyzer_core::badger_sidecar::BadgerSidecarRequest;

    let req = BadgerSidecarRequest::from_event(&event("abc123"), 16);

    assert!(req.key.starts_with("evt:shard:"));
    assert_eq!(req.event.commit_id, "abc123");
}

#[test]
fn producer_burst_distributes_events_across_sharded_prefixes() {
    use repo_analyzer_core::badger_sidecar::BadgerSidecarRequest;
    use std::collections::BTreeSet;

    let prefixes: BTreeSet<String> = (0..64)
        .map(|i| BadgerSidecarRequest::from_event(&event(&format!("commit-{i}")), 16).key)
        .map(|key| key.split(':').take(4).collect::<Vec<_>>().join(":"))
        .collect();

    assert!(prefixes.len() > 1, "burst writes must not share one global prefix");
}

#[test]
fn compliance_prune_removes_expired_raw_events_only() {
    use repo_analyzer_core::retention::RawRetentionDecision;

    let decision = RawRetentionDecision::new(60);

    assert!(decision.should_prune(100, 161));
    assert!(!decision.should_prune(100, 160));
}

#[test]
fn dashboard_query_uses_materialized_immutable_snapshot() {
    use repo_analyzer_core::snapshots::AnalyticsSnapshotManager;

    let tmp = tempfile::tempdir().unwrap();
    let source = tmp.path().join("analytics.duckdb");
    std::fs::write(&source, b"duckdb-bytes").unwrap();

    let snapshot = AnalyticsSnapshotManager::new(tmp.path())
        .materialize(source.to_str().unwrap(), 42)
        .unwrap();

    assert!(snapshot.immutable);
    assert!(snapshot.path.contains("snapshot-42.duckdb"));
    assert_eq!(std::fs::read(&snapshot.path).unwrap(), b"duckdb-bytes");
}

#[test]
fn governance_rejects_tampered_scoring_weights() {
    use repo_analyzer_core::scoring::{sign_weights, verify_signed_weights};

    let weights = ScoringWeights::default();
    let signed = sign_weights(&weights).unwrap();
    let mut tampered = signed.clone();
    tampered.weights.complexity_weight = 0.99;

    let err = verify_signed_weights(&tampered).expect_err("tampered config must fail verification");
    assert!(err.to_string().contains("signature verification failed"));
}

#[test]
fn team_policy_resolves_explicit_scoring_profile() {
    use repo_analyzer_core::policy::PolicyProfiles;

    let mut profiles = PolicyProfiles::default();
    let mut weights = ScoringWeights::default();
    weights.version = "security-team-v1".to_string();
    profiles.insert("security".to_string(), weights);

    let resolved = profiles.resolve("security").unwrap();

    assert_eq!(resolved.version, "security-team-v1");
}

#[test]
fn ast_cache_reuses_unchanged_file_fingerprint() {
    use repo_analyzer_core::ast_cache::AstCache;

    let mut cache = AstCache::default();

    let first = cache.fingerprint("src/lib.rs", "fn main() {}");
    let second = cache.fingerprint("src/lib.rs", "fn main() {}");

    assert_eq!(first, second);
    assert_eq!(cache.hits(), 1);
}

#[test]
fn ast_bead_emits_deterministic_language_metric() {
    use repo_analyzer_core::plugins::ast_metrics::AstMetricsPlugin;
    use repo_analyzer_core::plugins::BeadPlugin;
    use repo_analyzer_core::types::{AnalysisInput, RepoTarget};

    let plugin = AstMetricsPlugin;
    let input = AnalysisInput {
        repo: RepoTarget { name: "repo".to_string(), path: ".".to_string() },
        changed_files: vec!["src-tauri/src/lib.rs".to_string()],
    };

    let metrics = plugin.run(&input).unwrap();

    assert!(metrics.iter().any(|m| m.key == "ast_changed_files"));
}

#[test]
fn operator_sees_queue_depth_and_promotion_lag() {
    use repo_analyzer_core::observability::JobMetrics;

    let mut metrics = JobMetrics::default();
    metrics.record_enqueue(3);
    metrics.record_promotion_lag_ms(250);

    assert_eq!(metrics.queue_depth, 3);
    assert_eq!(metrics.promotion_lag_ms, 250);
}

#[test]
fn enterprise_git_provider_accepts_only_onprem_schemes() {
    use repo_analyzer_core::onprem::{GitProviderKind, ProviderEndpoint};

    assert!(ProviderEndpoint::new(GitProviderKind::GitHubEnterprise, "https://ghe.local/api").is_ok());
    assert!(ProviderEndpoint::new(GitProviderKind::GitLabSelfManaged, "https://gitlab.local/api").is_ok());
    assert!(ProviderEndpoint::new(GitProviderKind::BitbucketServer, "https://bitbucket.local/rest").is_ok());
    assert!(ProviderEndpoint::new(GitProviderKind::GitHubEnterprise, "https://api.github.com").is_err());
}

#[test]
fn ldap_group_cache_maps_admin_role_deterministically() {
    use repo_analyzer_core::onprem::DirectoryGroupCache;

    let mut cache = DirectoryGroupCache::default();
    cache.insert("alice", vec!["repo-admins".to_string()]);

    let roles = cache.roles_for("alice");

    assert_eq!(roles, vec!["admin".to_string()]);
}

#[test]
fn migration_bulk_import_deduplicates_commit_ids() {
    use repo_analyzer_core::import::BulkImportPlan;

    let plan = BulkImportPlan::from_events(vec![event("a"), event("a"), event("b")]);

    assert_eq!(plan.unique_events.len(), 2);
    assert_eq!(plan.duplicates, 1);
}

#[test]
fn admin_ui_exposes_all_maturity_commands() {
    let html = std::fs::read_to_string("../ui/index.html").unwrap();

    for id in [
        "ingestEvent",
        "promoteLifecycle",
        "queryAggregates",
        "committerScores",
        "rankPrs",
        "updateScoringWeights",
        "baselineReseed",
    ] {
        assert!(html.contains(id), "missing UI control id {id}");
    }
}
