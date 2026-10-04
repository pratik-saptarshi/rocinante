# RustSec Zero-Exception Remediation Plan

**Status:** Approved plan; implementation and validation remain outstanding.
**Decision record:** [`docs/decisions/decision-2026-10-04.md`](../decisions/decision-2026-10-04.md)
**Scope:** PR #108 readiness branch and supported Rust/native application dependency graphs.

## Goal

Remove all applicable RustSec findings from every supported dependency graph,
including migration tools, without advisory ignores or accepted exceptions.
The gate passes only when a fresh RustSec database reports no vulnerability,
unsoundness, or unmaintained-package findings with `--deny warnings`.

DuckDB remains the embedded analytics engine through verified, official,
precompiled shared libraries. The project must never compile DuckDB from source.

## Verified starting point

On 2026-10-04, an unfiltered audit of PR branch commit `7acd1a0` inspected 722
locked packages using `cargo-audit 0.22.2` and RustSec database revision
`ef6173cbc5c50ec8166f9a5b28f07834144373ee` (database timestamp
`2026-10-03T10:14:03+02:00`). It found zero vulnerability-class reports and
four advisory warnings:

| Advisory | Locked package | Dependency path | Required disposition |
|---|---|---|---|
| RUSTSEC-2024-0429 | `glib 0.18.5` | GTK -> Tauri/Wry | Retire the affected host dependency graph. |
| RUSTSEC-2024-0370 | `proc-macro-error 1.0.4` | GLib/GTK macros -> Tauri/Wry | Retire the affected host dependency graph. |
| RUSTSEC-2024-0384 | `instant 0.1.13` | Sled -> `parking_lot 0.11` | Remove from application and migration-tool graphs. |
| RUSTSEC-2025-0057 | `fxhash 0.2.1` | Sled | Remove from application and migration-tool graphs. |

The same registry holds 17 entries, all due 2026-08-06. RustSec marks eight
GTK-family entries withdrawn on 2026-08-14: RUSTSEC-2024-0411, 0412, 0413,
0415, 0416, 0418, 0419, and 0420. Five more entries refer to absent lockfile
packages: RUSTSEC-2025-0075, 0080, 0081, 0098, and 0100. Remove those 13
obsolete ignores and registry entries together after preserving evidence in
the dated disposition record. Keep the four applicable findings active until
their packages are eliminated. Do not renew dates or infer owner acceptance.

The existing hosted audit passed with repository ignores enabled. That result
does not establish the zero-exception goal. The unfiltered local audit is the
baseline for remediation.

## Implementation decisions and work

### Replace Sled ingestion with SQLite

SQLite ingestion plus DuckDB analytics supersedes the repository's previous
Badger-only target architecture. Reuse `rusqlite` and preserve the public
command names and payloads, authorization, sanitization, retention, scoring,
baseline behavior, and DuckDB analytics queries. Implement transactional,
byte-preserving key/value writes, ordered prefix scans, and deletes in a
dedicated SQLite database. Keep explicit path overrides and use the shared
per-user app-data directory for desktop defaults.

Make promotion restart-safe: commit analytical rows and a unique source-event
receipt together in DuckDB, then acknowledge/delete the corresponding SQLite
events. Replays must not lose events or duplicate analytical rows. Replace
Badger/Sled-specific backend settings with truthful SQLite configuration;
legacy configuration must fail with an actionable migration message rather
than silently redirecting data.

### Preserve legacy Sled data with an audited migration utility

Include a dedicated Sled reader in the audited workspace. Use a narrowly
maintained patch of the original Sled source that retains package identity,
version provenance, and licensing. Replace its `fxhash` map/set hashers with
standard-library hashers and upgrade `parking_lot` from 0.11 to the already
resolved 0.12 line. Verify compatibility against real stores. Audit every
reader dependency and feature normally; do not hide the reader from the
workspace audit or evade advisory matching by renaming the package.

Migration must stop writers, acquire exclusive ownership, copy the legacy
store, and read from the copy. Preserve all tree/key/value records and existing
DuckDB history. Verify sorted, length-delimited record hashes and SQLite
integrity before publishing. Preserve the original store, make retries
idempotent, and reject corrupt, unsupported, or conflicting stores without
overwriting them. Roll back before accepting new writes; after cutover, recovery
must retain new SQLite writes.

### Use only prebuilt DuckDB shared libraries

Remove DuckDB's `bundled` feature from every declaration and disable default
features explicitly. Reject `bundled-cmake` and any enabled feature that
transitively compiles DuckDB. Keep the embedded Rust API and dynamically link
official `.so`, `.dylib`, or `.dll` releases matching the Rust binding version.

Add one shared provisioning mechanism for local builds, CI, tests, and
packaging. Pin the binding/engine version pair and record each supported
platform artifact's official URL and SHA-256. Download to a version/target
cache, verify before use, and configure explicit library/header paths. Missing,
corrupt, mismatched, or unsupported artifacts fail the build; never fall back
to compiling DuckDB or to an unverified system library. Package the verified
library with the app and use executable-relative loading on Linux, bundle
loading and signing on macOS, and a colocated DLL on Windows. Add native
version, origin, checksum, and license to the bill of materials. RustSec does
not audit this separately distributed native library, so dependency update
reviews must also verify its provenance and security release status.

Before Rust compilation, check Cargo's resolved feature graph and fail if any
DuckDB source-build feature is active. In a clean cache, verify builds do not
compile DuckDB C/C++ translation units. Test installed apps with developer
library-path variables removed and query the packaged library on each target.
Keep this no-DuckDB-compilation rule scoped to DuckDB; other dependencies may
retain their existing build requirements.

### Retire the Tauri/GTK host

Repository-folder selection satisfies the existing chooser requirement. Finish
and validate the native-shell parity gates, then make the shell the supported
desktop entry point. Remove Tauri/Wry dependencies, build hooks, tests, and
packaging paths from the supported workspace. Port Tauri integration assertions
to shared-service and native-shell tests, preserving authorization and response
shapes. Prove GTK/GLib and their macro dependencies are absent across all
targets and features; disabling a target or retaining the old host elsewhere in
the supported workspace does not count as removal.

## Governance, tests, and acceptance

- Save per-advisory status, package, dependency path, database revision, and
  closure evidence in the decision record. Remove withdrawn/absent entries
  from `.cargo/audit.toml` and the active JSON registry together.
- Replace hard-coded “17 overdue” assertions with fixtures for stale entries,
  malformed metadata, ignore/registry mismatch, empty registry, and newly
  introduced advisories.
- Require a successful RustSec database refresh and audit of every supported
  Cargo lockfile with `--deny warnings`, empty ignores, and no target/severity
  filters. Failed fetches, malformed reports, vulnerabilities, warnings, or
  nonempty active exceptions fail the gate.
- Guard against reintroducing the retired Tauri/GTK graph or the four affected
  packages. Keep the Sled reader confined to the migration utility and out of
  all application dependency paths.
- Test migration of representative stores, binary keys, multiple trees, empty
  stores, retries, interruption, corruption, conflicts, and source backup
  preservation. Test ingestion durability, retention, concurrency, shutdown,
  and replay between DuckDB commit and SQLite acknowledgement.
- Run workspace formatting, Clippy, tests, UI type checks/tests/build, and
  installed cold/warm URI, restart, window/tray, notification, and shortcut
  acceptance on Linux, macOS, and Windows. Obtain interactive evidence where
  scripts do not exercise the real desktop action.
- Require terminal green aggregate and security checks on the final commit and
  repeat the refreshed zero-exception audit before merge.

## Delivery and completion

Implement on PR #108's existing branch in reviewable commits: this plan and
evidence record; prebuilt DuckDB provisioning; SQLite backend; audited legacy
migration; native parity and Tauri retirement; final governance and docs.
Synchronize the architecture, host migration milestones, repository maps,
bill of materials, test plan, and publish checklist with verified results.

Completion requires evidence-backed dispositions for all 17 original entries,
zero current RustSec findings or warnings across supported lockfiles, no active
exceptions or audit ignores, preserved legacy data, validated desktop parity,
verified prebuilt DuckDB artifacts on every supported platform, and green
required CI/security gates. Scheduled audits must block releases if new
advisories invalidate this state.
