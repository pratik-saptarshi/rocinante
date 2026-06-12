use repo_analyzer_core::team_policies::{
    resolve_team_weights, TeamPolicyCatalog, TeamPolicyProfile,
};
use repo_analyzer_core::types::ScoringWeights;

#[test]
fn resolves_team_specific_weights_when_profile_exists() {
    let catalog = TeamPolicyCatalog {
        default_weights: ScoringWeights::default(),
        profiles: vec![TeamPolicyProfile {
            team: "platform".to_string(),
            weights: ScoringWeights {
                version: "platform-v1".to_string(),
                pr_approval_weight: 0.45,
                ..ScoringWeights::default()
            },
        }],
    };

    let weights = resolve_team_weights(&catalog, Some("platform"));

    assert_eq!(weights.version, "platform-v1");
    assert_eq!(weights.pr_approval_weight, 0.45);
}

#[test]
fn falls_back_to_default_weights_for_unknown_team() {
    let catalog = TeamPolicyCatalog {
        default_weights: ScoringWeights {
            version: "default-v2".to_string(),
            pr_velocity_weight: 0.35,
            ..ScoringWeights::default()
        },
        profiles: vec![],
    };

    let weights = resolve_team_weights(&catalog, Some("security"));

    assert_eq!(weights.version, "default-v2");
    assert_eq!(weights.pr_velocity_weight, 0.35);
}
