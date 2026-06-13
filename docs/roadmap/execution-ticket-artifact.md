# Single-Source Execution Ticket Artifact

## Source of Truth (canonical grounding)
- `docs/roadmap/feature-backlog.html`
- `docs/roadmap/beads.html`
- `docs/roadmap/bead-issue-tracker.html`
- `docs/roadmap/test-plan.html`
- `docs/roadmap/plan-review-traceability.html`

This artifact is the authoritative execution index for implementation, TDD/BDD,
tracking, and gating. All ticket states and AC are grounded in the five files
above and must be kept in sync by updating those sources first.

## Remediation execution checkpoint (2026-06-08)

### TDD/BDD agent stream commits

- Stream B (Trust + Sanitization): `aa8cc39`
- Stream A (Storage): `bd189b8`
- Stream C (Sanitizer): `0c1a868`

- Remediation continuation checkpoint slice (2026-06-08):
  - `f7e10c6 chore(format): normalize merged rust sources`
  - `93262b3 chore(merge): apply pending auth and lockfile drift`

- Merge safety evidence:
  - Local `main` is fully synchronized with upstream `origin/main` for all upstream CodeQL / Dependabot history (`git fetch` + `git rev-list --left-right --count main...origin/main` => `30 0`).
  - Security workflows currently include CodeQL + TruffleHog (`.github/workflows/security.yml`).
  - Dependabot configuration exists in `.github/dependabot.yml`.

- Validation status for this checkpoint:
  - Focused tests attempted, but blocked by network dependency resolution (`sqlite-wasm-rs` from crates.io DNS failure), so functional retest is pending network-enabled environment.

### Current status mapping

- `F-008A` (storage engine conformance migration): Completed.
- `F-008B` (strict storage boundary enforcement): Completed.
- `F-008D` (retention compliance automation): Completed.
- `F-028` (async ingestion decoupling): Completed.
- `F-029` (UTF-safe sanitization): Completed.
- `F-030` (signed principal hardening): Completed.
- `FE-009` and `F-031` command contract tracks remain In Progress; requires UI commit-plane sweep.
- `F-015` command bridge now exposes all Tauri admin controls in the UI with test coverage; checkpoint validated on `feat/f015-admin-ui-bridge`.
- `F-008C` (snapshot/replica read model): Completed.
- `F-008E` (storage lock ownership): Completed.
- `F-008F` (promotion snapshot visibility): Completed.
- `F-020` (incremental AST cache and parser plugin): Completed; validates language-aware metrics with incremental cache hit/miss tracking.
- `F-021` (historical partition pruning and retention policies): Completed; release-partition pruning now applies per repository and keeps historical queries queryable via rollup.
- `F-032` (headless Playwright frontend behavioral and functional coverage): Completed.

### Remaining feature hierarchy

1. Control-plane convergence
   - `F-031` frontend structural decomposition
   - `FE-009` command schema and backend contract convergence
2. Dashboard and operational insight
   - `F-016` rich dashboard visualizations
   - `F-024` explainability panel
   - `F-025` release baseline management UI
   - `F-026` job observability
3. Governance and trust
   - `F-017` expanded sanitizer rules
   - `F-022` internal Git provider adapters
   - `F-023` AD/LDAP group mapping hardening
4. Scale and history
   - `F-027` bulk import utility
5. Untriaged backlog tail
   - `F-033`

## Roadmap Completion Snapshot (as of 2026-06-11)

- Completed features: `F-001` … `F-014`, `F-008A`, `F-008B`, `F-008C`, `F-008D`,
  `F-008E`, `F-008F`, `F-015`, `F-018`, `F-019`, `F-020`, `F-021`, `F-028`, `F-029`, `F-030`, `F-032` (29)
- In progress features: `F-031`, `F-016`, `F-017` (3)
- New backlog: `F-022` … `F-027`, `F-033` (7)
- Completion ratio: `29 / 39 = 74.4%`
- Readiness checkpoint (2026-06-10, branch `feat/bi-ready-queue-observability`):
  - Added queue backpressure observability for async ingestion (`enqueue_rejections`),
    validated by `async_ingestion_engine_tracks_enqueue_rejections_under_burst_pressure`
    in `storage_duallayer_tests.rs`.
  - `F-028` remains In Progress with expanded visibility telemetry evidence.
- Readiness checkpoint (2026-06-10, branch `feat/bi-ready-f028-queue-lag`):
  - Added queue lag + promotion throughput assertions under burst pressure
    (`async_ingestion_engine_tracks_queue_lag_and_promotion_throughput` in
    `storage_duallayer_tests.rs`) and aligned acceptance criteria evidence.
- Readiness checkpoint (2026-06-10, branch `feat/bi-ready-slice-008a-2026-06-10`):
  - Made release retention tie-break ordering deterministic (`ORDER BY MAX(ts) DESC, MAX(release) DESC`)
    in `prune_analytics_releases_with_retention` and reran `storage_duallayer_tests`;
    all 15 tests now pass, including long-release pruning regression.
- Readiness checkpoint (2026-06-10, branch `feat/bi-ready-slice-008a-2026-06-10`):
  - Normalized Badger sidecar endpoint parsing to trim whitespace before transport dispatch;
    added `badger_sidecar_inproc_transport_trims_endpoint_whitespace` in
    `ingestion_transport_tests.rs` and confirmed transport tests green.
- Readiness checkpoint (2026-06-10, branch
  `feat/bi-ready-slice-008a-endpoint-schema`):
  - Added strict Badger sidecar endpoint scheme validation in
    `IngestionBackendConfig::validate` (`inproc://` + `unix://` only),
    blocking unsupported schemes before transport dispatch.
  - Added `badger_sidecar_rejects_invalid_endpoint_scheme` and
    `badger_sidecar_rejects_empty_endpoint_with_whitespace` in
    `ingestion_backend_tests.rs`; updated transport unsupported-scheme regression to
    assert validation-time rejection.
- Readiness checkpoint (2026-06-10, branch
  `feat/bi-ready-slice-008a-ingest-durability`):
  - Added transport-failure durability regression coverage in
    `ingestion_transport_tests.rs` ensuring unix socket errors do not persist raw
    `evt:` rows in Sled while returning transport error.
- Readiness checkpoint (2026-06-11, branch
  `feat/bi-ready-slice-029-punct-emoji`):
  - Added punctuation/emoji regression coverage to sanitizer redaction with
    `scrubs_secret_values_with_emoji_separator_noise` in `sanitizer_tests.rs`.
  - Extended `redact_key_value` to tolerate punctuation/emoji noise between
    key and separator while still failing closed if a real token begins first.
  - Re-ran the full `src-tauri` test suite; all tests passed.

## Global Acceptance Criteria (Capability-level, BDD)

1. **Given** strict mode is enabled and non-conforming storage config is used, **when**
   any ingest command executes, **then** startup or first ingest fails without
   partial writes.
2. **Given** an admin command is invoked with malformed/unsigned/expired/wrong
   audience principal, **when** the command attempts authorization, **then**
   command is denied and returns explicit unauthorized reason.
3. **Given** Unicode-heavy sanitizer input, **when** redaction runs, **then**
   redaction is deterministic, UTF-8-safe, and non-panicking.
4. **Given** nested payload envelope with partial limits, **when** the UI applies it,
   **then** normalized defaults are deterministic and snapshot metrics stay stable.
5. **Given** bounded async enqueue under burst load, **when** queue depth rises, **then**
   inline promotion remains deferred to worker cycles and lag stays within SLO.
6. **Given** concurrent async retention and promotion operations on shared storage paths,
   **when** ownership and snapshot handoff occurs, **then** lock ownership and aggregate
   visibility remain deterministic for all query APIs.

## Execution Tickets (single source)

### Capability C1: Storage Integrity and Throughput

- Epic `E-STORE-01` — Strict dual-layer storage architecture
- Governing finding: `R1-F01`, `R1-F02`, `R1-F03`, `R1-F04`, `R1-F05`,
  `R2-F01`, `R2-F06`, `R2-F07`
- Epic AC:
  - Given strict/dual-mode config and concurrent producers, when ingest/query commands
    run, then storage workload never crosses engine boundaries and analytics continue
    under sustained writes.

#### Feature `F-008A` — Storage engine conformance migration
- Source: `docs/roadmap/feature-backlog.html`
- Ticket: `BI-001`, `BI-005`
- Bead context: `B-04`
- Current status: Completed
- TDD AC:
  - `StorageProfile::validate` and `IngestionBackendConfig::validate` reject non-Badger
    ingestion settings.
  - `tests/storage_policy_tests.rs` verifies red-to-green on valid/invalid profile.
  - `tests/ingestion_transport_tests.rs` verifies endpoint normalization for sidecar dispatch.
- Tasks:
  1. `TK-014` Persist to BadgerDB via sidecar boundary.
  2. `TK-015` Preserve parallel ingestion safety.
  3. `TK-016` Add durability/recovery checks.
  4. `TK-033` Shard key prefixes.
  5. `TK-034` Enforce raw event TTL and prune/roll-up.
- Function AC:
  - `StorageProfile::validate` fails closed when non-Badger ingest profile used.
  - `RetentionPolicy::is_raw_event_expired` returns stable expiry in retention tests.

#### Feature `F-008B` — Strict storage boundary enforcement
- Source: `docs/roadmap/feature-backlog.html`
- Ticket: `BI-003`
- Bead context: `B-09`
- Current status: Completed
- TDD AC:
  - `tests/storage_duallayer_tests.rs` rejects analytics command against ingest route and
    vice versa.
  - `tests/admin_service_tests.rs` exercises analytics command endpoints (`query_aggregates`,
    `committer_scores`, `rank_prs`) to ensure analytics-route usage remains enforced.
- Tasks:
  1. `TK-037` Enforce Badger-only writes and DuckDB-only analytics.
  2. Add boundary guard coverage in command layer.
- Function AC:
  - `StorageRoute::enforce` emits explicit boundary fault on mismatch.
  - `require_admin` only authorizes storage-layer compatible commands.

#### Feature `F-008C` — Snapshot/replica read model
- Source: `docs/roadmap/feature-backlog.html`
- Ticket: `BI-004`
- Bead context: `B-05`
- Current status: Completed
- TDD AC:
  - Snapshot query tests show reads succeed while promotion runs.
  - `tests/storage_duallayer_tests.rs::query_aggregates_stable_while_promotion_runs_via_immutable_snapshot`
  - `tests/storage_duallayer_tests.rs::committer_score_read_uses_published_snapshot_when_live_db_is_unavailable`
- Tasks:
  1. `TK-035` Read via immutable snapshot/replica strategy. ✅
  2. Add contention tests under burst writes. ✅
  3. `TK-036` Keep committer score read path on published snapshot during fallback. ✅
- Function AC:
  - `AnalyticsSnapshot::enforce_mode` blocks mutable reads under snapshot mode.
  - `query_aggregates` returns stable views under concurrent promotion.

#### Feature `F-008D` — Retention compliance automation
- Source: `docs/roadmap/feature-backlog.html`
- Ticket: `BI-002`
- Bead context: `B-05`
- Current status: Completed
- TDD AC:
  - `tests/storage_policy_tests.rs` validates TTL and roll-up execution windows.
  - `tests/storage_duallayer_tests.rs::async_ingestion_engine_applies_retention_before_promotion` validates background retention on async promotion cycles.
  - `tests/storage_duallayer_tests.rs::prunes_old_releases_and_preserves_queryability_by_rollup` validates long-term release rollup retention while preserving legacy queryability.
- Tasks:
  1. `TK-016` Add ingestion durability checks.
  2. `TK-034` Add short-term TTL + prune + roll-up schedule checks. complete
  3. `TK-036` Add release-partitioned compression + retention path for long-term marts.
- Function AC:
  - `RetentionPolicy::is_raw_event_expired` deterministically classifies records.
  - `promote_to_columnar_with_retention` retains short-term TTL + archive/publish release partitions in `telemetry_history_rollup`.

#### Feature `F-008E` — Storage lock ownership and path exclusivity
- Source: `docs/roadmap/feature-backlog.html`
- Ticket: `BI-011`
- Bead context: `B-10`
- Current status: Completed
- TDD AC:
  - `tests/storage_duallayer_tests.rs` never exposes shared lock panics.
  - `tests/storage_duallayer_tests.rs::promotes_events_and_reads_aggregates` requires deterministic ownership across promotion.
- Tasks:
  1. `TK-049` Register and serialize DB path ownership.
  2. `TK-050` Add re-open prevention across concurrent lifecycle calls.
- Function AC:
  - `StorageProfile::acquire_owner_token` rejects parallel conflicting open attempts.
  - `AsyncIngestionEngine` logs ownership transitions and route eligibility.

#### Feature `F-028` — Async ingestion decoupling completion
- Source: `docs/roadmap/feature-backlog.html`
- Ticket: `BI-006`
- Bead context: `B-05`
- Current status: Completed
- Governing finding: `R2-F01`
- TDD AC:
  - `AsyncIngestionEngine::enqueue` never invokes promotion inline.
  - Worker lag metrics are captured and asserted under burst scenarios.
- Tasks:
  1. `TK-017` Keep promotions worker-only.
  2. Add queue-lag and throughput assertions. ✅
  3. Add queue-depth telemetry. ✅
- Function AC:
  - `AsyncIngestionEngine::start` enforces bounded enqueue and worker scheduling.

#### Feature `F-008F` — Promotion snapshot consistency
- Source: `docs/roadmap/feature-backlog.html`
- Ticket: `BI-012`
- Bead context: `B-10`
- Current status: Completed
- TDD AC:
  - `tests/storage_duallayer_tests.rs::promotes_events_and_reads_aggregates` must return expected aggregate counts after handoff.
  - `query_aggregates` remains consistent between promotion start and completion.
- Tasks:
  1. `TK-051` Enforce snapshot publish barrier before read-route flips.
  2. `TK-052` Add end-to-end visibility assertions around promotion boundary.
- Function AC:
  - `StorageSnapshot::publish` blocks visibility until committed copy is complete.
  - `query_aggregates` returns committed values or explicit in-progress status.

### Capability C2: Security and Trust

- Epic `E-SEC-01` — Signed token principal & command trust boundary
- Governing finding: `R2-F02`
- Epic AC:
  - Given malformed/expired/unsigned principal claims, when any admin command executes,
    then command aborts with explicit denial and no storage side-effects.

#### Feature `F-030` — Signed principal hardening
- Source: `docs/roadmap/feature-backlog.html`
- Ticket: `BI-007`
- Bead context: `B-10`
- Current status: Completed
- Tasks:
  1. `TK-038` Replace plain decode with signed claim parsing.
  2. `TK-039` Validate expiry, issuer, audience.
  3. `TK-040` Add explicit deny-by-default behavior.
  4. `TK-041` Regression tests for claims and escalation attempts.
- Function AC:
  - `decode_principal` verifies signature + claims.
  - `require_admin` returns `DENY` for expired/wrong-audience/unsigned inputs.
  - `main.rs` command handlers do not open backends on auth failure.

#### Feature `F-007` — Existing RBAC baseline continuity
- Source: `docs/roadmap/feature-backlog.html`
- Governing reference: `B-08`, `R1-F02`
- AC:
  - Admin-only operations remain denied without valid claim/role.
- Functions:
  - `require_admin` returns explicit unauthorized and logs reason.

### Capability C3: Sanitization Correctness

- Epic `E-SAN-01` — Unicode-safe deterministic redaction
- Governing finding: `R2-F03`
- Epic AC:
  - Given payload with multibyte characters and punctuation boundaries, when redaction runs,
    then no sensitive token leaks and no panic occurs.

#### Feature `F-029` — UTF-safe sanitization hardening
- Source: `docs/roadmap/feature-backlog.html`
- Ticket: `BI-008`
- Bead context: `B-11`
- Current status: Completed
- Tasks:
  1. `TK-042` Replace byte-index parsing with UTF-8 scanners.
  2. `TK-043` Add boundary-aware inline key-value redaction.
  3. `TK-044` Add punctuation/emoji regression corpus.
- Function AC:
  - `scrub_text` and `redact_key_value` preserve UTF-8 and redact expected secret spans.
  - `scrub_record_strings` handles malformed/tricky separators safely.

### Capability C4: UX Architecture and Contract Safety

- Epic `E-FE-01` — UI decomposition and contract isolation
- Governing finding: `R2-F04`
- Epic AC:
  - Given malformed payloads or command failures, when UI refreshes, then rendering
    remains stable with explicit fallback/error contracts.

#### Feature `F-031` — Frontend structural decomposition
- Source: `docs/roadmap/feature-backlog.html`
- Ticket: `BI-FE-016`
- Bead context: `FE-008`
- Current status: In Progress
- Tasks:
  1. `TK-FE-028` Split orchestration and rendering concerns.
  2. `TK-FE-029` Define explicit command/payload contracts.
  3. `TK-FE-030` Add role-switch and snapshot decomposition tests.
- Function AC:
  - `readPayload` and `readLimits` fail-safe on malformed input.
  - App shell only orchestrates composed components; no business logic leakage.

- Readiness checkpoint:
  - Extracted `dashboard-contract` and `admin-bridge-contract` helpers from `App.tsx`.
  - Added unit coverage for nested payload/limit normalization and admin command payload mapping.
  - App shell now delegates payload normalization and admin bridge payload shaping to testable modules.
- Readiness checkpoint:
  - Extracted static dashboard copy and section finding groups into `dashboard-content`.
  - Added focused unit coverage for role copy and reusable finding-group content.
  - `App.tsx` now consumes shared dashboard content constants for the lead/manager/executive/security views.
- Readiness checkpoint:
  - Extracted `insight-engine` and `quality-pulse` helpers for the audience-pane decomposition slice.
  - Tightened App tests around exact recommendation messages and list scoping.
  - `App.tsx` now keeps routing text, score summaries, and action lists in testable helper modules.

#### Feature `FE-009` — Command schema and backend contract convergence
- Source: `docs/roadmap/beads.html`
- Ticket: `BI-FE-017`
- Bead context: `FE-009`
- Current status: In Progress
- Governing finding: `R2-F05`
- Tasks:
  1. `TK-FE-031` Define canonical argument schemas.
  2. `TK-FE-032` Add nested envelope/fallback contract tests.
  3. `TK-FE-033` Add transport/auth failure contract tests.
- Function AC:
  - `invokeAdminCommand` argument shape must map 1:1 to backend signatures.
  - `main.rs` command parse/dispatch emits deterministic error envelopes.

#### Feature `F-032` — Playwright frontend behavioral and functional coverage suite
- Source: `docs/roadmap/feature-backlog.html`
- Ticket: `BI-013`
- Bead context: `B-11`
- Current status: Completed
- Tasks:
  1. `TK-FE-034` Add headless-only browser harness against the local Vite server.
  2. `TK-FE-035` Cover dashboard shell, role switching, payload envelopes, and admin fallback flows.
  3. `TK-FE-036` Gate the suite in CI as the browser behavior contract for frontend changes.
- Function AC:
  - Headless Chromium runs verify the visible dashboard shell and core user flows.
  - Browser tests exercise behavioral and functional coverage without relying on headed mode.

### Frontend Delivery Strand (already active)

- Feature `F-015` — Admin UI integration for command set (`FE-007`-like lane)
- Ticket: `BI-FE-015`
- Status: Completed
- AC: End-user command controls exist for ingest/promote/query/committer-scores/PR-ranking/weight-update with explicit browser/Tauri fallback and test coverage.

- Feature `F-016` — Rich dashboard visualizations
- Ticket: `BI-FE-015` (continued operational context)
- Status: In Progress
- AC: trend/risk views are deterministic under valid and fallback payloads.
- Readiness checkpoint:
  - Extracted `dashboard-visuals` to centralize trend and PR risk ranking derivation.
  - Added UI coverage for the new trend/risk view and helper-backed ranking copy.
  - `App.tsx` now renders the trend/risk lane from shared helper output rather than inline composition.

- Feature `F-017` — Expanded sanitizer rules
- Ticket: `BI-008`
- Status: In Progress
- AC: additional policy packs apply without regressions in existing redaction engine tests.
- Readiness checkpoint:
  - Added `SanitizerPolicyPack` variants for General, Security, Privacy, and Payments.
  - Added pack-aware `scrub_text_with_pack(...)` coverage without changing the baseline `scrub_text(...)` contract.
  - Sanitizer regression tests now prove domain-specific redaction behavior and preserve existing emoji-separator handling.

- Feature `F-018` — Signed scoring-config integrity verification
- Ticket: `BI-014`
- Status: Completed
- AC: persisted scoring configs carry tamper-evident hash/signature envelopes and reject altered content.
- Readiness checkpoint:
  - Added signed scoring-config envelope persistence with hash and signature verification.
  - Kept legacy raw JSON compatibility for existing weight files.
  - Added tamper-rejection and persistence regression coverage.

- Feature `F-019` — Per-team policy profiles for scoring/approval weighting
- Ticket: `BI-015`
- Status: Completed
- AC: team policies resolve to deterministic score/approval weight profiles with a safe default fallback.
- Readiness checkpoint:
  - Added a shared policy catalog for security, frontend, and platform profiles.
  - Added per-team scoring weight resolution plus fallback coverage for unknown teams.
  - Team-specific approval weighting now shifts without altering the baseline defaults.

- Feature `F-020` — Incremental AST cache and parser plugin
- Ticket: `BI-016`
- Status: Completed
- AC: language-aware parser metrics classify supported file types and reuse cached summaries for unchanged file content.
- Readiness checkpoint:
  - Added `ParserPlugin` with incremental cache hit/miss accounting and language classification for Rust, TypeScript, JavaScript, Python, Markdown, and unknown files.
  - Registered the parser plugin in the default pipeline alongside the existing bead plugins.
  - Added parser-specific regression coverage for cache reuse, cache invalidation, and default pipeline exposure.

- Feature `F-021` — Historical partition pruning and retention policies
- Ticket: `BI-017`
- Status: Completed
- AC: release partition retention prunes per repository while keeping stale historical releases queryable through rollup.
- Readiness checkpoint:
  - Added per-repo historical partition pruning coverage.
  - Preserved older releases in rollup form while pruning raw release partitions.
  - Added regression coverage for cross-repo retention isolation and queryability.

## TDD/BDD Mapping by Capability

- `F-008A/B/C/D` ↔ `T-015`, `T-016`, `T-017`, `T-018`
- `F-008E` ↔ `T-019`
- `F-008F` ↔ `T-020`
- `F-028` ↔ `T-009`
- `F-029` ↔ `T-003`, `T-022`
- `F-030` ↔ `T-020`
- `F-020` ↔ `T-012`
- `F-021` ↔ `T-013`
- `F-031`/`FE-009` ↔ `T-FE-011`, `T-023`
- `FE-009` command failures and parity ↔ `T-021`, `T-023`
- Security-sensitive features additionally require `T-001` and `T-020` authorization checks.

## Parallel Agent Dispatch (three independent streams)

1. **Stream A — Storage Hardening**
   - Tickets: `BI-001`, `BI-002`, `BI-003`, `BI-004`, `BI-005`, `BI-006`, `BI-011`, `BI-012`
   - Dependency: complete before any storage-contract closure is marked done.
   - Exit gate: `R1-F01..R2-F07` risk evidence + `T-015..T-020`.

2. **Stream B — Trust/Identity + Sanitization**
   - Tickets: `BI-007`, `BI-008`
   - Dependency: `R2-F02` and `R2-F03` green in traceability.
   - Exit gate: `T-020`, `T-022`, auth no-side-effect verification.

3. **Stream C — Frontend Contract Safety**
   - Tickets: `BI-FE-016`, `BI-FE-017`
   - Dependency: UI component split and command schema map.
   - Exit gate: `T-FE-007`, `T-FE-008`, `T-FE-011`, `T-023`.

## Cross-file Drift Guard

- Any edit to this artifact must include corresponding source-of-truth updates in:
  - roadmaps (`feature-backlog`, `beads`) for scope/status.
  - issue tracker (`bead-issue-tracker`) for ticket state and ledger.
  - test-plan for test IDs and DoD adjustments.
  - traceability doc for new findings, capabilities, and AC changes.
