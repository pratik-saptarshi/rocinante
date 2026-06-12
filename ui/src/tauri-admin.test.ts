import { afterEach, describe, expect, it, vi } from 'vitest';
import { invokeAdminCommand } from './tauri-admin';

const originalTauri = (globalThis as { __TAURI__?: unknown }).__TAURI__;

afterEach(() => {
  const globalWithTauri = globalThis as { __TAURI__?: unknown };
  if (originalTauri === undefined) {
    delete globalWithTauri.__TAURI__;
  } else {
    globalWithTauri.__TAURI__ = originalTauri;
  }
});

describe('tauri admin bridge', () => {
  it('returns runtime fallback when Tauri is unavailable', async () => {
    const globalWithTauri = globalThis as { __TAURI__?: unknown };
    delete globalWithTauri.__TAURI__;

    const result = await invokeAdminCommand('ingest_event', {
      token: 'alice:admin',
      event: {
        commit_id: 'bridge-test',
        repo_name: 'sample-repo',
        release: 'v1.0.0',
        committer: 'ui',
        telemetry: []
      }
    });

    expect(result.ok).toBe(false);
    expect(result.command).toBe('ingest_event');
    expect(result.message).toMatch(/Tauri runtime not detected/i);
  });

  it('returns success when invoke resolves with string payload', async () => {
    const invoke = vi.fn().mockResolvedValue('ok');
    const globalWithTauri = globalThis as { __TAURI__?: { core?: { invoke?: typeof invoke } } };
    globalWithTauri.__TAURI__ = { core: { invoke } };

    const result = await invokeAdminCommand('query_aggregates', {
      token: 'alice:admin',
      name: 'sample-repo',
      release: 'v1.0.0'
    });

    expect(invoke).toHaveBeenCalledWith('query_aggregates', {
      token: 'alice:admin',
      name: 'sample-repo',
      release: 'v1.0.0'
    });
    expect(result.ok).toBe(true);
    expect(result.message).toBe('ok');
  });

  it('stringifies non-string invoke responses', async () => {
    const invoke = vi.fn().mockResolvedValue({ status: 'ok', count: 2 });
    const globalWithTauri = globalThis as { __TAURI__?: { core?: { invoke?: typeof invoke } } };
    globalWithTauri.__TAURI__ = { core: { invoke } };

    const result = await invokeAdminCommand('query_aggregates', {
      token: 'alice:admin',
      name: 'sample-repo',
      release: 'v1.0.0'
    });

    expect(result.ok).toBe(true);
    expect(result.message).toBe(JSON.stringify({ status: 'ok', count: 2 }));
  });

  it('returns error message when invoke rejects', async () => {
    const invoke = vi.fn().mockRejectedValue(new Error('boom'));
    const globalWithTauri = globalThis as { __TAURI__?: { core?: { invoke?: typeof invoke } } };
    globalWithTauri.__TAURI__ = { core: { invoke } };

    const result = await invokeAdminCommand('promote_lifecycle', {
      token: 'alice:admin'
    });

    expect(result.ok).toBe(false);
    expect(result.message).toBe('boom');
    expect(result.command).toBe('promote_lifecycle');
    expect(result.payload).toBe(JSON.stringify({ token: 'alice:admin' }));
  });

  it('invokes committer scores with representative query payload', async () => {
    const invoke = vi.fn().mockResolvedValue({ ok: true });
    const globalWithTauri = globalThis as { __TAURI__?: { core?: { invoke?: typeof invoke } } };
    globalWithTauri.__TAURI__ = { core: { invoke } };

    const result = await invokeAdminCommand('committer_scores', {
      token: 'alice:admin',
      name: 'sample-repo',
      release: 'v1.0.0'
    });

    expect(invoke).toHaveBeenCalledWith('committer_scores', {
      token: 'alice:admin',
      name: 'sample-repo',
      release: 'v1.0.0'
    });
    expect(result.ok).toBe(true);
  });

  it('invokes PR ranking with representative candidates', async () => {
    const invoke = vi.fn().mockResolvedValue({ ok: true });
    const globalWithTauri = globalThis as { __TAURI__?: { core?: { invoke?: typeof invoke } } };
    globalWithTauri.__TAURI__ = { core: { invoke } };

    const result = await invokeAdminCommand('rank_prs', {
      token: 'alice:admin',
      prs: [
        {
          pr_id: 'pr-001',
          repo_name: 'sample-repo',
          author: 'ui',
          release: 'v1.0.0',
          file_risk: 0.4,
          author_velocity: 0.6,
          approval_fidelity: 0.9
        }
      ]
    });

    expect(invoke).toHaveBeenCalledWith('rank_prs', {
      token: 'alice:admin',
      prs: [
        {
          pr_id: 'pr-001',
          repo_name: 'sample-repo',
          author: 'ui',
          release: 'v1.0.0',
          file_risk: 0.4,
          author_velocity: 0.6,
          approval_fidelity: 0.9
        }
      ]
    });
    expect(result.ok).toBe(true);
  });

  it('invokes scoring weight updates with representative weights', async () => {
    const invoke = vi.fn().mockResolvedValue('updated');
    const globalWithTauri = globalThis as { __TAURI__?: { core?: { invoke?: typeof invoke } } };
    globalWithTauri.__TAURI__ = { core: { invoke } };

    const result = await invokeAdminCommand('update_scoring_weights', {
      token: 'alice:admin',
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
    });

    expect(invoke).toHaveBeenCalledWith('update_scoring_weights', {
      token: 'alice:admin',
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
    });
    expect(result.ok).toBe(true);
  });

  it('returns generic error message for non-Error throws', async () => {
    const invoke = vi.fn().mockRejectedValue('plain-failure');
    const globalWithTauri = globalThis as { __TAURI__?: { core?: { invoke?: typeof invoke } } };
    globalWithTauri.__TAURI__ = { core: { invoke } };

    const result = await invokeAdminCommand('promote_lifecycle', {
      token: 'alice:admin'
    });

    expect(result.ok).toBe(false);
    expect(result.message).toBe('Command failed');
  });
});
