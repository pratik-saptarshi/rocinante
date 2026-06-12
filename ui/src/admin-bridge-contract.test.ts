import { describe, expect, it } from 'vitest';
import { buildAdminBridgeArgs } from './admin-bridge-contract';

describe('admin bridge contract helpers', () => {
  it('builds the full command payload map from a token', () => {
    const args = buildAdminBridgeArgs('alice:admin');

    expect(args.ingest_event.token).toBe('alice:admin');
    expect(args.ingest_event.event.commit_id).toBe('ui-bridge-001');
    expect(args.promote_lifecycle.token).toBe('alice:admin');
    expect(args.query_aggregates.release).toBe('v1.0.0');
    expect(args.committer_scores.name).toBe('sample-repo');
    expect(args.rank_prs.prs).toHaveLength(1);
    expect(args.update_scoring_weights.weights.version).toBe('v1');
  });

  it('reuses the same command map shape for different token values', () => {
    const args = buildAdminBridgeArgs('bob:admin');

    expect(args.ingest_event.token).toBe('bob:admin');
    expect(args.promote_lifecycle.token).toBe('bob:admin');
    expect(args.query_aggregates.token).toBe('bob:admin');
  });
});
