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
        title: 'Opportunity Signals',
        summary: 'Trim flaky tests',
        detail: '3 opportunity signal(s) are available to review; opportunities do not affect the pulse score.',
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
        summary: 'safe-2',
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
        title: 'Opportunity Signals',
        summary: 'Reduce release coupling',
        detail: '1 opportunity signal(s) are available to review; opportunities do not affect the pulse score.',
        status: 'good'
      }
    ]);
  });

  it('marks the score unavailable when imported data has no score inputs', () => {
    const pulse = buildQualityPulse(
      { commitRiskCards: [], bottlenecks: [], opportunities: [], stages: [] },
      { allowSampleFallbacks: false }
    );

    expect(buildExplainabilityTraces(pulse)[0]).toEqual({
      id: 'explain-score',
      title: 'Score Decomposition',
      summary: 'Overall score unavailable',
      detail: 'A pulse score requires at least one commit-risk record and one bottleneck record; one or both input sets are empty.',
      status: 'medium'
    });
    expect(buildExplainabilityTraces(pulse)[2]).toEqual({
      id: 'explain-bottleneck',
      title: 'Top Bottleneck',
      summary: 'Unavailable',
      detail: 'No bottleneck records are available for this import.',
      status: 'medium'
    });
    expect(buildExplainabilityTraces(pulse)[1]).toEqual({
      id: 'explain-risk',
      title: 'Top Risk Commit',
      summary: 'Unavailable',
      detail: 'No commit-risk records are available for this import.',
      status: 'medium'
    });
  });

  it('explains which score inputs are required when an import is partial', () => {
    const pulse = buildQualityPulse(
      { commitRiskCards: [{ id: 'actual-commit', score: 75, level: 'medium', reasons: [] }], bottlenecks: [], opportunities: [], stages: [] },
      { allowSampleFallbacks: false }
    );

    expect(pulse.overallScore).toBeNull();
    expect(buildExplainabilityTraces(pulse)[0].detail).toContain('at least one commit-risk record and one bottleneck record');
  });

  it('keeps opportunity signals out of the pulse score and explains that clearly', () => {
    const payload = {
      commits: [{ id: 'safe-commit', files: 1, changedLines: 8, dependencyChanges: 0, testTouch: true, failedAutomations: 0 }],
      stages: [{ name: 'healthy-stage', queueDepth: 1, throughput: 10, avgLatencyMs: 100 }]
    };
    const withoutOpportunities = buildQualityPulse(
      buildDashboardInsights({ ...payload, signals: [] }),
      { allowSampleFallbacks: false }
    );
    const withOpportunities = buildQualityPulse(
      buildDashboardInsights({
        ...payload,
        signals: [{ id: 'op-1', area: 'build', title: 'Reduce build time', impact: 5, effort: 2, confidence: 0.9 }]
      }),
      { allowSampleFallbacks: false }
    );

    expect(withOpportunities.overallScore).toBe(withoutOpportunities.overallScore);
    expect(buildExplainabilityTraces(withOpportunities)[3].detail).toContain('opportunities do not affect the pulse score');
    expect(buildExplainabilityTraces(withOpportunities)[3].detail).not.toMatch(/boost|contribut/i);
  });
});
