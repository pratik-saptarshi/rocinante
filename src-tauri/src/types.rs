use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoTarget {
    pub name: String,
    pub path: String,
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
pub struct Principal {
    pub user: String,
    pub roles: Vec<String>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitterScore {
    pub committer: String,
    pub score: f64,
    pub complexity_component: f64,
    pub coverage_component: f64,
    pub churn_component: f64,
    pub pipeline_component: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PrCandidate {
    pub pr_id: String,
    pub repo_name: String,
    pub author: String,
    pub release: String,
    pub file_risk: f64,
    pub author_velocity: f64,
    pub approval_fidelity: f64,
    #[serde(default)]
    pub files: Vec<PrFileSignal>,
    #[serde(default)]
    pub circuit_breaker_triggered: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PrFileSignal {
    pub path: String,
    pub risk: f64,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoringWeights {
    pub version: String,
    pub complexity_weight: f64,
    pub coverage_weight: f64,
    pub churn_weight: f64,
    pub pipeline_weight: f64,
    pub pr_file_risk_weight: f64,
    pub pr_velocity_weight: f64,
    pub pr_approval_weight: f64,
}

impl Default for ScoringWeights {
    fn default() -> Self {
        Self {
            version: "v1".to_string(),
            complexity_weight: 0.30,
            coverage_weight: 0.25,
            churn_weight: 0.20,
            pipeline_weight: 0.25,
            pr_file_risk_weight: 0.50,
            pr_velocity_weight: 0.20,
            pr_approval_weight: 0.30,
        }
    }
}
