import { describe, expect, it } from 'vitest';
import { getPayloadState, readLimits, readPayload, validatePayload } from './dashboard-contract';

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
    expect(() => readPayload({ payload: [] })).toThrow(/payload\.payload must be an object/);
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
    expect(readLimits({ limits: null, risks: 1, opportunities: 3 })).toEqual({
      risks: 1,
      opportunities: 3,
      severityThreshold: undefined,
      latencyP95Ms: undefined
    });
  });

  it('rejects invalid limits instead of silently ignoring them', () => {
    expect(() => readLimits({ limits: { latencyP95Ms: Number.POSITIVE_INFINITY } })).toThrow(/latencyP95Ms/);
    expect(() => readLimits({ limits: { risks: -1 } })).toThrow(/risks must not be negative/);
    expect(() => readLimits({ limits: [] })).toThrow(/limits must be an object/);
  });

  it('validates every nested collection record and finite numeric boundary', () => {
    const valid = {
      commits: [{ id: 'commit-1', files: 0, changedLines: 0, dependencyChanges: 0, testTouch: false, failedAutomations: 0 }],
      stages: [{ name: 'review', queueDepth: 0, throughput: 0, avgLatencyMs: 0 }],
      signals: [{ id: 'signal-1', area: 'tests', title: 'Improve tests', impact: 0, effort: 0, confidence: 1 }]
    };
    expect(validatePayload(valid)).toEqual(valid);
    expect(() => validatePayload({ stages: [{ name: { label: 'review' }, queueDepth: 1, throughput: 1, avgLatencyMs: 1 }] }))
      .toThrow(/stages\[0\]\.name must be a non-empty string/);
    expect(() => validatePayload({ commits: [{ ...valid.commits[0], changedLines: Number.NaN }] }))
      .toThrow(/commits\[0\]\.changedLines must be a finite non-negative number/);
    expect(() => validatePayload({ signals: [{ ...valid.signals[0], confidence: 1.1 }] }))
      .toThrow(/confidence must be a finite number between 0 and 1/);
    expect(() => validatePayload({ stages: {} })).toThrow(/stages must be an array/);
  });

  it('distinguishes missing, explicit empty, and imported records', () => {
    expect(getPayloadState({})).toBe('missing');
    expect(getPayloadState({ commits: [], stages: [], signals: [] })).toBe('empty');
    expect(getPayloadState({ commits: [{ id: 'x', files: 0, changedLines: 0, dependencyChanges: 0, testTouch: true, failedAutomations: 0 }] }))
      .toBe('imported');
  });
});
