use crate::auth::{decode_principal, require_admin};
use crate::engine::Pipeline;
use crate::errors::AnalyzerError;
use crate::git::discover_repositories;
use crate::scoring::{load_or_init_weights, update_weights_with_audit};
use crate::storage::{DualLayerStore, LifecycleStats};
use crate::telemetry::TelemetryStore;
use crate::types::{
    AdminQuery, AnalysisMetric, CommitIngestionEvent, CommitterScore, PrCandidate, PrRanking,
    ScoringWeights, TelemetryPoint,
};
use crate::storage::IngestionBackendConfig;

pub fn run_scan(root: &str, release: &str, db_path: &str) -> Result<usize, AnalyzerError> {
    let repos = discover_repositories(root);
    let pipeline = Pipeline::default();
    let store = TelemetryStore::open(db_path)?;

    for repo in &repos {
        let record = pipeline.analyze_repo(repo.clone(), release)?;
        store.insert_record(&record)?;
    }

    Ok(repos.len())
}

pub fn query_metrics(
    token: &str,
    query: AdminQuery,
    db_path: &str,
) -> Result<Vec<AnalysisMetric>, AnalyzerError> {
    let principal = decode_principal(token)?;
    require_admin(&principal)?;

    let store = TelemetryStore::open(db_path)?;
    store.query(&query)
}

pub fn ingest_event(
    token: &str,
    kv_path: &str,
    col_path: &str,
    event: CommitIngestionEvent,
    backend: &IngestionBackendConfig,
) -> Result<(), AnalyzerError> {
    let principal = decode_principal(token)?;
    require_admin(&principal)?;
    backend.validate()?;
    let store = DualLayerStore::open(kv_path, col_path)?;
    store.ingest_commit_event_with_backend(&event, backend)
}

pub fn promote_lifecycle(
    token: &str,
    kv_path: &str,
    col_path: &str,
) -> Result<LifecycleStats, AnalyzerError> {
    let principal = decode_principal(token)?;
    require_admin(&principal)?;
    let store = DualLayerStore::open(kv_path, col_path)?;
    store.promote_to_columnar()
}

pub fn query_aggregates(
    token: &str,
    kv_path: &str,
    col_path: &str,
    query: AdminQuery,
) -> Result<Vec<TelemetryPoint>, AnalyzerError> {
    let principal = decode_principal(token)?;
    require_admin(&principal)?;
    let store = DualLayerStore::open(kv_path, col_path)?;
    store.aggregate_by_query(&query)
}

pub fn committer_scores(
    token: &str,
    kv_path: &str,
    col_path: &str,
    query: AdminQuery,
    weights_path: &str,
) -> Result<Vec<CommitterScore>, AnalyzerError> {
    let principal = decode_principal(token)?;
    require_admin(&principal)?;
    let store = DualLayerStore::open(kv_path, col_path)?;
    let weights = load_or_init_weights(weights_path)?;
    store.compute_committer_scores(&query, &weights)
}

pub fn rank_prs(
    token: &str,
    kv_path: &str,
    col_path: &str,
    prs: Vec<PrCandidate>,
    weights_path: &str,
) -> Result<Vec<PrRanking>, AnalyzerError> {
    let principal = decode_principal(token)?;
    require_admin(&principal)?;
    let store = DualLayerStore::open(kv_path, col_path)?;
    let weights = load_or_init_weights(weights_path)?;
    store.rank_open_prs(&prs, &weights)
}

pub fn update_scoring_weights(
    token: &str,
    weights_path: &str,
    audit_path: &str,
    new_weights: ScoringWeights,
) -> Result<(), AnalyzerError> {
    let principal = decode_principal(token)?;
    require_admin(&principal)?;
    update_weights_with_audit(weights_path, audit_path, &principal.user, new_weights)
}
