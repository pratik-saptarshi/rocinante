use crate::types::ScoringWeights;
use crate::{authorization::authorize_admin, authorization::AdminPrincipal, types::Principal};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BaselineScoreInput {
    pub complexity: Option<f64>,
    pub baseline_complexity: Option<f64>,
    pub coverage_delta: Option<f64>,
    pub churn_efficiency: Option<f64>,
    pub pipeline_success: Option<f64>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BaselineScoreComponents {
    pub complexity: f64,
    pub coverage: f64,
    pub churn: f64,
    pub pipeline: f64,
}

pub trait ReleaseBaselineRepository {
    type Error;

    fn read_release_baseline(&self, repo_name: &str) -> Result<Option<f64>, Self::Error>;
    fn reseed_release_baseline(
        &self,
        repo_name: &str,
        baseline_complexity: f64,
    ) -> Result<f64, Self::Error>;
}

#[derive(Debug, PartialEq)]
pub enum ReleaseBaselineError<E> {
    PermissionDenied(String),
    Repository(E),
}

pub fn query_release_baseline<R: ReleaseBaselineRepository>(
    principal: &Principal,
    repository: &R,
    repo_name: &str,
) -> Result<Option<f64>, ReleaseBaselineError<R::Error>> {
    let _admin: AdminPrincipal<'_> = authorize_admin(principal)
        .ok_or_else(|| ReleaseBaselineError::PermissionDenied(principal.user.clone()))?;
    repository
        .read_release_baseline(repo_name)
        .map_err(ReleaseBaselineError::Repository)
}

pub fn reseed_release_baseline<R: ReleaseBaselineRepository>(
    principal: &Principal,
    repository: &R,
    repo_name: &str,
    baseline_complexity: f64,
) -> Result<f64, ReleaseBaselineError<R::Error>> {
    let _admin: AdminPrincipal<'_> = authorize_admin(principal)
        .ok_or_else(|| ReleaseBaselineError::PermissionDenied(principal.user.clone()))?;
    repository
        .reseed_release_baseline(repo_name, baseline_complexity)
        .map_err(ReleaseBaselineError::Repository)
}

pub fn score_baseline_components(
    input: BaselineScoreInput,
    weights: &ScoringWeights,
) -> BaselineScoreComponents {
    let delta_complexity =
        input.complexity.unwrap_or(0.0) - input.baseline_complexity.unwrap_or(0.0);
    let complexity =
        (1.0 / (1.0 + delta_complexity.max(0.0))) * (weights.complexity_weight * 100.0);
    let coverage = input
        .coverage_delta
        .map(|value| (value.max(-20.0) + 20.0) / 40.0 * (weights.coverage_weight * 100.0))
        .unwrap_or(weights.coverage_weight * 50.0);
    let churn = input
        .churn_efficiency
        .map(|value| value.clamp(0.0, 1.0) * (weights.churn_weight * 100.0))
        .unwrap_or(weights.churn_weight * 50.0);
    let pipeline = input
        .pipeline_success
        .map(|value| value.clamp(0.0, 1.0) * (weights.pipeline_weight * 100.0))
        .unwrap_or(weights.pipeline_weight * 50.0);

    BaselineScoreComponents {
        complexity,
        coverage,
        churn,
        pipeline,
    }
}
