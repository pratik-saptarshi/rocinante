import { describe, expect, it } from 'vitest';
import { buildDashboardInsights, type DashboardInsights } from '../insight-engine';
import { buildQualityPulse } from './quality-pulse';

describe('buildQualityPulse', () => {
  it('derives role-specific routing and recommendation buckets', () => {
    const pulse = buildQualityPulse(buildDashboardInsights());

    expect(pulse.riskBuckets.high).toBe(1);
    expect(pulse.bottleneckBuckets.critical).toBe(1);
    expect(pulse.bottleneckBuckets.high).toBe(2);
    expect(pulse.securitySignalCount).toBeGreaterThanOrEqual(1);
    expect(pulse.topBottleneckName).toMatch(/review/i);
    expect(pulse.recommendations.lead).toHaveLength(2);
    expect(pulse.recommendations.lead[0].message).toContain('Focus first on high-risk commit');
    expect(pulse.recommendations.manager[0].message).toContain('Critical stage(s): review');
    expect(pulse.recommendations.executive[0].message).toContain('Top opportunity:');
    expect(pulse.recommendations.security[0].message).toContain('Security-sensitive signals from');
    expect(pulse.actionRoutes.lead.owner).toBe('Lead Reviewer');
    expect(pulse.actionRoutes.manager.window).toBe('This week');
  });

  it('ranks insights before selecting top risks and opportunities', () => {
    const pulse = buildQualityPulse({
      commitRiskCards: [
        { id: 'low-risk', score: 12, level: 'good', reasons: [] },
        { id: 'high-risk', score: 92, level: 'high', reasons: ['Automation failures'] }
      ],
      bottlenecks: [],
      opportunities: [
        { id: 'low-opportunity', title: 'Low opportunity', priorityScore: 8 },
        { id: 'high-opportunity', title: 'High opportunity', priorityScore: 88 }
      ],
      stages: []
    } as DashboardInsights);

    expect(pulse.topRiskCommitId).toBe('high-risk');
    expect(pulse.topOpportunityTitle).toBe('High opportunity');
    expect(pulse.recommendations.lead[0].message).toContain('high-risk');
    expect(pulse.recommendations.executive[0].message).toContain('High opportunity');
  });

  it('uses fallback recommendations when no signals are available', () => {
    const pulse = buildQualityPulse({
      commitRiskCards: [],
      bottlenecks: [],
      opportunities: [],
      stages: []
    } as DashboardInsights);

    expect(pulse.topRiskCommitId).toBe('');
    expect(pulse.topOpportunityTitle).toBe('trim flaky tests');
    expect(pulse.topBottleneckName).toBe('review');
    expect(pulse.securitySignalCount).toBe(0);
    expect(pulse.recommendations.lead[0].severity).toBe('medium');
    expect(pulse.recommendations.executive[0].message).toContain('trim flaky tests');
    expect(pulse.recommendations.security[0].severity).toBe('good');
  });
});
