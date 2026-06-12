import type { BottleneckCard, CommitRiskCard, DashboardInsights, OpportunityCard } from '../insight-engine';

export type StakeholderAudience = 'lead' | 'manager' | 'executive' | 'security';

export interface PulseAction {
  id: string;
  message: string;
  severity: 'good' | 'medium' | 'bad';
}

export interface PulseRoute {
  owner: string;
  window: string;
  actions: string[];
}

export interface QualityPulse {
  overallScore: number;
  securitySignalCount: number;
  topBottleneckName: string;
  riskBuckets: {
    high: number;
    medium: number;
    good: number;
  };
  bottleneckBuckets: {
    critical: number;
    high: number;
    medium: number;
    good: number;
  };
  recommendations: Record<StakeholderAudience, PulseAction[]>;
  actionRoutes: Record<StakeholderAudience, PulseRoute>;
}

function summarizeRiskBuckets(commitRiskCards: CommitRiskCard[]): QualityPulse['riskBuckets'] {
  return commitRiskCards.reduce(
    (acc, card) => {
      acc[card.level] += 1;
      return acc;
    },
    { high: 0, medium: 0, good: 0 }
  );
}

function summarizeBottlenecks(bottlenecks: BottleneckCard[]): QualityPulse['bottleneckBuckets'] {
  return bottlenecks.reduce(
    (acc, card) => {
      acc[card.status] += 1;
      return acc;
    },
    { critical: 0, high: 0, medium: 0, good: 0 }
  );
}

function buildRecommendations(
  insights: DashboardInsights
): Record<StakeholderAudience, PulseAction[]> {
  const [topRisk] = insights.commitRiskCards;
  const [topOpportunity, secondOpportunity] = insights.opportunities;
  const [criticalStage] = insights.bottlenecks.filter((item) => item.status === 'critical' || item.status === 'high');
  const securitySignals = insights.commitRiskCards.filter((risk) =>
    risk.reasons.some((reason) => reason === 'Dependency risk' || reason === 'Automation failures')
  );

  return {
    lead: [
      {
        id: 'lead-1',
        message: `Focus first on high-risk commit ${topRisk?.id ?? 'sample'} before expanding the next cycle.`,
        severity: topRisk?.level ?? 'medium'
      },
      {
        id: 'lead-2',
        message: `Focus first on high-risk commit ${insights.commitRiskCards[1]?.id ?? 'sample'} after automation stabilizes.`,
        severity: insights.commitRiskCards[1]?.level ?? 'medium'
      }
    ],
    manager: [
      {
        id: 'manager-1',
        message: `Critical stage(s): ${criticalStage?.name ?? 'review'} need additional reviewer capacity.`,
        severity: 'bad'
      },
      {
        id: 'manager-2',
        message: `Critical stage(s): ${criticalStage?.name ?? 'review'} should stay unblocked until the queue drops.`,
        severity: 'bad'
      }
    ],
    executive: [
      {
        id: 'executive-1',
        message: `Top opportunity: ${topOpportunity?.title ?? 'trim flaky tests'}`,
        severity: 'good'
      },
      {
        id: 'executive-2',
        message: `Top opportunity: ${secondOpportunity?.title ?? 'reduce dependency churn'}`,
        severity: 'good'
      }
    ],
    security: securitySignals.length
      ? securitySignals.slice(0, 2).map((signal, index) => ({
          id: `security-${index + 1}`,
          message: `Security-sensitive signals from ${signal.id} should be reviewed before release.`,
          severity: signal.level
        }))
      : [
          {
            id: 'security-1',
            message: 'Security-sensitive signals from the sample window should be reviewed before release.',
            severity: 'good'
          }
        ]
  };
}

function buildRoutes(): QualityPulse['actionRoutes'] {
  return {
    lead: {
      owner: 'Lead Reviewer',
      window: 'Sprint now',
      actions: ['Focus first on high-risk commit A-124', 'Pair with the author to clear the hot path']
    },
    manager: {
      owner: 'Engineering Manager',
      window: 'This week',
      actions: ['Critical stage(s): review queue should be rebalanced', 'Clear review backlog before the next merge window']
    },
    executive: {
      owner: 'Delivery Leadership',
      window: 'This month',
      actions: ['Top opportunity: trim flaky tests', 'Top opportunity: reduce dependency churn']
    },
    security: {
      owner: 'Security Operations',
      window: 'Before release',
      actions: ['Security-sensitive signals from risky commits need review', 'Block release until policy drift is resolved']
    }
  };
}

export function buildQualityPulse(insights: DashboardInsights): QualityPulse {
  const riskBuckets = summarizeRiskBuckets(insights.commitRiskCards);
  const bottleneckBuckets = summarizeBottlenecks(insights.bottlenecks);
  const topBottleneck = [...insights.bottlenecks].sort((left, right) => right.impact - left.impact)[0];
  const securitySignalCount = insights.commitRiskCards.filter((risk) =>
    risk.reasons.some((reason) => reason === 'Dependency risk' || reason === 'Automation failures')
  ).length;
  const overallScore = Math.max(45, 100 - riskBuckets.high * 10 - bottleneckBuckets.critical * 15 - bottleneckBuckets.high * 5);

  return {
    overallScore,
    securitySignalCount,
    topBottleneckName: topBottleneck?.name ?? 'review',
    riskBuckets,
    bottleneckBuckets,
    recommendations: buildRecommendations(insights),
    actionRoutes: buildRoutes()
  };
}
