# Bill of Materials - Markdown Snapshot

_Captured: 2026-10-03_

## Repository and Source Control

- Repository: `https://github.com/pratik-saptarshi/rocinante`
- Primary branch: `main`
- Remote: `origin`
- Current working slice is `main` after merging `feat/bi-047-decision-paths`.
- Roadmap source-of-truth for execution: `docs/roadmap/bead-issue-tracker.html`

## Branch and Sync State

- The remediation branch `fix/rocinante-readiness-remediation` is based on
  `main` at `4c28d9f`. It preserves the existing BI-048 extraction and
  governance edits as working changes; they are not committed or pushed.
- Remaining open slices continue via PR checkpoints with explicit roadmap/checklist
  evidence and conventional commits.
- Lane-scope refinement now keeps docs-only and non-functional edits out of storage/coverage-heavy lanes while preserving core rust gate visibility.

## Runtime Surface

- Backend: `src-tauri/Cargo.toml`, `src-tauri/src/*.rs`
- Host-agnostic Rust contracts: `src-tauri/crates/rocinante-core/`
- GTK-free desktop shell: `src-tauri/crates/rocinante-desktop-shell/` (eframe/winit, rfd XDG portal, optional notify-rust, tray-icon/ksni, and fs2 inbox locking)
- Shared authenticated repository analysis: `src-tauri/crates/rocinante-analysis/`
- Frontend: `ui/package.json`, `ui/src/**`, `ui/e2e/**`
- Automation: `.github/workflows/*.yml`, `scripts/*.sh`, `scripts/*.mjs`
- Governance artifacts: `docs/bill-of-materials.html`, `docs/publish-readiness-checklist.html`,
  `docs/roadmap/*`, `README.md`, `SECURITY.md`

## Active Governance and Planned Slices

- `BI-047` — F-047 Desktop parity evaluation and host decision (completed on PR run `28988956969`)
- `BI-046` — F-046 GTK/glib dependency-floor governance (in progress; 17 exception reviews overdue since 2026-08-06)
- `BI-048` — F-048 Core extraction and host-agnostic contract (completed locally; nine contract tests pass)
- `BI-049` — F-049 GTK-free native desktop MVP (in progress; eframe/winit shell with lossless paths, authenticated scans, saved-metric reload, and desktop notification requests)
- `BI-052` — F-052 Dependabot esbuild remediation (tracked esbuild alert confirmed closed by live query on 2026-09-30; other release blockers remain)
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
- `scripts/dependency-floor-proof.sh` confirms GTK 0.18.2 and GLib 0.18.5 remain in the Tauri/Wry tree; the registry's 17 `review_by` dates (2026-08-06) are overdue.
- Core-only contract suite has nine passing tests, including host-independent claim validation and an admin-authorized release-baseline repository contract. The no-default-features test targets compile, and the default analytics library check passes. The auth integration test executable build was cancelled after several minutes without progress.
- The esbuild version remediation is merged in PR #59 and the lockfile is at `0.28.1`; a live query on 2026-09-30 confirmed its Dependabot alert is closed. Main CI run 29415214522 logged a malformed response and did not verify closure. Five other alerts and GTK/GLib alert #1 remain recorded as release blockers.
- Hosted Security run [36424294477](https://github.com/pratik-saptarshi/rocinante/actions/runs/36424294477) failed on 2026-09-28 with vulnerable `rustls@0.23.40`, vulnerable `rkyv@0.7.46`, and yanked `chacha20@0.10.1`. The local lockfile now updates `rustls` to 0.23.45, `rust_decimal` to 1.43.0 (removing `rkyv@0.7.46`), `chacha20` to 0.10.2, and `serde_with` plus macros to 3.21.0. The documented root audit command passes; hosted validation remains outstanding. The live `serde_with` Dependabot alert is still open on main pending merge.
- `publish-readiness-checklist.html` remains open because RT-RC-001 is still active and publish still requires a formal release branch / merge checkpoint, even though BI-047 is merged and the CI recovery and CI lane slices now pass their latest remote checks.
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
  - `rust-workspace-tests` (full-workspace clippy and tests)
  - `ui-quality` (pnpm `12.9.0`, typecheck, unit tests, production build)
  - `rust-coverage` (release-only coverage)

## Current Remediation State Notes

- On 2026-10-03, `cargo update` refreshed 193 Rust lockfile packages to their
  latest semver-compatible stable releases; this includes Tauri `2.12.1` and
  `serde_with` `3.24.0`. Cargo audit refreshed RustSec and completed with no
  findings. Full-workspace clippy completed with no issues after moving the
  trailing analysis function ahead of its test module. Full-workspace tests
  are compiling the updated desktop analyzer and native dependency stack.
- On 2026-10-03, UI direct dependencies were updated to current stable releases:
  MUI `9.4.0`, React `19.3.0`, Vite `8.3.2`, Vitest `5.0.3`, TypeScript
  `7.0.2`, Playwright `1.63.0`, and esbuild `0.28.2`. `pnpm outdated` reports
  no outdated direct dependencies, and the full pnpm audit reports no known
  vulnerabilities. Typecheck and production build pass after the MUI v9
  system-props codemod; the build reports a chunk-size advisory. Vitest has
  stalled before reporting test results.
- Rust formatting, the advisory-governance, roadmap-doc, Dependabot-checker,
  and native-shell dependency-guard contracts pass. The live advisory checker
  still fails closed for all 17 overdue reviews; no owner disposition is
  recorded. UI unit tests have not returned results. Hosted checks have not
  run for this branch.
- CI now tests the full Cargo workspace on Rust changes and routes changes to
  `.github/workflows/ci.yml` through the Rust checks. Cross-platform native URL
  and restart acceptance remains dependent on the corresponding hosted jobs.
- The UI manifest pins pnpm `12.9.0`, the latest stable release verified on
  2026-10-03. Local validation of this version is pending.

- The latest UI lockfile resolves `esbuild@0.28.2`; the version-floor checker still requires a fresh run.
- At the 2026-09-30 snapshot, the UI suite and coverage passed 62 tests, the
  build passed, and the lockfile used `esbuild@0.28.1`, Vitest `4.1.11`, and
  PostCSS `8.5.23`. The 2026-10-03 update supersedes those dependency versions;
  current audit and validation results are recorded above.

## Release Artifacts

- Backend coverage artifact: `target/coverage/lcov.info` (published as `rust-coverage-lcov`).
- Policy and exception records: `docs/roadmap/security-advisory-exceptions.json`.
- CI contract for release-floor and feature-to-feature mapping tests: `src-tauri/tests/ci_gate_tests.rs`.
