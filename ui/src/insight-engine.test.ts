import { describe, expect, it } from 'vitest';
import { buildDashboardInsights, type InsightPayload } from './insight-engine';

describe('insight engine', () => {
  it('falls back to sample data when payload is empty', () => {
    const dashboard = buildDashboardInsights();

    expect(dashboard.commitRiskCards).toHaveLength(3);
    expect(dashboard.bottlenecks).toHaveLength(3);
    expect(dashboard.opportunities).toHaveLength(3);
  });

  it('normalizes provided payload and applies limits', () => {
    const payload: InsightPayload = {
      commits: [
        { id: 'custom-1', files: 14, changedLines: 310, dependencyChanges: 2, testTouch: false, failedAutomations: 1 }
      ],
      stages: [{ name: 'build', queueDepth: 20, throughput: 10, avgLatencyMs: 900 }],
      signals: [
        { id: 'custom-op', area: 'infra', title: 'Tune infra', impact: 4, effort: 1, confidence: 1.2 }
      ]
    };

    const dashboard = buildDashboardInsights(payload, { risks: 1, opportunities: 1, latencyP95Ms: 1000 });

    expect(dashboard.commitRiskCards).toHaveLength(1);
    expect(dashboard.commitRiskCards[0].id).toBe('custom-1');
    expect(dashboard.bottlenecks).toHaveLength(1);
    expect(dashboard.bottlenecks[0].name).toBe('build');
    expect(dashboard.opportunities).toHaveLength(1);
    expect(dashboard.opportunities[0].id).toBe('custom-op');
  });
});
