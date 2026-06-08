# Rocinante Roadmap Completion Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking. REQUIRED METHOD: behavior-driven TDD. For every feature, write a behavior test first, run it to observe RED, implement minimal GREEN, then refactor only while green.

**Goal:** Complete the Rocinante maturity roadmap from MVP+/alpha to production-ready on-prem repo analyzer by mapping every remaining feature to executable BDD/TDD slices.

**Architecture:** Preserve strict dual-layer storage: BadgerDB sidecar owns write-heavy ingestion, DuckDB owns analytics. Treat storage boundaries, retention, snapshots, governance, and observability as user-visible behaviors, not implementation details.

**Tech Stack:** Rust 2021, Tauri 2, DuckDB, transitional sled tests only where explicitly marked, Unix socket sidecar transport, local HTML UI, Cargo integration tests.

---

## Plan-Review Integration Summary

| Finding | Priority | Disposition | Plan Coverage |
|---|---:|---|---|
| R1-F01 BadgerDB required | P0 | must-fix | Phase 1 |
| R1-F02 storage boundaries non-bypassable | P1 | must-fix | Phase 2 |
| R1-F03 retention split | P1 | must-fix | Phase 3 |
| R1-F04 snapshot/replica reads | P1 | must-fix | Phase 4 |
| R1-F05 sharded key strategy | P2 | bundled | Phase 1 + Phase 7 |

**Final Recommendation:** Applied with caveats. Caveat: existing full suite compilation previously timed out while building Tauri/DuckDB dependencies; each task includes targeted test commands plus final full-suite verification.

---

## Progressive Feature Map

| Phase | Features | BDD Behavior | Primary Tests |
|---|---|---|---|
| 0 | test harness | As maintainer, I can run focused architecture tests quickly | all existing targeted tests |
| 1 | F-008A, TK-014, TK-016, TK-033 | As CI, strict mode persists ingestion through Badger sidecar only | `ingestion_transport_tests.rs`, `storage_policy_tests.rs` |
| 2 | F-008B, TK-037 | As security admin, cross-layer misuse is denied before I/O | `storage_boundary_tests.rs`, `admin_ingestion_guard_tests.rs` |
| 3 | F-008D, F-021, TK-034, TK-036 | As compliance admin, raw events expire but release history remains | `retention_lifecycle_tests.rs` |
| 4 | F-008C, TK-035 | As dashboard user, analytics reads use immutable snapshots | `analytics_snapshot_tests.rs` |
| 5 | F-018, F-019 | As governance owner, scoring policy is signed and team-scoped | `scoring_integrity_tests.rs`, `policy_profile_tests.rs` |
| 6 | F-020 | As engineering lead, AST metrics are deterministic per language | `ast_plugin_tests.rs` |
| 7 | F-026, T-009 | As operator, I can see queue depth, throughput, and lag | `observability_tests.rs`, `ingestion_concurrency_tests.rs` |
| 8 | F-015, F-016, F-024, F-025 | As admin, UI exposes ingestion, trends, rationale, and baselines | `ui_contract_tests.rs` |
| 9 | F-022, F-023 | As enterprise admin, provider and directory adapters are explicit contracts | `provider_adapter_tests.rs`, `directory_provider_tests.rs` |
| 10 | F-027 | As migration owner, historical telemetry can be imported idempotently | `bulk_import_tests.rs` |

---

## File Structure

### Modify
- `src-tauri/src/storage.rs` — storage boundary, Badger sidecar client, retention, snapshots, metrics.
- `src-tauri/src/admin.rs` — admin-facing behavior orchestration and RBAC-gated commands.
- `src-tauri/src/main.rs` — Tauri command registration and state wiring.
- `src-tauri/src/scoring.rs` — signed scoring config and team policy profile logic.
- `src-tauri/src/onprem.rs` — provider/directory adapter contracts.
- `src-tauri/src/plugins/mod.rs` — AST plugin registration.
- `src-tauri/src/types.rs` — new request/response types.
- `ui/index.html` — dashboard/admin workflow UI.
- `docs/roadmap/feature-backlog.html` — status updates after each phase.
- `docs/roadmap/test-plan.html` — feature-test matrix updates after each phase.
- `docs/roadmap/bead-issue-tracker.html` — sprint status updates.

### Create
- `src-tauri/src/badger_sidecar.rs` — production ingestion client boundary.
- `src-tauri/src/retention.rs` — raw TTL prune and release retention policy.
- `src-tauri/src/snapshots.rs` — immutable analytics snapshot materialization.
- `src-tauri/src/policy.rs` — per-team scoring policy profiles.
- `src-tauri/src/ast_cache.rs` — incremental AST cache model.
- `src-tauri/src/observability.rs` — queue depth, throughput, lag metrics.
- `src-tauri/src/import.rs` — historical bulk import service.
- `src-tauri/tests/retention_lifecycle_tests.rs`.
- `src-tauri/tests/scoring_integrity_tests.rs`.
- `src-tauri/tests/policy_profile_tests.rs`.
- `src-tauri/tests/ast_plugin_tests.rs`.
- `src-tauri/tests/observability_tests.rs`.
- `src-tauri/tests/ingestion_concurrency_tests.rs`.
- `src-tauri/tests/ui_contract_tests.rs`.
- `src-tauri/tests/provider_adapter_tests.rs`.
- `src-tauri/tests/directory_provider_tests.rs`.
- `src-tauri/tests/bulk_import_tests.rs`.

---

## Global BDD/TDD Gate

For every task:

- [ ] **Step A: State behavior in test name**
  Use `#[test] fn <actor>_<observable_behavior>()` naming.

- [ ] **Step B: Write one failing test**
  Test public behavior through existing public APIs where possible.

- [ ] **Step C: Run exact focused test and verify RED**
  Run: `cargo test --manifest-path src-tauri/Cargo.toml <test_name> --test <test_file_stem> -- --exact`
  Expected: FAIL because behavior is absent, not because imports or syntax are wrong.

- [ ] **Step D: Implement minimal GREEN**
  Add only code needed for the failing behavior.

- [ ] **Step E: Run exact focused test and verify GREEN**
  Expected: PASS.

- [ ] **Step F: Run related regression tests**
  Run related test file plus any neighboring test files named in task.

- [ ] **Step G: Refactor only while green**
  No new behavior during refactor.

- [ ] **Step H: Commit**
  Commit message format: `feat(<feature-id>): <observable behavior>`.

---

## Phase 0: Stabilize Verification Harness

**Features:** prerequisite for all roadmap completion.

**Behavior:** As a maintainer, I can run focused test slices without waiting for full GUI build feedback.

**Files:**
- Modify: `docs/roadmap/test-plan.html`
- No production code unless tests expose harness defect.

- [ ] **Step 1: Run current targeted storage policy test**

Run:
```bash
cargo test --manifest-path src-tauri/Cargo.toml storage_profile_accepts_strict_badger_duckdb --test storage_policy_tests -- --exact
```
Expected: PASS after dependencies compile. If compile times out, rerun once with `timeout 600` and record result in `docs/roadmap/bead-issue-tracker.html`.

- [ ] **Step 2: Run current targeted boundary test**

Run:
```bash
cargo test --manifest-path src-tauri/Cargo.toml ingestion_route_rejects_analytics_queries --test storage_boundary_tests -- --exact
```
Expected: PASS.

- [ ] **Step 3: Update test-plan verification note**

Add a `Focused Test Commands` section to `docs/roadmap/test-plan.html` listing the stable command pattern:
```bash
cargo test --manifest-path src-tauri/Cargo.toml <test_name> --test <test_file_stem> -- --exact
```

- [ ] **Step 4: Commit**

```bash
git add docs/roadmap/test-plan.html docs/roadmap/bead-issue-tracker.html
git commit -m "docs(test): document focused BDD verification commands"
```

---

## Phase 1: Badger Sidecar Conformance and Sharded Ingestion

**Features:** `F-008A`, `TK-014`, `TK-016`, `TK-033`, `R1-F01`, `R1-F05`.

**BDD Story:** As a CI/CD producer, when strict mode is enabled, ingestion persists through the Badger sidecar boundary with sharded keys and never silently falls back to embedded sled.

**Files:**
- Create: `src-tauri/src/badger_sidecar.rs`
- Modify: `src-tauri/src/lib.rs`
- Modify: `src-tauri/src/storage.rs`
- Modify: `src-tauri/src/types.rs`
- Test: `src-tauri/tests/ingestion_transport_tests.rs`
- Test: `src-tauri/tests/ingestion_concurrency_tests.rs`

### Task 1.1: strict mode rejects in-process fallback

- [ ] **Step 1: Write failing behavior test**

Add to `src-tauri/tests/ingestion_transport_tests.rs`:
```rust
use repo_analyzer_core::storage::{IngestionBackendConfig, IngestionBackendKind};

#[test]
fn ci_strict_badger_rejects_inproc_fallback_endpoint() {
    let backend = IngestionBackendConfig {
        kind: IngestionBackendKind::BadgerSidecar,
        strict_badger_required: true,
        endpoint: Some("inproc://dev-sidecar".to_string()),
    };

    let err = backend.validate().expect_err("strict production mode must reject inproc fallback");
    assert!(err.to_string().contains("inproc fallback is not allowed in strict mode"));
}
```

- [ ] **Step 2: Verify RED**

Run:
```bash
cargo test --manifest-path src-tauri/Cargo.toml ci_strict_badger_rejects_inproc_fallback_endpoint --test ingestion_transport_tests -- --exact
```
Expected: FAIL because `validate()` currently allows `inproc://`.

- [ ] **Step 3: Implement minimal GREEN**

In `src-tauri/src/storage.rs`, inside `IngestionBackendConfig::validate`, after endpoint trim:
```rust
if self.strict_badger_required && endpoint.starts_with("inproc://") {
    return Err(AnalyzerError::Db(
        "inproc fallback is not allowed in strict mode".to_string(),
    ));
}
```

- [ ] **Step 4: Verify GREEN**

Run same command. Expected: PASS.

- [ ] **Step 5: Regression**

Run:
```bash
cargo test --manifest-path src-tauri/Cargo.toml --test ingestion_backend_tests
cargo test --manifest-path src-tauri/Cargo.toml --test admin_ingestion_guard_tests
```
Expected: PASS. If tests assume strict inproc, change only tests that explicitly model development mode by setting `strict_badger_required: false`.

- [ ] **Step 6: Commit**

```bash
git add src-tauri/src/storage.rs src-tauri/tests/ingestion_transport_tests.rs
git commit -m "feat(F-008A): reject in-process Badger fallback in strict mode"
```

### Task 1.2: sidecar client owns serialization boundary

- [ ] **Step 1: Write failing behavior test**

Add to `src-tauri/tests/ingestion_transport_tests.rs`:
```rust
use repo_analyzer_core::badger_sidecar::BadgerSidecarRequest;
use repo_analyzer_core::types::{CommitIngestionEvent, TelemetryPoint};

#[test]
fn ci_sidecar_request_serializes_commit_event_with_shard_key() {
    let event = CommitIngestionEvent {
        commit_id: "abc123".to_string(),
        repo_name: "repo".to_string(),
        release: "v1".to_string(),
        committer: "dev".to_string(),
        telemetry: vec![TelemetryPoint {
            plugin: "complexity".to_string(),
            metric_key: "estimated_cyclomatic_complexity".to_string(),
            metric_value: 7.0,
            details: "clean".to_string(),
        }],
    };

    let req = BadgerSidecarRequest::from_event(&event, 16);

    assert!(req.key.starts_with("evt:"));
    assert!(req.key.contains(":shard:"));
    assert_eq!(req.event.commit_id, "abc123");
}
```

- [ ] **Step 2: Verify RED**

Run:
```bash
cargo test --manifest-path src-tauri/Cargo.toml ci_sidecar_request_serializes_commit_event_with_shard_key --test ingestion_transport_tests -- --exact
```
Expected: FAIL because `badger_sidecar` module does not exist.

- [ ] **Step 3: Implement minimal module**

Create `src-tauri/src/badger_sidecar.rs`:
```rust
use crate::types::CommitIngestionEvent;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BadgerSidecarRequest {
    pub key: String,
    pub event: CommitIngestionEvent,
}

impl BadgerSidecarRequest {
    pub fn from_event(event: &CommitIngestionEvent, shard_count: u16) -> Self {
        let shard_count = shard_count.max(1);
        let shard = stable_shard(&event.commit_id, shard_count);
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        Self {
            key: format!("evt:shard:{shard:04}:{ts}:{}", event.commit_id),
            event: event.clone(),
        }
    }
}

fn stable_shard(value: &str, shard_count: u16) -> u16 {
    let sum = value.bytes().fold(0u16, |acc, b| acc.wrapping_add(b as u16));
    sum % shard_count
}
```

Modify `src-tauri/src/lib.rs`:
```rust
pub mod badger_sidecar;
```

- [ ] **Step 4: Verify GREEN**

Run same test. Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add src-tauri/src/lib.rs src-tauri/src/badger_sidecar.rs src-tauri/tests/ingestion_transport_tests.rs
git commit -m "feat(F-008A): add Badger sidecar request boundary"
```

### Task 1.3: sharded keys avoid global write prefix

- [ ] **Step 1: Write failing behavior test**

Create `src-tauri/tests/ingestion_concurrency_tests.rs`:
```rust
use repo_analyzer_core::badger_sidecar::BadgerSidecarRequest;
use repo_analyzer_core::types::CommitIngestionEvent;
use std::collections::BTreeSet;

fn event(commit_id: &str) -> CommitIngestionEvent {
    CommitIngestionEvent {
        commit_id: commit_id.to_string(),
        repo_name: "repo".to_string(),
        release: "v1".to_string(),
        committer: "dev".to_string(),
        telemetry: vec![],
    }
}

#[test]
fn producer_burst_distributes_events_across_sharded_prefixes() {
    let prefixes: BTreeSet<String> = (0..64)
        .map(|i| BadgerSidecarRequest::from_event(&event(&format!("commit-{i}")), 16).key)
        .map(|key| key.split(':').take(3).collect::<Vec<_>>().join(":"))
        .collect();

    assert!(prefixes.len() > 1, "burst writes must not share one global prefix");
}
```

- [ ] **Step 2: Verify RED**

Run:
```bash
cargo test --manifest-path src-tauri/Cargo.toml producer_burst_distributes_events_across_sharded_prefixes --test ingestion_concurrency_tests -- --exact
```
Expected: FAIL if key prefix extraction does not include shard number. If it passes immediately, adjust prefix extraction to `take(4)` and re-run RED; the test must prove sharding.

- [ ] **Step 3: Minimal GREEN**

Ensure `BadgerSidecarRequest::from_event` key format is exactly:
```rust
format!("evt:shard:{shard:04}:{ts}:{}", event.commit_id)
```

- [ ] **Step 4: Verify GREEN and regression**

Run:
```bash
cargo test --manifest-path src-tauri/Cargo.toml --test ingestion_concurrency_tests
cargo test --manifest-path src-tauri/Cargo.toml --test ingestion_transport_tests
```
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add src-tauri/src/badger_sidecar.rs src-tauri/tests/ingestion_concurrency_tests.rs
git commit -m "feat(F-008A): shard Badger ingestion keys"
```

---

## Phase 2: Non-Bypassable Storage Boundary Guards

**Features:** `F-008B`, `TK-037`, `R1-F02`.

**BDD Story:** As a security administrator, storage route misuse is rejected before touching any backend.

**Files:**
- Modify: `src-tauri/src/storage.rs`
- Modify: `src-tauri/src/admin.rs`
- Test: `src-tauri/tests/storage_boundary_tests.rs`
- Test: `src-tauri/tests/admin_ingestion_guard_tests.rs`

### Task 2.1: admin ingest enforces ingestion route before backend validation

- [ ] **Step 1: Write failing behavior test**

Add to `src-tauri/tests/admin_ingestion_guard_tests.rs`:
```rust
use repo_analyzer_core::storage::{StorageOperation, StorageRoute};

#[test]
fn admin_ingest_contract_allows_only_ingestion_writes() {
    assert!(StorageRoute::Ingestion.enforce(StorageOperation::IngestWrite).is_ok());
    let err = StorageRoute::Analytics
        .enforce(StorageOperation::IngestWrite)
        .expect_err("analytics route must reject ingestion writes");
    assert!(err.to_string().contains("BadgerDB"));
}
```

- [ ] **Step 2: Verify RED**

Run:
```bash
cargo test --manifest-path src-tauri/Cargo.toml admin_ingest_contract_allows_only_ingestion_writes --test admin_ingestion_guard_tests -- --exact
```
Expected: FAIL only if route message/guard absent.

- [ ] **Step 3: Minimal GREEN**

In `admin::ingest_event`, call:
```rust
StorageRoute::Ingestion.enforce(StorageOperation::IngestWrite)?;
```
before `DualLayerStore::open`.

- [ ] **Step 4: Verify GREEN**

Run same test and `cargo test --manifest-path src-tauri/Cargo.toml --test admin_ingestion_guard_tests`.

- [ ] **Step 5: Commit**

```bash
git add src-tauri/src/admin.rs src-tauri/tests/admin_ingestion_guard_tests.rs
git commit -m "feat(F-008B): enforce admin ingestion route contract"
```

### Task 2.2: analytics calls enforce DuckDB route before opening store

- [ ] **Step 1: Write failing behavior test**

Add to `src-tauri/tests/storage_boundary_tests.rs`:
```rust
use repo_analyzer_core::storage::{StorageOperation, StorageRoute};

#[test]
fn dashboard_analytics_contract_allows_only_analytics_queries() {
    assert!(StorageRoute::Analytics.enforce(StorageOperation::AnalyticsQuery).is_ok());
    let err = StorageRoute::Ingestion
        .enforce(StorageOperation::AnalyticsQuery)
        .expect_err("ingestion route must reject analytics queries");
    assert!(err.to_string().contains("DuckDB"));
}
```

- [ ] **Step 2: Verify RED**

Run:
```bash
cargo test --manifest-path src-tauri/Cargo.toml dashboard_analytics_contract_allows_only_analytics_queries --test storage_boundary_tests -- --exact
```
Expected: FAIL only if guard/message absent.

- [ ] **Step 3: Minimal GREEN**

In `admin::query_aggregates`, `admin::committer_scores`, and `admin::rank_prs`, call:
```rust
StorageRoute::Analytics.enforce(StorageOperation::AnalyticsQuery)?;
```
before opening `DualLayerStore`.

- [ ] **Step 4: Verify GREEN + admin regressions**

Run:
```bash
cargo test --manifest-path src-tauri/Cargo.toml --test storage_boundary_tests
cargo test --manifest-path src-tauri/Cargo.toml --test admin_service_tests
```
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add src-tauri/src/admin.rs src-tauri/tests/storage_boundary_tests.rs
git commit -m "feat(F-008B): enforce analytics route contracts"
```

---

## Phase 3: Retention Lifecycle and Release Partitions

**Features:** `F-008D`, `F-021`, `TK-034`, `TK-036`, `R1-F03`.

**BDD Story:** As a compliance admin, raw ingestion events expire by short TTL, while promoted release history remains queryable by release.

**Files:**
- Create: `src-tauri/src/retention.rs`
- Modify: `src-tauri/src/lib.rs`
- Modify: `src-tauri/src/storage.rs`
- Test: `src-tauri/tests/retention_lifecycle_tests.rs`

### Task 3.1: raw TTL prune removes only expired raw events

- [ ] **Step 1: Write failing behavior test**

Create `src-tauri/tests/retention_lifecycle_tests.rs`:
```rust
use repo_analyzer_core::retention::RawRetentionDecision;

#[test]
fn compliance_prune_removes_expired_raw_events_only() {
    let decision = RawRetentionDecision::new(60);

    assert!(decision.should_prune(100, 161));
    assert!(!decision.should_prune(100, 160));
}
```

- [ ] **Step 2: Verify RED**

Run:
```bash
cargo test --manifest-path src-tauri/Cargo.toml compliance_prune_removes_expired_raw_events_only --test retention_lifecycle_tests -- --exact
```
Expected: FAIL because module/type absent.

- [ ] **Step 3: Minimal GREEN**

Create `src-tauri/src/retention.rs`:
```rust
#[derive(Debug, Clone)]
pub struct RawRetentionDecision {
    ttl_secs: i64,
}

impl RawRetentionDecision {
    pub fn new(ttl_secs: i64) -> Self {
        Self { ttl_secs }
    }

    pub fn should_prune(&self, event_ts: i64, now_ts: i64) -> bool {
        now_ts - event_ts > self.ttl_secs
    }
}
```

Modify `src-tauri/src/lib.rs`:
```rust
pub mod retention;
```

- [ ] **Step 4: Verify GREEN**

Run same test. Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add src-tauri/src/lib.rs src-tauri/src/retention.rs src-tauri/tests/retention_lifecycle_tests.rs
git commit -m "feat(F-008D): model raw event TTL pruning"
```

### Task 3.2: promoted release history survives raw prune

- [ ] **Step 1: Write failing behavior test**

Add to `src-tauri/tests/retention_lifecycle_tests.rs`:
```rust
use repo_analyzer_core::storage::DualLayerStore;
use repo_analyzer_core::types::{AdminQuery, CommitIngestionEvent, TelemetryPoint};

#[test]
fn compliance_prune_preserves_promoted_release_history() {
    let tmp = tempfile::tempdir().unwrap();
    let kv = tmp.path().join("kv");
    let duck = tmp.path().join("analytics.duckdb");
    let store = DualLayerStore::open(kv.to_str().unwrap(), duck.to_str().unwrap()).unwrap();

    store.ingest_commit_event(&CommitIngestionEvent {
        commit_id: "c1".to_string(),
        repo_name: "repo".to_string(),
        release: "v1".to_string(),
        committer: "dev".to_string(),
        telemetry: vec![TelemetryPoint {
            plugin: "complexity".to_string(),
            metric_key: "estimated_cyclomatic_complexity".to_string(),
            metric_value: 5.0,
            details: "ok".to_string(),
        }],
    }).unwrap();
    store.promote_to_columnar().unwrap();
    let stats = store.prune_raw_events_older_than(0).unwrap();

    assert_eq!(stats.pruned_events, 0);
    let rows = store.aggregate_by_query(&AdminQuery { name: Some("repo".to_string()), release: Some("v1".to_string()) }).unwrap();
    assert_eq!(rows.len(), 1);
}
```

- [ ] **Step 2: Verify RED**

Run:
```bash
cargo test --manifest-path src-tauri/Cargo.toml compliance_prune_preserves_promoted_release_history --test retention_lifecycle_tests -- --exact
```
Expected: FAIL because `prune_raw_events_older_than` and `pruned_events` stats do not exist.

- [ ] **Step 3: Minimal GREEN**

In `src-tauri/src/storage.rs`, add:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionStats {
    pub pruned_events: usize,
}
```

Add method on `DualLayerStore`:
```rust
pub fn prune_raw_events_older_than(&self, ttl_secs: i64) -> Result<RetentionStats, AnalyzerError> {
    let now = now_ts();
    let mut pruned = 0usize;
    for row in self.kv.scan_prefix("evt:") {
        let (k, _) = row.map_err(|e| AnalyzerError::Db(e.to_string()))?;
        let key = String::from_utf8_lossy(&k).to_string();
        let ts = key.split(':').nth(1).and_then(|s| s.parse::<i64>().ok()).unwrap_or(now);
        if now - ts > ttl_secs {
            self.kv.remove(k).map_err(|e| AnalyzerError::Db(e.to_string()))?;
            pruned += 1;
        }
    }
    self.kv.flush().map_err(|e| AnalyzerError::Db(e.to_string()))?;
    Ok(RetentionStats { pruned_events: pruned })
}
```

- [ ] **Step 4: Verify GREEN + storage regressions**

Run:
```bash
cargo test --manifest-path src-tauri/Cargo.toml --test retention_lifecycle_tests
cargo test --manifest-path src-tauri/Cargo.toml --test storage_duallayer_tests
```
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add src-tauri/src/storage.rs src-tauri/tests/retention_lifecycle_tests.rs
git commit -m "feat(F-008D): preserve analytics history during raw prune"
```

---

## Phase 4: Immutable Analytics Snapshots

**Features:** `F-008C`, `TK-035`, `R1-F04`.

**BDD Story:** As a dashboard user, analytics queries execute against immutable snapshot paths while promotion can continue.

**Files:**
- Create: `src-tauri/src/snapshots.rs`
- Modify: `src-tauri/src/lib.rs`
- Modify: `src-tauri/src/storage.rs`
- Test: `src-tauri/tests/analytics_snapshot_tests.rs`

### Task 4.1: snapshot materialization creates immutable read target

- [ ] **Step 1: Write failing behavior test**

Add to `src-tauri/tests/analytics_snapshot_tests.rs`:
```rust
use repo_analyzer_core::snapshots::AnalyticsSnapshotManager;

#[test]
fn dashboard_query_uses_materialized_immutable_snapshot() {
    let tmp = tempfile::tempdir().unwrap();
    let source = tmp.path().join("analytics.duckdb");
    std::fs::write(&source, b"duckdb-bytes").unwrap();

    let snapshot = AnalyticsSnapshotManager::new(tmp.path())
        .materialize(source.to_str().unwrap(), 42)
        .unwrap();

    assert!(snapshot.immutable);
    assert!(snapshot.path.contains("snapshot-42.duckdb"));
    assert_eq!(std::fs::read(&snapshot.path).unwrap(), b"duckdb-bytes");
}
```

- [ ] **Step 2: Verify RED**

Run:
```bash
cargo test --manifest-path src-tauri/Cargo.toml dashboard_query_uses_materialized_immutable_snapshot --test analytics_snapshot_tests -- --exact
```
Expected: FAIL because `snapshots` module absent.

- [ ] **Step 3: Minimal GREEN**

Create `src-tauri/src/snapshots.rs`:
```rust
use crate::errors::AnalyzerError;
use crate::storage::AnalyticsSnapshot;
use std::path::{Path, PathBuf};

pub struct AnalyticsSnapshotManager {
    root: PathBuf,
}

impl AnalyticsSnapshotManager {
    pub fn new(root: &Path) -> Self {
        Self { root: root.to_path_buf() }
    }

    pub fn materialize(&self, source_path: &str, snapshot_id: u64) -> Result<AnalyticsSnapshot, AnalyzerError> {
        let target = self.root.join(format!("snapshot-{snapshot_id}.duckdb"));
        std::fs::copy(source_path, &target).map_err(|e| AnalyzerError::Io(e.to_string()))?;
        Ok(AnalyticsSnapshot::new(target.to_str().unwrap_or_default(), snapshot_id))
    }
}
```

Modify `src-tauri/src/lib.rs`:
```rust
pub mod snapshots;
```

- [ ] **Step 4: Verify GREEN**

Run same test. Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add src-tauri/src/lib.rs src-tauri/src/snapshots.rs src-tauri/tests/analytics_snapshot_tests.rs
git commit -m "feat(F-008C): materialize immutable analytics snapshots"
```

---

## Phase 5: Signed and Team-Scoped Scoring Policy

**Features:** `F-018`, `F-019`.

**BDD Story:** As a governance owner, scoring configs are tamper-evident and teams can use approved weight profiles.

**Files:**
- Create: `src-tauri/src/policy.rs`
- Modify: `src-tauri/src/lib.rs`
- Modify: `src-tauri/src/scoring.rs`
- Modify: `src-tauri/src/types.rs`
- Test: `src-tauri/tests/scoring_integrity_tests.rs`
- Test: `src-tauri/tests/policy_profile_tests.rs`

### Task 5.1: tampered scoring config fails verification

- [ ] **Step 1: Write failing behavior test**

Create `src-tauri/tests/scoring_integrity_tests.rs`:
```rust
use repo_analyzer_core::scoring::{sign_weights, verify_signed_weights};
use repo_analyzer_core::types::ScoringWeights;

#[test]
fn governance_rejects_tampered_scoring_weights() {
    let weights = ScoringWeights::default();
    let signed = sign_weights(&weights).unwrap();
    let mut tampered = signed.clone();
    tampered.weights.complexity_weight = 0.99;

    let err = verify_signed_weights(&tampered).expect_err("tampered config must fail verification");
    assert!(err.to_string().contains("signature verification failed"));
}
```

- [ ] **Step 2: Verify RED**

Run:
```bash
cargo test --manifest-path src-tauri/Cargo.toml governance_rejects_tampered_scoring_weights --test scoring_integrity_tests -- --exact
```
Expected: FAIL because signing API absent.

- [ ] **Step 3: Minimal GREEN**

In `src-tauri/src/scoring.rs`, add deterministic hash-based tamper evidence using `std::collections::hash_map::DefaultHasher` first. Do not add cryptographic key management until tests require it.

- [ ] **Step 4: Verify GREEN + scoring regressions**

Run:
```bash
cargo test --manifest-path src-tauri/Cargo.toml --test scoring_integrity_tests
cargo test --manifest-path src-tauri/Cargo.toml --test scoring_audit_tests
cargo test --manifest-path src-tauri/Cargo.toml --test scoring_helpers_tests
```
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add src-tauri/src/scoring.rs src-tauri/tests/scoring_integrity_tests.rs
git commit -m "feat(F-018): verify scoring policy integrity"
```

### Task 5.2: team policy profile resolves approved weights

- [ ] **Step 1: Write failing behavior test**

Create `src-tauri/tests/policy_profile_tests.rs`:
```rust
use repo_analyzer_core::policy::PolicyProfiles;
use repo_analyzer_core::types::ScoringWeights;

#[test]
fn team_policy_resolves_explicit_scoring_profile() {
    let mut profiles = PolicyProfiles::default();
    let mut weights = ScoringWeights::default();
    weights.version = "security-team-v1".to_string();
    profiles.insert("security".to_string(), weights.clone());

    let resolved = profiles.resolve("security").unwrap();

    assert_eq!(resolved.version, "security-team-v1");
}
```

- [ ] **Step 2: Verify RED**

Run:
```bash
cargo test --manifest-path src-tauri/Cargo.toml team_policy_resolves_explicit_scoring_profile --test policy_profile_tests -- --exact
```
Expected: FAIL because module absent.

- [ ] **Step 3: Minimal GREEN**

Create `src-tauri/src/policy.rs` with a `BTreeMap<String, ScoringWeights>` wrapper and `insert`/`resolve` methods returning `AnalyzerError` for missing profiles.

Modify `src-tauri/src/lib.rs`:
```rust
pub mod policy;
```

- [ ] **Step 4: Verify GREEN**

Run same test. Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add src-tauri/src/lib.rs src-tauri/src/policy.rs src-tauri/tests/policy_profile_tests.rs
git commit -m "feat(F-019): resolve team scoring policy profiles"
```

---

## Phase 6: Incremental AST Cache and Parser Bead

**Features:** `F-020`.

**BDD Story:** As an engineering lead, language-aware metrics are deterministic and re-use cached AST fingerprints when files do not change.

**Files:**
- Create: `src-tauri/src/ast_cache.rs`
- Modify: `src-tauri/src/lib.rs`
- Modify: `src-tauri/src/plugins/mod.rs`
- Create: `src-tauri/src/plugins/ast_metrics.rs`
- Test: `src-tauri/tests/ast_plugin_tests.rs`

### Task 6.1: AST cache reuses unchanged file fingerprint

- [ ] **Step 1: Write failing behavior test**

Create `src-tauri/tests/ast_plugin_tests.rs`:
```rust
use repo_analyzer_core::ast_cache::AstCache;

#[test]
fn ast_cache_reuses_unchanged_file_fingerprint() {
    let mut cache = AstCache::default();

    let first = cache.fingerprint("src/lib.rs", "fn main() {}");
    let second = cache.fingerprint("src/lib.rs", "fn main() {}");

    assert_eq!(first, second);
    assert_eq!(cache.hits(), 1);
}
```

- [ ] **Step 2: Verify RED**

Run:
```bash
cargo test --manifest-path src-tauri/Cargo.toml ast_cache_reuses_unchanged_file_fingerprint --test ast_plugin_tests -- --exact
```
Expected: FAIL because module absent.

- [ ] **Step 3: Minimal GREEN**

Create `src-tauri/src/ast_cache.rs` with a `BTreeMap<String, u64>` cache using `DefaultHasher`; increment `hits` when path hash matches previous content hash.

Modify `src-tauri/src/lib.rs`:
```rust
pub mod ast_cache;
```

- [ ] **Step 4: Verify GREEN**

Run same test. Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add src-tauri/src/lib.rs src-tauri/src/ast_cache.rs src-tauri/tests/ast_plugin_tests.rs
git commit -m "feat(F-020): cache unchanged AST fingerprints"
```

### Task 6.2: AST bead emits deterministic language metric

- [ ] **Step 1: Write failing behavior test**

Add to `src-tauri/tests/ast_plugin_tests.rs`:
```rust
use repo_analyzer_core::plugins::ast_metrics::AstMetricsPlugin;
use repo_analyzer_core::plugins::BeadPlugin;
use repo_analyzer_core::types::{AnalysisInput, RepoTarget};

#[test]
fn ast_bead_emits_deterministic_language_metric() {
    let plugin = AstMetricsPlugin;
    let input = AnalysisInput {
        repo: RepoTarget { name: "repo".to_string(), path: ".".to_string() },
        changed_files: vec!["src-tauri/src/lib.rs".to_string()],
    };

    let metrics = plugin.run(&input).unwrap();

    assert!(metrics.iter().any(|m| m.key == "ast_changed_files"));
}
```

- [ ] **Step 2: Verify RED**

Run:
```bash
cargo test --manifest-path src-tauri/Cargo.toml ast_bead_emits_deterministic_language_metric --test ast_plugin_tests -- --exact
```
Expected: FAIL because plugin absent.

- [ ] **Step 3: Minimal GREEN**

Create `src-tauri/src/plugins/ast_metrics.rs` implementing `BeadPlugin` and returning one metric `ast_changed_files` equal to `input.changed_files.len() as f64`.

Modify `src-tauri/src/plugins/mod.rs`:
```rust
pub mod ast_metrics;
```

- [ ] **Step 4: Verify GREEN**

Run same test. Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add src-tauri/src/plugins/mod.rs src-tauri/src/plugins/ast_metrics.rs src-tauri/tests/ast_plugin_tests.rs
git commit -m "feat(F-020): add deterministic AST metrics bead"
```

---

## Phase 7: Job Observability and Ingestion Throughput

**Features:** `F-026`, `T-009`, `TK-019`.

**BDD Story:** As an operator, I can observe queue depth, worker throughput, and promotion lag during ingestion bursts.

**Files:**
- Create: `src-tauri/src/observability.rs`
- Modify: `src-tauri/src/lib.rs`
- Modify: `src-tauri/src/storage.rs`
- Test: `src-tauri/tests/observability_tests.rs`

### Task 7.1: ingestion metrics record queue depth and lag

- [ ] **Step 1: Write failing behavior test**

Create `src-tauri/tests/observability_tests.rs`:
```rust
use repo_analyzer_core::observability::JobMetrics;

#[test]
fn operator_sees_queue_depth_and_promotion_lag() {
    let mut metrics = JobMetrics::default();
    metrics.record_enqueue(3);
    metrics.record_promotion_lag_ms(250);

    assert_eq!(metrics.queue_depth, 3);
    assert_eq!(metrics.promotion_lag_ms, 250);
}
```

- [ ] **Step 2: Verify RED**

Run:
```bash
cargo test --manifest-path src-tauri/Cargo.toml operator_sees_queue_depth_and_promotion_lag --test observability_tests -- --exact
```
Expected: FAIL because module absent.

- [ ] **Step 3: Minimal GREEN**

Create `src-tauri/src/observability.rs`:
```rust
#[derive(Debug, Default, Clone)]
pub struct JobMetrics {
    pub queue_depth: usize,
    pub promotion_lag_ms: u64,
    pub promoted_events: usize,
}

impl JobMetrics {
    pub fn record_enqueue(&mut self, depth: usize) {
        self.queue_depth = depth;
    }

    pub fn record_promotion_lag_ms(&mut self, lag_ms: u64) {
        self.promotion_lag_ms = lag_ms;
    }
}
```

Modify `src-tauri/src/lib.rs`:
```rust
pub mod observability;
```

- [ ] **Step 4: Verify GREEN**

Run same test. Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add src-tauri/src/lib.rs src-tauri/src/observability.rs src-tauri/tests/observability_tests.rs
git commit -m "feat(F-026): expose ingestion job metrics"
```

---

## Phase 8: Admin UI Completion and Explainability

**Features:** `F-015`, `F-016`, `F-024`, `F-025`.

**BDD Story:** As an admin, I can ingest, promote, query trends, inspect PR ranking rationale, and manage baselines from UI controls.

**Files:**
- Modify: `ui/index.html`
- Modify: `src-tauri/src/main.rs`
- Modify: `src-tauri/src/admin.rs`
- Test: `src-tauri/tests/ui_contract_tests.rs`

### Task 8.1: UI exposes all admin command controls

- [ ] **Step 1: Write failing behavior test**

Create `src-tauri/tests/ui_contract_tests.rs`:
```rust
#[test]
fn admin_ui_exposes_all_maturity_commands() {
    let html = std::fs::read_to_string("../ui/index.html").unwrap();

    for id in [
        "ingestEvent",
        "promoteLifecycle",
        "queryAggregates",
        "committerScores",
        "rankPrs",
        "updateScoringWeights",
        "baselineReseed",
    ] {
        assert!(html.contains(id), "missing UI control id {id}");
    }
}
```

- [ ] **Step 2: Verify RED**

Run:
```bash
cargo test --manifest-path src-tauri/Cargo.toml admin_ui_exposes_all_maturity_commands --test ui_contract_tests -- --exact
```
Expected: FAIL because current UI only has scan/query controls.

- [ ] **Step 3: Minimal GREEN**

Add one panel to `ui/index.html` with buttons/sections using the exact IDs in the test. Wire existing Tauri `invoke` command names only after each related control has a failing contract test.

- [ ] **Step 4: Verify GREEN**

Run same test. Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add ui/index.html src-tauri/tests/ui_contract_tests.rs
git commit -m "feat(F-015): expose maturity admin controls in UI"
```

### Task 8.2: PR ranking rationale renders score decomposition

- [ ] **Step 1: Write failing behavior test**

Add to `src-tauri/tests/ui_contract_tests.rs`:
```rust
#[test]
fn admin_ui_renders_pr_ranking_rationale_columns() {
    let html = std::fs::read_to_string("../ui/index.html").unwrap();

    for label in ["Risk", "Velocity", "Approval", "Rationale"] {
        assert!(html.contains(label), "missing ranking label {label}");
    }
}
```

- [ ] **Step 2: Verify RED**

Run exact test. Expected: FAIL until labels exist.

- [ ] **Step 3: Minimal GREEN**

Add a PR ranking table with columns `Risk`, `Velocity`, `Approval`, and `Rationale`.

- [ ] **Step 4: Verify GREEN**

Run:
```bash
cargo test --manifest-path src-tauri/Cargo.toml --test ui_contract_tests
```
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add ui/index.html src-tauri/tests/ui_contract_tests.rs
git commit -m "feat(F-024): render PR ranking rationale in UI"
```

---

## Phase 9: Internal Provider and Directory Adapters

**Features:** `F-022`, `F-023`.

**BDD Story:** As an enterprise admin, internal Git providers and directory groups are explicit, testable contracts with local/on-prem auth boundaries.

**Files:**
- Modify: `src-tauri/src/onprem.rs`
- Test: `src-tauri/tests/provider_adapter_tests.rs`
- Test: `src-tauri/tests/directory_provider_tests.rs`

### Task 9.1: Git provider adapter declares supported schemes

- [ ] **Step 1: Write failing behavior test**

Create `src-tauri/tests/provider_adapter_tests.rs`:
```rust
use repo_analyzer_core::onprem::{GitProviderKind, ProviderEndpoint};

#[test]
fn enterprise_git_provider_accepts_only_onprem_schemes() {
    assert!(ProviderEndpoint::new(GitProviderKind::GitHubEnterprise, "https://ghe.local/api").is_ok());
    assert!(ProviderEndpoint::new(GitProviderKind::GitLabSelfManaged, "https://gitlab.local/api").is_ok());
    assert!(ProviderEndpoint::new(GitProviderKind::BitbucketServer, "https://bitbucket.local/rest").is_ok());
    assert!(ProviderEndpoint::new(GitProviderKind::GitHubEnterprise, "https://api.github.com").is_err());
}
```

- [ ] **Step 2: Verify RED**

Run exact test. Expected: FAIL because types absent.

- [ ] **Step 3: Minimal GREEN**

Add `GitProviderKind` enum and `ProviderEndpoint::new` to `src-tauri/src/onprem.rs`; reject known public SaaS hosts.

- [ ] **Step 4: Verify GREEN**

Run:
```bash
cargo test --manifest-path src-tauri/Cargo.toml --test provider_adapter_tests
cargo test --manifest-path src-tauri/Cargo.toml --test onprem_and_errors_tests
```
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add src-tauri/src/onprem.rs src-tauri/tests/provider_adapter_tests.rs
git commit -m "feat(F-022): validate on-prem Git provider endpoints"
```

### Task 9.2: directory group cache maps admin role deterministically

- [ ] **Step 1: Write failing behavior test**

Create `src-tauri/tests/directory_provider_tests.rs`:
```rust
use repo_analyzer_core::onprem::DirectoryGroupCache;

#[test]
fn ldap_group_cache_maps_admin_role_deterministically() {
    let mut cache = DirectoryGroupCache::default();
    cache.insert("alice", vec!["repo-admins".to_string()]);

    let roles = cache.roles_for("alice");

    assert_eq!(roles, vec!["admin".to_string()]);
}
```

- [ ] **Step 2: Verify RED**

Run exact test. Expected: FAIL because cache absent.

- [ ] **Step 3: Minimal GREEN**

Add `DirectoryGroupCache` to `src-tauri/src/onprem.rs` using `BTreeMap<String, Vec<String>>`; map `repo-admins` to `admin`.

- [ ] **Step 4: Verify GREEN**

Run exact test. Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add src-tauri/src/onprem.rs src-tauri/tests/directory_provider_tests.rs
git commit -m "feat(F-023): map directory groups to admin roles"
```

---

## Phase 10: Historical Bulk Import

**Features:** `F-027`.

**BDD Story:** As a migration owner, historical telemetry import is idempotent and sanitizes imported records before promotion.

**Files:**
- Create: `src-tauri/src/import.rs`
- Modify: `src-tauri/src/lib.rs`
- Modify: `src-tauri/src/admin.rs`
- Test: `src-tauri/tests/bulk_import_tests.rs`

### Task 10.1: bulk import deduplicates commit IDs

- [ ] **Step 1: Write failing behavior test**

Create `src-tauri/tests/bulk_import_tests.rs`:
```rust
use repo_analyzer_core::import::BulkImportPlan;
use repo_analyzer_core::types::CommitIngestionEvent;

fn event(id: &str) -> CommitIngestionEvent {
    CommitIngestionEvent {
        commit_id: id.to_string(),
        repo_name: "repo".to_string(),
        release: "v1".to_string(),
        committer: "dev".to_string(),
        telemetry: vec![],
    }
}

#[test]
fn migration_bulk_import_deduplicates_commit_ids() {
    let plan = BulkImportPlan::from_events(vec![event("a"), event("a"), event("b")]);

    assert_eq!(plan.unique_events.len(), 2);
    assert_eq!(plan.duplicates, 1);
}
```

- [ ] **Step 2: Verify RED**

Run:
```bash
cargo test --manifest-path src-tauri/Cargo.toml migration_bulk_import_deduplicates_commit_ids --test bulk_import_tests -- --exact
```
Expected: FAIL because module absent.

- [ ] **Step 3: Minimal GREEN**

Create `src-tauri/src/import.rs` with `BulkImportPlan { unique_events, duplicates }` using `BTreeSet` on `commit_id`.

Modify `src-tauri/src/lib.rs`:
```rust
pub mod import;
```

- [ ] **Step 4: Verify GREEN**

Run same test. Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add src-tauri/src/lib.rs src-tauri/src/import.rs src-tauri/tests/bulk_import_tests.rs
git commit -m "feat(F-027): deduplicate historical bulk import events"
```

---

## Final Verification Gate

- [ ] **Step 1: Run all Rust tests**

```bash
cargo test --manifest-path src-tauri/Cargo.toml
```
Expected: PASS. If dependency compile exceeds local timeout, rerun with 900-second timeout before declaring incomplete.

- [ ] **Step 2: Run roadmap grep audit**

```bash
rg "F-008A|F-008B|F-008C|F-008D|F-015|F-016|F-017|F-018|F-019|F-020|F-021|F-022|F-023|F-024|F-025|F-026|F-027" docs/roadmap src-tauri/tests src-tauri/src
```
Expected: every roadmap feature appears in docs and/or tests.

- [ ] **Step 3: Update roadmap status**

Move completed features in `docs/roadmap/feature-backlog.html` from `In Progress`/`New Backlog` to `Completed Features` only after corresponding tests pass.

- [ ] **Step 4: Update issue tracker**

Update `docs/roadmap/bead-issue-tracker.html` with BDD cycle notes:
```text
Analysis: roadmap gap mapped to behavior.
Hypothesis: failing behavior test proves gap and minimal implementation closes it.
Verification: focused test and full regression command passed.
Conclusion: feature is stable enough to mark complete.
```

- [ ] **Step 5: Run graph update if code changed**

```bash
graphify update .
```
Expected: graph metadata refresh completes.

- [ ] **Step 6: Persist architectural memory**

Use NeuroStrata namespace `rocinante` to save the completed phase summary with anchors to changed files.

---

## Traceability Table

| ID | Severity | Summary | Category | Action Taken |
|---|---|---|---|---|
| R1-F01 | CRITICAL | BadgerDB required | Must-fix | Phase 1 enforces strict Badger sidecar and rejects fallback |
| R1-F02 | HIGH | storage boundary non-bypassable | Must-fix | Phase 2 adds route guard tests and admin enforcement |
| R1-F03 | HIGH | retention split missing | Must-fix | Phase 3 adds TTL prune and release history preservation |
| R1-F04 | HIGH | snapshot isolation missing | Must-fix | Phase 4 materializes immutable analytics snapshots |
| R1-F05 | MEDIUM | global lock risk | Bundle | Phase 1 sharded keys + Phase 7 metrics/concurrency tests |

Total findings: 5 | Must-fix: 4 | Bundle: 1 | Defer: 0 | Info: 0

Dissent Ledger: none

---

## Execution Handoff

Plan complete and saved to `docs/superpowers/plans/2026-06-07-roadmap-bdd-tdd-completion.md`.

Execution options:

1. **Subagent-Driven (recommended)** — dispatch fresh agent per phase/task, review between tasks, fastest feedback.
2. **Inline Execution** — execute in this session using executing-plans, with checkpoints after every phase.
