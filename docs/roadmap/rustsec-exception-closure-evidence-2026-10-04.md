# RustSec Exception Closure Evidence (2026-10-04)

**Current follow-up status (2026-10-05):** the 15 withdrawal/absence
dispositions below remain evidence-backed. Phase 4C subsequently removed the
two remaining affected package paths from both supported lockfiles, without
security-owner acceptance or review-date extension. The active registry and
Cargo audit ignore list are empty, both lockfiles pass fresh unfiltered audits,
and hosted governance, RustSec audit, and CodeQL passed on PR #112 source head
`ff367c4`. The live Dependabot query returned zero open alerts. Sections below
that describe two retained exceptions are the pre-retirement snapshot and are
superseded by the dependency-avoidance closure. Final readiness must use
terminal-green hosted checks from the latest PR head.

This records the evidence for closing 15 entries from
`security-advisory-exceptions.json` and `.cargo/audit.toml`. The RustSec
advisory database was refreshed to revision
[`ef6173cbc5c50ec8166f9a5b28f07834144373ee`](https://github.com/RustSec/advisory-db/commit/ef6173cbc5c50ec8166f9a5b28f07834144373ee),
last updated 2026-10-03. The repository owner authorized closing these 15 entries on
withdrawal or supported-lockfile absence evidence.

## Withdrawn advisories

The following eight upstream records have `withdrawn = "2026-08-14"` at the
database revision above:

- `RUSTSEC-2024-0411` — `gdkwayland-sys`
- `RUSTSEC-2024-0412` — `gdk`
- `RUSTSEC-2024-0413` — `atk`
- `RUSTSEC-2024-0415` — `gtk`
- `RUSTSEC-2024-0416` — `atk-sys`
- `RUSTSEC-2024-0418` — `gdk-sys`
- `RUSTSEC-2024-0419` — `gtk3-macros`
- `RUSTSEC-2024-0420` — `gtk-sys`

## Advisory packages absent from supported lockfiles

These seven advisory records remain present upstream. Their affected packages
do not appear in either supported Cargo lockfile, so these dependency paths are
avoided by the current application and isolated migrator graphs:

| Advisory | Affected package |
| --- | --- |
| `RUSTSEC-2024-0384` | `instant` |
| `RUSTSEC-2025-0057` | `fxhash` |
| `RUSTSEC-2025-0075` | `unic-char-range` |
| `RUSTSEC-2025-0080` | `unic-common` |
| `RUSTSEC-2025-0081` | `unic-char-property` |
| `RUSTSEC-2025-0098` | `unic-ucd-version` |
| `RUSTSEC-2025-0100` | `unic-ucd-ident` |

The lockfile inventory contains only `src-tauri/Cargo.lock` and
`tools/sled-migration/Cargo.lock`; searches of both files found none of the
seven package names. A refreshed unfiltered audit of the 715-package app graph
reported zero vulnerabilities and only the two live warnings listed below.
The 82-package migrator graph reported no findings or warnings.

## Historical exceptions retained at the pre-retirement snapshot

The following active findings remain in both registries with their existing
review date (`2026-08-06`). No date was extended and no acceptance was inferred:

| Advisory | Finding | Status |
| --- | --- | --- |
| `RUSTSEC-2024-0370` | `proc-macro-error 1.0.4` is unmaintained | Awaiting security-owner disposition |
| `RUSTSEC-2024-0429` | `glib 0.18.5` has unsound iterator implementations | Awaiting security-owner disposition |

With an empty ignore list, the app audit reports exactly these two warnings;
with only these two entries configured, `cargo audit --deny warnings` passes
for both lockfiles. The advisory governance checker validates two registry
entries against two Cargo ignores and still exits nonzero because both review
dates are overdue. This is the intended fail-closed state until the owner
dispositions are recorded.


## Pre-retirement refresh confirmation (2026-10-05)

The authorized rtk cargo-audit refresh then completed using a writable temporary
clone outside the repository's audit-ignore configuration. The clone fetched
no newer advisory database commit and remains at revision
ef6173cbc5c50ec8166f9a5b28f07834144373ee, last committed 2026-10-03.

The unfiltered application audit examined 715 dependencies, found no
vulnerability-class findings, and reported only RUSTSEC-2024-0370
(proc-macro-error 1.0.4, unmaintained) and RUSTSEC-2024-0429 (glib 0.18.5,
unsound). With --deny warnings it correctly exits nonzero. The isolated
migration lockfile audit examined 82 dependencies and passed --deny warnings
with no findings or warnings. The application ignore configuration and JSON
registry still contain only these two active IDs; the governance checker fails
closed because both review dates are overdue. The clean migration audit and
withdrawal/absence evidence for the other 15 records are unchanged.

## Dependency-avoidance closure (2026-10-05)

The GTK/Wry/Tauri application host was retired from the supported workspace:
the root package no longer declares a binary, Tauri runtime, or `tauri-build`;
its Tauri bootstrap and installer configurations were removed. The regenerated
`src-tauri/Cargo.lock` and the separate `tools/sled-migration/Cargo.lock` both
exclude `glib`, `proc-macro-error`, GTK, Wry, and the Tauri runtime. The
workspace still contains `tauri-winrt-notification`, a Windows notification
backend selected by `notify-rust`; it is not the Tauri desktop runtime and does
not reintroduce either advisory package.

The repository owner required the two live advisories to be eliminated or
completely avoided. Their packages are now absent from both supported lockfiles,
so no security-owner acceptance or review-date extension was used. The active
registry and Cargo audit ignore list are empty. The governance checker and its
empty-registry contract pass.

Local `cargo check --offline --workspace --all-targets`, all-target/all-feature
Clippy, formatting, and the full workspace test suite in serial mode pass. The
storage suite passes with two test threads; the default-parallel run stalls on
this machine. From the cached 1,290-advisory RustSec database at revision
`ef6173cbc5c50ec8166f9a5b28f07834144373ee`, unfiltered
`cargo audit --no-fetch --deny warnings` passes for the current application
lockfile (517 dependencies) and migration lockfile (82 dependencies). A fresh
database fetch with a writable temporary Cargo home failed while connecting to
GitHub, so the audit pass is not a fresh-upstream result.

The six native-shell package contract tests and macOS installed lifecycle
acceptance pass locally. UI typecheck, 63 unit tests, and production build pass
as diagnostics under global pnpm 12.8.1; exact pinned pnpm 12.9.1 retrieval
failed because registry DNS did not resolve. Same-head hosted Security,
pinned-UI, Linux/Windows package and lifecycle, and aggregate checks remain
required before Phase 5 and Phase 6 close.
