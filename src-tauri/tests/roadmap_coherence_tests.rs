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

#[test]
fn active_host_migration_beads_and_test_plan_mappings_stay_in_sync() {
    let feature_list = include_str!("../../docs/feature-list.html");
    let feature_backlog = include_str!("../../docs/roadmap/feature-backlog.html");
    let bead_tracker = include_str!("../../docs/roadmap/bead-issue-tracker.html");
    let test_plan = include_str!("../../docs/roadmap/test-plan.html");

    assert!(
        feature_list.contains("| P0 | EP-06 Host migration planning | F-047 Desktop parity evaluation and host decision |") &&
            feature_list.contains("| In Progress | BI-047 | In Progress |"),
        "feature list is missing active BI-047 host-migration tracking",
    );

    assert!(
        feature_list.contains(
            "| P2 | EP-05 Release/security governance | F-052 Dependabot esbuild remediation |"
        ) && feature_list.contains("| In Progress | BI-052 | In Progress |"),
        "feature list is missing active BI-052 remediation tracking",
    );

    assert!(
        feature_backlog.contains("`F-047` Desktop parity evaluation and host decision")
            && feature_backlog.contains("`F-049` GTK-free native desktop MVP")
            && feature_backlog.contains("`F-052` Dependabot esbuild remediation"),
        "feature backlog is missing host-migration dependencies for active tracks",
    );

    assert!(
        bead_tracker.contains(
            "| BI-047 | Stage 0 | EP-06 Host migration planning | F-047 Desktop parity evaluation and host decision |",
        ) && bead_tracker.contains("| In Progress | In Progress |") &&
            bead_tracker.contains("BI-047"),
        "bead tracker is missing BI-047 with active status",
    );

    assert!(
        bead_tracker.contains(
            "| BI-052 | Stage 4 | EP-05 Release/security governance | F-052 Dependabot esbuild remediation |",
        ) && bead_tracker.contains("| In Progress | In Progress |") &&
            bead_tracker.contains("BI-052"),
        "bead tracker is missing BI-052 progress state",
    );

    assert!(
        test_plan.contains("`F-047` -> `T-049`")
            && test_plan.contains("`F-048` -> `T-050`")
            && test_plan.contains("`F-049` -> `T-051`")
            && test_plan.contains("`F-050` -> `T-052`")
            && test_plan.contains("`F-051` -> `T-053`")
            && test_plan.contains("`F-052` -> `T-054`"),
        "host-migration test plan mappings are stale or missing",
    );

    assert!(
        test_plan.contains("`F-053` -> `T-055`")
            && test_plan.contains("`F-054` -> `T-056`")
            && test_plan.contains("`F-055` -> `T-057`"),
        "ci optimization test plan mappings for active CI tracks are stale or missing",
    );
}
