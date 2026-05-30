use crate::errors::AnalyzerError;
use crate::git::git_stdout;
use crate::plugins::BeadPlugin;
use crate::types::{AnalysisInput, AnalysisMetric};

pub struct PrApprovalPlugin;

impl BeadPlugin for PrApprovalPlugin {
    fn name(&self) -> &'static str {
        "pr_approval"
    }

    fn run(&self, input: &AnalysisInput) -> Result<Vec<AnalysisMetric>, AnalyzerError> {
        let message_blob = git_stdout(
            &input.repo.path,
            &["log", "--since=30.days", "--pretty=%B----END----"],
        )
        .unwrap_or_default();

        let messages: Vec<&str> = message_blob.split("----END----").collect();
        let total = messages.iter().filter(|m| !m.trim().is_empty()).count() as f64;
        let approved = messages
            .iter()
            .filter(|m| m.contains("Approved-by:"))
            .count() as f64;

        let fidelity = if total > 0.0 { approved / total } else { 1.0 };

        Ok(vec![AnalysisMetric {
            plugin: self.name().to_string(),
            key: "approval_fidelity_30d".to_string(),
            value: fidelity,
            details: "Share of commits in 30 days containing Approved-by trailer".to_string(),
        }])
    }
}
