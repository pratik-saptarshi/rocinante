use std::fs;
use std::path::PathBuf;

use repo_analyzer_core::ci_gate::build_ci_gate_comment;
use repo_analyzer_core::risk_contract::{evaluate_pr_risk, PrRiskSchema};
use repo_analyzer_core::types::PrCandidate;

fn sample_candidate(
    pr_id: &str,
    file_risk: f64,
    author_velocity: f64,
    approval_fidelity: f64,
) -> PrCandidate {
    PrCandidate {
        pr_id: pr_id.to_string(),
        repo_name: "repo-a".to_string(),
        author: "alice".to_string(),
        release: "v1.0.0".to_string(),
        file_risk,
        author_velocity,
        approval_fidelity,
        files: vec![],
        circuit_breaker_triggered: false,
    }
}

fn read_repo_file(relative_path: &str) -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(relative_path);
    fs::read_to_string(path).expect("read repo file")
}

fn normalize_workflow_shell_text(text: &str) -> String {
    text.replace('\\', " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn extract_named_step_run_body(workflow: &str, step_name: &str) -> String {
    let lines = workflow.lines().collect::<Vec<_>>();
    let step_header = format!("- name: {step_name}");
    let step_index = lines
        .iter()
        .position(|line| line.trim() == step_header)
        .unwrap_or_else(|| panic!("workflow missing step `{step_name}`"));
    let step_indent = lines[step_index]
        .chars()
        .take_while(|ch| *ch == ' ')
        .count();

    let mut index = step_index + 1;
    while index < lines.len() {
        let line = lines[index];
        let trimmed = line.trim();
        let indent = line.chars().take_while(|ch| *ch == ' ').count();

        if indent == step_indent && trimmed.starts_with("- name: ") {
            break;
        }

        if let Some(rest) = trimmed.strip_prefix("run:") {
            let rest = rest.trim_start();
            if !rest.is_empty() && rest != "|" && rest != ">" {
                return rest.to_string();
            }

            let run_indent = indent;
            let mut body = Vec::new();
            index += 1;
            while index < lines.len() {
                let body_line = lines[index];
                let body_trimmed = body_line.trim();
                let body_indent = body_line.chars().take_while(|ch| *ch == ' ').count();

                if !body_trimmed.is_empty() && body_indent <= run_indent {
                    break;
                }

                body.push(body_line.trim_start().to_string());
                index += 1;
            }

            return body.join("\n");
        }

        index += 1;
    }

    panic!("workflow step `{step_name}` missing run body");
}

fn extract_named_step_block(workflow: &str, step_name: &str) -> String {
    let lines = workflow.lines().collect::<Vec<_>>();
    let step_header = format!("- name: {step_name}");
    let step_index = lines
        .iter()
        .position(|line| line.trim() == step_header)
        .unwrap_or_else(|| panic!("workflow missing step `{step_name}`"));
    let step_indent = lines[step_index]
        .chars()
        .take_while(|ch| *ch == ' ')
        .count();

    let mut body = Vec::new();
    let mut index = step_index;
    while index < lines.len() {
        let line = lines[index];
        let trimmed = line.trim();
        let indent = line.chars().take_while(|ch| *ch == ' ').count();

        if index > step_index && indent == step_indent && trimmed.starts_with("- name: ") {
            break;
        }

        body.push(line.to_string());
        index += 1;
    }

    body.join("\n")
}

fn assert_step_run_contains_all(workflow: &str, step_name: &str, fragments: &[&str]) {
    let run_body = normalize_workflow_shell_text(&extract_named_step_run_body(workflow, step_name));
    for fragment in fragments {
        let normalized_fragment = normalize_workflow_shell_text(fragment);
        assert!(
            run_body.contains(&normalized_fragment),
            "step `{step_name}` run body `{run_body}` missing fragment `{normalized_fragment}`"
        );
    }
}

fn assert_step_block_contains_all(workflow: &str, step_name: &str, fragments: &[&str]) {
    let step_block = extract_named_step_block(workflow, step_name);
    for fragment in fragments {
        assert!(
            step_block.contains(fragment),
            "step `{step_name}` block `{step_block}` missing fragment `{fragment}`"
        );
    }
}

#[test]
fn ci_gate_comment_blocks_high_risk_prs_with_a_stable_summary() {
    let schema = PrRiskSchema::default();
    let evaluation = evaluate_pr_risk(&sample_candidate("pr-31", 0.9, 0.35, 0.45), &schema);
    let comment = build_ci_gate_comment(&evaluation);

    assert!(comment.should_block_merge);
    assert!(comment.summary.contains("block merge"));
    assert!(comment.summary.contains("security-review"));
    assert!(comment.body.contains("Decision: Block merge"));
    assert!(comment.body.contains("Review requirement: security-review"));
    assert!(comment.body.contains("Reason codes:"));
    assert!(comment.body.contains("file_risk=0.90"));
}

#[test]
fn ci_gate_comment_allows_low_risk_prs_with_a_stable_summary() {
    let schema = PrRiskSchema::default();
    let evaluation = evaluate_pr_risk(&sample_candidate("pr-32", 0.1, 0.9, 0.95), &schema);
    let comment = build_ci_gate_comment(&evaluation);

    assert!(!comment.should_block_merge);
    assert!(comment.summary.contains("allow merge"));
    assert!(comment.summary.contains("none"));
    assert!(comment.body.contains("Decision: Allow merge"));
    assert!(comment.body.contains("Review requirement: none"));
}

#[test]
fn ci_gate_comment_round_trips_through_json() {
    let schema = PrRiskSchema::default();
    let evaluation = evaluate_pr_risk(&sample_candidate("pr-33", 0.8, 0.5, 0.6), &schema);
    let comment = build_ci_gate_comment(&evaluation);

    let raw = serde_json::to_string(&comment).expect("serialize gate comment");
    let round_trip = serde_json::from_str(&raw).expect("deserialize gate comment");

    assert_eq!(comment, round_trip);
}

#[test]
fn ci_workflow_includes_the_ci_gate_contract_step() {
    let workflow = read_repo_file("../.github/workflows/ci.yml");

    assert!(workflow.contains("CI gate contract"));
    assert_step_run_contains_all(
        &workflow,
        "CI gate contract",
        &[
            "cargo test",
            "--locked",
            "--manifest-path src-tauri/Cargo.toml",
            "--test ci_gate_tests",
        ],
    );
}

#[test]
fn repo_pins_the_rust_toolchain_to_a_specific_stable_release() {
    let toolchain = read_repo_file("../rust-toolchain.toml");
    let manifest = read_repo_file("Cargo.toml");

    assert!(toolchain.contains("channel = \"1.96.1\""));
    assert!(toolchain.contains("profile = \"minimal\""));
    assert!(toolchain.contains("\"clippy\""));
    assert!(toolchain.contains("\"rustfmt\""));
    assert!(manifest.contains("rust-version = \"1.96.1\""));
}

#[test]
fn ci_workflow_uses_the_pinned_toolchain_and_locked_rust_commands() {
    let workflow = read_repo_file("../.github/workflows/ci.yml");
    let manifest = read_repo_file("Cargo.toml");

    assert!(workflow.contains("dtolnay/rust-toolchain@1.96.1"));
    assert!(workflow.contains("components: clippy, rustfmt"));
    assert_step_run_contains_all(
        &workflow,
        "Lint",
        &[
            "cargo clippy",
            "--locked",
            "--manifest-path src-tauri/Cargo.toml",
            "--features analytics",
            "-A dead_code -D warnings",
        ],
    );
    assert_step_run_contains_all(
        &workflow,
        "Test",
        &[
            "cargo test",
            "--locked",
            "--manifest-path src-tauri/Cargo.toml",
            "--lib",
            "--tests",
        ],
    );
    assert!(manifest.contains("test = false"));
}

#[test]
fn ci_workflow_has_a_non_blocking_backend_rust_coverage_job() {
    let workflow = read_repo_file("../.github/workflows/ci.yml");

    assert!(workflow.contains("rust-coverage:"));
    assert!(workflow.contains("dtolnay/rust-toolchain@1.96.1"));
    assert_step_block_contains_all(
        &workflow,
        "Install cargo-llvm-cov",
        &["taiki-e/install-action@v2", "tool: cargo-llvm-cov"],
    );
    assert_step_run_contains_all(
        &workflow,
        "Rust coverage",
        &[
            "mkdir -p target/coverage",
            "cargo llvm-cov",
            "--no-clean",
            "--locked",
            "--manifest-path src-tauri/Cargo.toml",
            "--features analytics",
            "--lcov",
            "--output-path target/coverage/lcov.info",
        ],
    );
    assert_step_block_contains_all(
        &workflow,
        "Upload Rust coverage report",
        &[
            "actions/upload-artifact@v4",
            "name: rust-coverage-lcov",
            "target/coverage/lcov.info",
        ],
    );
    assert!(!workflow.contains("--fail-under-lines"));
    assert!(!workflow.contains("--fail-under-regions"));
    assert!(!workflow.contains("--fail-under-functions"));
}

#[test]
fn ci_workflow_verifies_esbuild_lock_floor_in_release_ci() {
    let workflow = read_repo_file("../.github/workflows/ci.yml");
    let check_script = read_repo_file("../scripts/check-esbuild-lock.mjs");

    assert_step_block_contains_all(
        &workflow,
        "Verify UI esbuild floor",
        &["node scripts/check-esbuild-lock.mjs"],
    );
    assert!(check_script.contains("const MIN_MAJOR"));
    assert!(check_script.contains("MIN_MINOR"));
    assert!(check_script.contains("MIN_PATCH"));
    assert!(check_script.contains("esbuild"));
}

#[test]
fn ci_workflow_verifies_esbuild_dependabot_alert_status_in_release_ci() {
    let workflow = read_repo_file("../.github/workflows/ci.yml");
    let dependabot_gate = read_repo_file("../scripts/check-dependabot-esbuild-alert.sh");
    let plan = read_repo_file("../docs/roadmap/dependabot-esbuild-remediation-plan.html");

    assert!(plan.contains("GHSA-g7r4-m6w7-qqqr"));
    assert!(plan.contains("Release pipeline now runs `scripts/check-dependabot-esbuild-alert.sh`"));
    assert_step_block_contains_all(
        &workflow,
        "Verify open esbuild Dependabot alert is closed",
        &[
            "scripts/check-dependabot-esbuild-alert.sh",
            "GH_TOKEN: ${{ github.token }}",
        ],
    );
    assert!(dependabot_gate.contains("dependabot/alerts"));
    assert!(dependabot_gate.contains("GHSA-g7r4-m6w7-qqqr"));
    assert!(dependabot_gate.contains("ALERTHITS"));
}

#[test]
fn ci_workflow_has_a_release_only_build_seed_job() {
    let workflow = read_repo_file("../.github/workflows/ci.yml");

    assert!(workflow.contains("rust-build-seed:"));
    assert!(workflow.contains("build-scope"));
    assert!(workflow.contains("outputs:"));
    assert_step_run_contains_all(
        &workflow,
        "Release build seed",
        &[
            "cargo test",
            "--locked",
            "--manifest-path src-tauri/Cargo.toml",
            "--features analytics",
            "--bins",
            "--lib",
            "--tests",
            "--no-run",
        ],
    );
    assert_step_run_contains_all(
        &workflow,
        "Delta build seed",
        &[
            "cargo test",
            "--locked",
            "--manifest-path src-tauri/Cargo.toml",
            "--no-default-features",
            "--lib",
            "--tests",
            "--no-run",
        ],
    );
}

#[test]
fn ci_workflow_uses_scope_specific_release_cache_prefixes() {
    let workflow = read_repo_file("../.github/workflows/ci.yml");

    assert!(workflow.contains("workspaces: src-tauri"));
    assert!(workflow.contains("prefix-key: rust-cache-${{ steps.build-scope.outputs.scope }}-v2"));
    assert!(workflow.contains("prefix-key: rust-cache-release-v2"));
}

#[test]
fn ci_workflow_differentiates_release_and_delta_lanes() {
    let workflow = read_repo_file("../.github/workflows/ci.yml");

    assert!(workflow.contains("rust-quality-gates:"));
    assert!(workflow.contains("rust-tests:"));
    assert!(workflow.contains("strategy:"));
    assert!(workflow.contains("lane: [core, storage]"));
    assert!(workflow.contains(
        "if: ${{ matrix.lane == 'core' || github.ref == 'refs/heads/main' || startsWith(github.ref, 'refs/heads/release/') }}"
    ));
    assert_step_block_contains_all(
        &workflow,
        "Lint",
        &[
            "if [[ \"$CI_RUST_BUILD_SCOPE\" == \"release\" ]]; then",
            "cargo clippy \\",
            "--locked \\",
            "--manifest-path src-tauri/Cargo.toml \\",
            "--features analytics \\",
            "-- -A dead_code -D warnings",
            "else",
            "cargo clippy --locked --manifest-path src-tauri/Cargo.toml --no-default-features --lib -- -A dead_code -D warnings",
            "fi",
        ],
    );
    assert_step_block_contains_all(
        &workflow,
        "Test",
        &[
            "TEST_START_TS=$(date +%s)",
            "if [[ \"$CI_RUST_BUILD_SCOPE\" == \"release\" ]]; then",
            "if [[ \"${{ matrix.lane }}\" == \"storage\" ]]; then",
            "--test storage_duallayer_tests",
            "else",
            "--test admin_ingestion_guard_tests",
            "--test verifier_tests",
            "else",
            "cargo test --locked --manifest-path src-tauri/Cargo.toml --no-default-features --lib --tests",
            "fi",
        ],
    );
    assert!(workflow.contains("lane=storage"));
    assert!(workflow.contains("if: ${{ github.ref == 'refs/heads/main' || startsWith(github.ref, 'refs/heads/release/') }}"));
}

#[test]
fn ci_workflow_runs_coverage_only_for_release_lanes() {
    let workflow = read_repo_file("../.github/workflows/ci.yml");

    assert!(workflow.contains("rust-coverage:"));
    assert!(workflow.contains(
        "if: ${{ github.ref == 'refs/heads/main' || startsWith(github.ref, 'refs/heads/release/') }}"
    ));
    assert!(workflow.contains(
        "rust-coverage:\n    if: ${{ github.ref == 'refs/heads/main' || startsWith(github.ref, 'refs/heads/release/') }}\n    needs: [rust-build-seed]"
    ));
}

#[test]
fn ci_workflow_ci_gate_contract_respects_release_and_delta_scope() {
    let workflow = read_repo_file("../.github/workflows/ci.yml");

    assert_step_block_contains_all(
        &workflow,
        "CI gate contract",
        &[
            "if [[",
            "== \"release\" ]]; then",
            "cargo test --locked --manifest-path src-tauri/Cargo.toml --test ci_gate_tests",
            "cargo test --locked --manifest-path src-tauri/Cargo.toml --no-default-features --test ci_gate_tests",
            "fi",
        ],
    );
}

#[test]
fn ci_workflow_releases_share_compilation_cache_and_release_seed_runs_in_parallel_with_quality_gate(
) {
    let workflow = read_repo_file("../.github/workflows/ci.yml");

    assert!(workflow.contains("rust-quality-gates:\n    needs: [rust-build-seed]"));
    assert!(workflow.contains("rust-tests:\n    if: ${{ matrix.lane == 'core' || github.ref == 'refs/heads/main' || startsWith(github.ref, 'refs/heads/release/') }}\n    needs: [rust-build-seed]"));
    assert!(workflow.contains("needs: [rust-build-seed]"));
    assert!(workflow.contains("save-if: false"));
    assert!(workflow.contains(
        "save-if: |\n            ${{ github.ref == 'refs/heads/main' || startsWith(github.ref, 'refs/heads/release/') }}"
    ));
}

#[test]
fn ci_workflow_includes_release_and_quality_timing_telemetry() {
    let workflow = read_repo_file("../.github/workflows/ci.yml");

    assert!(workflow.contains("::notice title=Release build seed::scope=release"));
    assert!(workflow.contains("::notice title=Release build seed::scope=delta"));
    assert!(workflow.contains("::notice title=Rust quality lint::"));
    assert!(workflow.contains("::notice title=Rust quality test::"));
    assert!(workflow.contains("::notice title=Rust coverage::"));
}

#[test]
fn ci_workflow_does_not_redundantly_run_release_bin_only_check() {
    let workflow = read_repo_file("../.github/workflows/ci.yml");

    assert!(!workflow.contains("- name: Binary check"));
    assert!(!workflow.contains("cargo check --locked --manifest-path src-tauri/Cargo.toml --bins"));
}

#[test]
fn ci_workflow_marks_release_build_floor_and_delta_scope() {
    let workflow = read_repo_file("../.github/workflows/ci.yml");

    assert!(workflow.contains(
        "if [[ \"${{ github.ref }}\" == \"refs/heads/main\" || \"${{ startsWith(github.ref, 'refs/heads/release/') }}\" == \"true\" ]]; then"
    ));
    assert!(workflow.contains("scope=release"));
    assert!(workflow.contains("scope=delta"));
    let delta_seed = extract_named_step_block(&workflow, "Delta build seed");
    assert!(!delta_seed.contains("--all-targets"));
    let release_seed = extract_named_step_block(&workflow, "Release build seed");
    assert!(release_seed.contains("--bins"));
    assert!(release_seed.contains("--lib"));
    assert!(release_seed.contains("--tests"));
    assert!(release_seed.contains("--no-run"));
}

#[test]
fn security_workflow_uses_the_same_pinned_toolchain_for_rust_analysis() {
    let workflow = read_repo_file("../.github/workflows/security.yml");
    let audit_config = read_repo_file("../.cargo/audit.toml");

    assert!(workflow.contains("dtolnay/rust-toolchain@1.96.1"));
    assert!(workflow.contains("components: clippy, rustfmt"));
    assert!(workflow.contains("taiki-e/install-action@v2"));
    assert!(workflow.contains("tool: cargo-audit"));
    assert_step_run_contains_all(
        &workflow,
        "Audit Rust dependencies",
        &[
            "cargo audit",
            "--file src-tauri/Cargo.lock",
            "--deny warnings",
        ],
    );
    assert!(audit_config.contains("RUSTSEC-2024-0411"));
    assert!(audit_config.contains("RUSTSEC-2024-0429"));
    assert!(audit_config.contains("RUSTSEC-2025-0100"));
}

#[test]
fn workflow_step_contract_matching_tolerates_reordered_multiline_commands() {
    let workflow = r#"
jobs:
  test:
    steps:
      - name: CI gate contract
        run: |
          cargo test \
            --test ci_gate_tests \
            --manifest-path src-tauri/Cargo.toml \
            --locked \
            --nocapture
"#;

    assert_step_run_contains_all(
        workflow,
        "CI gate contract",
        &[
            "cargo test",
            "--locked",
            "--manifest-path src-tauri/Cargo.toml",
            "--test ci_gate_tests",
        ],
    );
}

#[test]
#[should_panic(expected = "missing fragment `--locked`")]
fn workflow_step_contract_matching_rejects_missing_required_fragments() {
    let workflow = r#"
jobs:
  test:
    steps:
      - name: CI gate contract
        run: cargo test --manifest-path src-tauri/Cargo.toml --test ci_gate_tests
"#;

    assert_step_run_contains_all(
        workflow,
        "CI gate contract",
        &[
            "cargo test",
            "--locked",
            "--manifest-path src-tauri/Cargo.toml",
            "--test ci_gate_tests",
        ],
    );
}
