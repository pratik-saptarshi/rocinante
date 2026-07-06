use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;

use serde_json::Value;

fn read_repo_file(relative_path: &str) -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(relative_path);
    fs::read_to_string(path).expect("read repo file")
}

fn audit_ignore_ids(audit: &str) -> BTreeSet<String> {
    let mut in_ignore = false;
    let mut ids = BTreeSet::new();

    for line in audit.lines() {
        let trimmed = line.trim();
        if trimmed == "ignore = [" {
            in_ignore = true;
            continue;
        }
        if in_ignore && trimmed == "]" {
            break;
        }
        if in_ignore && trimmed.starts_with('"') {
            ids.insert(trimmed.trim_matches(',').trim_matches('"').to_string());
        }
    }

    ids
}

fn registry_entries() -> Vec<Value> {
    serde_json::from_str(&read_repo_file(
        "../docs/roadmap/security-advisory-exceptions.json",
    ))
    .expect("parse security advisory registry")
}

#[test]
fn security_advisory_exceptions_cover_all_audit_ignores() {
    let audit = read_repo_file("../.cargo/audit.toml");
    let audit_ids = audit_ignore_ids(&audit);
    let registry = registry_entries();
    let registry_ids = registry
        .iter()
        .map(|entry| entry["id"].as_str().expect("registry id").to_string())
        .collect::<BTreeSet<_>>();

    assert_eq!(audit_ids.len(), registry.len());
    assert_eq!(audit_ids, registry_ids);
}

#[test]
fn security_advisory_exceptions_have_owner_review_date_and_exit_condition() {
    for entry in registry_entries() {
        let entry = entry.as_object().expect("registry object");
        assert!(!entry["owner"].as_str().expect("owner").is_empty());
        assert_eq!(entry["review_by"], "2026-08-06");
        assert!(!entry["exit_condition"]
            .as_str()
            .expect("exit condition")
            .is_empty());
        assert!(!entry["affected_path"]
            .as_str()
            .expect("affected path")
            .is_empty());
        assert!(!entry["reason"].as_str().expect("reason").is_empty());
        assert!(entry["tracking_id"]
            .as_str()
            .expect("tracking id")
            .starts_with("RT-RC-001-"));
    }
}

#[test]
fn gtk_glib_dependency_floor_is_tracked_as_release_blocking() {
    let baseline = read_repo_file("../docs/roadmap/repository-security-baseline.html");
    let checklist = read_repo_file("../docs/publish-readiness-checklist.html");
    let proof_script = read_repo_file("../scripts/dependency-floor-proof.sh");
    let audit = read_repo_file("../.cargo/audit.toml");

    assert!(baseline
        .contains("Release readiness is blocked by the tracked GTK/glib advisory exception."));
    assert!(baseline.contains("docs/roadmap/security-advisory-exceptions.json"));
    assert!(checklist.contains(
        "dependency audit exceptions are registry-backed, time-boxed, and release-blocking"
    ));
    assert!(checklist.contains("Release remains blocked until RT-RC-001 is closed"));
    assert!(proof_script.contains("cargo tree --manifest-path \"$repo_root/src-tauri/Cargo.toml\" -i glib --locked --target all"));
    assert!(proof_script.contains("cargo tree --manifest-path \"$repo_root/src-tauri/Cargo.toml\" -i gtk --locked --target all"));
    assert!(audit.contains("RUSTSEC-2024-0429"));
}

#[test]
fn roadmap_does_not_claim_complete_backlog_with_active_security_exception() {
    let feature_list = read_repo_file("../docs/feature-list.html");
    let product_roadmap = read_repo_file("../docs/product-roadmap.html");
    let bead_tracker = read_repo_file("../docs/roadmap/bead-issue-tracker.html");
    let test_plan = read_repo_file("../docs/roadmap/test-plan.html");

    assert!(feature_list.contains("F-046 GTK/glib dependency-floor governance"));
    assert!(product_roadmap.contains("Stage 4 | Release/security governance"));
    assert!(product_roadmap.contains("Open release/security slice remains: RT-RC-001"));
    assert!(bead_tracker.contains("`RT-RC-001` GTK/glib dependency-floor governance is active"));
    assert!(bead_tracker.contains("BI-046"));
    assert!(test_plan.contains("RT-RC-001` -> `T-048"));
    assert!(test_plan
        .contains("T-048 Security advisory exception registry and dependency-floor proof tests"));
}

#[test]
fn gtk_free_host_migration_plan_is_stage_gated_and_tdd_driven() {
    let migration_plan = read_repo_file("../docs/roadmap/gtk-free-host-migration-plan.html");
    let feature_list = read_repo_file("../docs/feature-list.html");
    let feature_backlog = read_repo_file("../docs/roadmap/feature-backlog.html");
    let bead_tracker = read_repo_file("../docs/roadmap/bead-issue-tracker.html");
    let test_plan = read_repo_file("../docs/roadmap/test-plan.html");
    let bom = read_repo_file("../docs/bill-of-materials.html");
    let checklist = read_repo_file("../docs/publish-readiness-checklist.html");
    let codemap = read_repo_file("../codemap.md");

    assert!(migration_plan.contains("GTK-Free Host Migration Plan"));
    assert!(migration_plan.contains("Use TDD for every phase"));
    assert!(migration_plan.contains("Phase 0"));
    assert!(migration_plan.contains("Phase 1"));
    assert!(migration_plan.contains("Phase 2"));
    assert!(migration_plan.contains("Phase 3"));
    assert!(migration_plan.contains("Phase 4"));
    assert!(migration_plan.contains("F-047"));
    assert!(migration_plan.contains("F-051"));
    assert!(feature_list.contains("EP-06 Host migration planning"));
    assert!(feature_list.contains("F-047 Desktop parity evaluation and host decision"));
    assert!(feature_list.contains("F-051 Remove Tauri/GTK/GLib"));
    assert!(feature_backlog.contains("F-047 Desktop parity evaluation and host decision"));
    assert!(feature_backlog.contains("F-051 Remove Tauri/GTK/GLib"));
    assert!(feature_backlog.contains("docs/roadmap/gtk-free-host-migration-plan.html"));
    assert!(bead_tracker.contains("`RT-RC-002` GTK-free host migration planning is active"));
    assert!(bead_tracker.contains("BI-047"));
    assert!(bead_tracker.contains("BI-051"));
    assert!(test_plan.contains("RT-RC-002` GTK-free host migration planning"));
    assert!(test_plan.contains("T-049 Parity matrix and host-decision tests"));
    assert!(test_plan.contains("T-053 Dependency-floor removal tests"));
    assert!(bom.contains("docs/roadmap/gtk-free-host-migration-plan.html"));
    assert!(bom.contains("scripts/dependency-floor-proof.sh"));
    assert!(checklist.contains("GTK-free host migration plan is documented and phased"));
    assert!(checklist.contains("RT-RC-002 is active"));
    assert!(codemap.contains("gtk-free-host-migration-plan.html"));
    assert!(codemap.contains("security-advisory-exceptions.json"));
    assert!(codemap.contains("dependency-floor-proof.sh"));
}
