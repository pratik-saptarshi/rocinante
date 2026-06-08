use crate::errors::AnalyzerError;
use crate::plugins::BeadPlugin;
use crate::types::{AnalysisInput, AnalysisMetric};
use std::fs;
use std::path::Path;

pub struct ComplexityPlugin;

impl BeadPlugin for ComplexityPlugin {
    fn name(&self) -> &'static str {
        "complexity"
    }

    fn run(&self, input: &AnalysisInput) -> Result<Vec<AnalysisMetric>, AnalyzerError> {
        let mut complexity = 1.0;
        let decision_tokens = ["if ", "match ", "for ", "while ", "&&", "||"];

        if !input.changed_files.is_empty() {
            for rel in &input.changed_files {
                let full = Path::new(&input.repo.path).join(rel);
                if let Ok(contents) = fs::read_to_string(full) {
                    for token in decision_tokens {
                        complexity += contents.matches(token).count() as f64;
                    }
                }
            }
        } else {
            for entry in walkdir::WalkDir::new(&input.repo.path)
                .into_iter()
                .flatten()
            {
                if !entry.file_type().is_file() {
                    continue;
                }
                if let Ok(contents) = fs::read_to_string(entry.path()) {
                    for token in decision_tokens {
                        complexity += contents.matches(token).count() as f64;
                    }
                }
            }
        }

        Ok(vec![AnalysisMetric {
            plugin: self.name().to_string(),
            key: "estimated_cyclomatic_complexity".to_string(),
            value: complexity,
            details: "Token-based estimate across repository files".to_string(),
        }])
    }
}
