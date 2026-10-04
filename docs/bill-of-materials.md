# Bill of Materials - Markdown Snapshot

_Captured: 2026-10-04_

## Repository and Source Control

- Repository: `https://github.com/pratik-saptarshi/rocinante`
- Primary branch: `main`
- Remote: `origin`
- Current work is on PR #108's remediation branch, `fix/rocinante-readiness-remediation`.
- Roadmap source-of-truth for execution: `docs/roadmap/bead-issue-tracker.html`

## Branch and Sync State

- PR #108 is open on `fix/rocinante-readiness-remediation` at published head
  `b991f99`, based on `main` at `cdd29b9`. Continue on this branch through its
  protected review/check flow; all 30 inline review threads are resolved.
- The complete phase sequence and exit evidence are in
  `docs/roadmap/rustsec-zero-exception-remediation-plan.md`.
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

## Validation Snapshot (2026-10-04)

- The UI lockfile is frozen and installs with `pnpm@12.9.1`, the latest
  upstream stable release verified on 2026-10-04. TypeScript build checking,
  all 62 Vitest cases, and the production build pass. Vite reports one 506 KB
  chunk-size advisory; it does not fail the build.
- DuckDB is pinned to Rust binding `1.10506.0` / engine `1.5.6`, dynamically
  linked from the platform-specific official archive. The archive, extracted
  library, and headers are SHA-256 pinned in
  `scripts/duckdb-prebuilt-artifacts.json`; build lanes verify and stage the
  binary before Cargo. DuckDB source-build features are forbidden by a tested
  feature-graph guard. Tauri production bundles now use verified per-platform
  resources and loader paths; the new three-platform bundle gate must pass on
  the current PR head before installed Tauri packaging is considered verified.
- The provisioner contract tests (9), DuckDB feature-guard contract, CI-scope
  contract, macOS installer contract, and Linux installer/deep-link contract
  pass locally. An actual macOS shell build was packaged into an `.app`; `otool`
  shows `@rpath/libduckdb.dylib` and only the app-relative
  `@executable_path/../Frameworks` run path. The full installed macOS lifecycle
  also passes cold/warm Launch Services URLs, tray behavior, notification
  request, and saved-state restart. Hosted run
  [37198628846](https://github.com/pratik-saptarshi/rocinante/actions/runs/37198628846)
  passes Linux/macOS/Windows URL and saved-state restart acceptance, and Linux
  visible notification delivery.
- The analysis/core/storage/desktop-shell workspace test suite passes all 75
  tests, including the verified DuckDB engine-version query and SQLite
  migration-path test. Strict Clippy passes for those extracted crates. Full
  root Tauri tests and full-workspace Clippy did not return terminal results on
  this Mac; the root test compile produced no new artifacts for 16 minutes
  before interruption. Hosted run `37198628846` now passes full workspace
  tests, Rust quality gates, formatting, Clippy, and core/storage test lanes.
- The refreshed unfiltered Cargo audit has four warnings on the existing graph:
  GLib through Tauri/Wry and `fxhash` / `instant` through Sled, plus the
  unmaintained `proc-macro-error`. No advisory dates have been renewed; the
  governance gate remains fail-closed until the phased removal work is done.
- The hosted aggregate still fails closed because `security-exception-governance`
  lists all 17 review dates as overdue. The security workflow and Dependency
  Review pass under the existing exception configuration; neither proves the
  zero-exception RustSec requirement.
- PR #108 now includes `tauri-runtime-bundle`, which creates Linux `.deb`,
  macOS `.app`, and Windows NSIS packages and checks for their DuckDB runtime.
  This was added in response to a new P1 review finding; record the check's
  terminal result and resolve its thread only after the package evidence passes.

## Dependency Controls and Security Gate Stack

- Rust toolchain: `1.99.0` in CI (`rust-toolchain.toml` remains the local floor)
- CI release floor: `src-tauri/Cargo.toml`, `.cargo/audit.toml`
- UI floor check: `scripts/check-esbuild-lock.mjs`
- Dependabot alert gate: `scripts/check-dependabot-esbuild-alert.sh` on `main` lane
- DuckDB binding/engine: `duckdb` `1.10506.0` / DuckDB `1.5.6`, the official
  current stable engine on 2026-10-04 ([release](https://github.com/duckdb/duckdb/releases/tag/v1.5.6)), MIT; official
  native artifacts and archive/library/header SHA-256 values are pinned in
  `scripts/duckdb-prebuilt-artifacts.json`. DuckDB must never be compiled from
  source; installed apps package the matching `.so`, `.dylib`, or `.dll`.
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
- `ui-quality` (pnpm `12.9.1`, typecheck, unit tests, production build)
  - `rust-coverage` (release-only coverage)

## Current Remediation State Notes

- The current pnpm pin is `12.9.1`; the frozen install and UI typecheck, unit
  suite, and production build pass locally. The direct dependency set was
  already at its latest stable versions before the pnpm metadata refresh.
- The PR workflow selects Rust lint and workspace tests for Cargo manifest,
  lockfile, workflow, and Rust source changes. The CI-scope path contract passes.
- DuckDB prebuilt provisioning and installed app packaging are validated
  locally and in the hosted Linux, macOS, and Windows lifecycle jobs. Rust test
  jobs also stage the verified runtime in Cargo's `debug/deps` directory.
- The advisory gate is intentionally blocked by 17 overdue registry
  dispositions and four current audit warnings. No review dates or owner
  acceptance statements have been fabricated. The phased zero-exception
  remediation sequence is documented in the roadmap plan.
- The dependency-floor and exception-registry phase is not complete. Do not
  use historical green CI runs above as evidence for PR #108's current head.
  Current evidence is the terminal run `37198628846`; the four unfiltered
  RustSec warnings and 17 overdue registry entries still block release.

## Release Artifacts

- Backend coverage artifact: `target/coverage/lcov.info` (published as `rust-coverage-lcov`).
- Policy and exception records: `docs/roadmap/security-advisory-exceptions.json`.
- CI contract for release-floor and feature-to-feature mapping tests: `src-tauri/tests/ci_gate_tests.rs`.
