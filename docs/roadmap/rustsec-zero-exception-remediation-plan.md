# RustSec Zero-Exception Remediation Plan

**Status:** The previous Phase 0 findings are resolved. The current PR review
round found a Tauri resource-glob failure, an aggregate-gate omission, Rust
scope misclassification, missing macOS post-link signing, launcher-relative
storage paths, and scan-root-dependent repository names. Local fixes are
implemented and targeted contracts pass; hosted rerun 37204604888 predates
these fixes. The local storage phase now
removes Sled from the application lockfile and adds SQLite WAL ingestion with
replay receipts. Its local audit against the cached 2026-10-03 RustSec database
reports two remaining warnings (`glib` and `proc-macro-error`); a fresh database
fetch failed because GitHub was unreachable. Governance still fails closed on
17 overdue entries. Existing Sled stores must complete Phase 3 before upgrade.
**Decision record:** [`docs/decisions/decision-2026-10-04.md`](../decisions/decision-2026-10-04.md)
**Scope:** PR #108 readiness branch and supported Rust/native application dependency graphs.

## Goal

Remove all applicable RustSec findings from every supported dependency graph,
including migration tools, without advisory ignores or accepted exceptions.
The gate passes only when a fresh RustSec database reports no vulnerability,
unsoundness, or unmaintained-package findings with `--deny warnings`.

DuckDB remains the embedded analytics engine through verified, official,
precompiled shared libraries. The project must never compile DuckDB from source.
The official DuckDB release page and install selector report `1.5.6` as the
current stable version on 2026-10-04; pin the matching Rust binding and artifact
hashes together.

## Phased execution and validation gates

Execute these phases in order on PR #108's existing branch. A phase closes
only when its implementation is committed and every validation listed for
that phase has terminal evidence. Keep the security gate fail-closed throughout
the work; do not renew or invent advisory acceptance.

### Phase 0 — Reproduce the baseline and clear current review blockers

Record the current supported lockfiles, resolved dependency paths, RustSec
database revision, unfiltered audit output, PR check state, and unresolved
review threads. Review the open PR #108 threads and handle their findings:

- [Governance must not make every PR fail](https://github.com/pratik-saptarshi/rocinante/pull/108#discussion_r4172600314): keep the required gate and clear the obsolete exception entries only when withdrawal or package-absence evidence is recorded and the remaining package paths are actually removed.
- [Cargo manifests must run the Rust lanes](https://github.com/pratik-saptarshi/rocinante/pull/108#discussion_r4176536547): classify `src-tauri/**` before generic manifest/metadata patterns and add a contract check proving manifest and lockfile changes select Rust validation. This is implemented and the standalone classifier contract passes.
- [The public scan example must match the authenticated API](https://github.com/pratik-saptarshi/rocinante/pull/108#discussion_r4176644619): correct the README example or preserve a compatible wrapper, and verify the documented call compiles.
- [Stage the Windows installer DLL fixture](https://github.com/pratik-saptarshi/rocinante/pull/108#discussion_r4177140136): copy the provisioned, hash-verified DLL into the test fixture. The Windows registration and cold/warm/restart job passed on the current PR head.
- [Populate CI scope fallback outputs](https://github.com/pratik-saptarshi/rocinante/pull/108#discussion_r4177150369): delegate fallback output generation to the scope classifier in forced-Rust mode, and contract-test that the workflow uses that path.
- The metrics-read lock and graceful-shutdown concern is already tracked as BI-060 in `docs/roadmap/bead-issue-tracker.html`, following the author's request to handle it as a distinct feature.
- The UI pin was `pnpm@12.9.0`; the current npm stable registry reports `12.9.1`. Keep `ui/package.json`, the CI action, lockfile metadata, README, bill of materials, and codemap synchronized to that version.

The governance decision thread is resolved with an explicit policy response.
The fail-closed gate remains required until the registry and affected
dependency graphs are actually clean. Do not weaken it to make this PR green,
renew dates, or represent an owner disposition that was not provided.

The latest review round is also part of Phase 0. Make the Tauri runtime bundle
matrix a required dependency of the aggregate gate and fail if any matrix leg
is skipped or fails. Stage the SHA-verified DuckDB binary into an existing
resource directory before Tauri's Cargo build-script validation, while keeping
the post-build loader rewrite before packaging. Route rust-toolchain.toml,
.cargo/config.toml, and .cargo/config changes through Rust validation.
Sign the macOS library, executable, and final bundle after all Mach-O edits,
then verify the signature. Default admin stores must share the stable
per-user data directory used by telemetry. Use a path-derived identity for
repository metrics so changing scan roots and duplicate basenames do not
change the persisted key; document the repo_name value-format compatibility
change. Do not close the DuckDB packaging thread until the hosted Linux,
macOS, and Windows installer matrix passes on the updated head.
Historical telemetry rows remain queryable through the previous root-derived
aliases without rewriting stored records; retain coverage for basename and
duplicate-name legacy aliases while stable hashed IDs are used for new rows.

**Validation:** inventory every Cargo lockfile; run `cargo tree` for the four
live package paths; refresh and run cargo-audit from outside the repository's
`.cargo/audit.toml` scope with no target/severity filters; confirm its JSON has
an empty `settings.ignore`; record the exact report and database revision;
inspect current PR checks and unresolved review threads. Run the new CI-scope
contract, then run UI install/typecheck/unit/build gates under the updated pnpm
pin. Compile the README/API example in the Rust integration suite after
Phase 1 stages the verified DuckDB binary; never invoke Cargo against a
bundled DuckDB source-build graph.

**Exit gate:** baseline facts are reproducible; each code finding is fixed and
its review thread is resolved or explicitly tracked; the security review
thread has a documented decision response while the fail-closed gate remains
red until the dependency and registry evidence is complete; changing any Rust
manifest/lockfile makes the Rust build, lint, and workspace-test lanes run.

**Local progress (2026-10-04):** the path-classification contract passes for
Rust manifests, lockfiles, and workflow changes. The README admin example now
uses the authenticated four-argument scan API, and the
`readme_admin_api_contract` compile-contract test covers that documented call.
The local targeted root test has not returned a terminal result. UI checks pass under pnpm
`12.9.1` (typecheck, 62 unit tests, build; Vite reports a 506 KB chunk-size
advisory). A refreshed, unfiltered RustSec audit reports zero vulnerability
findings, four warnings, and an empty ignore list. The governance gate remains
fail-closed on the 17 overdue entries; no owner acceptance or date extension
was invented. PR #108 remains relevant: it is open and mergeable, its head is
the current remediation branch, and its base matches `main` at `cdd29b9`.
Hosted CI run [37195996758](https://github.com/pratik-saptarshi/rocinante/actions/runs/37195996758)
is terminal and failed. Workflow parsing, scope detection, UI quality,
Windows registration plus cold/warm/restart URL delivery, and the build-seed
job passed. Linux and macOS acceptance could not find the staged DuckDB runtime
beside the Cargo output binary; root workspace tests and the Rust quality gate
failed three contract assertions that still expected the old inline scope
logic; governance rejected the 17 overdue exception records. Security run
[37195996741](https://github.com/pratik-saptarshi/rocinante/actions/runs/37195996741)
passed Rust audit, secret scan, and CodeQL, but the audit still uses the current
exceptions. Dependency Review run `37195996688` also passed.

The working tree now stages the checksum-verified DuckDB runtime into the
binary's sibling `deps` directory before macOS/Linux packaging acceptance,
updates the root Rust contracts to inspect the extracted scope classifier,
and routes the baseline-missing CI fallback through that classifier in
forced-Rust mode so every scope output is populated. The standalone
CI-scope contract and provisioner suite (7/7), Rust formatting check, and
`git diff --check` pass. The local staging command verified the pinned macOS
runtime and its SHA-256. The targeted root Rust test did not produce a
terminal result during this run, and the macOS GUI acceptance could not scan
its temporary bundle through Launch Services in this sandbox. Neither is
claimed as a pass; the fixed code is awaiting hosted reruns.

**Current Phase 0 follow-up (2026-10-04; PR head `b991f99`):** all 30 inline
review threads are resolved. The empty-repository metrics query now returns a
clear error before opening its database, with a focused regression test. The
root workspace tests previously failed at runtime because the staged DuckDB
library was not in Cargo's `debug/deps`; the provisioner and four CI test jobs
now stage the verified runtime there, guarded by the CI scope contract.

**New review finding (2026-10-04; PR head `038f7c4`):** the subsequent review
opened P1 thread
[`discussion_r4177301463`](https://github.com/pratik-saptarshi/rocinante/pull/108#discussion_r4177301463)
on `src-tauri/Cargo.toml`: a production Tauri installer did not bundle the
dynamically linked DuckDB runtime. The branch now provisions the verified
prebuilt before the Cargo build, stages it in Tauri's bundle resource path,
rewrites Linux/macOS loader paths, and leaves the Windows DLL beside the app
executable. A required matrix builds Linux `.deb`, macOS `.app`, and Windows
NSIS packages and inspects each for its runtime. Local contract tests pass
7/7, and actionlint passes; the review thread stays open until this hosted
matrix passes on the updated branch head.

**README follow-up (2026-10-04; PR head `f160d27`):** review thread
[`discussion_r4177394737`](https://github.com/pratik-saptarshi/rocinante/pull/108#discussion_r4177394737)
found that the native-shell installer examples provisioned DuckDB into Cargo's
cache without staging the runtime beside each release binary. The Linux,
macOS, and Windows examples now include `--stage-runtime-for-binary`, and the
packaging contract checks the documented build/stage/install order. This
follow-up is locally validated and included on the PR branch; the P1 package
matrix remains the release check for the Tauri finding.

Hosted CI run [37198628846](https://github.com/pratik-saptarshi/rocinante/actions/runs/37198628846)
is terminal. Workflow parsing, UI quality, Linux/macOS/Windows lifecycle
acceptance, full workspace tests, build seed, Rust quality gates, core/storage
test lanes, formatting, and Clippy passed. The aggregate gate failed only at
`security-exception-governance`, which listed all 17 records as overdue on
2026-10-04. Security run
[37198628844](https://github.com/pratik-saptarshi/rocinante/actions/runs/37198628844)
and Dependency Review run
[37198628881](https://github.com/pratik-saptarshi/rocinante/actions/runs/37198628881)
passed; the Security workflow's existing ignore configuration does not prove
the zero-exception requirement.

The refreshed unfiltered audit used a writable temporary copy of the RustSec
database because the default database lock path is read-only in this
environment. The refresh succeeded and upstream remained at revision
`ef6173cbc5c50ec8166f9a5b28f07834144373ee` (2026-10-03; 722 locked packages).
The report has zero vulnerability findings, four warnings (`glib`,
`proc-macro-error`, `instant`, and `fxhash`), and an empty ignore list; it
still exits nonzero with `--deny warnings`. No review date or owner acceptance
was fabricated.

### Phase 1 — Provision DuckDB only from verified prebuilt artifacts

Remove every source-build feature, pin the Rust binding and native engine
pair, and add checksum-verified provisioning for every supported platform.
Build and package against the official shared library with no unverified
system-library fallback. The current Cargo lock resolves the `duckdb` crate
to `1.10506.0`, which maps to DuckDB engine `1.5.6`. Pin this pair explicitly.
The official DuckDB `v1.5.6` release assets currently report these SHA-256
digests; verify them against the downloaded archives before checking them in
as the provisioning manifest:

| Target | Official asset | SHA-256 |
|---|---|---|
| `x86_64-unknown-linux-gnu` | [`libduckdb-linux-amd64.zip`](https://github.com/duckdb/duckdb/releases/download/v1.5.6/libduckdb-linux-amd64.zip) | `b845005f5132a7d8180057c35e14a7626632258782f871a90861b19c1c03841b` |
| `x86_64-apple-darwin`, `aarch64-apple-darwin` | [`libduckdb-osx-universal.zip`](https://github.com/duckdb/duckdb/releases/download/v1.5.6/libduckdb-osx-universal.zip) | `e0bc007d9b0094c0970ac1847a8601d10aad07cbd2910ce586ec810f77b638d6` |
| `x86_64-pc-windows-msvc` | [`libduckdb-windows-amd64.zip`](https://github.com/duckdb/duckdb/releases/download/v1.5.6/libduckdb-windows-amd64.zip) | `44cf59583f9951d2cb09b1bf115a63ecb2d8901e363903029d86c7d8683fe96a` |

**Validation:** resolve the all-target Cargo feature graph and prove there is
no `bundled`, `bundled-cmake`, or transitive DuckDB source-build feature; test
the provisioner for missing, corrupt, wrong-version, and valid artifacts; in a
clean cache capture compiler invocations and prove no DuckDB C/C++ translation
unit is compiled; query the linked engine version; build and run storage tests
on Linux, macOS, and Windows; launch installed apps with developer library-path
variables removed. Build the Tauri `.deb`, `.app`, and NSIS installer from the
verified prebuilt, inspect each package for its platform library, and confirm
the installed binary's loader path resolves to that packaged file.

**Exit gate:** all supported target artifacts have official URLs and checked-in
SHA-256 values, all builds use only those verified artifacts, and installed
apps load the packaged library on each OS.

**Local progress (2026-10-04):** the official macOS archive and extracted
library hashes validate; the seven provisioner tests, DuckDB source-build
feature contract, resolved feature-graph check, and engine `SELECT version()`
test pass. The analysis/core/storage/desktop-shell workspace suite passes all
75 tests, including a macOS-portable SQLite migration-path check. Strict
Clippy passes for the extracted workspace crates. Full root Tauri Clippy and
tests did not return a terminal result on this Mac; the root test build stopped
producing target artifacts for 16 minutes before interruption, so these gates
remain pending hosted CI. Shell installer contracts pass on this host. A
real macOS app bundle loads DuckDB from `Contents/Frameworks`, and the full
local lifecycle passed cold/warm Launch Services delivery, tray actions,
notification request, and saved-state restart in a prior run. The current
hosted Windows installer and URI lifecycle job passed. The macOS and Linux
acceptance jobs failed in run `37195996758` because the scripts did not stage
the verified library into the build output's sibling `deps` directory. That
packaging gap is fixed in the working tree and awaits a hosted rerun. A local
GUI revalidation is unverified because this sandbox could not scan the
temporary app bundle through Launch Services.

**Hosted follow-up (2026-10-04; PR head `b991f99`):** the prebuilt-only
provisioning path passed Linux, macOS, and Windows cold/warm/restart
acceptance. The full workspace test job also passed after staging the native
library into Cargo's test dependency directory. The aggregate remains blocked
by advisory governance and the four unfiltered RustSec warnings, not by
DuckDB packaging or the Rust/UI quality jobs.

### Phase 2 — Replace Sled ingestion with SQLite without changing contracts

Move ingestion into the shared storage crate on SQLite while retaining the
public commands, payloads, authorization, sanitization, retention, scoring,
baseline behavior, and DuckDB analytical queries. Use transactional writes,
ordered prefix scans, deletes, and restart-safe promotion receipts.

**Validation:** run the existing storage and command-integration suites plus
new SQLite parity cases for binary keys/values, prefixes, ordering, deletes,
retention, concurrent writers, crash/restart, duplicate promotion, and the
DuckDB-commit/SQLite-acknowledgement interruption window. Compare public
command names and serialized request/response fixtures before and after.

**Exit gate:** all existing data-path contracts pass against SQLite, replay
cannot lose or duplicate events, and Sled is absent from the application graph.

**Implementation evidence (2026-10-04, local worktree):** `DualLayerStore`
uses SQLite WAL at `ingestion.sqlite3`, full synchronous durability, binary-safe
ordered prefix scans, and transactional deletion. DuckDB writes telemetry,
baseline rows, and a unique event receipt in one transaction; SQLite acknowledges
events after that commit, so restart replay skips an already recorded event.
The default `SqliteWal` backend preserves command names and request/response
payloads. The old `SledTransitional` config now fails with a migration hint.
Startup also refuses to create SQLite when an unmigrated Sled directory is
present, preserving old data until the audited migration utility is delivered.

Nine `rocinante-storage` unit tests pass, including restart persistence,
binary prefix ordering, concurrent writers, legacy-store refusal, and the
DuckDB-commit/SQLite-ack interruption. Strict storage Clippy passes. The shared
workspace test run excluding the Tauri adapter passed 83 tests. The relevant
root integration binaries passed 17 storage, 5 transport, 5 backend, 2 admin
ingestion, 5 Tauri command, and 1 README/API contract tests with the staged
DuckDB runtime. `cargo check --tests` also passed for the complete Tauri test
target set. The enclosing Cargo test command did not return and was interrupted.
The lockfile and all-feature dependency tree no longer contain
`sled`, `fxhash`, or `instant`.

An unfiltered audit of the updated 715-package lockfile using the locally
cached database revision `ef6173cbc5c50ec8166f9a5b28f07834144373ee`
(2026-10-03) reports no vulnerability findings and two warnings:
RUSTSEC-2024-0429 (`glib`) and RUSTSEC-2024-0370 (`proc-macro-error`). The
authorized `rtk cargo audit` refresh could not reach GitHub, so this is not a
fresh-database release audit. Hosted aggregate CI and the P1 package matrix also
remain unverified for the current worktree.

### Phase 3 — Preserve legacy Sled data with an isolated audited reader

Add the one-time migration utility, retaining legacy stores read-only and
preserving existing DuckDB history. Keep the Sled package's true identity and
provenance in the audited workspace, but patch its obsolete hashers and lock
implementation so the migration graph contains neither `fxhash` nor
`instant`; never suppress the affected advisories or rename the package.

**Validation:** compare sorted, length-delimited hashes of every tree/key/value
record; check SQLite integrity; test binary keys, multiple and empty trees,
retry, interruption, corruption, conflicting target data, backup preservation,
and rollback/recovery. Show the migration utility alone depends on Sled and its
resolved graph is clean under fresh `cargo audit --deny warnings`.

**Exit gate:** the source store remains intact, the migration is repeatable and
recoverable, migrated records and DuckDB history are preserved, and the
audited migration graph has no RustSec findings.

### Phase 4 — Finish native-shell parity and retire Tauri/GTK

Use the native shell as the supported desktop host after all required behavior
is covered. Repository-folder selection satisfies the existing chooser
requirement. Move integrations to shared services and remove Tauri, Wry,
GTK/GLib and their macros from every supported target, feature, test, and
packaging path.

**Validation:** run the existing cold/warm URL, restart, notifications,
window/tray and registration checks on Linux, macOS, and Windows; validate
stable per-user app-data paths from shortcut, URL handler, and terminal launch;
check visible interactive behavior where CI cannot observe it; inspect all
target/feature dependency trees and fail if GTK/GLib/Tauri/Wry remain.

**Exit gate:** native-host parity has direct test evidence on all three OSes;
the application and migration graphs contain no affected GTK packages; public
command and payload contracts remain stable.

### Phase 5 — Remove obsolete governance records and enforce zero exceptions

After Phases 2–4 close, write evidence-backed dispositions for all 17 original
registry records. Remove withdrawn/absent advisories based on refreshed
RustSec metadata and every supported lockfile; remove active records only
after their package paths are gone. Then remove all audit ignores and make the
checker enforce an empty exception registry.

**Validation:** run the governance checker and its contract tests against
withdrawn, absent, newly introduced, malformed, stale, empty-registry, and
ignore/registry-mismatch fixtures. Refresh RustSec data and audit every Cargo
lockfile with `--deny warnings`; verify JSON shows an empty ignore list and no
warnings/findings. Confirm the hosting security-audit job also uses no
exceptions.

**Exit gate:** all 17 dispositions cite evidence; both governance stores are
empty; refreshed, unfiltered audits pass for all supported lockfiles; the
fail-closed governance and cargo-audit CI jobs pass.

### Phase 6 — Full release-readiness verification and documentation closeout

Synchronize `codemap.md`, architecture and host-migration roadmaps, README,
test plan, bill of materials, and publish checklist with actual code and
terminal results. Include native DuckDB version/provenance/checksum/license,
storage boundaries, migration status, advisory dispositions, and remaining
platform evidence. Do not describe hosted or interactive checks as complete
before they pass.

**Validation:** run pinned-toolchain `cargo fmt --check`, full-workspace
Clippy/tests, UI typecheck/unit tests/build with the current pinned pnpm,
security/governance/dependency-floor/roadmap contracts, and installed lifecycle
acceptance on all three OSes. Require terminal success for the aggregate PR
gate and all required security/platform checks on the final branch commit.

**Exit gate:** no stale docs or unsupported readiness claims remain; all
required checks are green for the same final commit; the branch is synchronized
with PR #108's remote head and is ready for protected review/merge.

## Verified starting point

On 2026-10-04, an unfiltered audit of PR branch commit `e9ae452` inspected 722
locked packages using `cargo-audit 0.22.2` and RustSec database revision
`ef6173cbc5c50ec8166f9a5b28f07834144373ee` (database timestamp
`2026-10-03T10:14:03+02:00`). The refreshed report found zero vulnerability
reports and the four warnings shown below. It exited 1 with `--deny warnings`,
as required. The supported Rust workspace currently has one lockfile:
`src-tauri/Cargo.lock`.

The RustSec refresh was repeated on 2026-10-04 through `rtk proxy cargo audit`;
upstream had no newer database commit, and the report still contains zero
vulnerability reports, four warnings, and an empty `settings.ignore` list.

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

The current PR #108 checks are not ready: its governance check fails on the
17 overdue records, while CodeQL and the Rust quality aggregate were still
running when inspected. The hosted rust-audit check passed with repository
ignores enabled; it does not establish the zero-exception goal. The unfiltered
local audit above is the baseline for remediation.

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
events. Replays must not lose events or duplicate analytical rows. Set the
default backend to the truthful SQLite WAL configuration. Keep explicit
Badger-sidecar settings for existing deployments, and make the retired
Sled-specific configuration fail with an actionable migration message rather
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
the supported workspace does not count as removal. The current full-workspace
tree traces `glib 0.18.5` through GTK and the retained Tauri/Wry host, while
`proc-macro-error 1.0.4` is pulled by the GTK/GLib macro crates. The isolated
desktop-shell graph is already free of both packages on all targets; removing
only the Linux shell's GTK path therefore cannot clear the lockfile audit. The
required pivot is to finish native-shell parity and remove the old Tauri host
from the supported workspace before closing these two advisories.

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
