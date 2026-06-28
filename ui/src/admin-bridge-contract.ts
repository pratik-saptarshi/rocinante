import type {
  AdminBridgeCommand,
  AdminCommitterScoresPayload,
  AdminIngestPayload,
  AdminPromotePayload,
  AdminQueryPayload,
  AdminReleaseBaselineQueryPayload,
  AdminReleaseBaselineReseedPayload,
  AdminRankPrsPayload,
  AdminUpdateWeightsPayload
} from './tauri-admin';

export type AdminBridgeArgsMap = {
  ingest_event: AdminIngestPayload;
  promote_lifecycle: AdminPromotePayload;
  query_aggregates: AdminQueryPayload;
  committer_scores: AdminCommitterScoresPayload;
  rank_prs: AdminRankPrsPayload;
  query_release_baseline: AdminReleaseBaselineQueryPayload;
  reseed_release_baseline: AdminReleaseBaselineReseedPayload;
  update_scoring_weights: AdminUpdateWeightsPayload;
};

export function buildAdminBridgeArgs(token: string): AdminBridgeArgsMap {
  return {
    ingest_event: {
      token,
      event: {
        commit_id: 'ui-bridge-001',
        repo_name: 'sample-repo',
        release: 'v1.0.0',
        committer: 'ui',
        telemetry: [
          {
            plugin: 'ui',
            metric_key: 'bridge_probe',
            metric_value: 1,
            details: 'admin bridge probe'
          }
        ]
      }
    },
    promote_lifecycle: {
      token
    },
    query_aggregates: {
      token,
      name: 'sample-repo',
      release: 'v1.0.0'
    },
    committer_scores: {
      token,
      name: 'sample-repo',
      release: 'v1.0.0'
    },
    query_release_baseline: {
      token,
      repoName: 'sample-repo'
    },
    reseed_release_baseline: {
      token,
      repoName: 'sample-repo',
      baselineComplexity: 18.5
    },
    rank_prs: {
      token,
      prs: [
        {
          pr_id: 'pr-001',
          repo_name: 'sample-repo',
          author: 'ui',
          release: 'v1.0.0',
          file_risk: 0.4,
          author_velocity: 0.6,
          approval_fidelity: 0.9,
          files: [
            {
              path: 'src/ui-bridge.ts',
              risk: 0.72
            }
          ],
          circuit_breaker_triggered: true
        }
      ]
    },
    update_scoring_weights: {
      token,
      weights: {
        version: 'v1',
        complexity_weight: 0.3,
        coverage_weight: 0.25,
        churn_weight: 0.2,
        pipeline_weight: 0.25,
        pr_file_risk_weight: 0.5,
        pr_velocity_weight: 0.2,
        pr_approval_weight: 0.3
      }
    }
  };
}

export function buildAdminBridgePayload(command: AdminBridgeCommand, token: string) {
  return buildAdminBridgeArgs(token)[command];
}
