use crate::errors::AnalyzerError;
use crate::plugins::BeadPlugin;
use crate::types::{AnalysisInput, AnalysisMetric};

pub struct AstMetricsPlugin;

impl BeadPlugin for AstMetricsPlugin {
    fn name(&self) -> &'static str {
        "ast_metrics"
    }

    fn run(&self, input: &AnalysisInput) -> Result<Vec<AnalysisMetric>, AnalyzerError> {
        Ok(vec![AnalysisMetric {
            plugin: self.name().to_string(),
            key: "ast_changed_files".to_string(),
            value: input.changed_files.len() as f64,
            details: "deterministic changed-file AST proxy".to_string(),
        }])
    }
}
