import { describe, expect, it } from 'vitest';
import { buildDashboardInsights } from './insight-engine';

describe('buildDashboardInsights', () => {
  it('builds the default sample insight view model', () => {
    const insights = buildDashboardInsights();

    expect(insights.commitRiskCards).toHaveLength(3);
    expect(insights.commitRiskCards[0]).toEqual(
      expect.objectContaining({
        id: 'A-124',
        score: 100,
        level: 'high'
      })
    );
    expect(insights.bottlenecks).toHaveLength(3);
    expect(insights.opportunities).toHaveLength(3);
  });

  it('derives custom payload insights from telemetry envelopes', () => {
    const insights = buildDashboardInsights(
      {
        commits: [
          {
            id: 'custom-999',
            files: 25,
            changedLines: 800,
            dependencyChanges: 1,
            testTouch: false,
            failedAutomations: 1
          }
        ],
        stages: [{ name: 'build', queueDepth: 8, throughput: 8, avgLatencyMs: 1500 }],
        signals: [{ id: 'custom-op-1', area: 'infra', title: 'Cache invalidation map', impact: 5, effort: 2, confidence: 0.9 }]
      },
      {
        risks: 1,
        opportunities: 1,
        severityThreshold: 4,
        latencyP95Ms: 700
      }
    );

    expect(insights.commitRiskCards).toHaveLength(1);
    expect(insights.commitRiskCards[0]).toEqual(
      expect.objectContaining({
        id: 'custom-999',
        score: 100,
        level: 'high'
      })
    );
    expect(insights.bottlenecks).toHaveLength(1);
    expect(insights.opportunities).toHaveLength(1);
  });
});
