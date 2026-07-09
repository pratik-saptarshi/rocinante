# Bill of Materials - Markdown Snapshot

_Captured: 2026-07-08_

## Repository and Source Control

- Repository: `https://github.com/pratik-saptarshi/rocinante`
- Primary branch: `main`
- Remote: `origin`
- Current working slice is `main` after merging `feat/bi-056-timeout-contract`.
- Roadmap source-of-truth for execution: `docs/roadmap/bead-issue-tracker.html`

## Branch and Sync State

- `origin/main` and local `main` are aligned on commit `ce2dd48` (squash merge of `feat/bi-055-delta-fast-path`).
- Remaining open slices continue via PR checkpoints with explicit roadmap/checklist
  evidence and conventional commits.
- Lane-scope refinement now keeps docs-only and non-functional edits out of storage/coverage-heavy lanes while preserving core rust gate visibility.

## Runtime Surface

- Backend: `src-tauri/Cargo.toml`, `src-tauri/src/*.rs`
- Frontend: `ui/package.json`, `ui/src/**`, `ui/e2e/**`
- Automation: `.github/workflows/*.yml`, `scripts/*.sh`, `scripts/*.mjs`
- Governance artifacts: `docs/bill-of-materials.html`, `docs/publish-readiness-checklist.html`,
  `docs/roadmap/*`, `README.md`, `SECURITY.md`

## Active Governance and Planned Slices

- `BI-047` — F-047 Desktop parity evaluation and host decision (in progress)
- `BI-052` — F-052 Dependabot esbuild remediation (in progress)
- `BI-053` — F-053 CI bootstrap and workflow parseability (completed; validated on PR run `28983234703`)
- `BI-054` — F-054 CI lane orchestration and gating (completed; validated on PR run `28983234703`)
- `BI-055` — F-054 CI lane orchestration and gating (completed)
- `BI-056` — F-055 Release-path performance optimization (completed)
- `BI-057` — CI bootstrap + workflow parseability recovery (Red->Green complete)
- `RT-RC-001` — GTK/glib dependency-floor governance (active)
- `RT-RC-002` — GTK-free host migration planning (active)

## Validation Snapshot (Latest Local Run)

- `cargo fmt --manifest-path src-tauri/Cargo.toml --all -- --check` passes.
- `cargo test --locked --manifest-path src-tauri/Cargo.toml --test ci_gate_tests` passes on the pinned `1.96.1` toolchain.
- `node scripts/check-esbuild-lock.mjs` passes and confirms `esbuild >= 0.28.1` floor.
- `scripts/check-dependabot-esbuild-alert.sh` is being corrected to query advisory IDs explicitly; remote Dependabot state now shows open alert `GHSA-wrw7-89jp-8q8g` for `glib`.
- `publish-readiness-checklist.html` remains open because RT-RC-001 is still active and publish still requires a formal release branch / merge checkpoint, even though the CI recovery and CI lane slices now pass their latest remote checks.
- Duplicate feature mapping cleanup completed by removing legacy duplicate `F-027` row from `docs/feature-list.html` (test traceability consolidation pass complete).
- Remote PR run `28987645462` is green for `ci-health`, `ci-workflow-parse`, `ci-scope`, `rust-build-seed`, `rust-quality-gates`, `rust-lint`, `rust-tests`, and the aggregate `test` gate.

## Dependency Controls and Security Gate Stack

- Rust toolchain: `1.96.1` in `rust-toolchain.toml`
- CI release floor: `src-tauri/Cargo.toml`, `.cargo/audit.toml`
- UI floor check: `scripts/check-esbuild-lock.mjs`
- Dependabot alert gate: `scripts/check-dependabot-esbuild-alert.sh` on `main` lane
- Security checks in CI: TruffleHog, CodeQL, `cargo-audit`, dependency review

## Validation Entry Points (Publish Gating)

- `cargo fmt --manifest-path src-tauri/Cargo.toml --all -- --check`
- `cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -A dead_code` (warnings logged; dead-code allowed in-place)
- `cargo test --manifest-path src-tauri/Cargo.toml`
- `pnpm -C ui exec tsc -b`
- `pnpm -C ui exec vitest run`
- `pnpm -C ui exec playwright test`
- `cargo llvm-cov --locked --manifest-path src-tauri/Cargo.toml --lcov --output-path target/coverage/lcov.info`

## Pipeline Layout Snapshot

- CI split now enforces explicit jobs:
  - `rust-quality-gates` (fmt, clippy, CI gate contract)
  - `rust-tests` (lane matrix: `core`, `storage`)
  - `rust-coverage` (release-only coverage)

## Current Remediation State Notes

- `ui/pnpm-lock.yaml` lockfile currently resolves `esbuild@0.28.1` and `scripts/check-esbuild-lock.mjs` passes locally.
- Remote Dependabot state currently exposes `GHSA-wrw7-89jp-8q8g` on `glib` (`open`) and is tracked by RT-RC-001 as a release-blocking item.

## Release Artifacts

- Backend coverage artifact: `target/coverage/lcov.info` (published as `rust-coverage-lcov`).
- Policy and exception records: `docs/roadmap/security-advisory-exceptions.json`.
- CI contract for release-floor and feature-to-feature mapping tests: `src-tauri/tests/ci_gate_tests.rs`.
