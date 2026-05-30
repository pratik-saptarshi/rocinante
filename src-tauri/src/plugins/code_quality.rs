use crate::errors::AnalyzerError;
use crate::plugins::BeadPlugin;
use crate::types::{AnalysisInput, AnalysisMetric};
use std::fs;

pub struct CodeQualityPlugin;

impl BeadPlugin for CodeQualityPlugin {
    fn name(&self) -> &'static str {
        "code_quality"
    }

    fn run(&self, input: &AnalysisInput) -> Result<Vec<AnalysisMetric>, AnalyzerError> {
        let mut todo_hits = 0.0;
        for entry in walkdir::WalkDir::new(&input.repo.path).into_iter().flatten() {
            if !entry.file_type().is_file() {
                continue;
            }
            if let Ok(contents) = fs::read_to_string(entry.path()) {
                todo_hits += contents.matches("TODO").count() as f64;
            }
        }

        Ok(vec![AnalysisMetric {
            plugin: self.name().to_string(),
            key: "todo_markers".to_string(),
            value: todo_hits,
            details: "Lower values indicate better lint hygiene".to_string(),
        }])
    }
}
