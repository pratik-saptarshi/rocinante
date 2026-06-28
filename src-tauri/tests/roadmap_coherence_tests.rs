#[test]
fn stage3_convergence_has_a_single_release_gate_and_test_mapping() {
    let feature_list = include_str!("../../docs/feature-list.html");
    let product_roadmap = include_str!("../../docs/product-roadmap.html");
    let bead_tracker = include_str!("../../docs/roadmap/bead-issue-tracker.html");
    let test_plan = include_str!("../../docs/roadmap/test-plan.html");

    assert!(
        feature_list.contains("| P2 | EP-03 Convergence | F-030 Shared release gates |")
            && feature_list.contains("| Green | BI-029 | Completed |"),
        "feature list is missing the BI-029 convergence row"
    );
    assert!(
        product_roadmap.contains(
            "| Stage 3 | Convergence | Merge both tracks into a single release gate and harden the shared workflow. |"
        ),
        "product roadmap is missing the Stage 3 release-gate row"
    );
    assert!(
        bead_tracker
            .contains("| BI-029 | Stage 3 | EP-03 Convergence | F-030 Shared release gates |")
            && bead_tracker.contains("Red->Green complete")
            && bead_tracker.contains("Completed"),
        "bead tracker is missing the BI-029 convergence row"
    );
    assert!(
        test_plan.contains("- `F-030` -> `T-046`"),
        "test plan is missing the F-030 to T-046 mapping"
    );
    assert!(
        test_plan.contains("- `T-046` Roadmap coherence tests:"),
        "test plan is missing the T-046 roadmap coherence track"
    );
}
