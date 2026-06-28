use crate::{admin, risk_contract::PrRiskEvaluation, types::PrCandidate};

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
    admin::query_release_baseline(&token, &kv_path, &col_path, &repo_name).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn reseed_release_baseline(
    token: String,
    kv_path: String,
    col_path: String,
    repo_name: String,
    baseline_complexity: f64,
) -> Result<f64, String> {
    admin::reseed_release_baseline(&token, &kv_path, &col_path, &repo_name, baseline_complexity)
        .map_err(|e| e.to_string())
}
