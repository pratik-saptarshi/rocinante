use crate::errors::AnalyzerError;
use crate::git::changed_files_since_tag;
use crate::plugins::code_quality::CodeQualityPlugin;
use crate::plugins::complexity::ComplexityPlugin;
use crate::plugins::parser::ParserPlugin;
use crate::plugins::pr_approval::PrApprovalPlugin;
use crate::plugins::sanitizer::{scrub_metric, MandatorySanitizerPlugin};
use crate::plugins::velocity::ContributionVelocityPlugin;
use crate::plugins::BeadPlugin;
use crate::types::{AnalysisInput, AnalysisRecord, RepoTarget};
use std::sync::Arc;

pub struct Pipeline {
    mandatory_pre: Arc<dyn BeadPlugin>,
    beads: Vec<Arc<dyn BeadPlugin>>,
}

impl Pipeline {
    pub fn register<P: BeadPlugin + 'static>(&mut self, plugin: P) {
        self.beads.push(Arc::new(plugin));
    }

    pub fn analyze_repo(
        &self,
        repo: RepoTarget,
        release: &str,
    ) -> Result<AnalysisRecord, AnalyzerError> {
        let changed_files = changed_files_since_tag(&repo.path, release).unwrap_or_default();
        let input = AnalysisInput {
            repo: repo.clone(),
            changed_files,
        };
        let mut metrics = Vec::new();

        // Mandatory, non-bypassable pre-processor bead.
        metrics.extend(self.mandatory_pre.run(&input)?);

        std::thread::scope(|scope| -> Result<(), AnalyzerError> {
            let mut handles = Vec::new();
            for bead in &self.beads {
                let bead = Arc::clone(bead);
                let local_input = input.clone();
                handles.push(scope.spawn(move || bead.run(&local_input)));
            }
            for handle in handles {
                metrics.extend(
                    handle
                        .join()
                        .map_err(|_| AnalyzerError::Io("bead thread panic".to_string()))??,
                );
            }
            Ok(())
        })?;

        if metrics.is_empty() {
            return Err(AnalyzerError::Io("no bead results".to_string()));
        }
        for metric in &mut metrics {
            scrub_metric(metric);
        }

        Ok(AnalysisRecord {
            repo_name: repo.name,
            release: release.to_string(),
            metrics,
        })
    }
}

impl Default for Pipeline {
    fn default() -> Self {
        let mut p = Self {
            mandatory_pre: Arc::new(MandatorySanitizerPlugin),
            beads: Vec::new(),
        };
        p.register(CodeQualityPlugin);
        p.register(ComplexityPlugin);
        p.register(ParserPlugin::new());
        p.register(ContributionVelocityPlugin);
        p.register(PrApprovalPlugin);
        p
    }
}
