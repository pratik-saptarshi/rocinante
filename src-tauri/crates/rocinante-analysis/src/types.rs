use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub use rocinante_core::risk_contract::{PrCandidate, PrFileSignal};
pub use rocinante_core::types::{CommitterScore, Principal, ScoringWeights};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoTarget {
    pub name: String,
    pub path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisInput {
    pub repo: RepoTarget,
    pub changed_files: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisMetric {
    pub plugin: String,
    pub key: String,
    pub value: f64,
    pub details: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RepositoryMetric {
    pub repo_name: String,
    pub release: String,
    pub plugin: String,
    pub key: String,
    pub value: f64,
    pub details: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisRecord {
    pub repo_name: String,
    pub release: String,
    pub metrics: Vec<AnalysisMetric>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminQuery {
    pub name: Option<String>,
    pub release: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryPoint {
    pub plugin: String,
    pub metric_key: String,
    pub metric_value: f64,
    pub details: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitIngestionEvent {
    pub commit_id: String,
    pub repo_name: String,
    pub release: String,
    pub committer: String,
    pub telemetry: Vec<TelemetryPoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PrRanking {
    pub pr_id: String,
    pub repo_name: String,
    pub author: String,
    pub rank_score: f64,
    pub rationale: String,
    pub highest_risk_file: Option<String>,
    pub used_fallback_risk: bool,
    pub circuit_breaker_triggered: bool,
}
