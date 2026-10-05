use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Principal {
    pub user: String,
    pub roles: Vec<String>,
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
