# Graph Report - /Users/neo/projects/Git-SCM/rocinante/.worktrees/roadmap-completion  (2026-06-07)

## Corpus Check
- 45 files · ~32,558 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 251 nodes · 409 edges · 17 communities detected
- Extraction: 61% EXTRACTED · 39% INFERRED · 0% AMBIGUOUS · INFERRED: 160 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Community Hubs (Navigation)
- [[_COMMUNITY_Community 0|Community 0]]
- [[_COMMUNITY_Community 1|Community 1]]
- [[_COMMUNITY_Community 2|Community 2]]
- [[_COMMUNITY_Community 3|Community 3]]
- [[_COMMUNITY_Community 4|Community 4]]
- [[_COMMUNITY_Community 5|Community 5]]
- [[_COMMUNITY_Community 6|Community 6]]
- [[_COMMUNITY_Community 7|Community 7]]
- [[_COMMUNITY_Community 8|Community 8]]
- [[_COMMUNITY_Community 9|Community 9]]
- [[_COMMUNITY_Community 10|Community 10]]
- [[_COMMUNITY_Community 11|Community 11]]
- [[_COMMUNITY_Community 12|Community 12]]
- [[_COMMUNITY_Community 13|Community 13]]
- [[_COMMUNITY_Community 14|Community 14]]
- [[_COMMUNITY_Community 15|Community 15]]
- [[_COMMUNITY_Community 16|Community 16]]

## God Nodes (most connected - your core abstractions)
1. `scrub_text()` - 11 edges
2. `decode_principal()` - 10 edges
3. `ingest_event()` - 10 edges
4. `admin_services_roundtrip_happy_path()` - 9 edges
5. `DualLayerStore` - 9 edges
6. `load_or_init_weights()` - 8 edges
7. `require_admin()` - 8 edges
8. `committer_scores()` - 8 edges
9. `rank_prs()` - 8 edges
10. `run_scan()` - 7 edges

## Surprising Connections (you probably didn't know these)
- `query_metrics()` --calls--> `query_metrics()`  [INFERRED]
  /Users/neo/projects/Git-SCM/rocinante/.worktrees/roadmap-completion/src-tauri/src/main.rs → /Users/neo/projects/Git-SCM/rocinante/.worktrees/roadmap-completion/src-tauri/src/admin.rs
- `ingest_event()` --calls--> `ingest_event()`  [INFERRED]
  /Users/neo/projects/Git-SCM/rocinante/.worktrees/roadmap-completion/src-tauri/src/main.rs → /Users/neo/projects/Git-SCM/rocinante/.worktrees/roadmap-completion/src-tauri/src/admin.rs
- `promote_lifecycle()` --calls--> `promote_lifecycle()`  [INFERRED]
  /Users/neo/projects/Git-SCM/rocinante/.worktrees/roadmap-completion/src-tauri/src/main.rs → /Users/neo/projects/Git-SCM/rocinante/.worktrees/roadmap-completion/src-tauri/src/admin.rs
- `query_aggregates()` --calls--> `query_aggregates()`  [INFERRED]
  /Users/neo/projects/Git-SCM/rocinante/.worktrees/roadmap-completion/src-tauri/src/main.rs → /Users/neo/projects/Git-SCM/rocinante/.worktrees/roadmap-completion/src-tauri/src/admin.rs
- `committer_scores()` --calls--> `committer_scores()`  [INFERRED]
  /Users/neo/projects/Git-SCM/rocinante/.worktrees/roadmap-completion/src-tauri/src/main.rs → /Users/neo/projects/Git-SCM/rocinante/.worktrees/roadmap-completion/src-tauri/src/admin.rs

## Communities

### Community 0 - "Community 0"
Cohesion: 0.06
Nodes (21): run_scan(), mutable_mode_is_rejected_for_analytics_queries(), snapshot_descriptor_is_immutable_and_versioned(), snapshot_mode_is_read_only(), AstMetricsPlugin, CodeQualityPlugin, ComplexityPlugin, Pipeline (+13 more)

### Community 1 - "Community 1"
Cohesion: 0.13
Nodes (22): committer_scores(), ingest_event(), promote_lifecycle(), query_aggregates(), query_metrics(), rank_prs(), admin_services_reject_non_admin(), admin_services_roundtrip_happy_path() (+14 more)

### Community 2 - "Community 2"
Cohesion: 0.08
Nodes (17): AstCache, hash_value(), BadgerSidecarRequest, stable_shard(), BulkImportPlan, DirectoryGroupCache, PolicyProfiles, ast_cache_reuses_unchanged_file_fingerprint() (+9 more)

### Community 3 - "Community 3"
Cohesion: 0.08
Nodes (16): badger_sidecar_requires_endpoint(), strict_mode_rejects_transitional_sled_backend(), ci_strict_badger_rejects_inproc_fallback_endpoint(), AnalyticsEngine, AnalyticsQueryMode, AnalyticsSnapshot, AsyncIngestionEngine, IngestionBackendConfig (+8 more)

### Community 4 - "Community 4"
Cohesion: 0.15
Nodes (12): MandatorySanitizerPlugin, redact_emails(), redact_key_value(), redact_phone_like(), redact_with_patterns(), redacts_token_and_email(), scrub_metric(), scrub_record_strings() (+4 more)

### Community 5 - "Community 5"
Cohesion: 0.17
Nodes (16): governance_rejects_tampered_scoring_weights(), governance_scoring_signature_uses_sha256_hex_digest(), append_audit_log(), normalize_scores_orders_and_scales(), top_prs_returns_highest_first(), load_or_init_weights(), normalize_scores(), now_ts() (+8 more)

### Community 6 - "Community 6"
Cohesion: 0.14
Nodes (12): AdminQuery, AnalysisInput, AnalysisMetric, AnalysisRecord, CommitIngestionEvent, CommitterScore, PrCandidate, Principal (+4 more)

### Community 7 - "Community 7"
Cohesion: 0.15
Nodes (7): ActiveDirectoryProvider, local_git_provider_builds_internal_url(), DirectoryProvider, GitProviderKind, InternalGitProvider, LocalGitProvider, ProviderEndpoint

### Community 8 - "Community 8"
Cohesion: 0.2
Nodes (9): AppState, committer_scores(), ingest_event(), promote_lifecycle(), query_aggregates(), query_metrics(), rank_prs(), ScanPayload (+1 more)

### Community 9 - "Community 9"
Cohesion: 0.4
Nodes (2): JobMetrics, operator_sees_queue_depth_and_promotion_lag()

### Community 10 - "Community 10"
Cohesion: 0.4
Nodes (3): rejects_analytics_query_on_ingestion_route(), rejects_ingestion_write_on_analytics_route(), StorageRoute

### Community 11 - "Community 11"
Cohesion: 0.67
Nodes (2): blocks_ingestion_when_strict_mode_not_badger(), sample_event()

### Community 12 - "Community 12"
Cohesion: 0.5
Nodes (1): RawRetentionDecision

### Community 13 - "Community 13"
Cohesion: 0.67
Nodes (1): AnalyzerError

### Community 14 - "Community 14"
Cohesion: 1.0
Nodes (0): 

### Community 15 - "Community 15"
Cohesion: 1.0
Nodes (1): BeadPlugin

### Community 16 - "Community 16"
Cohesion: 1.0
Nodes (0): 

## Knowledge Gaps
- **25 isolated node(s):** `RepoTarget`, `AnalysisInput`, `AnalysisMetric`, `AnalysisRecord`, `AdminQuery` (+20 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **Thin community `Community 14`** (2 nodes): `main()`, `build.rs`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `Community 15`** (2 nodes): `BeadPlugin`, `mod.rs`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `Community 16`** (1 nodes): `lib.rs`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `main()` connect `Community 0` to `Community 8`, `Community 3`?**
  _High betweenness centrality (0.087) - this node is a cross-community bridge._
- **Why does `DualLayerStore` connect `Community 1` to `Community 3`?**
  _High betweenness centrality (0.075) - this node is a cross-community bridge._
- **Are the 5 inferred relationships involving `scrub_text()` (e.g. with `scrubs_pii_and_secret_values()` and `.query()`) actually correct?**
  _`scrub_text()` has 5 INFERRED edges - model-reasoned connections that need verification._
- **Are the 9 inferred relationships involving `decode_principal()` (e.g. with `admin_role_is_accepted()` and `non_admin_is_rejected()`) actually correct?**
  _`decode_principal()` has 9 INFERRED edges - model-reasoned connections that need verification._
- **Are the 9 inferred relationships involving `ingest_event()` (e.g. with `admin_services_roundtrip_happy_path()` and `admin_services_reject_non_admin()`) actually correct?**
  _`ingest_event()` has 9 INFERRED edges - model-reasoned connections that need verification._
- **Are the 7 inferred relationships involving `admin_services_roundtrip_happy_path()` (e.g. with `ingest_event()` and `promote_lifecycle()`) actually correct?**
  _`admin_services_roundtrip_happy_path()` has 7 INFERRED edges - model-reasoned connections that need verification._
- **What connects `RepoTarget`, `AnalysisInput`, `AnalysisMetric` to the rest of the system?**
  _25 weakly-connected nodes found - possible documentation gaps or missing edges._