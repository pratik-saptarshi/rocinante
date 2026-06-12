import { describe, expect, it } from 'vitest';
import { buildDashboardInsights } from './insight-engine';
import { buildQualityPulse } from './domain/quality-pulse';
import { buildTrendRiskCards } from './dashboard-visuals';

describe('dashboard visuals', () => {
  it('builds deterministic trend and risk cards from sample insights', () => {
    const pulse = buildQualityPulse(buildDashboardInsights());

    expect(buildTrendRiskCards(pulse)).toEqual([
      {
        id: 'risk-trend',
        title: 'Risk Trend',
        summary: '2 high-risk commit(s) out of 3',
        detail: '0 medium-risk commit(s) keep the pre-merge queue active.',
        status: 'bad'
      },
      {
        id: 'bottleneck-trend',
        title: 'Bottleneck Trend',
        summary: '1 critical / 2 high bottleneck(s)',
        detail: 'review',
        status: 'bad'
      },
      {
        id: 'opportunity-trend',
        title: 'Opportunity Trend',
        summary: '3 actionable opportunity(s)',
        detail: 'Top opportunity: Gate dependency updates through staged canary',
        status: 'good'
      }
    ]);
  });

  it('adapts the trend and risk cards for a fallback payload', () => {
    const pulse = buildQualityPulse(
      buildDashboardInsights(
        {
          commits: [
            {
              id: 'custom-1',
              files: 24,
              changedLines: 650,
              dependencyChanges: 1,
              testTouch: false,
              failedAutomations: 1
            }
          ],
          stages: [
            {
              name: 'build',
              queueDepth: 8,
              throughput: 4,
              avgLatencyMs: 1500
            }
          ],
          signals: [
            {
              id: 'op-1',
              area: 'infra',
              title: 'Reduce release coupling',
              impact: 5,
              effort: 3,
              confidence: 0.8
            }
          ]
        },
        { risks: 1, opportunities: 1, severityThreshold: 1, latencyP95Ms: 1200 }
      )
    );

    expect(buildTrendRiskCards(pulse)).toEqual([
      {
        id: 'risk-trend',
        title: 'Risk Trend',
        summary: '1 high-risk commit(s) out of 1',
        detail: '0 medium-risk commit(s) keep the pre-merge queue active.',
        status: 'bad'
      },
      {
        id: 'bottleneck-trend',
        title: 'Bottleneck Trend',
        summary: '1 critical / 0 high bottleneck(s)',
        detail: 'build',
        status: 'bad'
      },
      {
        id: 'opportunity-trend',
        title: 'Opportunity Trend',
        summary: '1 actionable opportunity(s)',
        detail: 'Top opportunity: Reduce release coupling',
        status: 'good'
      }
    ]);
  });
});
