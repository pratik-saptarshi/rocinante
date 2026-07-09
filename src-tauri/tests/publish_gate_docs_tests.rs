use std::fs;
use std::path::PathBuf;

fn read_repo_file(relative_path: &str) -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(relative_path);
    fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("read repo file {}: {err}", path.display()))
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
fn publish_gate_documents_reflect_bi_047_merge_and_current_main_snapshot() {
    let checklist = read_repo_file("../docs/publish-readiness-checklist.html");
    let bom = read_repo_file("../docs/bill-of-materials.html");
    let codemap = read_repo_file("../codemap.md");

    assert!(
        bom.contains("Current working slice is `main` after merging `feat/bi-047-decision-paths`.")
    );
    assert!(bom.contains("BI-047"));
    assert!(checklist.contains("BI-047 is complete"));
    assert!(checklist.contains("local `main` matches `origin/main` tree after BI-047"));
    assert!(codemap.contains("BI-047 merged"));
    assert!(codemap.contains("main matches origin/main tree after BI-047"));
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
