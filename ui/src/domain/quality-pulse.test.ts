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

  it('does not invent sample recommendations or routes for imported empty or partial data', () => {
    const emptyPulse = buildQualityPulse({
      commitRiskCards: [],
      bottlenecks: [],
      opportunities: [],
      stages: []
    } as DashboardInsights, { allowSampleFallbacks: false });

    expect(emptyPulse.overallScore).toBeNull();
    expect(emptyPulse.topRiskCommitId).toBeNull();
    expect(emptyPulse.topBottleneckName).toBeNull();
    expect(Object.values(emptyPulse.recommendations).flat()).toEqual([]);
    expect(Object.values(emptyPulse.actionRoutes).flatMap((route) => route.actions)).toEqual([]);
    expect(Object.values(emptyPulse.actionRoutes).every((route) => route.window === 'Awaiting telemetry')).toBe(true);

    const partialPulse = buildQualityPulse({
      commitRiskCards: [{ id: 'actual-commit', score: 75, level: 'medium', reasons: [] }],
      bottlenecks: [],
      opportunities: [],
      stages: []
    } as DashboardInsights, { allowSampleFallbacks: false });

    expect(partialPulse.overallScore).toBeNull();
    expect(partialPulse.topRiskCommitId).toBe('actual-commit');
    expect(partialPulse.topBottleneckName).toBeNull();
    expect(partialPulse.recommendations.lead).toHaveLength(1);
    expect(partialPulse.recommendations.lead[0].message).toContain('actual-commit');
    expect(partialPulse.recommendations.manager).toEqual([]);
    expect(partialPulse.recommendations.executive).toEqual([]);
    expect(partialPulse.recommendations.security).toEqual([]);
    expect(partialPulse.actionRoutes.lead.window).toBe('Current import');
    expect(partialPulse.actionRoutes.lead.actions[0]).toContain('actual-commit: medium risk (score 75)');
    expect(partialPulse.actionRoutes.manager.window).toBe('Awaiting telemetry');

    const opportunityOnlyPulse = buildQualityPulse({
      commitRiskCards: [],
      bottlenecks: [],
      opportunities: [{ id: 'actual-opportunity', title: 'Reduce build time', priorityScore: 70 }],
      stages: []
    } as DashboardInsights, { allowSampleFallbacks: false });
    expect(opportunityOnlyPulse.overallScore).toBeNull();
  });

  it('derives populated imported routes from the active risk, stage, and opportunity records', () => {
    const pulse = buildQualityPulse({
      commitRiskCards: [
        { id: 'imported-commit', score: 81, level: 'high', reasons: ['Automation failures'] },
        { id: 'other-commit', score: 54, level: 'medium', reasons: ['Dependency risk'] }
      ],
      bottlenecks: [
        { name: 'imported-review', status: 'critical', impact: 15, rationale: 'Imported review queue is above threshold.' },
        { name: 'imported-build', status: 'good', impact: 2, rationale: 'Imported build stage is within threshold.' }
      ],
      opportunities: [
        { id: 'imported-signal', title: 'Shorten imported build', priorityScore: 76 }
      ],
      stages: []
    } as DashboardInsights, { allowSampleFallbacks: false });

    expect(pulse.actionRoutes.lead.window).toBe('Current import');
    expect(pulse.actionRoutes.lead.actions[0]).toContain('imported-commit: high risk (score 81)');
    expect(pulse.actionRoutes.lead.actions.join(' ')).not.toContain('A-124');
    expect(pulse.actionRoutes.manager.window).toBe('Current import');
    expect(pulse.actionRoutes.manager.actions[0]).toContain('imported-review: critical pressure');
    expect(pulse.actionRoutes.executive.window).toBe('Current import');
    expect(pulse.actionRoutes.executive.actions[0]).toBe('Shorten imported build (score 76).');
    expect(pulse.actionRoutes.security.window).toBe('Current import');
    expect(pulse.actionRoutes.security.actions).toEqual(['imported-commit: review automation failures.', 'other-commit: review dependency risk.']);
  });

  it('reports healthy imported stages and absent security signals as observed data', () => {
    const pulse = buildQualityPulse({
      commitRiskCards: [{ id: 'healthy-import', score: 18, level: 'good', reasons: [] }],
      bottlenecks: [{ name: 'healthy-stage', status: 'good', impact: 1, rationale: 'Stage is within thresholds.' }],
      opportunities: [],
      stages: []
    } as DashboardInsights, { allowSampleFallbacks: false });

    expect(pulse.actionRoutes.manager.window).toBe('Current import');
    expect(pulse.actionRoutes.manager.actions).toEqual(['No pressured stages were identified in the imported telemetry.']);
    expect(pulse.actionRoutes.security.window).toBe('Current import');
    expect(pulse.actionRoutes.security.actions).toEqual(['No dependency or automation-failure signals were found in the imported commits.']);
    expect(pulse.actionRoutes.executive.window).toBe('Awaiting telemetry');
  });
});
