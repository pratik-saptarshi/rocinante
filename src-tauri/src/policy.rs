use crate::errors::AnalyzerError;
use crate::types::ScoringWeights;
use std::collections::BTreeMap;

#[derive(Debug, Default, Clone)]
pub struct PolicyProfiles {
    profiles: BTreeMap<String, ScoringWeights>,
}

impl PolicyProfiles {
    pub fn insert(&mut self, team: String, weights: ScoringWeights) {
        self.profiles.insert(team, weights);
    }

    pub fn resolve(&self, team: &str) -> Result<ScoringWeights, AnalyzerError> {
        self.profiles
            .get(team)
            .cloned()
            .ok_or_else(|| AnalyzerError::Db(format!("missing policy profile: {team}")))
    }
}
