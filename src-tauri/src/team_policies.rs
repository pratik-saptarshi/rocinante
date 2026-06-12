use crate::types::ScoringWeights;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamPolicyProfile {
    pub team: String,
    pub weights: ScoringWeights,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TeamPolicyCatalog {
    pub default_weights: ScoringWeights,
    pub profiles: Vec<TeamPolicyProfile>,
}

pub fn resolve_team_weights(catalog: &TeamPolicyCatalog, team: Option<&str>) -> ScoringWeights {
    match team {
        Some(team_name) => catalog
            .profiles
            .iter()
            .find(|profile| profile.team == team_name)
            .map(|profile| profile.weights.clone())
            .unwrap_or(catalog.default_weights.clone()),
        None => catalog.default_weights.clone(),
    }
}
