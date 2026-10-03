//! Host-neutral persistence adapters shared by the Tauri and native desktop hosts.

pub mod errors {
    pub use rocinante_analysis::errors::*;
}

pub mod auth {
    pub use rocinante_analysis::auth::*;
}

pub mod types {
    pub use rocinante_analysis::types::*;
}

pub mod plugins {
    pub mod sanitizer {
        pub use rocinante_analysis::plugins::sanitizer::*;
    }
}

pub mod telemetry {
    pub use rocinante_analysis::telemetry::*;
}

pub mod risk_contract {
    pub use rocinante_core::risk_contract::*;
}

pub mod admin;
pub mod scoring;
pub mod storage;

use errors::AnalyzerError;
use serde::Deserialize;
use storage::BaselineStore;

pub const ADMIN_BRIDGE_COMMANDS: [&str; 8] = [
    "ingest_event",
    "promote_lifecycle",
    "query_aggregates",
    "committer_scores",
    "rank_prs",
    "query_release_baseline",
    "reseed_release_baseline",
    "update_scoring_weights",
];

/// Return the exact default paths used by the existing Tauri host. Both hosts
/// can override these locations with the same environment variables.
pub fn default_analytics_store_paths() -> (String, String) {
    let kv = std::env::var("ROCINANTE_KV_PATH").unwrap_or_else(|_| "telemetry-kv".into());
    let columnar =
        std::env::var("ROCINANTE_COLUMNAR_PATH").unwrap_or_else(|_| "analytics.duckdb".into());
    (kv, columnar)
}

pub fn default_scoring_paths() -> (String, String) {
    let weights = std::env::var("ROCINANTE_SCORING_WEIGHTS_PATH")
        .unwrap_or_else(|_| "scoring-weights.json".into());
    let audit = std::env::var("ROCINANTE_SCORING_AUDIT_PATH")
        .unwrap_or_else(|_| "scoring-audit.jsonl".into());
    (weights, audit)
}

pub fn default_ingestion_backend_config() -> storage::IngestionBackendConfig {
    storage::IngestionBackendConfig {
        kind: storage::IngestionBackendKind::BadgerSidecar,
        strict_badger_required: true,
        endpoint: Some(
            std::env::var("ROCINANTE_BADGER_SIDECAR_ENDPOINT")
                .unwrap_or_else(|_| "unix:///var/run/badger.sock".into()),
        ),
    }
}

/// Run one command from the companion admin bridge through the shared service
/// layer. JSON payloads omit the token; the token is supplied separately.
pub fn execute_admin_bridge_command(
    command: &str,
    token: &str,
    payload: serde_json::Value,
    kv_path: &str,
    columnar_path: &str,
    weights_path: &str,
    audit_path: &str,
) -> Result<serde_json::Value, AnalyzerError> {
    if !ADMIN_BRIDGE_COMMANDS.contains(&command) {
        return Err(AnalyzerError::Db(format!(
            "unknown admin command: {command}"
        )));
    }
    auth::require_configured_token_secret()?;

    let value = match command {
        "ingest_event" => {
            #[derive(Deserialize)]
            struct Request {
                event: types::CommitIngestionEvent,
            }
            let request: Request = decode_payload(payload)?;
            admin::ingest_event(
                token,
                kv_path,
                columnar_path,
                request.event,
                &default_ingestion_backend_config(),
            )?;
            serde_json::Value::Null
        }
        "promote_lifecycle" => {
            serde_json::json!(
                admin::promote_lifecycle(token, kv_path, columnar_path)?.promoted_events
            )
        }
        "query_aggregates" => {
            let query: types::AdminQuery = decode_payload(payload)?;
            encode_payload(admin::query_aggregates(
                token,
                kv_path,
                columnar_path,
                query,
            )?)?
        }
        "committer_scores" => {
            let query: types::AdminQuery = decode_payload(payload)?;
            encode_payload(admin::committer_scores(
                token,
                kv_path,
                columnar_path,
                query,
                weights_path,
            )?)?
        }
        "rank_prs" => {
            #[derive(Deserialize)]
            struct Request {
                prs: Vec<types::PrCandidate>,
            }
            let request: Request = decode_payload(payload)?;
            encode_payload(admin::rank_prs(
                token,
                kv_path,
                columnar_path,
                request.prs,
                weights_path,
            )?)?
        }
        "query_release_baseline" => {
            #[derive(Deserialize)]
            struct Request {
                #[serde(alias = "repoName")]
                repo_name: String,
            }
            let request: Request = decode_payload(payload)?;
            encode_payload(admin::query_release_baseline(
                token,
                kv_path,
                columnar_path,
                &request.repo_name,
            )?)?
        }
        "reseed_release_baseline" => {
            #[derive(Deserialize)]
            struct Request {
                #[serde(alias = "repoName")]
                repo_name: String,
                #[serde(alias = "baselineComplexity")]
                baseline_complexity: f64,
            }
            let request: Request = decode_payload(payload)?;
            encode_payload(admin::reseed_release_baseline(
                token,
                kv_path,
                columnar_path,
                &request.repo_name,
                request.baseline_complexity,
            )?)?
        }
        "update_scoring_weights" => {
            #[derive(Deserialize)]
            struct Request {
                weights: types::ScoringWeights,
            }
            let request: Request = decode_payload(payload)?;
            admin::update_scoring_weights(token, weights_path, audit_path, request.weights)?;
            serde_json::Value::Null
        }
        _ => unreachable!("validated admin command"),
    };
    Ok(value)
}

fn decode_payload<T: for<'de> Deserialize<'de>>(
    payload: serde_json::Value,
) -> Result<T, AnalyzerError> {
    serde_json::from_value(payload).map_err(|error| AnalyzerError::Db(error.to_string()))
}

fn encode_payload<T: serde::Serialize>(value: T) -> Result<serde_json::Value, AnalyzerError> {
    serde_json::to_value(value).map_err(|error| AnalyzerError::Db(error.to_string()))
}

pub fn query_release_baseline(
    token: &str,
    kv_path: &str,
    columnar_path: &str,
    repo_name: &str,
) -> Result<Option<f64>, AnalyzerError> {
    auth::require_configured_token_secret()?;
    let principal = auth::decode_principal(token)?;
    auth::require_admin(&principal)?;
    let store = BaselineStore::open(kv_path, columnar_path)?;
    rocinante_core::baseline::query_release_baseline(&principal, &store, repo_name).map_err(
        |error| match error {
            rocinante_core::baseline::ReleaseBaselineError::PermissionDenied(user) => {
                AnalyzerError::PermissionDenied(user)
            }
            rocinante_core::baseline::ReleaseBaselineError::Repository(error) => error,
        },
    )
}

pub fn reseed_release_baseline(
    token: &str,
    kv_path: &str,
    columnar_path: &str,
    repo_name: &str,
    baseline_complexity: f64,
) -> Result<f64, AnalyzerError> {
    auth::require_configured_token_secret()?;
    let principal = auth::decode_principal(token)?;
    auth::require_admin(&principal)?;
    let store = BaselineStore::open(kv_path, columnar_path)?;
    rocinante_core::baseline::reseed_release_baseline(
        &principal,
        &store,
        repo_name,
        baseline_complexity,
    )
    .map_err(|error| match error {
        rocinante_core::baseline::ReleaseBaselineError::PermissionDenied(user) => {
            AnalyzerError::PermissionDenied(user)
        }
        rocinante_core::baseline::ReleaseBaselineError::Repository(error) => error,
    })
}

#[cfg(test)]
mod tests {
    use super::storage::BaselineStore;

    #[cfg(feature = "analytics")]
    #[test]
    fn shared_baseline_adapter_roundtrips() {
        let dir = tempfile::tempdir().expect("temporary storage directory");
        let kv = dir.path().join("kv");
        let columnar = dir.path().join("analytics.duckdb");
        let store = BaselineStore::open(
            kv.to_str().expect("UTF-8 temporary path"),
            columnar.to_str().expect("UTF-8 temporary path"),
        )
        .expect("open shared baseline store");

        assert_eq!(store.read_release_baseline("repo-a").expect("read"), None);
        assert_eq!(
            store
                .reseed_release_baseline("repo-a", 16.25)
                .expect("reseed"),
            16.25
        );
        assert_eq!(
            store.read_release_baseline("repo-a").expect("read"),
            Some(16.25)
        );
    }
}
