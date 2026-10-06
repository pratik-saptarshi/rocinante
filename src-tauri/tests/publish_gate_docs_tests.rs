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
    let shell = read_repo_file("../crates/rocinante-desktop-shell/src/lib.rs");

    assert!(bom.contains("Current shutdown source uses the shared `quit_explicitly` helper"));
    assert!(bom.contains("flush storage before removing the tray icon"));
    assert!(bom.contains("`std::process::exit(0)` on macOS"));
    assert!(checklist.contains("fix/weighted-rollup-aggregation"));
    assert!(checklist.contains("PR #112"));
    assert!(checklist.contains("Readiness requires terminal-green checks on the latest PR head"));
    assert!(checklist.contains("Merge gate (`test`) requires native-shell package jobs"));
    assert!(codemap.contains("Current shutdown source uses the shared `quit_explicitly` helper"));

    let quit_start = shell
        .find("fn quit_explicitly")
        .expect("quit_explicitly helper exists");
    let quit = shell[quit_start..]
        .split("fn persist_shell_state")
        .next()
        .expect("quit_explicitly helper body");
    let persist_start = shell
        .find("fn persist_shell_state")
        .expect("persist_shell_state helper exists");
    let persist = shell[persist_start..]
        .split("fn open_repository_folder")
        .next()
        .expect("persist_shell_state helper body");

    let save_call = persist
        .find("eframe::App::save(self, storage);")
        .expect("shell state is persisted");
    let flush = persist
        .find("storage.flush();")
        .expect("storage is flushed");
    let persist_call = quit
        .find("self.persist_shell_state(frame);")
        .expect("quit persists shell state");
    let tray_drop = quit
        .find("drop(self.tray_icon.take());")
        .expect("quit drops the tray icon");
    let macos_guard = quit
        .find("#[cfg(target_os = \"macos\")]")
        .expect("direct exit is macOS-only");
    let process_exit = quit
        .find("std::process::exit(0);")
        .expect("quit exits the macOS process");

    assert!(save_call < flush, "state must be saved before storage flush");
    assert!(
        persist_call < tray_drop && tray_drop < macos_guard && macos_guard < process_exit,
        "quit must persist and flush before tray removal and macOS process exit"
    );
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
