use crate::admin;
use crate::storage::{BaselineStore, IngestionBackendConfig};
use crate::types::{
    AdminQuery, AnalysisMetric, CommitIngestionEvent, CommitterScore, PrCandidate, PrRanking,
    ScoringWeights, TelemetryPoint,
};
use serde::Deserialize;
use std::sync::Mutex;

pub struct AppState {
    pub db_path: Mutex<String>,
    pub kv_path: Mutex<String>,
    pub columnar_path: Mutex<String>,
    pub weights_path: Mutex<String>,
    pub audit_path: Mutex<String>,
    pub ingestion_backend: Mutex<IngestionBackendConfig>,
}

#[derive(Deserialize)]
struct ScanPayload {
    root: String,
    release: String,
}

pub fn app_state() -> AppState {
    AppState {
        db_path: Mutex::new("telemetry.db".to_string()),
        kv_path: Mutex::new("telemetry-kv".to_string()),
        columnar_path: Mutex::new("analytics.duckdb".to_string()),
        weights_path: Mutex::new("scoring-weights.json".to_string()),
        audit_path: Mutex::new("scoring-audit.jsonl".to_string()),
        ingestion_backend: Mutex::new(IngestionBackendConfig {
            kind: crate::storage::IngestionBackendKind::BadgerSidecar,
            strict_badger_required: true,
            endpoint: Some("unix:///var/run/badger.sock".to_string()),
        }),
    }
}

pub fn release_baseline_paths(state: &AppState) -> Result<(String, String), String> {
    let kv = state.kv_path.lock().map_err(|e| e.to_string())?.clone();
    let col = state.columnar_path.lock().map_err(|e| e.to_string())?.clone();
    Ok((kv, col))
}

pub fn release_baseline_store(state: &AppState) -> Result<BaselineStore, String> {
    let (kv, col) = release_baseline_paths(state)?;
    BaselineStore::open(&kv, &col).map_err(|e| e.to_string())
}

pub fn build_app<R: tauri::Runtime>(
    builder: tauri::Builder<R>,
    state: AppState,
) -> tauri::Builder<R> {
    builder
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            run_scan,
            query_metrics,
            ingest_event,
            promote_lifecycle,
            query_aggregates,
            committer_scores,
            rank_prs,
            evaluate_pr_risk,
            query_release_baseline,
            reseed_release_baseline,
            update_scoring_weights
        ])
}

#[tauri::command]
fn run_scan(state: tauri::State<AppState>, payload: ScanPayload) -> Result<usize, String> {
    let db = state.db_path.lock().map_err(|e| e.to_string())?.clone();
    admin::run_scan(&payload.root, &payload.release, &db).map_err(|e| e.to_string())
}

#[tauri::command]
fn query_metrics(
    state: tauri::State<AppState>,
    token: String,
    name: Option<String>,
    release: Option<String>,
) -> Result<Vec<AnalysisMetric>, String> {
    let db = state.db_path.lock().map_err(|e| e.to_string())?.clone();
    admin::query_metrics(&token, AdminQuery { name, release }, &db).map_err(|e| e.to_string())
}

#[tauri::command]
fn ingest_event(
    state: tauri::State<AppState>,
    token: String,
    event: CommitIngestionEvent,
) -> Result<(), String> {
    let kv = state.kv_path.lock().map_err(|e| e.to_string())?.clone();
    let col = state
        .columnar_path
        .lock()
        .map_err(|e| e.to_string())?
        .clone();
    let backend = state
        .ingestion_backend
        .lock()
        .map_err(|e| e.to_string())?
        .clone();
    admin::ingest_event(&token, &kv, &col, event, &backend).map_err(|e| e.to_string())
}

#[tauri::command]
fn promote_lifecycle(state: tauri::State<AppState>, token: String) -> Result<usize, String> {
    let kv = state.kv_path.lock().map_err(|e| e.to_string())?.clone();
    let col = state
        .columnar_path
        .lock()
        .map_err(|e| e.to_string())?
        .clone();
    admin::promote_lifecycle(&token, &kv, &col)
        .map(|stats| stats.promoted_events)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn query_aggregates(
    state: tauri::State<AppState>,
    token: String,
    name: Option<String>,
    release: Option<String>,
) -> Result<Vec<TelemetryPoint>, String> {
    let kv = state.kv_path.lock().map_err(|e| e.to_string())?.clone();
    let col = state
        .columnar_path
        .lock()
        .map_err(|e| e.to_string())?
        .clone();
    admin::query_aggregates(&token, &kv, &col, AdminQuery { name, release })
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn committer_scores(
    state: tauri::State<AppState>,
    token: String,
    name: Option<String>,
    release: Option<String>,
) -> Result<Vec<CommitterScore>, String> {
    let kv = state.kv_path.lock().map_err(|e| e.to_string())?.clone();
    let col = state
        .columnar_path
        .lock()
        .map_err(|e| e.to_string())?
        .clone();
    let weights = state
        .weights_path
        .lock()
        .map_err(|e| e.to_string())?
        .clone();
    admin::committer_scores(&token, &kv, &col, AdminQuery { name, release }, &weights)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn rank_prs(
    state: tauri::State<AppState>,
    token: String,
    prs: Vec<PrCandidate>,
) -> Result<Vec<PrRanking>, String> {
    let kv = state.kv_path.lock().map_err(|e| e.to_string())?.clone();
    let col = state
        .columnar_path
        .lock()
        .map_err(|e| e.to_string())?
        .clone();
    let weights = state
        .weights_path
        .lock()
        .map_err(|e| e.to_string())?
        .clone();
    admin::rank_prs(&token, &kv, &col, prs, &weights).map_err(|e| e.to_string())
}

#[tauri::command]
fn evaluate_pr_risk(
    token: String,
    candidate: PrCandidate,
) -> Result<crate::risk_contract::PrRiskEvaluation, String> {
    crate::tauri_commands::evaluate_pr_risk(token, candidate)
}

#[tauri::command]
fn update_scoring_weights(
    state: tauri::State<AppState>,
    token: String,
    weights: ScoringWeights,
) -> Result<(), String> {
    let weights_path = state
        .weights_path
        .lock()
        .map_err(|e| e.to_string())?
        .clone();
    let audit_path = state.audit_path.lock().map_err(|e| e.to_string())?.clone();
    admin::update_scoring_weights(&token, &weights_path, &audit_path, weights)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn query_release_baseline(
    state: tauri::State<AppState>,
    token: String,
    repo_name: String,
) -> Result<Option<f64>, String> {
    let store = release_baseline_store(&state)?;
    crate::tauri_commands::query_release_baseline_with_store(token, store, repo_name)
}

#[tauri::command]
fn reseed_release_baseline(
    state: tauri::State<AppState>,
    token: String,
    repo_name: String,
    baseline_complexity: f64,
) -> Result<f64, String> {
    let store = release_baseline_store(&state)?;
    crate::tauri_commands::reseed_release_baseline_with_store(
        token,
        store,
        repo_name,
        baseline_complexity,
    )
}
