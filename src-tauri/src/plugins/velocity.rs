use crate::errors::AnalyzerError;
use crate::git::git_stdout;
use crate::plugins::BeadPlugin;
use crate::types::{AnalysisInput, AnalysisMetric};

pub struct ContributionVelocityPlugin;

impl BeadPlugin for ContributionVelocityPlugin {
    fn name(&self) -> &'static str {
        "contribution_velocity"
    }

    fn run(&self, input: &AnalysisInput) -> Result<Vec<AnalysisMetric>, AnalyzerError> {
        let commits = git_stdout(
            &input.repo.path,
            &["rev-list", "--count", "--since=30.days", "HEAD"],
        )
        .unwrap_or_else(|_| "0".to_string())
        .parse::<f64>()
        .unwrap_or(0.0);

        let stat = git_stdout(
            &input.repo.path,
            &["log", "--since=30.days", "--numstat", "--pretty="],
        )
        .unwrap_or_default();
        let churn: f64 = stat
            .lines()
            .filter_map(|line| {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    let add = parts[0].parse::<f64>().ok().unwrap_or(0.0);
                    let del = parts[1].parse::<f64>().ok().unwrap_or(0.0);
                    Some(add + del)
                } else {
                    None
                }
            })
            .sum();

        Ok(vec![
            AnalysisMetric {
                plugin: self.name().to_string(),
                key: "commits_30d".to_string(),
                value: commits,
                details: "Total commits in trailing 30 days".to_string(),
            },
            AnalysisMetric {
                plugin: self.name().to_string(),
                key: "loc_churn_30d".to_string(),
                value: churn,
                details: "Sum of insertions and deletions over trailing 30 days".to_string(),
            },
        ])
    }
}
