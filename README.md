# Rocinante Tauri Repo Analyzer

Rust + Tauri local repository analyzer with a bead-based plugin engine, dual-layer telemetry storage, and an admin control plane.

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

- `src-tauri/src/engine.rs` runs a deterministic repository analysis pipeline.
- `BeadPlugin` trait in `src-tauri/src/plugins/mod.rs` lets new analyzers be added in isolation.
- Built-in beads in `src-tauri/src/plugins/*`:
  - `code_quality`: counts TODO markers.
  - `complexity`: token-based cyclomatic estimate.
  - `parser`: language-aware AST-like structural estimate with incremental digest cache.
  - `velocity`: commit/churn velocity windows.
  - `pr_approval`: PR/commit approval-fidelity signals.
  - `sanitizer`: mandatory pre-processor that redacts sensitive values from all metric outputs.

### 2) Security controls and governance

- JWT token validation and admin role enforcement in `src-tauri/src/auth.rs`.
- Admin command surface is explicit and closed over command names in `src-tauri/src/main.rs` and `src-tauri/src/admin.rs`:
  - `run_scan`
  - `query_metrics`
  - `ingest_event`
  - `promote_lifecycle`
  - `query_aggregates`
  - `committer_scores`
  - `rank_prs`
  - `update_scoring_weights`
- Mandatory privacy redaction via sanitizer policy packs (`general`, `security`, `privacy`, `payments`).
- Signed scoring-weight config plus append-only change log in `scoring.rs` for tamper visibility.
- CI security posture documented in `.github/workflows/security.yml` and `docs/publish-readiness-checklist.html`.

### 3) Storage model and boundary enforcement

Rocinante uses two logical lanes in `src-tauri/src/storage.rs`:

- **Ingestion route** → raw commit event intake and write path.
- **Analytics route** → promotion, aggregate read-paths, and query workloads.

Storage route checks prevent cross-use of the wrong backend for a given operation.

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
use repo_analyzer_core::storage::{IngestionBackendConfig, IngestionBackendKind};
use repo_analyzer_core::types::{AdminQuery, PrCandidate, ScoringWeights};

let token = issue_test_token("auditor", &["admin"], 900);
let backend = IngestionBackendConfig {
    kind: IngestionBackendKind::BadgerSidecar,
    strict_badger_required: true,
    endpoint: Some("inproc://badger".to_string()),
};

// 1) run a baseline scan
let repo_count = admin::run_scan("/path/to/repos", "release-2026.06", "telemetry.db")?;

// 2) ingest commit telemetry
let event = /* CommitIngestionEvent */;
admin::ingest_event(&token, "telemetry-kv", "analytics.duckdb", event, &backend)?;

// 3) push raw telemetry into analytics model
let promoted = admin::promote_lifecycle(&token, "telemetry-kv", "analytics.duckdb")?;

// 4) query risk evidence by repo/release
let aggregates = admin::query_aggregates(
    &token,
    "telemetry-kv",
    "analytics.duckdb",
    AdminQuery { name: Some("repo-a".into()), release: Some("release-2026.06".into()) },
)?;

// 5) rank contributors/PRs with auditable formulas
let scores = admin::committer_scores(&token, "telemetry-kv", "analytics.duckdb", AdminQuery { name: None, release: Some("release-2026.06".into()) }, "scoring-weights.json")?;
let ranked = admin::rank_prs(&token, "telemetry-kv", "analytics.duckdb", vec![PrCandidate { .. }], "scoring-weights.json")?;

// 6) update model controls (logged)
admin::update_scoring_weights(
    &token,
    "scoring-weights.json",
    "scoring-audit.jsonl",
    ScoringWeights::default(),
)?;
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
- Node.js + `pnpm` (`ui/package.json` declares `pnpm@11.4.0`).
- Optional: Linux desktop deps for Tauri packaging if running full app packaging workflows.

### 2) Build UI bundle

```bash
cd ui
pnpm install
pnpm run build
```

### 3) Build and run backend/app shell

```bash
cd ../src-tauri
cargo test --manifest-path Cargo.toml          # validate Rust behavior first
cargo run --manifest-path Cargo.toml
```

> If you are only validating pipeline outputs and not running the desktop shell, running tests and targeted Rust unit tests above is usually sufficient for CI-style verification.

### 4) Run checks before review/publish

```bash
cargo fmt --manifest-path src-tauri/Cargo.toml --all -- --check
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml
cd ui
pnpm exec tsc -b
pnpm exec vitest run
pnpm exec playwright test
```

### 5) Governance artifacts

- `docs/publish-readiness-checklist.html` (required gates)
- `docs/bill-of-materials.html` (release inventory)
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
