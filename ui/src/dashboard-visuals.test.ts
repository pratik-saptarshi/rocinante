import { describe, expect, it } from 'vitest';
import { buildDashboardInsights } from './insight-engine';
import { buildDashboardVisuals } from './dashboard-visuals';

describe('buildDashboardVisuals', () => {
  it('summarizes the default dashboard risk and opportunity lanes', () => {
    const visuals = buildDashboardVisuals(buildDashboardInsights());

    expect(visuals.summary).toContain('high-risk commits');
    expect(visuals.trendLines).toHaveLength(3);
    expect(visuals.trendLines[0]).toMatchObject({
      label: 'PR Risk Trajectory',
      value: '1 high-risk commits'
    });
    expect(visuals.prRiskRankings[0]).toMatchObject({
      id: 'A-124',
      title: 'A-124 score 100',
      tone: 'bad'
    });
  });

  it('adapts to custom payload risk and bottleneck data', () => {
    const visuals = buildDashboardVisuals(
      buildDashboardInsights({
        commits: [
          { id: 'custom-999', files: 25, changedLines: 800, dependencyChanges: 1, testTouch: false, failedAutomations: 1 }
        ],
        stages: [{ name: 'build', queueDepth: 8, throughput: 8, avgLatencyMs: 1500 }],
        signals: [{ id: 'custom-op-1', area: 'infra', title: 'Cache invalidation map', impact: 5, effort: 2, confidence: 0.9 }]
      })
    );

    expect(visuals.summary).toContain('1 high-risk commits');
    expect(visuals.trendLines[0].rationale).toContain('custom-999');
    expect(visuals.trendLines[1]).toMatchObject({
      label: 'Bottleneck Pressure',
      value: '1 pressured stages'
    });
    expect(visuals.prRiskRankings[0]).toMatchObject({
      id: 'custom-999',
      tone: 'bad'
    });
  });

  it('renders a neutral summary when no signals are present', () => {
    const visuals = buildDashboardVisuals({
      commitRiskCards: [],
      bottlenecks: [],
      opportunities: [],
      stages: []
    });

    expect(visuals.summary).toBe('No risk signals available');
    expect(visuals.trendLines[0]).toMatchObject({
      value: 'No high-risk commits',
      tone: 'good'
    });
    expect(visuals.trendLines[1]).toMatchObject({
      value: '0 pressured stages',
      tone: 'good'
    });
    expect(visuals.trendLines[2]).toMatchObject({
      value: '0 actionable opportunities',
      tone: 'good'
    });
  });
});
