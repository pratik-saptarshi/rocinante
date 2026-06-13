import type { AuditStatus } from './dashboard-content';
import type { QualityPulse } from './domain/quality-pulse';

export interface ExplainabilityTrace {
  id: string;
  title: string;
  summary: string;
  detail: string;
  status: AuditStatus;
}

export function buildExplainabilityTraces(pulse: QualityPulse): ExplainabilityTrace[] {
  return [
    {
      id: 'explain-score',
      title: 'Score Decomposition',
      summary: `Overall score ${pulse.overallScore}/100`,
      detail: `Risk ${pulse.riskBuckets.high} high / ${pulse.riskBuckets.medium} medium, bottlenecks ${pulse.bottleneckBuckets.critical} critical / ${pulse.bottleneckBuckets.high} high`,
      status: pulse.overallScore >= 75 ? 'good' : pulse.overallScore >= 45 ? 'medium' : 'bad'
    },
    {
      id: 'explain-risk',
      title: 'Top Risk Commit',
      summary: pulse.topRiskCommitId,
      detail: pulse.riskBuckets.high > 0 ? 'High-risk commit drives merge caution.' : 'No high-risk commit currently dominates the pulse.',
      status: pulse.riskBuckets.high > 0 ? 'bad' : 'good'
    },
    {
      id: 'explain-bottleneck',
      title: 'Top Bottleneck',
      summary: pulse.topBottleneckName,
      detail: pulse.bottleneckBuckets.critical > 0 ? 'Critical stage limits delivery confidence.' : 'No critical stage is currently suppressing flow.',
      status: pulse.bottleneckBuckets.critical > 0 ? 'bad' : 'medium'
    },
    {
      id: 'explain-opportunity',
      title: 'Opportunity Lift',
      summary: pulse.topOpportunityTitle,
      detail:
        pulse.opportunityCount > 0
          ? `${pulse.opportunityCount} opportunity signal(s) are boosting the score.`
          : 'No opportunity signals are contributing to the current score.',
      status: pulse.opportunityCount > 0 ? 'good' : 'medium'
    }
  ];
}
