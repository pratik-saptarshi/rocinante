use crate::errors::AnalyzerError;
use crate::plugins::BeadPlugin;
use crate::types::{AnalysisInput, AnalysisMetric};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Mutex;

#[derive(Debug, Clone, Serialize)]
struct ParsedFileSummary {
    digest: String,
    language: String,
    node_estimate: f64,
}

#[derive(Debug, Default)]
pub struct ParserPlugin {
    cache: Mutex<HashMap<String, ParsedFileSummary>>,
}

impl ParserPlugin {
    pub fn new() -> Self {
        Self::default()
    }

    fn analyze_file(
        &self,
        repo_root: &Path,
        rel_path: &str,
        counts: &mut HashMap<String, f64>,
        hits: &mut f64,
        misses: &mut f64,
        node_estimate: &mut f64,
    ) {
        let full_path = repo_root.join(rel_path);
        let contents = match fs::read_to_string(&full_path) {
            Ok(contents) => contents,
            Err(_) => return,
        };
        let digest = digest_for(&contents);
        let language = language_for(rel_path);

        let summary = {
            let mut cache = self.cache.lock().expect("parser cache lock");
            match cache.get(rel_path) {
                Some(cached) if cached.digest == digest => {
                    *hits += 1.0;
                    cached.clone()
                }
                _ => {
                    *misses += 1.0;
                    let summary = ParsedFileSummary {
                        digest,
                        language: language.to_string(),
                        node_estimate: estimate_nodes(&contents, &language),
                    };
                    cache.insert(rel_path.to_string(), summary.clone());
                    summary
                }
            }
        };

        *counts
            .entry(format!("language_{}_files", summary.language))
            .or_insert(0.0) += 1.0;
        *node_estimate += summary.node_estimate;
    }
}

impl BeadPlugin for ParserPlugin {
    fn name(&self) -> &'static str {
        "parser"
    }

    fn run(&self, input: &AnalysisInput) -> Result<Vec<AnalysisMetric>, AnalyzerError> {
        let mut counts: HashMap<String, f64> = HashMap::new();
        let mut hits = 0.0;
        let mut misses = 0.0;
        let mut node_estimate = 0.0;
        let repo_root = Path::new(&input.repo.path);

        if input.changed_files.is_empty() {
            for entry in walkdir::WalkDir::new(repo_root).into_iter().flatten() {
                if !entry.file_type().is_file() {
                    continue;
                }
                let rel = entry
                    .path()
                    .strip_prefix(repo_root)
                    .unwrap_or(entry.path())
                    .to_string_lossy()
                    .to_string();
                self.analyze_file(
                    repo_root,
                    &rel,
                    &mut counts,
                    &mut hits,
                    &mut misses,
                    &mut node_estimate,
                );
            }
        } else {
            for rel in &input.changed_files {
                self.analyze_file(
                    repo_root,
                    rel,
                    &mut counts,
                    &mut hits,
                    &mut misses,
                    &mut node_estimate,
                );
            }
        }

        let mut metrics = vec![
            AnalysisMetric {
                plugin: self.name().to_string(),
                key: "ast_cache_hits".to_string(),
                value: hits,
                details: "Incremental cache hits for unchanged file digests".to_string(),
            },
            AnalysisMetric {
                plugin: self.name().to_string(),
                key: "ast_cache_misses".to_string(),
                value: misses,
                details: "Incremental cache misses for changed or uncached files".to_string(),
            },
            AnalysisMetric {
                plugin: self.name().to_string(),
                key: "ast_node_estimate".to_string(),
                value: node_estimate,
                details: "Language-aware structural node estimate across parsed files".to_string(),
            },
        ];

        let mut language_metrics: Vec<_> = counts.into_iter().collect();
        language_metrics.sort_by(|a, b| a.0.cmp(&b.0));
        metrics.extend(
            language_metrics
                .into_iter()
                .map(|(key, value)| AnalysisMetric {
                    plugin: self.name().to_string(),
                    key,
                    value,
                    details: "Language-aware file classification count".to_string(),
                }),
        );

        Ok(metrics)
    }
}

fn digest_for(contents: &str) -> String {
    let digest = Sha256::digest(contents.as_bytes());
    digest.iter().map(|byte| format!("{:02x}", byte)).collect()
}

fn language_for(path: &str) -> &'static str {
    match Path::new(path).extension().and_then(|ext| ext.to_str()) {
        Some("rs") => "rust",
        Some("ts") | Some("tsx") => "typescript",
        Some("js") | Some("jsx") => "javascript",
        Some("py") => "python",
        Some("md") => "markdown",
        _ => "unknown",
    }
}

fn estimate_nodes(contents: &str, language: &str) -> f64 {
    let lines = contents
        .lines()
        .filter(|line| !line.trim().is_empty())
        .count() as f64;
    let structural_tokens = match language {
        "rust" => [
            "fn ", "impl ", "match ", "if ", "for ", "while ", "struct ", "enum ",
        ]
        .iter()
        .map(|token| contents.matches(token).count() as f64)
        .sum(),
        "typescript" | "javascript" => [
            "function ",
            "const ",
            "let ",
            "=>",
            "if ",
            "for ",
            "while ",
            "class ",
        ]
        .iter()
        .map(|token| contents.matches(token).count() as f64)
        .sum(),
        "python" => ["def ", "class ", "if ", "for ", "while ", "elif "]
            .iter()
            .map(|token| contents.matches(token).count() as f64)
            .sum(),
        _ => contents.matches('{').count() as f64 + contents.matches('(').count() as f64,
    };
    (lines + structural_tokens).max(1.0)
}
