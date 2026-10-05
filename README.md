# Rocinante Repo Analyzer

Rust native desktop repository analyzer with a bead-based plugin engine, dual-layer telemetry storage, and an admin control plane.

This public README summarizes what the repo can do today, with a specific focus on security and audit workflows.

## Purpose

Rocinante helps teams measure and explain repository risk using on-prem analysis pipelines:

- structural code/commit quality telemetry (`TODO` hygiene, complexity, churn, PR approval signals),
- deterministic plugin execution and explainable scoring,
- role-gated administrative workflows for querying and ranking evidence,
- local-only persistence with strict storage route governance.

---

## Core capabilities (today)

### 1) Scan pipeline + plugin architecture

- `src-tauri/crates/rocinante-analysis/src/engine.rs` runs a deterministic repository analysis pipeline.
- `BeadPlugin` trait in `src-tauri/crates/rocinante-analysis/src/plugins/mod.rs` lets new analyzers be added in isolation.
- Built-in beads in `src-tauri/crates/rocinante-analysis/src/plugins/*`:
  - `code_quality`: counts TODO markers.
  - `complexity`: token-based cyclomatic estimate.
  - `parser`: language-aware AST-like structural estimate with incremental digest cache.
  - `velocity`: commit/churn velocity windows.
  - `pr_approval`: PR/commit approval-fidelity signals.
  - `sanitizer`: mandatory pre-processor that redacts sensitive values from all metric outputs.

### 2) Security controls and governance

- JWT token validation and admin role enforcement in `src-tauri/crates/rocinante-analysis/src/auth.rs`.
- Migrated command names and payload contracts are recorded in `docs/roadmap/native-shell-command-contract-inventory.md`. The former Tauri `window.__TAURI__.core.invoke` transport is retired; the native shell calls shared Rust services directly. Native admin actions are closed over command names in `src-tauri/crates/rocinante-storage/src/lib.rs`:
  - `run_scan`
  - `query_metrics`
  - `ingest_event`
  - `promote_lifecycle`
  - `query_aggregates`
  - `committer_scores`
  - `rank_prs`
  - `evaluate_pr_risk`
  - `query_release_baseline`
  - `reseed_release_baseline`
  - `update_scoring_weights`
- Mandatory privacy redaction via sanitizer policy packs (`general`, `security`, `privacy`, `payments`).
- Signed scoring-weight config plus append-only change log in `scoring.rs` for tamper visibility.
- CI security posture documented in `.github/workflows/security.yml` and `docs/publish-readiness-checklist.html`.

### 3) Storage model and boundary enforcement

Rocinante uses two logical lanes in the shared `rocinante-storage` crate:

- **Ingestion route** → raw commit events in `<ROCINANTE_KV_PATH>/ingestion.sqlite3`, using SQLite WAL and durable transactions.
- **Analytics route** → promotion receipts, DuckDB history, aggregates, and query workloads.

Storage route checks prevent cross-use of the wrong backend for a given operation.
The default backend is SQLite WAL. Explicit Badger sidecar configuration remains
available for existing deployments; the retired `SledTransitional` setting now
returns an actionable configuration error. Existing Sled data is preserved, and
startup refuses to create a new SQLite store when it detects an unmigrated Sled
directory. Before upgrading an installation that contains Sled data, stop
Rocinante and run the isolated migrator against the directory containing its
`conf` and `db` markers (normally `ROCINANTE_KV_PATH`):

```sh
cargo run --locked --manifest-path tools/sled-migration/Cargo.toml -- /path/to/rocinante-kv
```

The migrator acquires the storage lock, leaves the Sled source intact, writes
and verifies `ingestion.sqlite3` beside it, and reports the record and tree
counts. Restart Rocinante only after it reports success. The migration does not
touch DuckDB files; DuckDB continues to use the checksum-verified official
prebuilt binary.

### 4) Audit and explainability outputs

- Score normalization and ranking helpers in `src-tauri/src/scoring.rs`.
- PR and committer rank outputs are surfaced through typed payloads in UI/domain modules.
- `ui/src/dashboard-explainability.ts` and `ui/src/App.tsx` expose rationale, traces, and action recommendations by audience (lead/manager/executive/security).

### 5) On-prem and enterprise compatibility

- Internal git-provider abstraction in `src-tauri/src/git_providers.rs` for GitHub Enterprise, GitLab, and Bitbucket Server URL/auth conventions.
- Directory/AD-style checks for role resolution in `src-tauri/src/onprem.rs`.

---

## How an auditor might use this repo

### Audit intent

1. Produce repository snapshots without external egress.
2. Detect repo quality drift by release and repository name.
3. Quantify commit-level and PR-level risk indicators.
4. Correlate metric deltas and score shifts to remediation actions.
5. Validate that privileged commands are authorized, boundary-checked, and reproducible.

### Auditor workflow (library-level)

Use the `repo_analyzer_core` library directly in Rust tests or a private internal service:

```rust
use repo_analyzer_core::admin;
use repo_analyzer_core::auth::issue_test_token;
use repo_analyzer_core::risk_contract::PrCandidate;
use repo_analyzer_core::storage::{IngestionBackendConfig, IngestionBackendKind};
use repo_analyzer_core::types::{
    AdminQuery, CommitIngestionEvent, ScoringWeights, TelemetryPoint,
};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Use a live signed admin JWT in an application; this helper is for examples/tests.
    let token = issue_test_token("auditor", &["admin"], 900);
    let backend = IngestionBackendConfig {
        kind: IngestionBackendKind::BadgerSidecar,
        strict_badger_required: true,
        endpoint: Some("inproc://badger".to_owned()),
    };

    // 1) Run an authenticated baseline scan.
    let _scan_summary = admin::run_scan(
        &token,
        "/path/to/repos",
        "release-2026.06",
        Path::new("telemetry.db"),
    )?;

    // 2) Ingest commit telemetry.
    let event = CommitIngestionEvent {
        commit_id: "commit-001".to_owned(),
        repo_name: "repo-a".to_owned(),
        release: "release-2026.06".to_owned(),
        committer: "auditor".to_owned(),
        telemetry: vec![TelemetryPoint {
            plugin: "example".to_owned(),
            metric_key: "complexity".to_owned(),
            metric_value: 1.0,
            details: "example metric".to_owned(),
        }],
    };
    admin::ingest_event(&token, "telemetry-kv", "analytics.duckdb", event, &backend)?;

    // 3) Promote raw telemetry into the analytics model.
    let _promoted = admin::promote_lifecycle(&token, "telemetry-kv", "analytics.duckdb")?;

    // 4) Query risk evidence by repository and release.
    let _aggregates = admin::query_aggregates(
        &token,
        "telemetry-kv",
        "analytics.duckdb",
        AdminQuery {
            name: Some("repo-a".to_owned()),
            release: Some("release-2026.06".to_owned()),
        },
    )?;

    // 5) Rank contributors and pull requests with auditable formulas.
    let _scores = admin::committer_scores(
        &token,
        "telemetry-kv",
        "analytics.duckdb",
        AdminQuery {
            name: None,
            release: Some("release-2026.06".to_owned()),
        },
        "scoring-weights.json",
    )?;
    let _ranked = admin::rank_prs(
        &token,
        "telemetry-kv",
        "analytics.duckdb",
        vec![PrCandidate {
            pr_id: "pr-001".to_owned(),
            repo_name: "repo-a".to_owned(),
            author: "auditor".to_owned(),
            release: "release-2026.06".to_owned(),
            file_risk: 0.4,
            author_velocity: 0.6,
            approval_fidelity: 0.9,
            ..PrCandidate::default()
        }],
        "scoring-weights.json",
    )?;

    // 6) Update model controls and write an audit record.
    admin::update_scoring_weights(
        &token,
        "scoring-weights.json",
        "scoring-audit.jsonl",
        ScoringWeights::default(),
    )?;
    Ok(())
}
```

### Auditor workflow (desktop UI)

- Open the app and use the **Admin Command Bridge** to run privileged operations.
- Paste sample payloads into **Telemetry payload JSON** and map outputs to **Explainability/Trend & Risk** sections for evidence.
- Capture exported artifacts (`.jsonl` scoring-audit + SQLite/duckdb + generated snapshots) for evidentiary retention.
- Verify role-denied behavior is enforced for non-admin callers via command result status.

---

## Public usage (step-by-step)

### 1) Prerequisites

- Rust stable toolchain.
- Node.js + `pnpm` (`ui/package.json` declares `pnpm@12.9.1`).
- Optional: Linux window-system, Vulkan, and udev development libraries for building the native shell.

### 2) Build UI bundle

```bash
cd ui
pnpm install
pnpm run build
```

### 3) Build and run the native desktop app

Before any Cargo command that builds analytics or the desktop shell, provision
the official prebuilt DuckDB library for the current target. The provisioner
checks the pinned archive and native-library SHA-256 values; the Cargo feature
guard rejects DuckDB source-build features. Installer commands below also
stage the runtime beside the built shell executable.

Before starting the native shell, set `RUNICIPAL_TOKEN_SECRET` in the process
environment. The shell validates it during startup, so setting it only before
scanning is too late. It must be at least 32 bytes, and the admin JWT must be
signed with the same secret. For an interactive Bash session, enter the secret
without echoing it or placing it in shell history:

```bash
read -r -s -p "RUNICIPAL_TOKEN_SECRET (32+ bytes): " RUNICIPAL_TOKEN_SECRET
export RUNICIPAL_TOKEN_SECRET
printf '\n'
```

For PowerShell, enter the secret through a hidden prompt:

```powershell
$secureSecret = Read-Host "RUNICIPAL_TOKEN_SECRET (32+ bytes)" -AsSecureString
$secretPointer = [Runtime.InteropServices.Marshal]::SecureStringToBSTR($secureSecret)
try {
  $env:RUNICIPAL_TOKEN_SECRET = [Runtime.InteropServices.Marshal]::PtrToStringBSTR($secretPointer)
} finally {
  [Runtime.InteropServices.Marshal]::ZeroFreeBSTR($secretPointer)
}
```

Use the same secret in the admin token signer. The per-user installers below do
not provision credentials to desktop-launched processes, so launch the app
from a process with this variable set until secure per-platform credential
setup tracked by BI-061 is available.

```bash
python3 scripts/provision_duckdb.py
```

```bash
(cd src-tauri && cargo test --workspace --locked --manifest-path Cargo.toml)
(cd src-tauri && cargo run --manifest-path Cargo.toml -p rocinante-desktop-shell)
```

The GTK-free native shell is the supported desktop host:

```bash
cargo run --manifest-path src-tauri/Cargo.toml -p rocinante-desktop-shell
```

On Linux, install the shell and register its URL handler for the current user:

```bash
cargo build --release --manifest-path src-tauri/Cargo.toml -p rocinante-desktop-shell
python3 scripts/provision_duckdb.py --stage-runtime-for-binary \
  src-tauri/target/release/rocinante-desktop-shell
sh src-tauri/crates/rocinante-desktop-shell/packaging/linux/install-user.sh \
  src-tauri/target/release/rocinante-desktop-shell
```

On macOS, install a built shell as a per-user app bundle and register its URL
scheme with Launch Services:

```bash
cargo build --release --manifest-path src-tauri/Cargo.toml -p rocinante-desktop-shell
python3 scripts/provision_duckdb.py --stage-runtime-for-binary \
  src-tauri/target/release/rocinante-desktop-shell
sh src-tauri/crates/rocinante-desktop-shell/packaging/macos/install-user.sh \
  src-tauri/target/release/rocinante-desktop-shell
```

On Windows, build the release shell and register its URL handler for the
current user with:

```powershell
cargo build --release --manifest-path src-tauri/Cargo.toml -p rocinante-desktop-shell
python scripts/provision_duckdb.py --stage-runtime-for-binary `
  src-tauri/target/release/rocinante-desktop-shell.exe
powershell -ExecutionPolicy Bypass -File `
  src-tauri/crates/rocinante-desktop-shell/packaging/windows/install-user.ps1 `
  src-tauri/target/release/rocinante-desktop-shell.exe
```

Do not commit production secrets. Choose a repository folder, enter a release
and an admin JWT signed by the configured secret, then analyze it. The shell
can reload metrics already stored for the selected repository tree and release
without rescanning. Repository metrics retain a sanitized directory label
followed by a stable hash of the canonical
local repository path, so selecting the same repository from a different scan
root or alongside same-named repositories reaches the same saved metrics. This
changes the value format of the existing repo_name field; consumers that
assume it contains only a repository basename should accept the hash suffix.
Legacy telemetry rows remain stored and are queried through their prior
root-derived names. Repositories with duplicate basenames may need to be
selected from the same parent tree used when their legacy rows were written.
Default telemetry, ingestion, analytics, and scoring files share the
platform-specific per-user Rocinante data directory; the existing
ROCINANTE_*_PATH variables still override individual files. Scan completion
requests a success or failure desktop notification; delivery still needs
runtime validation per platform. It
shows the selected repository and release context, metric and analyzer counts,
and the recorded values/details after a scan or saved-metric reload. It
parses cold-launch arguments using:

`rocinante://repository/open?path=<percent-encoded-absolute-path>`

Linux and macOS user-installation artifacts and a bounded per-user inbox for
second-instance URI delivery are in place. To install the GTK-free shell for
the current user on Linux, build and install it with:

```bash
cargo build --release --manifest-path src-tauri/Cargo.toml -p rocinante-desktop-shell
python3 scripts/provision_duckdb.py --stage-runtime-for-binary \
  src-tauri/target/release/rocinante-desktop-shell
sh src-tauri/crates/rocinante-desktop-shell/packaging/linux/install-user.sh \
  src-tauri/target/release/rocinante-desktop-shell
```

This installs the binary under `~/.local/bin`, registers the desktop entry and
`rocinante://` handler in the user's XDG data directory, and refreshes the
desktop/MIME databases when their tools are installed. It places the verified
`libduckdb.so` in `~/.local/lib/rocinante` and sets an executable-relative
runtime path. The Windows installer
copies the executable into `%LOCALAPPDATA%` and registers a current-user
`rocinante://` command under `HKCU`; it places the matching `duckdb.dll` beside
the executable. Its PowerShell source contract is checked on this host.
The macOS installer embeds `libduckdb.dylib` in `Contents/Frameworks`, rewrites
the app-relative loader path, and declares the scheme in the `.app` bundle
before registering it through Launch Services. The installed macOS lifecycle
acceptance passes cold/warm URL delivery, tray actions, notification request,
and saved-state restart on this host. The native-shell package matrix builds
and inspects the Linux, macOS, and Windows installations; local macOS
acceptance passes, while current-head hosted package results and Linux and
Windows acceptance remain pending.
Visible notification delivery on macOS and Windows, physical tray-menu clicks,
and macOS foreground activation still need interactive validation. Release
distribution must sign the completed app bundle after packaging.

> If you are only validating pipeline outputs and not running the desktop shell, running tests and targeted Rust unit tests above is usually sufficient for CI-style verification.

### 4) Run checks before review/publish

```bash
cargo fmt --manifest-path src-tauri/Cargo.toml --all -- --check
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml
cargo audit --file src-tauri/Cargo.lock --deny warnings
mkdir -p target/coverage
cargo llvm-cov --locked --manifest-path src-tauri/Cargo.toml --lcov --output-path target/coverage/lcov.info
cd ui
pnpm exec tsc -b
pnpm exec vitest run
pnpm exec playwright test
```

The Rust workspace test command includes `rocinante-core`,
`rocinante-analysis`, `rocinante-storage`, and `rocinante-desktop-shell` plus
the shared service library. Use the `pnpm@12.9.1` version declared in
`ui/package.json` for UI checks.

### 5) Governance artifacts

- `docs/publish-readiness-checklist.html` (required gates)
- `docs/bill-of-materials.html` (release inventory, security tooling, and coverage artifacts)
- `docs/roadmap/execution-ticket-artifact.md` (single-source for work status)

---

## For external reviewers/auditors

- Do not commit secrets (`SECURITY.md` and `.gitignore` are your guardrails).
- Treat `RUNICIPAL_TOKEN_SECRET` as production-only secret material.
- Use the release checklist as part of audit entry/exit criteria.
- Verify branch protection and required status checks via `scripts/harden-github.sh`.
- Confirm local-only telemetry retention and command role boundaries are preserved after every release.

---

## Current status and intended direction

- Completed: modular plugin pipeline, sanitizer enforcement, dual-path storage, command-plane scaffolding, explainability rails, and front-end audit-focused interactions.
- In progress: control-plane and command-contract convergence improvements (as reflected in docs under `docs/roadmap`).
- Publish posture: check `docs/publish-readiness-checklist.html` and complete all `[ ]` gates before creating release PRs.
