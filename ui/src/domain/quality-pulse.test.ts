import { describe, expect, it } from 'vitest';
import { buildDashboardInsights } from '../insight-engine';
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
});
