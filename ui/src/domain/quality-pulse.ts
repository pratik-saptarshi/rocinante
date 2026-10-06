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
  overallScore: number | null;
  securitySignalCount: number;
  topBottleneckName: string | null;
  topRiskCommitId: string | null;
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

function getAllCommitRiskCards(insights: DashboardInsights): CommitRiskCard[] {
  return insights.allCommitRiskCards ?? insights.commitRiskCards;
}

function getSecuritySignals(insights: DashboardInsights): CommitRiskCard[] {
  return rankRiskCards(getAllCommitRiskCards(insights).filter((risk) =>
    risk.reasons.some((reason) => reason === 'Dependency risk' || reason === 'Automation failures')
  ));
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
  const securitySignals = getSecuritySignals(insights);

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

function buildRoutes(
  insights: DashboardInsights,
  allowSampleFallbacks: boolean
): QualityPulse['actionRoutes'] {
  if (!allowSampleFallbacks) {
    const rankedRiskCards = rankRiskCards(insights.commitRiskCards);
    const actionableBottlenecks = insights.bottlenecks.filter((item) => item.status !== 'good');
    const rankedOpportunities = rankOpportunities(insights.opportunities);
    const securitySignals = getSecuritySignals(insights);
    const allCommitRiskCards = getAllCommitRiskCards(insights);

    return {
      lead: insights.commitRiskCards.length === 0
        ? { owner: 'Lead Reviewer', window: 'Awaiting telemetry', actions: [] }
        : {
            owner: 'Lead Reviewer',
            window: 'Current import',
            actions: rankedRiskCards.slice(0, 2).map((risk) => {
              const reasons = risk.reasons.length > 0 ? risk.reasons.join(', ') : 'no risk factors were provided';
              return `${risk.id}: ${risk.level} risk (score ${risk.score}); ${reasons}.`;
            })
          },
      manager: insights.bottlenecks.length === 0
        ? { owner: 'Engineering Manager', window: 'Awaiting telemetry', actions: [] }
        : {
            owner: 'Engineering Manager',
            window: 'Current import',
            actions: actionableBottlenecks.length > 0
              ? actionableBottlenecks.slice(0, 2).map((stage) => `${stage.name}: ${stage.status} pressure — ${stage.rationale}`)
              : ['No pressured stages were identified in the imported telemetry.']
          },
      executive: insights.opportunities.length === 0
        ? { owner: 'Delivery Leadership', window: 'Awaiting telemetry', actions: [] }
        : {
            owner: 'Delivery Leadership',
            window: 'Current import',
            actions: rankedOpportunities.slice(0, 2).map((opportunity) => `${opportunity.title} (score ${opportunity.priorityScore}).`)
          },
      security: allCommitRiskCards.length === 0
        ? { owner: 'Security Operations', window: 'Awaiting telemetry', actions: [] }
        : {
            owner: 'Security Operations',
            window: 'Current import',
            actions: securitySignals.length > 0
              ? securitySignals.slice(0, 2).map((signal) => `${signal.id}: review ${signal.reasons.filter((reason) => reason === 'Dependency risk' || reason === 'Automation failures').join(' and ').toLowerCase()}.`)
              : ['No dependency or automation-failure signals were found in the imported commits.']
          }
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
  const securitySignalCount = getSecuritySignals(insights).length;
  const opportunityCount = insights.opportunities.length;
  const calculatedScore = Math.max(45, 100 - riskBuckets.high * 10 - bottleneckBuckets.critical * 15 - bottleneckBuckets.high * 5);
  // Imported empty collections contain no observations; a score needs both inputs.
  const hasCompleteScoreInputs = insights.commitRiskCards.length > 0 && insights.bottlenecks.length > 0;
  const overallScore = !allowSampleFallbacks && !hasCompleteScoreInputs ? null : calculatedScore;

  return {
    overallScore,
    securitySignalCount,
    topBottleneckName: topBottleneck?.name ?? (allowSampleFallbacks ? 'review' : null),
    opportunityCount,
    topRiskCommitId: topRisk?.id ?? (allowSampleFallbacks ? '' : null),
    topOpportunityTitle: topOpportunity?.title ?? 'trim flaky tests',
    riskBuckets,
    bottleneckBuckets,
    recommendations: buildRecommendations(insights, allowSampleFallbacks),
    actionRoutes: buildRoutes(insights, allowSampleFallbacks)
  };
}
