# Bill of Materials - Markdown Snapshot

_Captured: 2026-07-07_

## Repository and Source Control

- Repository: `https://github.com/pratik-saptarshi/rocinante`
- Primary branch: `main`
- Remote: `origin`
- Active branch for current slice: `feat/bi-047-bom-readiness-sync-slice`
- Roadmap source-of-truth for execution: `docs/roadmap/bead-issue-tracker.html`

## Branch and Sync State

- `origin/main` currently points to commit `e339963`.
- `main` contains legacy local history and is not a fast-forward descendant of `origin/main`.
- Current work should proceed through feature branches and explicit checkpoint merges.
- Merge strategy for next checkpoint: align main to `origin/main`, then merge feature slices with explicit evidence and PR gates.

## Runtime Surface

- Backend: `src-tauri/Cargo.toml`, `src-tauri/src/*.rs`
- Frontend: `ui/package.json`, `ui/src/**`, `ui/e2e/**`
- Automation: `.github/workflows/*.yml`, `scripts/*.sh`, `scripts/*.mjs`
- Governance artifacts: `docs/bill-of-materials.html`, `docs/publish-readiness-checklist.html`,
  `docs/roadmap/*`, `README.md`, `SECURITY.md`

## Active Governance and Planned Slices

- `BI-047` — F-047 Desktop parity evaluation and host decision (in progress)
- `BI-052` — F-052 Dependabot esbuild remediation (in progress)
- `RT-RC-001` — GTK/glib dependency-floor governance (active)
- `RT-RC-002` — GTK-free host migration planning (active)

## Validation Snapshot (Latest Local Run)

- `cargo test --locked --manifest-path src-tauri/Cargo.toml --test roadmap_coherence_tests` passes for the active host-migration coherence contract.
- `cargo test --locked --manifest-path src-tauri/Cargo.toml --test security_advisory_exception_tests -- --nocapture` can currently be built and is part of the slice validation surface.
- `publish-readiness-checklist.html` still treats the remote Dependabot alert and some release validation steps as open and blocked until CI confirms closure.

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

- `ui/pnpm-lock.yaml` lockfile currently resolves `esbuild@0.28.1` and
  `scripts/check-esbuild-lock.mjs` passes locally.
- Remote Dependabot alert gate check still reports open `GHSA-g7r4-m6w7-qqqr` via
  `scripts/check-dependabot-esbuild-alert.sh` and must be re-checked in network-available CI.

## Release Artifacts

- Backend coverage artifact: `target/coverage/lcov.info` (published as `rust-coverage-lcov`).
- Policy and exception records: `docs/roadmap/security-advisory-exceptions.json`.
- CI contract for release-floor and feature-to-feature mapping tests: `src-tauri/tests/ci_gate_tests.rs`.
