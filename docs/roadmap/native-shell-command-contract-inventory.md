# Retired Host Command Compatibility Inventory

**Date:** 2026-10-04
**Branch:** `fix/rocinante-readiness-remediation`
**Baseline source:** PR #108 commit `d42b2bc3ab3799d04c904d1a1b76c5a5b525e15a`
**Parent plan:** [`rustsec-zero-exception-remediation-plan.md`](rustsec-zero-exception-remediation-plan.md)

This inventory preserves the eleven public command names and wire shapes from
the retired Tauri host. `src-tauri/src/command_compat.rs` records those shapes
as host-neutral migration metadata and keeps the former Rust module path as a
compatibility re-export. It does not register IPC handlers. The native shell
calls the shared services directly, and the React/Vite UI is a browser preview
surface rather than a production desktop transport.

**Transport compatibility change:** the supported desktop no longer exposes
the former `window.__TAURI__.core.invoke` IPC surface. The inventory and the
service request/response contracts preserve the operation names and payload
shapes for migration and Rust callers; they do not supply a replacement
JavaScript IPC endpoint.

| Migrated command | Request contract | Response contract | Shared owner and current native route |
|---|---|---|---|
| `run_scan` | `payload: { token, root, release }` | `TelemetryImportSummary` | `rocinante-analysis::run_scan`; native shell calls `run_scan_with_metrics` and renders its summary plus metrics. |
| `query_metrics` | `token`, optional `name`, optional `release` | `Vec<AnalysisMetric>` (`plugin`, `key`, `value`, `details`) | `rocinante-storage::admin::query_metrics` to `rocinante-analysis::TelemetryStore::query`; stable/legacy duplicates are suppressed per basename and release without changing the response shape. |
| `ingest_event` | `token`, `event` | unit / JSON `null` | `rocinante-storage::admin::ingest_event`, also routed by the shared admin bridge. |
| `promote_lifecycle` | `token` | promoted event count (`usize`) | `rocinante-storage::admin::promote_lifecycle`, also routed by the shared admin bridge. |
| `query_aggregates` | `token`, optional `name`, optional `release` | `Vec<TelemetryPoint>` | `rocinante-storage::admin::query_aggregates`, also routed by the shared admin bridge. |
| `committer_scores` | `token`, optional `name`, optional `release` | `Vec<CommitterScore>` | `rocinante-storage::admin::committer_scores`, also routed by the shared admin bridge. |
| `rank_prs` | `token`, `prs` | `Vec<PrRanking>` | `rocinante-storage::admin::rank_prs`, also routed by the shared admin bridge. |
| `evaluate_pr_risk` | `token`, `candidate` | `PrRiskEvaluation` | `rocinante-storage::admin::evaluate_pr_risk` backed by the shared core risk contract and routed by the shared admin bridge. |
| `query_release_baseline` | `token`, `repoName` | optional `f64` | `rocinante-storage::admin::query_release_baseline`, also routed by the shared admin bridge. |
| `reseed_release_baseline` | `token`, `repoName`, `baselineComplexity` | `f64` | `rocinante-storage::admin::reseed_release_baseline`, also routed by the shared admin bridge. |
| `update_scoring_weights` | `token`, `weights` | unit / JSON `null` | `rocinante-storage::admin::update_scoring_weights`, also routed by the shared admin bridge. |

The retired Tauri adapter accepted camel-case names for multiword parameters.
The compatibility inventory preserves those names and payload shapes. The
native shell exposes supported scan, saved-metric, and admin operations
through shared Rust services; no desktop IPC transport is implied by this
inventory. The command name and `{ token, candidate }` payload are documented
for migration compatibility.

## Validation Evidence

- `MIGRATED_COMMAND_CONTRACTS` in `src-tauri/src/command_compat.rs` records the
  retired command surface as host-neutral compatibility metadata.
- `rocinante-storage/tests/admin_bridge_contract.rs` checks bridge command
  names, JSON request fields, response values, and admin authorization.
- `evaluate_pr_risk` remains available through the shared admin service with
  its existing `{ token, candidate }` payload and role check.
- `rocinante-analysis/tests/repository_scan.rs` covers authenticated scanning
  and saved metrics. `legacy_metric_query.rs` covers stable/legacy duplicate
  suppression and confirms older release rows remain visible.
- `rocinante-core` and storage admin-service tests cover PR-risk evaluation.
- The new telemetry compatibility tests pass (2/2); all analysis crate tests
  pass (11 tests across 5 suites), and strict analysis Clippy, workspace
  formatting, and `git diff --check` pass locally.
- Phase 4B local validation on 2026-10-04: storage bridge contract tests pass
  (3/3), the native-shell sample-payload contract passes (1/1), and Clippy for
  storage plus desktop-shell all-targets passes with warnings denied. Workspace
  Rust formatting passes. TypeScript build checking passes; Vitest passes all
  63 tests and Vite production build succeeds with its existing chunk-size
  advisory. These UI commands used installed local binaries because the pinned
  `pnpm@12.9.1` launch could not complete registry signature verification in
  this environment. Roadmap doc contracts (10/10) and advisory checker
  contracts pass. At that historical point the live advisory governance check
  remained blocked by two overdue records. Current local disposition and
  validation status are tracked in the remediation plan; hosted validation for
  the Tauri-retirement worktree is pending.

## Current Compatibility Boundaries

- Keep the eleven request/response shapes covered without `tauri::command`
  macros or Tauri runtime types; the source list and inventory are checked
  together by `native_shell_command_contract_tests.rs`.
- Preserve analytics and scoring working-directory defaults until BI-058
  migrates existing files with backup and rollback evidence.
- Complete hosted native-shell packaging and lifecycle validation before
  treating the desktop migration as release-ready.
