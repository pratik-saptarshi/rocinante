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
  topRiskCommitId: string;
  topOpportunityTitle: string;
  opportunityCount: number;
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

function rankRiskCards(commitRiskCards: CommitRiskCard[]): CommitRiskCard[] {
  return [...commitRiskCards].sort((left, right) => right.score - left.score || left.id.localeCompare(right.id));
}

function rankOpportunities(opportunities: OpportunityCard[]): OpportunityCard[] {
  return [...opportunities].sort((left, right) => right.priorityScore - left.priorityScore || left.id.localeCompare(right.id));
}

function normalizePulseSeverity(level: CommitRiskCard['level'] | undefined): PulseAction['severity'] {
  if (level === 'high') {
    return 'bad';
  }

  return level ?? 'medium';
}

function buildRecommendations(
  insights: DashboardInsights,
  allowSampleFallbacks: boolean
): Record<StakeholderAudience, PulseAction[]> {
  const rankedRiskCards = rankRiskCards(insights.commitRiskCards);
  const rankedOpportunities = rankOpportunities(insights.opportunities);
  const [topRisk] = rankedRiskCards;
  const [topOpportunity, secondOpportunity] = rankedOpportunities;
  const criticalStages = insights.bottlenecks.filter((item) => item.status === 'critical' || item.status === 'high');
  const securitySignals = rankedRiskCards.filter((risk) =>
    risk.reasons.some((reason) => reason === 'Dependency risk' || reason === 'Automation failures')
  );

  const lead = rankedRiskCards.slice(0, 2).map((risk, index) => ({
    id: `lead-${index + 1}`,
    message:
      index === 0
        ? `Focus first on high-risk commit ${risk.id} before expanding the next cycle.`
        : `Focus first on high-risk commit ${risk.id} after automation stabilizes.`,
    severity: normalizePulseSeverity(risk.level)
  }));
  const manager = allowSampleFallbacks
    ? [
        {
          id: 'manager-1',
          message: `Critical stage(s): ${criticalStages[0]?.name ?? 'review'} need additional reviewer capacity.`,
          severity: 'bad' as const
        },
        {
          id: 'manager-2',
          message: `Critical stage(s): ${criticalStages[0]?.name ?? 'review'} should stay unblocked until the queue drops.`,
          severity: 'bad' as const
        }
      ]
    : criticalStages.slice(0, 2).map((stage, index) => ({
        id: `manager-${index + 1}`,
        message: `Critical stage: ${stage.name} needs additional reviewer capacity.`,
        severity: 'bad' as const
      }));
  const executive = rankedOpportunities.slice(0, 2).map((opportunity, index) => ({
    id: `executive-${index + 1}`,
    message: `Top opportunity: ${opportunity.title}`,
    severity: 'good' as const
  }));
  const security = securitySignals.slice(0, 2).map((signal, index) => ({
    id: `security-${index + 1}`,
    message: `Security-sensitive signals from ${signal.id} should be reviewed before release.`,
    severity: normalizePulseSeverity(signal.level)
  }));

  if (allowSampleFallbacks) {
    if (lead.length === 0) {
      lead.push(
        { id: 'lead-1', message: `Focus first on high-risk commit ${topRisk?.id ?? 'sample'} before expanding the next cycle.`, severity: normalizePulseSeverity(topRisk?.level) },
        { id: 'lead-2', message: `Focus first on high-risk commit ${rankedRiskCards[1]?.id ?? 'sample'} after automation stabilizes.`, severity: normalizePulseSeverity(rankedRiskCards[1]?.level) }
      );
    }
    if (executive.length === 0) {
      executive.push(
        { id: 'executive-1', message: `Top opportunity: ${topOpportunity?.title ?? 'trim flaky tests'}`, severity: 'good' },
        { id: 'executive-2', message: `Top opportunity: ${secondOpportunity?.title ?? 'reduce dependency churn'}`, severity: 'good' }
      );
    }
    if (security.length === 0) {
      security.push({
        id: 'security-1',
        message: 'Security-sensitive signals from the sample window should be reviewed before release.',
        severity: 'good'
      });
    }
  }

  return { lead, manager, executive, security };
}

function buildRoutes(allowSampleFallbacks: boolean): QualityPulse['actionRoutes'] {
  if (!allowSampleFallbacks) {
    return {
      lead: { owner: 'Lead Reviewer', window: 'Awaiting telemetry', actions: [] },
      manager: { owner: 'Engineering Manager', window: 'Awaiting telemetry', actions: [] },
      executive: { owner: 'Delivery Leadership', window: 'Awaiting telemetry', actions: [] },
      security: { owner: 'Security Operations', window: 'Awaiting telemetry', actions: [] }
    };
  }

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

export function buildQualityPulse(
  insights: DashboardInsights,
  { allowSampleFallbacks = true }: { allowSampleFallbacks?: boolean } = {}
): QualityPulse {
  const riskBuckets = summarizeRiskBuckets(insights.commitRiskCards);
  const bottleneckBuckets = summarizeBottlenecks(insights.bottlenecks);
  const [topRisk] = rankRiskCards(insights.commitRiskCards);
  const topOpportunity = rankOpportunities(insights.opportunities)[0];
  const topBottleneck = [...insights.bottlenecks].sort((left, right) => right.impact - left.impact)[0];
  const securitySignalCount = insights.commitRiskCards.filter((risk) =>
    risk.reasons.some((reason) => reason === 'Dependency risk' || reason === 'Automation failures')
  ).length;
  const opportunityCount = insights.opportunities.length;
  const overallScore = Math.max(45, 100 - riskBuckets.high * 10 - bottleneckBuckets.critical * 15 - bottleneckBuckets.high * 5);

  return {
    overallScore,
    securitySignalCount,
    topBottleneckName: topBottleneck?.name ?? 'review',
    opportunityCount,
    topRiskCommitId: topRisk?.id ?? '',
    topOpportunityTitle: topOpportunity?.title ?? 'trim flaky tests',
    riskBuckets,
    bottleneckBuckets,
    recommendations: buildRecommendations(insights, allowSampleFallbacks),
    actionRoutes: buildRoutes(allowSampleFallbacks)
  };
}
