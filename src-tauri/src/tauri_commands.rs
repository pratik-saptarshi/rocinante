use crate::admin;
use crate::auth::{decode_principal, require_admin};
use crate::risk_contract::PrRiskEvaluation;
use crate::storage::BaselineStore;
use crate::types::PrCandidate;

#[tauri::command]
pub fn evaluate_pr_risk(
    token: String,
    candidate: PrCandidate,
) -> Result<PrRiskEvaluation, String> {
    admin::evaluate_pr_risk(&token, candidate).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn query_release_baseline(
    token: String,
    kv_path: String,
    col_path: String,
    repo_name: String,
) -> Result<Option<f64>, String> {
    let store = BaselineStore::open(&kv_path, &col_path).map_err(|e| e.to_string())?;
    query_release_baseline_with_store(token, store, repo_name)
}

#[tauri::command]
pub fn reseed_release_baseline(
    token: String,
    kv_path: String,
    col_path: String,
    repo_name: String,
    baseline_complexity: f64,
) -> Result<f64, String> {
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
