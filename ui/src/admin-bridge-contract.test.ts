import { describe, expect, it } from 'vitest';
import { ADMIN_BRIDGE_ACTIONS, buildAdminBridgeArgs } from './admin-bridge-contract';

describe('admin bridge contract helpers', () => {
  it('exposes the command catalog used by the UI shell', () => {
    expect(ADMIN_BRIDGE_ACTIONS).toEqual([
      { command: 'ingest_event', label: 'Ingest Event' },
      { command: 'promote_lifecycle', label: 'Promote Lifecycle' },
      { command: 'query_aggregates', label: 'Query Aggregates' },
      { command: 'committer_scores', label: 'Committer Scores' },
      { command: 'rank_prs', label: 'Rank PRs' },
      { command: 'update_scoring_weights', label: 'Update Scoring Weights' }
    ]);
  });

  it('builds the full command payload map from a token', () => {
    const args = buildAdminBridgeArgs('alice:admin');

    expect(args.ingest_event.token).toBe('alice:admin');
    expect(args.ingest_event.event.commit_id).toBe('ui-bridge-001');
    expect(args.promote_lifecycle.token).toBe('alice:admin');
    expect(args.query_aggregates.release).toBe('v1.0.0');
    expect(args.committer_scores.name).toBe('sample-repo');
    expect(args.query_release_baseline.repoName).toBe('sample-repo');
    expect(args.reseed_release_baseline.baselineComplexity).toBe(18.5);
    expect(args.rank_prs.prs).toHaveLength(1);
    expect(args.rank_prs.prs[0].files).toHaveLength(1);
    expect(args.rank_prs.prs[0].circuit_breaker_triggered).toBe(true);
    expect(args.update_scoring_weights.weights.version).toBe('v1');
  });

  it('reuses the same command map shape for different token values', () => {
    const args = buildAdminBridgeArgs('bob:admin');

    expect(args.ingest_event.token).toBe('bob:admin');
    expect(args.promote_lifecycle.token).toBe('bob:admin');
    expect(args.query_aggregates.token).toBe('bob:admin');
  });
});
