import { describe, expect, it } from 'vitest';
import { buildQualityPulse } from './quality-pulse';
import { buildDashboardInsights, type InsightPayload } from '../insight-engine';

describe('quality pulse', () => {
  it('derives pulse health score and recommendations from sample signals', () => {
    const pulse = buildQualityPulse(buildDashboardInsights());

    expect(pulse.overallScore).toBeGreaterThan(20);
    expect(pulse.overallScore).toBeLessThanOrEqual(100);
    expect(pulse.riskBuckets.high).toBe(2);
    expect(pulse.bottleneckBuckets.critical).toBe(1);
    expect(pulse.opportunityCount).toBe(3);
    expect(pulse.securitySignalCount).toBeGreaterThan(0);
    expect(pulse.recommendations.lead[0].message).toContain('Focus first on high-risk commit');
    expect(pulse.recommendations.security[0].message).toContain('Security-sensitive signals');
    expect(pulse.actionRoutes.lead.owner).toBe('Lead Reviewer');
    expect(pulse.actionRoutes.manager.window).toBe('This week');
    expect(pulse.actionRoutes.security.actions.length).toBeGreaterThan(0);
  });

  it('adapts recommendations when payload adds sustained high risk', () => {
    const payload: InsightPayload = {
      commits: [
        { id: 'hot-1', files: 24, changedLines: 1200, dependencyChanges: 4, testTouch: false, failedAutomations: 3 },
        { id: 'hot-2', files: 30, changedLines: 900, dependencyChanges: 2, testTouch: false, failedAutomations: 2 }
      ],
      stages: [],
      signals: []
    };

    const pulse = buildQualityPulse(buildDashboardInsights(payload, { risks: 2, opportunities: 2 }));

    expect(pulse.opportunityCount).toBe(2);
    expect(pulse.recommendations.lead[2].message).toContain('Prioritize quick-win opportunities');
    expect(pulse.recommendations.executive[0].message).toContain('high-risk');
    expect(pulse.recommendations.security[0].message).toMatch(/Security-sensitive signals|No critical security signals/);
    expect(pulse.actionRoutes.executive.owner).toBe('Delivery Leadership');
    expect(pulse.actionRoutes.executive.actions.length).toBeGreaterThan(0);
  });

  it('keeps manager critical-stage recommendation when bottlenecks are critical without high-risk commits', () => {
    const payload: InsightPayload = {
      commits: [
        { id: 'safe-1', files: 1, changedLines: 12, dependencyChanges: 0, testTouch: true, failedAutomations: 0 },
        { id: 'safe-2', files: 1, changedLines: 8, dependencyChanges: 0, testTouch: true, failedAutomations: 0 }
      ],
      stages: [{ name: 'review', queueDepth: 40, throughput: 1, avgLatencyMs: 6000 }],
      signals: []
    };

    const pulse = buildQualityPulse(
      buildDashboardInsights(payload, { risks: 2, opportunities: 2, latencyP95Ms: 1000, severityThreshold: 1 })
    );

    expect(pulse.riskBuckets.high).toBe(0);
    expect(pulse.bottleneckBuckets.critical).toBeGreaterThan(0);
    expect(pulse.recommendations.manager[0].message).toContain('Critical stage(s): review');
  });
});
