use std::fs;
use std::path::PathBuf;

fn read_repo_file(relative_path: &str) -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(relative_path);
    fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("read repo file {}: {err}", path.display()))
}

fn normalize_whitespace(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[test]
fn publish_gate_documents_backend_rust_coverage_lane() {
    let checklist = read_repo_file("../docs/publish-readiness-checklist.html");
    let bom = read_repo_file("../docs/bill-of-materials.html");

    for document in [&checklist, &bom] {
        assert!(document.contains("rust-coverage"));
        assert!(document.contains("cargo llvm-cov"));
        assert!(document.contains("target/coverage/lcov.info"));
        assert!(document.contains("cargo-llvm-cov"));
        assert!(document.contains("informational"));
    }
}

#[test]
fn publish_gate_documents_reflect_current_follow_up_pr_snapshot() {
    let checklist =
        normalize_whitespace(&read_repo_file("../docs/publish-readiness-checklist.html"));
    let bom = normalize_whitespace(&read_repo_file("../docs/bill-of-materials.html"));
    let codemap = normalize_whitespace(&read_repo_file("../codemap.md"));

    assert!(bom.contains("Current work is on the PR #112"));
    assert!(bom.contains("BI-047"));
    assert!(checklist.contains("Current branch snapshot — 2026-10-05"));
    assert!(checklist.contains("fix/weighted-rollup-aggregation"));
    assert!(checklist.contains("PR #112"));
    assert!(checklist.contains("PR #112 OPEN/MERGEABLE at `ce5a1e1`"));
    assert!(checklist.contains("CI run `37341472090`, Security run `37341472140`"));
    assert!(checklist.contains("Dependency Review run `37341472151` all succeeded"));
    assert!(checklist.contains("all three"));
    assert!(checklist.contains("review threads were resolved after the same-head checks passed"));
    assert!(codemap.contains("Current Architecture Status (2026-10-05)"));
    assert!(codemap.contains("PR #112 is OPEN/MERGEABLE at `ce5a1e1`"));
    assert!(codemap
        .contains("CI `37341472090`, Security `37341472140`, and Dependency Review `37341472151`"));
    assert!(codemap.contains("all three review threads are resolved"));
}

#[test]
fn publish_gate_ignores_local_validation_artifacts() {
    let gitignore = read_repo_file("../.gitignore");

    for pattern in [
        ".target-*/",
        ".pnpm-store/",
        ".slim/",
        ".tmp/",
        "ui/dist/",
        "ui/node_modules/",
        "ui/*.tsbuildinfo",
        "ui/vite.config.js",
    ] {
        assert!(
            gitignore.contains(pattern),
            "missing ignore pattern {pattern}"
        );
    }
}
