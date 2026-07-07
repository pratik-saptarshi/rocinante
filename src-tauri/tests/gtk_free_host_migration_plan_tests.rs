use std::fs;
use std::path::PathBuf;

fn read_repo_file(relative_path: &str) -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(relative_path);
    fs::read_to_string(path).expect("read repo file")
}

#[test]
fn parity_matrix_artifact_exists_with_tdd_gate_targets() {
    let matrix = read_repo_file("../docs/roadmap/desktop-parity-matrix.html");

    assert!(matrix.contains("Desktop Parity Matrix and Host-Decision Record"));
    assert!(matrix.contains("RT-RC-002"));
    assert!(matrix.contains("F-047"));
    assert!(matrix.contains("T-049"));
    assert!(matrix.contains("Must-have"));
    assert!(matrix.contains("Should-have"));
    assert!(matrix.contains("Can-defer"));
    assert!(matrix.contains("BI-047"));
}

#[test]
fn parity_matrix_records_host_path_and_fallback_scope() {
    let matrix = read_repo_file("../docs/roadmap/desktop-parity-matrix.html");
    let feature_list = read_repo_file("../docs/feature-list.html");
    let bead_tracker = read_repo_file("../docs/roadmap/bead-issue-tracker.html");
    let feature_backlog = read_repo_file("../docs/roadmap/feature-backlog.html");

    assert!(matrix.contains("native-shell"));
    assert!(matrix.contains("browser-sidecar"));
    assert!(matrix.contains("Decision owner"));
    assert!(matrix.contains("Review-by date"));
    assert!(matrix.contains("RT-RC-002"));
    assert!(feature_list.contains("Desktop parity evaluation and host decision"));
    assert!(bead_tracker.contains("`RT-RC-002` GTK-free host migration planning is active"));
    assert!(feature_backlog.contains("F-047` Desktop parity evaluation and host decision"));
}

#[test]
fn parity_decision_artifacts_stay_stable_during_planned_work() {
    let matrix = read_repo_file("../docs/roadmap/desktop-parity-matrix.html");
    let migration_plan = read_repo_file("../docs/roadmap/gtk-free-host-migration-plan.html");
    let test_plan = read_repo_file("../docs/roadmap/test-plan.html");
    let checklist = read_repo_file("../docs/publish-readiness-checklist.html");

    assert!(matrix.contains("Gate: `RT-RC-002`"));
    assert!(migration_plan.contains("Phase 0:"));
    assert!(test_plan.contains("`F-047` -> `T-049`"));
    assert!(migration_plan.contains("F-047"));
    assert!(checklist.contains("RT-RC-002 is active"));
}
