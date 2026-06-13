use crate::types::ScoringWeights;

#[derive(Debug, Clone)]
pub struct TeamPolicyProfile {
    pub team: String,
    pub label: String,
    pub approval_note: String,
    pub scoring_weights: ScoringWeights,
}

#[derive(Debug, Clone)]
pub struct TeamPolicyCatalog {
    default_profile: TeamPolicyProfile,
    profiles: Vec<TeamPolicyProfile>,
}

impl Default for TeamPolicyCatalog {
    fn default() -> Self {
        Self {
            default_profile: TeamPolicyProfile {
                team: "default".to_string(),
                label: "General Delivery".to_string(),
                approval_note: "Balanced approval fidelity for mixed-release teams".to_string(),
                scoring_weights: ScoringWeights::default(),
            },
            profiles: vec![
                TeamPolicyProfile {
                    team: "security".to_string(),
                    label: "Security Review".to_string(),
                    approval_note: "Bias toward stronger approval fidelity and pipeline stability"
                        .to_string(),
                    scoring_weights: ScoringWeights {
                        version: "policy-security".to_string(),
                        complexity_weight: 0.24,
                        coverage_weight: 0.18,
                        churn_weight: 0.16,
                        pipeline_weight: 0.22,
                        pr_file_risk_weight: 0.58,
                        pr_velocity_weight: 0.16,
                        pr_approval_weight: 0.34,
                    },
                },
                TeamPolicyProfile {
                    team: "frontend".to_string(),
                    label: "Frontend Delivery".to_string(),
                    approval_note: "Favor coverage and PR velocity when UX changes are routine"
                        .to_string(),
                    scoring_weights: ScoringWeights {
                        version: "policy-frontend".to_string(),
                        complexity_weight: 0.22,
                        coverage_weight: 0.32,
                        churn_weight: 0.16,
                        pipeline_weight: 0.20,
                        pr_file_risk_weight: 0.44,
                        pr_velocity_weight: 0.26,
                        pr_approval_weight: 0.30,
                    },
                },
                TeamPolicyProfile {
                    team: "platform".to_string(),
                    label: "Platform Reliability".to_string(),
                    approval_note: "Prioritize pipeline health and churn containment".to_string(),
                    scoring_weights: ScoringWeights {
                        version: "policy-platform".to_string(),
                        complexity_weight: 0.30,
                        coverage_weight: 0.22,
                        churn_weight: 0.24,
                        pipeline_weight: 0.24,
                        pr_file_risk_weight: 0.50,
                        pr_velocity_weight: 0.18,
                        pr_approval_weight: 0.32,
                    },
                },
            ],
        }
    }
}

impl TeamPolicyCatalog {
    pub fn resolve(&self, team: &str) -> TeamPolicyProfile {
        let normalized = team.trim().to_lowercase();
        self.profiles
            .iter()
            .find(|profile| profile.team == normalized)
            .cloned()
            .unwrap_or_else(|| self.default_profile.clone())
    }
}

pub fn resolve_team_policy(team: &str) -> TeamPolicyProfile {
    TeamPolicyCatalog::default().resolve(team)
}

pub fn resolve_team_scoring_weights(team: &str) -> ScoringWeights {
    resolve_team_policy(team).scoring_weights
}

#[cfg(test)]
mod tests {
    use super::{resolve_team_policy, resolve_team_scoring_weights, TeamPolicyCatalog};

    #[test]
    fn resolves_security_policy_profile() {
        let profile = resolve_team_policy("Security");
        assert_eq!(profile.team, "security");
        assert!(profile.scoring_weights.pr_approval_weight > 0.30);
        assert!(profile.approval_note.contains("approval fidelity"));
    }

    #[test]
    fn resolves_frontend_policy_profile() {
        let weights = resolve_team_scoring_weights("frontend");
        assert!(weights.coverage_weight > weights.churn_weight);
        assert_eq!(weights.version, "policy-frontend");
    }

    #[test]
    fn falls_back_to_default_profile_for_unknown_teams() {
        let catalog = TeamPolicyCatalog::default();
        let profile = catalog.resolve("unknown-team");
        assert_eq!(profile.team, "default");
        assert_eq!(profile.scoring_weights.version, "v1");
    }
}
