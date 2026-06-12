import type { AuditStatus } from './dashboard-content';
import type { QualityPulse } from './domain/quality-pulse';

export interface TrendRiskCard {
  id: string;
  title: string;
  summary: string;
  detail: string;
  status: AuditStatus;
}

export function buildTrendRiskCards(pulse: QualityPulse): TrendRiskCard[] {
  const totalRisk = Math.max(1, pulse.riskBuckets.low + pulse.riskBuckets.medium + pulse.riskBuckets.high);
  const riskStatus: AuditStatus = pulse.riskBuckets.high > 0 ? 'bad' : pulse.riskBuckets.medium > 0 ? 'medium' : 'good';
  const bottleneckStatus: AuditStatus =
    pulse.bottleneckBuckets.critical > 0 ? 'bad' : pulse.bottleneckBuckets.high > 0 ? 'medium' : 'good';
  const opportunityStatus: AuditStatus = pulse.opportunityCount > 0 ? 'good' : 'medium';

  return [
    {
      id: 'risk-trend',
      title: 'Risk Trend',
      summary: `${pulse.riskBuckets.high} high-risk commit(s) out of ${totalRisk}`,
      detail: `${pulse.riskBuckets.medium} medium-risk commit(s) keep the pre-merge queue active.`,
      status: riskStatus
    },
    {
      id: 'bottleneck-trend',
      title: 'Bottleneck Trend',
      summary: `${pulse.bottleneckBuckets.critical} critical / ${pulse.bottleneckBuckets.high} high bottleneck(s)`,
      detail: pulse.topBottleneckName,
      status: bottleneckStatus
    },
    {
      id: 'opportunity-trend',
      title: 'Opportunity Trend',
      summary: `${pulse.opportunityCount} actionable opportunity(s)`,
      detail: `Top opportunity: ${pulse.topOpportunityTitle}`,
      status: opportunityStatus
    }
  ];
}
