# Native Shell Command Contract Inventory

**Date:** 2026-10-04
**Branch:** `fix/rocinante-readiness-remediation`
**Baseline source:** PR #108 commit `d42b2bc3ab3799d04c904d1a1b76c5a5b525e15a`
**Parent plan:** [`rustsec-zero-exception-remediation-plan.md`](rustsec-zero-exception-remediation-plan.md)

This inventory covers the eleven handlers registered by
`app_support::build_app`. It captures their argument and response contracts
before the Tauri adapter is retired. The shared service crates remain the
source of behavior; native-shell transport and controls must not silently
change these payloads.

| Registered command | Request fields | Response | Shared owner and current native route |
|---|---|---|---|
| `run_scan` | `payload: { token, root, release }` | `TelemetryImportSummary` | `rocinante-analysis::run_scan`; native shell calls `run_scan_with_metrics` and renders its summary plus metrics. |
| `query_metrics` | `token`, optional `name`, optional `release` | `Vec<AnalysisMetric>` (`plugin`, `key`, `value`, `details`) | `rocinante-storage::admin::query_metrics` to `rocinante-analysis::TelemetryStore::query`; stable/legacy duplicates are suppressed per basename and release without changing the response shape. |
| `ingest_event` | `token`, `event` | unit / JSON `null` | `rocinante-storage::admin::ingest_event`, also routed by the shared admin bridge. |
| `promote_lifecycle` | `token` | promoted event count (`usize`) | `rocinante-storage::admin::promote_lifecycle`, also routed by the shared admin bridge. |
| `query_aggregates` | `token`, optional `name`, optional `release` | `Vec<TelemetryPoint>` | `rocinante-storage::admin::query_aggregates`, also routed by the shared admin bridge. |
| `committer_scores` | `token`, optional `name`, optional `release` | `Vec<CommitterScore>` | `rocinante-storage::admin::committer_scores`, also routed by the shared admin bridge. |
| `rank_prs` | `token`, `prs` | `Vec<PrRanking>` | `rocinante-storage::admin::rank_prs`, also routed by the shared admin bridge. |
| `evaluate_pr_risk` | `token`, `candidate` | `PrRiskEvaluation` | `rocinante-storage::admin::evaluate_pr_risk` backed by the shared core risk contract. |
| `query_release_baseline` | `token`, `repoName` | optional `f64` | `rocinante-storage::admin::query_release_baseline`, also routed by the shared admin bridge. |
| `reseed_release_baseline` | `token`, `repoName`, `baselineComplexity` | `f64` | `rocinante-storage::admin::reseed_release_baseline`, also routed by the shared admin bridge. |
| `update_scoring_weights` | `token`, `weights` | unit / JSON `null` | `rocinante-storage::admin::update_scoring_weights`, also routed by the shared admin bridge. |

Tauri's command adapter accepts camel-case names for multiword parameters;
the shared bridge also accepts the existing camel-case baseline payload keys.
The React bridge currently exercises the eight storage/admin bridge actions.
Scanning and saved-metric loading have native-shell UI paths. PR-risk
evaluation remains available as a shared service; Phase 4B must retain its
command-level access or document a compatibility decision before removing the
Tauri adapter.

## Validation Evidence

- The registration list is defined once in `app_support::build_app` and every
  registered command above maps to a shared analysis, storage, or core service.
- `rocinante-storage/tests/admin_bridge_contract.rs` checks bridge command
  names, JSON request fields, response values, and admin authorization.
- `rocinante-analysis/tests/repository_scan.rs` covers authenticated scanning
  and saved metrics. `legacy_metric_query.rs` covers stable/legacy duplicate
  suppression and confirms older release rows remain visible.
- `rocinante-core` and storage admin-service tests cover PR-risk evaluation.
- The new telemetry compatibility tests pass (2/2); all analysis crate tests
  pass (11 tests across 5 suites), and strict analysis Clippy, workspace
  formatting, and `git diff --check` pass locally.

## Phase 4B Work Remaining

- Keep the eleven request/response shapes covered without `tauri::command`
  macros or Tauri runtime types.
- Keep PR-risk evaluation callable through the host-neutral application
  boundary.
- Move or remove the remaining Tauri adapter and package bootstrap only after
  native parity gates in BI-049/BI-050 are accepted.
- Preserve the present working-directory defaults for analytics and scoring
  until BI-058 migrates existing files with backup and rollback evidence.
