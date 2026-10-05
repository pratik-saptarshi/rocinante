use crate::admin;
use crate::auth::{decode_principal, require_admin};
use crate::risk_contract::PrRiskEvaluation;
use crate::storage::BaselineStore;
use crate::types::PrCandidate;

/// Stable names and wire shapes from the retired host, retained as a
/// host-neutral migration contract while the native shell calls shared APIs.
pub const MIGRATED_COMMAND_CONTRACTS: [(&str, &str, &str); 11] = [
    (
        "run_scan",
        "payload: { token, root, release }",
        "TelemetryImportSummary",
    ),
    (
        "query_metrics",
        "token, optional name, optional release",
        "Vec<AnalysisMetric> (plugin, key, value, details)",
    ),
    ("ingest_event", "token, event", "unit / JSON null"),
    ("promote_lifecycle", "token", "promoted event count (usize)"),
    (
        "query_aggregates",
        "token, optional name, optional release",
        "Vec<TelemetryPoint>",
    ),
    (
        "committer_scores",
        "token, optional name, optional release",
        "Vec<CommitterScore>",
    ),
    ("rank_prs", "token, prs", "Vec<PrRanking>"),
    ("evaluate_pr_risk", "token, candidate", "PrRiskEvaluation"),
    ("query_release_baseline", "token, repoName", "optional f64"),
    (
        "reseed_release_baseline",
        "token, repoName, baselineComplexity",
        "f64",
    ),
    (
        "update_scoring_weights",
        "token, weights",
        "unit / JSON null",
    ),
];

fn authorize_baseline_access(token: &str) -> Result<(), String> {
    let principal = decode_principal(token).map_err(|e| e.to_string())?;
    require_admin(&principal).map_err(|e| e.to_string())
}

pub fn evaluate_pr_risk(token: String, candidate: PrCandidate) -> Result<PrRiskEvaluation, String> {
    admin::evaluate_pr_risk(&token, candidate).map_err(|e| e.to_string())
}

pub fn query_release_baseline(
    token: String,
    kv_path: String,
    col_path: String,
    repo_name: String,
) -> Result<Option<f64>, String> {
    authorize_baseline_access(&token)?;
    let store = BaselineStore::open(&kv_path, &col_path).map_err(|e| e.to_string())?;
    query_release_baseline_with_store(token, store, repo_name)
}

pub fn reseed_release_baseline(
    token: String,
    kv_path: String,
    col_path: String,
    repo_name: String,
    baseline_complexity: f64,
) -> Result<f64, String> {
    authorize_baseline_access(&token)?;
    let store = BaselineStore::open(&kv_path, &col_path).map_err(|e| e.to_string())?;
    reseed_release_baseline_with_store(token, store, repo_name, baseline_complexity)
}

pub fn query_release_baseline_with_store(
    token: String,
    store: BaselineStore,
    repo_name: String,
) -> Result<Option<f64>, String> {
    let principal = decode_principal(&token).map_err(|e| e.to_string())?;
    require_admin(&principal).map_err(|e| e.to_string())?;
    store
        .read_release_baseline(&repo_name)
        .map_err(|e| e.to_string())
}

pub fn reseed_release_baseline_with_store(
    token: String,
    store: BaselineStore,
    repo_name: String,
    baseline_complexity: f64,
) -> Result<f64, String> {
    let principal = decode_principal(&token).map_err(|e| e.to_string())?;
    require_admin(&principal).map_err(|e| e.to_string())?;
    store
        .reseed_release_baseline(&repo_name, baseline_complexity)
        .map_err(|e| e.to_string())
}
