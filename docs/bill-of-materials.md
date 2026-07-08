# Bill of Materials - Markdown Snapshot

_Captured: 2026-07-08_

## Repository and Source Control

- Repository: `https://github.com/pratik-saptarshi/rocinante`
- Primary branch: `main`
- Remote: `origin`
- Active branch for current slice: `feat/ci-bim-055-timeout-artifact-hardening`
- Roadmap source-of-truth for execution: `docs/roadmap/bead-issue-tracker.html`

## Branch and Sync State

- `origin/main` currently points to `dc48f273d7ff08d80ad78d241dbbdbe290b1876d` and local `main` is synchronized (`main` tracks `origin/main`).
- Mainline protection remains in place, so changes proceed through checkpointed branches and are merged by PR after contract checks.
- Remaining open slices continue via branch checkpoints with explicit `roadmap/checklist` evidence.

## Runtime Surface

- Backend: `src-tauri/Cargo.toml`, `src-tauri/src/*.rs`
- Frontend: `ui/package.json`, `ui/src/**`, `ui/e2e/**`
- Automation: `.github/workflows/*.yml`, `scripts/*.sh`, `scripts/*.mjs`
- Governance artifacts: `docs/bill-of-materials.html`, `docs/publish-readiness-checklist.html`,
  `docs/roadmap/*`, `README.md`, `SECURITY.md`

## Active Governance and Planned Slices

- `BI-047` — F-047 Desktop parity evaluation and host decision (in progress)
- `BI-052` — F-052 Dependabot esbuild remediation (in progress)
- `BI-053` — F-053 CI bootstrap and workflow parseability (completed with PR gate recovery + merge checkpoint)
- `BI-054` — F-054 CI lane orchestration and gating (in progress)
- `BI-056` — F-055 Release-path performance optimization (in progress)
- `BI-057` — CI bootstrap + workflow parseability recovery (Red->Green complete)
- `RT-RC-001` — GTK/glib dependency-floor governance (active)
- `RT-RC-002` — GTK-free host migration planning (active)

## Validation Snapshot (Latest Local Run)

- `cargo fmt --manifest-path src-tauri/Cargo.toml --all -- --check` passes.
- Targeted contract compile (`cargo test --manifest-path src-tauri/Cargo.toml --locked --test ci_gate_tests --no-run`) was previously validated in earlier slices and is re-checked as part of this slice changes.
- `node scripts/check-esbuild-lock.mjs` passes and confirms `esbuild >= 0.28.1` floor.
- `scripts/check-dependabot-esbuild-alert.sh` remains part of the release-only scope and explicitly tracks `GHSA-g7r4-m6w7-qqqr`.
- `scripts/check-dependabot-esbuild-alert.sh` still reports the open GTK/glib advisory `GHSA-wrw7-89jp-8q8g` in live CI checks.
- `publish-readiness-checklist.html` remains open because clippy, full Rust tests, and UI suites are still pending full re-run after this gate slice.
- CI coverage tooling was updated in this slice to `actions/upload-artifact@v5` to address Node runtime warning pressure while preserving artifact path and retention.

## Dependency Controls and Security Gate Stack

- Rust toolchain: `1.96.1` in `rust-toolchain.toml`
- CI release floor: `src-tauri/Cargo.toml`, `.cargo/audit.toml`
- UI floor check: `scripts/check-esbuild-lock.mjs`
- Dependabot alert gate: `scripts/check-dependabot-esbuild-alert.sh` on `main` lane
- Security checks in CI: TruffleHog, CodeQL, `cargo-audit`, dependency review

## Validation Entry Points (Publish Gating)

- `cargo fmt --manifest-path src-tauri/Cargo.toml --all -- --check`
- `cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -A dead_code -D warnings`
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
