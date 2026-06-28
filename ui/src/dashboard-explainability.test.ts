import { describe, expect, it } from 'vitest';
import { buildDashboardInsights } from './insight-engine';
import { buildQualityPulse } from './domain/quality-pulse';
import { buildExplainabilityTraces } from './dashboard-explainability';

describe('dashboard explainability', () => {
  it('builds deterministic score decomposition traces from sample insights', () => {
    const pulse = buildQualityPulse(buildDashboardInsights());

    expect(buildExplainabilityTraces(pulse)).toEqual([
      {
        id: 'explain-score',
        title: 'Score Decomposition',
        summary: 'Overall score 65/100',
        detail: 'Risk 1 high / 0 medium, bottlenecks 1 critical / 2 high',
        status: 'medium'
      },
      {
        id: 'explain-risk',
        title: 'Top Risk Commit',
        summary: 'A-124',
        detail: 'High-risk commit drives merge caution.',
        status: 'bad'
      },
      {
        id: 'explain-bottleneck',
        title: 'Top Bottleneck',
        summary: 'review',
        detail: 'Critical stage limits delivery confidence.',
        status: 'bad'
      },
      {
        id: 'explain-opportunity',
        title: 'Opportunity Lift',
        summary: 'Trim flaky tests',
        detail: '3 opportunity signal(s) are boosting the score.',
        status: 'good'
      }
    ]);
  });

  it('adapts traces for a low-risk payload with no opportunities', () => {
    const pulse = buildQualityPulse(
      buildDashboardInsights(
        {
          commits: [
            { id: 'safe-1', files: 1, changedLines: 8, dependencyChanges: 0, testTouch: true, failedAutomations: 0 },
            { id: 'safe-2', files: 1, changedLines: 12, dependencyChanges: 0, testTouch: true, failedAutomations: 0 }
          ],
          stages: [{ name: 'scan', queueDepth: 1, throughput: 20, avgLatencyMs: 300 }],
          signals: [{ id: 'op-1', area: 'infra', title: 'Reduce release coupling', impact: 5, effort: 3, confidence: 0.8 }]
        },
        { risks: 1, opportunities: 1, latencyP95Ms: 1000, severityThreshold: 1 }
      )
    );

    expect(buildExplainabilityTraces(pulse)).toEqual([
      {
        id: 'explain-score',
        title: 'Score Decomposition',
        summary: 'Overall score 100/100',
        detail: 'Risk 0 high / 0 medium, bottlenecks 0 critical / 0 high',
        status: 'good'
      },
      {
        id: 'explain-risk',
        title: 'Top Risk Commit',
        summary: 'safe-1',
        detail: 'No high-risk commit currently dominates the pulse.',
        status: 'good'
      },
      {
        id: 'explain-bottleneck',
        title: 'Top Bottleneck',
        summary: 'scan',
        detail: 'No critical stage is currently suppressing flow.',
        status: 'medium'
      },
      {
        id: 'explain-opportunity',
        title: 'Opportunity Lift',
        summary: 'Reduce release coupling',
        detail: '1 opportunity signal(s) are boosting the score.',
        status: 'good'
      }
    ]);
  });
});
