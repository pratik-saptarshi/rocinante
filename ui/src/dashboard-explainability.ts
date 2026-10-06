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
      summary: pulse.overallScore === null ? 'Overall score unavailable' : `Overall score ${pulse.overallScore}/100`,
      detail:
        pulse.overallScore === null
          ? 'A pulse score requires at least one commit-risk record and one bottleneck record; one or both input sets are empty.'
          : `Risk ${pulse.riskBuckets.high} high / ${pulse.riskBuckets.medium} medium, bottlenecks ${pulse.bottleneckBuckets.critical} critical / ${pulse.bottleneckBuckets.high} high`,
      status: pulse.overallScore === null ? 'medium' : pulse.overallScore >= 75 ? 'good' : pulse.overallScore >= 45 ? 'medium' : 'bad'
    },
    {
      id: 'explain-risk',
      title: 'Top Risk Commit',
      summary: pulse.topRiskCommitId ?? 'Unavailable',
      detail:
        pulse.topRiskCommitId === null
          ? 'No commit-risk records are available for this import.'
          : pulse.riskBuckets.high > 0
            ? 'High-risk commit drives merge caution.'
            : 'No high-risk commit currently dominates the pulse.',
      status: pulse.topRiskCommitId === null ? 'medium' : pulse.riskBuckets.high > 0 ? 'bad' : 'good'
    },
    {
      id: 'explain-bottleneck',
      title: 'Top Bottleneck',
      summary: pulse.topBottleneckName ?? 'Unavailable',
      detail:
        pulse.topBottleneckName === null
          ? 'No bottleneck records are available for this import.'
          : pulse.bottleneckBuckets.critical > 0
            ? 'Critical stage limits delivery confidence.'
            : 'No critical stage is currently suppressing flow.',
      status: pulse.topBottleneckName === null ? 'medium' : pulse.bottleneckBuckets.critical > 0 ? 'bad' : 'medium'
    },
    {
      id: 'explain-opportunity',
      title: 'Opportunity Signals',
      summary: pulse.opportunityCount > 0 ? pulse.topOpportunityTitle : 'No opportunity signal',
      detail:
        pulse.opportunityCount > 0
          ? `${pulse.opportunityCount} opportunity signal(s) are available to review; opportunities do not affect the pulse score.`
          : 'No opportunity signals are available; opportunities do not affect the pulse score.',
      status: pulse.opportunityCount > 0 ? 'good' : 'medium'
    }
  ];
}
