use repo_analyzer_core::team_policies::{
    resolve_team_policy, resolve_team_scoring_weights, TeamPolicyCatalog,
};

#[test]
fn resolves_team_specific_policy_profile() {
    let profile = resolve_team_policy("platform");

    assert_eq!(profile.team, "platform");
    assert_eq!(profile.label, "Platform Reliability");
    assert_eq!(profile.scoring_weights.version, "policy-platform");
}

#[test]
fn resolves_team_specific_weights_when_profile_exists() {
    let weights = resolve_team_scoring_weights("security");

    assert_eq!(weights.version, "policy-security");
    assert!(weights.pr_approval_weight > 0.30);
}

#[test]
fn falls_back_to_default_profile_for_unknown_team() {
    let catalog = TeamPolicyCatalog::default();
    let profile = catalog.resolve("unknown-team");

    assert_eq!(profile.team, "default");
    assert_eq!(profile.scoring_weights.version, "v1");
}
