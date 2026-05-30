pub mod code_quality;
pub mod complexity;
pub mod pr_approval;
pub mod sanitizer;
pub mod velocity;

use crate::errors::AnalyzerError;
use crate::types::{AnalysisInput, AnalysisMetric};

pub trait BeadPlugin: Send + Sync {
    fn name(&self) -> &'static str;
    fn run(&self, input: &AnalysisInput) -> Result<Vec<AnalysisMetric>, AnalyzerError>;
}
