# Bill of Materials - Markdown Snapshot

_Captured: 2026-10-05_

## Repository and Source Control

### Current local execution snapshot (2026-10-05)

PR #108 is merged to `main` at `eb83be9`. The active follow-up is PR #112 on
`fix/weighted-rollup-aggregation`, based on that main tip. The latest recorded
hosted snapshot is PR head `27b9e21`: CI run `37311344183` had 24 passed
checks, no failures, and one informational coverage skip. Security run
`37311344136` and Dependency Review run `37311344143` passed. At the last
successful GitHub refresh, PR #112 was OPEN/CLEAN, its review threads were
resolved, and the live zero-open-alert query passed. A fresh GitHub refresh in
this pass could not connect, so these are last-known remote facts.

Tauri/Wry and GTK/GLib are removed from supported application manifests and
lockfiles. Fifteen withdrawn/absent advisory entries were closed with evidence;
the two remaining affected package paths were removed. The exception registry
and audit ignore list are empty. The last successful fresh audits passed both
supported lockfiles against RustSec revision
`ef6173cbc5c50ec8166f9a5b28f07834144373ee`; a new fetch in this pass failed,
while cached `--no-fetch --deny warnings` scans still passed. DuckDB remains a
checksum-verified prebuilt library and is never compiled from source. The hosted UI lane passed
with pinned pnpm `12.9.1`; Linux/macOS/Windows package and URL lifecycle checks
also passed on `27b9e21`. Local macOS lifecycle checks passed URL delivery and
saved-state restart, while physical tray-menu delivery and foreground
activation remain unverified (`accepted=false`).

- Repository: `https://github.com/pratik-saptarshi/rocinante`
- Primary branch: `main`
- Remote: `origin`
- Current work is on PR #112's follow-up branch, `fix/weighted-rollup-aggregation`.
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
- `BI-046` — F-046 GTK/glib dependency-floor governance (fresh audits and hosted governance passed on `27b9e21`; refresh current remote checks when connectivity returns)
- `BI-048` — F-048 Core extraction and host-agnostic contract (completed locally; nine contract tests pass)
- `BI-049` — F-049 GTK-free native desktop MVP (in progress; broader platform and user-visible parity gaps remain)
- `BI-050` — F-050 Parity closure and fallback containment (planned; must-have gaps need implementation or an approved, documented deferral)
- `BI-051` — F-051 Tauri/GTK/GLib retirement (dependency/removal gate and hosted check passed on `27b9e21`; refresh current remote checks when connectivity returns)
- `BI-052` — F-052 Dependabot esbuild remediation (lock floor passes; live Dependabot query returned zero open alerts on 2026-10-05)
- `BI-053` — F-053 CI bootstrap and workflow parseability (completed; validated on PR run `28983234703`)
- `BI-054` — F-054 CI lane orchestration and gating (completed; validated on PR run `28983234703`)
- `BI-055` — F-054 CI lane orchestration and gating (completed)
- `BI-056` — F-055 Release-path performance optimization (completed)
- `BI-057` — CI bootstrap + workflow parseability recovery (Red->Green complete)
- `RT-RC-001` — GTK/glib dependency-floor governance (active)
- `RT-RC-002` — GTK-free host migration planning (active)

## Historical Validation Snapshot (2026-10-04)

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

## Earlier Validation Snapshot (2026-10-05; source head `ff367c4`)

- CI run `37305039584` passed the full workspace tests, Rust formatting and
  quality gates, UI quality with pnpm `12.9.1`, security governance, aggregate
  gate, Linux/macOS/Windows packages, URI lifecycle, Windows registration, and
  Linux visible notification delivery. It reported 24 passed checks, no
  failures, and one informational coverage skip. Security run `37305039580`
  passed RustSec audit, CodeQL, and secret scan; Dependency Review passed.
- Both supported lockfiles passed fresh unfiltered audits with
  `--deny warnings` against RustSec revision
  `ef6173cbc5c50ec8166f9a5b28f07834144373ee` (1,290 advisories; 517 app and 82
  migration-tool dependencies). Governance reports zero exception entries and
  zero audit ignores. The live Dependabot query returned no open alerts.
- The DuckDB prebuilt-only and desktop dependency-floor guards pass locally.
  Linux/macOS/Windows package checks verified runtime loading without a source
  build. The desktop dependency guard excludes GTK, GLib, Tauri, and Wry across
  all workspace targets and features; this closes BI-051 on the source head.
- The installed macOS acceptance passed cold/warm URL delivery,
  close-to-tray/Show/Quit handling, notification request, visible restore, and
  saved-state restart. AppKit returned `accepted=false`; physical tray-menu
  click delivery and foreground activation remain unverified. BI-049/BI-050
  also retain the parity gaps listed in the desktop parity matrix.
- This earlier snapshot is retained for per-check history. The latest recorded
  hosted snapshot is PR head `27b9e21`; a fresh GitHub status refresh remains
  pending network availability.

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
  fetching the exact pin. Hosted `ui-quality` passed typecheck, unit tests, and
  production build with the pinned version on source head `27b9e21`.
- The PR workflow selects Rust lint and workspace tests for Cargo manifest,
  lockfile, workflow, and Rust source changes. The CI-scope contract passes.
- DuckDB remains an official checksum-verified prebuilt and the source-build
  feature guard passes. Linux/macOS/Windows package and URL lifecycle checks
  passed on `27b9e21`; local macOS acceptance also passed URL and restart checks.
- The advisory registry and Cargo audit ignores are empty. Fresh unfiltered
  audits pass both supported lockfiles on RustSec revision
  `ef6173cbc5c50ec8166f9a5b28f07834144373ee`; hosted audit and governance
  passed on `27b9e21`. No owner acceptance or review-date renewal was inferred.
- Formatting, warning-denied Clippy, and the full serial workspace test suite
  pass locally. Hosted workspace tests and aggregate passed on `27b9e21`; a
  fresh live PR check refresh remains required when GitHub is reachable.
- BI-049/BI-050 remain open for native parity gaps. In particular, physical
  tray-menu delivery and macOS foreground activation are not proven by the
  scripted lifecycle checks.

## Release Artifacts

- Backend coverage artifact: `target/coverage/lcov.info` (published as `rust-coverage-lcov`).
- Policy and exception records: `docs/roadmap/security-advisory-exceptions.json`.
- CI contract for release-floor and feature-to-feature mapping tests: `src-tauri/tests/ci_gate_tests.rs`.
