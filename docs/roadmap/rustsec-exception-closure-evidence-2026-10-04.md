# RustSec Exception Closure Evidence (2026-10-04)

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

## Exceptions retained pending owner disposition

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


## Refresh confirmation (2026-10-05)

The authorized rtk cargo-audit refresh completed using a writable temporary
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
