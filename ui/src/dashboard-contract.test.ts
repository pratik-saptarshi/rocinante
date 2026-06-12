import { describe, expect, it } from 'vitest';
import { readLimits, readPayload } from './dashboard-contract';

describe('dashboard contract helpers', () => {
  it('extracts nested payload objects while preserving root fallback', () => {
    const envelope = {
      payload: {
        commits: [{ id: 'nested-1', files: 3 }]
      }
    };

    expect(readPayload(envelope)).toEqual(envelope.payload);
    expect(readPayload({ payload: null, commits: [{ id: 'root-1' }] })).toEqual({
      payload: null,
      commits: [{ id: 'root-1' }]
    });
  });

  it('reads nested limits and falls back to the envelope root when needed', () => {
    expect(
      readLimits({
        limits: {
          risks: 3,
          opportunities: 2,
          severityThreshold: 4.5,
          latencyP95Ms: 700
        }
      })
    ).toEqual({
      risks: 3,
      opportunities: 2,
      severityThreshold: 4.5,
      latencyP95Ms: 700
    });

    expect(readLimits({ risks: 2, opportunities: 1, severityThreshold: 8, latencyP95Ms: 900 })).toEqual({
      risks: 2,
      opportunities: 1,
      severityThreshold: 8,
      latencyP95Ms: 900
    });
  });

  it('ignores malformed limit values without throwing', () => {
    expect(
      readLimits({
        limits: {
          risks: 'bad',
          opportunities: null,
          severityThreshold: Number.NaN,
          latencyP95Ms: 'oops'
        }
      })
    ).toEqual({
      risks: undefined,
      opportunities: undefined,
      severityThreshold: undefined,
      latencyP95Ms: undefined
    });
  });
});
