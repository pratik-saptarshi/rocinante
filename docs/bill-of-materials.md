# Bill of Materials - Markdown Snapshot

_Captured: 2026-10-05_

## Repository and Source Control

### Current local execution snapshot (2026-10-05)

The remediation worktree on PR #108 is based on pushed commit `3020d9e` and
contains uncommitted host-retirement and zero-exception changes. Tauri/Wry and
GTK/GLib have been removed from supported workspace manifests and lockfiles.
The native-shell package contract passes locally and the installed macOS
URL/restart acceptance passes; the cross-platform package matrix has not run
hosted on this worktree.
The 15 withdrawn/absent advisory entries and the two avoided package paths are
closed with evidence; both the exception registry and audit ignore list are
empty. Governance contracts and cached RustSec audits pass. A fresh database
refresh could not connect to GitHub. Formatting, all-target/all-feature Clippy,
and the full serial workspace suite pass locally; the storage suite also passes
with two threads, while its default-parallel run stalls in the retention case.
UI typecheck, 63 tests, and production build pass as diagnostics with global
pnpm 12.8.1; pinned pnpm 12.9.1 could not be fetched due registry DNS failure.
The installed macOS URL/restart acceptance passes, but its AppKit activation
request returned false. Linux/Windows package/lifecycle and same-head hosted
validation remain required. Do not treat earlier hosted Tauri package or CI
runs as evidence for this local change.

- Repository: `https://github.com/pratik-saptarshi/rocinante`
- Primary branch: `main`
- Remote: `origin`
- Current work is on PR #108's remediation branch, `fix/rocinante-readiness-remediation`.
- Roadmap source-of-truth for execution: `docs/roadmap/bead-issue-tracker.html`

## Historical hosted branch snapshot (2026-10-04)

- PR #108 is open on `fix/rocinante-readiness-remediation`; its latest hosted
  validation head is `1be093a`, based on `main` tip `cdd29b9`. All 47 inline
  review threads are resolved. The latest hosted aggregate failed on the then
  17 overdue security exception entries. A local evidence-backed update now
  retains two live entries; CI for that update is pending.
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
- One-time legacy-store migration: isolated `tools/sled-migration/` workspace
  and lockfile; Sled is not in the application dependency graph. The migration
  snapshot copies `conf`, `db`, and `blobs/` under the storage lock; the 1 MiB
  off-log-value regression is included in its 8-test suite.
- Governance artifacts: `docs/bill-of-materials.html`, `docs/publish-readiness-checklist.html`,
  `docs/roadmap/*`, `README.md`, `SECURITY.md`

## Active Governance and Planned Slices

- `BI-047` — F-047 Desktop parity evaluation and host decision (completed on PR run `28988956969`)
- `BI-046` — F-046 GTK/glib dependency-floor governance (in progress; registry and ignores are now empty locally, with fresh audit and hosted proof pending)
- `BI-048` — F-048 Core extraction and host-agnostic contract (completed locally; nine contract tests pass)
- `BI-049` — F-049 GTK-free native desktop MVP (in progress; eframe/winit shell with lossless paths, authenticated scans, saved-metric reload, and desktop notification requests)
- `BI-051` — F-051 Tauri/GTK/GLib retirement (in progress; host removed locally, native platform package validation pending)
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
  resources and loader paths. All three package inspections passed on hosted
  source head `1be093a` in run `37235903810`; at that source head the required
  governance lane failed on the then-current 17 overdue exception reviews.
- The provisioner contract tests (9), DuckDB feature-guard contract, CI-scope
  contract, macOS installer contract, and Linux installer/deep-link contract
  pass locally. An actual macOS shell build was packaged into an `.app`; `otool`
  shows `@rpath/libduckdb.dylib` and only the app-relative
  `@executable_path/../Frameworks` run path. The full installed macOS lifecycle
  also passes cold/warm Launch Services URLs, tray behavior, notification
  request, and saved-state restart. Hosted run
  [37235903810](https://github.com/pratik-saptarshi/rocinante/actions/runs/37235903810)
  passes Linux/macOS/Windows URL and saved-state restart acceptance, the
  three-platform Tauri DuckDB bundle checks, and Linux visible notification
  delivery. All hosted Rust lint and test shards also passed.
- The hosted full-workspace test lane passed on source head `d703a4a` in run
  `37233328503`. The workspace test run excluding the Tauri adapter passes all 83 tests,
  including SQLite persistence, prefix ordering, concurrent writes, legacy
  Sled refusal, and replay receipts. Strict Clippy passes for every
  `rocinante-storage` target. The root storage, transport, backend,
  admin-ingestion, Tauri-command, and README/API test binaries pass 17, 5, 5, 2,
  5, and 1 tests with the verified DuckDB runtime. `cargo check --tests` passes
  for all root test targets. Current local follow-up validation also passes
  all 9 analysis crate tests and analysis Clippy without warnings.
- The application lockfile (715 packages at the refreshed audit) no longer contains
  `sled`, `fxhash`, or `instant`. The separate
  `tools/sled-migration/Cargo.lock` contains the patched Sled reader, but no
  `fxhash` or `instant`. Its unfiltered audit reports zero findings and
  warnings against RustSec revision `ef6173cbc5c50ec8166f9a5b28f07834144373ee`
  (last updated 2026-10-03; a 2026-10-04 refresh returned no newer revision).
  The app audit against that revision reports
  two warnings (`glib 0.18.5` and `proc-macro-error 1.0.4`) and an empty ignore
  list; it exits nonzero with `--deny warnings`. The configured audit and
  registry now contain only those two live warnings. Eight withdrawn advisory
  records and seven entries whose packages are absent from both supported
  lockfiles were removed with evidence recorded in
  `docs/roadmap/rustsec-exception-closure-evidence-2026-10-04.md`. The two
  remaining reviews are overdue; their dates were not renewed.
- Hosted CI run `37235903810` on `1be093a` completed with code, UI, platform,
  package, workspace-test, format, Clippy, and contract jobs passed. Its
  aggregate failed because the registry at that source head still had 17
  overdue review dates. Security run `37235903812` and Dependency Review
  `37235903871` passed. CI for the local two-entry registry update is pending;
  these prior runs do not establish the zero-exception RustSec requirement.
- PR #108's required `tauri-runtime-bundle` passed on source head `1be093a`
  in run `37235903810`: Linux `.deb`, macOS `.app`, and Windows NSIS packages all
  contain the verified DuckDB runtime. The macOS check also rejects any
  remaining `duckdb-download` Cargo-cache RPATH. The aggregate remains blocked
  by the separate advisory governance failure.

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
- `cargo fmt --manifest-path tools/sled-migration/Cargo.toml --all -- --check`
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

- The current pnpm pin is `12.9.1`, the latest stable upstream release on
  2026-10-05. Local UI typecheck, 63 unit tests, and production build pass with
  installed dependencies under global pnpm `12.8.1`; registry DNS prevented
  fetching the exact pin. The hosted pinned-version UI lane remains required.
- The PR workflow selects Rust lint and workspace tests for Cargo manifest,
  lockfile, workflow, and Rust source changes. The CI-scope contract passes.
- DuckDB remains an official checksum-verified prebuilt and the source-build
  feature guard passes. Local macOS packaging/lifecycle acceptance passes;
  Linux/Windows package and lifecycle runs are still required on this worktree.
- The advisory registry and Cargo audit ignores are empty. Governance contracts
  and cached-database audits pass for both supported lockfiles. A fresh RustSec
  database fetch could not reach GitHub, and hosted Security checks remain
  pending; no owner acceptance or review-date renewal was recorded.
- Formatting, all-target/all-feature Clippy, and the full serial workspace test
  suite pass locally. Same-head hosted aggregate validation remains outstanding;
  earlier green Tauri-head results do not validate the native-shell retirement.

## Release Artifacts

- Backend coverage artifact: `target/coverage/lcov.info` (published as `rust-coverage-lcov`).
- Policy and exception records: `docs/roadmap/security-advisory-exceptions.json`.
- CI contract for release-floor and feature-to-feature mapping tests: `src-tauri/tests/ci_gate_tests.rs`.
