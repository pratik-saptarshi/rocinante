import { type CommitRisk, type RiskLevel } from './risk';
import type { DashboardInsights } from '../insight-engine';
import type { Bottleneck } from './bottleneck';
import type { Opportunity } from './opportunity';

export type StakeholderAudience = 'lead' | 'manager' | 'executive' | 'security';
export type PulseSeverity = 'good' | 'medium' | 'bad';

interface BucketCounts {
  low: number;
  medium: number;
  high: number;
}

interface ActionRecommendation {
  id: string;
  severity: PulseSeverity;
  message: string;
}

interface ActionRoute {
  owner: string;
  window: string;
  actions: string[];
}

export interface QualityPulse {
  overallScore: number;
  riskBuckets: BucketCounts;
  bottleneckBuckets: {
    medium: number;
    high: number;
    critical: number;
  };
  opportunityCount: number;
  topRiskCommitId: string;
  topOpportunityTitle: string;
  topBottleneckName: string;
  securitySignalCount: number;
  recommendations: Record<StakeholderAudience, ActionRecommendation[]>;
  actionRoutes: Record<StakeholderAudience, ActionRoute>;
}

function clamp(value: number): number {
  return Math.max(0, Math.min(100, value));
}

function classifyByLevel(level: RiskLevel): keyof BucketCounts {
  if (level === 'high') {
    return 'high';
  }
  if (level === 'medium') {
    return 'medium';
  }
  return 'low';
}

function topReasoningCommit(risks: CommitRisk[]): string {
  return risks[0]?.id || 'none';
}

function topOpportunityTitle(opportunities: Opportunity[]): string {
  return opportunities[0]?.title || 'No opportunities available';
}

function topBottleneck(opportunities: Bottleneck[]): string {
  return opportunities[0]?.name || 'No severe bottleneck';
}

function countSecuritySignals(risks: CommitRisk[]): string[] {
  return risks
    .filter((risk) => risk.reasons.some((reason) => reason === 'Dependency risk' || reason === 'Automation failures'))
    .map((risk) => risk.id)
    .filter(Boolean);
}

function buildRecommendations(
  risks: CommitRisk[],
  bottlenecks: Bottleneck[],
  opportunities: Opportunity[],
  riskBuckets: BucketCounts,
  bottleneckBuckets: {
    medium: number;
    high: number;
    critical: number;
  },
  securitySignals: string[]
): Record<StakeholderAudience, ActionRecommendation[]> {
  const criticalStageNames = bottlenecks
    .filter((item) => item.status === 'critical')
    .map((item) => item.name)
    .join(', ');
  const highStageNames = bottlenecks.filter((item) => item.status === 'high').map((item) => item.name).join(', ');

  const topRisk = topReasoningCommit(risks);
  const topOpportunity = topOpportunityTitle(opportunities);
  const hasSignals = securitySignals.length > 0;

  return {
    lead: [
      ...(riskBuckets.high > 0
        ? [
            {
              id: `lead-${topRisk}-risk`,
              severity: 'bad' as const,
              message: `Focus first on high-risk commit ${topRisk} before merge.`
            }
          ]
        : []),
      {
        id: `lead-${riskBuckets.medium}-medium-risk`,
        severity: riskBuckets.high > 0 ? 'medium' : 'good',
        message:
          riskBuckets.medium > 0
            ? `${riskBuckets.medium} medium-risk commit(s) need pre-merge coaching`
            : 'No medium-risk commits blocking immediate merge.'
      },
      {
        id: `lead-opportunity-${opportunities.length}`,
        severity: opportunities.length > 0 ? 'good' : 'medium',
        message:
          opportunities.length > 0
            ? 'Prioritize quick-win opportunities before adding new scope.'
            : 'Collect more improvement signals for guided coaching.'
      }
    ],
    manager: [
      ...(criticalStageNames
        ? [
            {
              id: `manager-${criticalStageNames || 'no-critical'}-critical`,
              severity: criticalStageNames ? ('bad' as const) : ('good' as const),
              message: criticalStageNames
                ? `Critical stage(s): ${criticalStageNames}. Increase approval throughput and staffing.`
              : 'No critical bottlenecks; staffing signals look stable.'
            }
          ]
        : []),
      {
        id: 'manager-high-severity',
        severity: bottleneckBuckets.high > 0 ? 'medium' : 'good',
        message:
          bottleneckBuckets.high > 0
            ? `High-severity stages (${highStageNames}). Review queue policies and handoff SLAs.`
            : 'High-severity queue pressure is within tolerance.'
      }
    ],
    executive: [
      {
        id: `exec-${riskBuckets.high}-high-risks`,
        severity: riskBuckets.high > 2 ? 'bad' : 'medium',
        message: `${riskBuckets.high} of ${riskBuckets.low + riskBuckets.medium + riskBuckets.high} commits are high-risk.`
      },
      {
        id: `exec-${topOpportunity}-opp`,
        severity: opportunities.length > 0 ? 'good' : 'medium',
        message: `Top opportunity: ${topOpportunity}.`
      },
      {
        id: `exec-${bottleneckBuckets.critical}-critical-bot`,
        severity: bottleneckBuckets.critical > 0 ? 'bad' : 'good',
        message:
          bottleneckBuckets.critical > 0 ? 'Bottleneck risk is limiting delivery confidence.' : 'Delivery confidence is stable today.'
      }
    ],
    security: [
      {
        id: `security-${securitySignals.length}-signals`,
        severity: hasSignals ? ('bad' as const) : ('good' as const),
        message: hasSignals ? `Security-sensitive signals from ${securitySignals.length} commit(s): ${securitySignals.join(', ')}` : 'No critical security signals in sample window.'
      }
    ]
  };
}

function toActionRoutes(recommendations: Record<StakeholderAudience, ActionRecommendation[]>): Record<StakeholderAudience, ActionRoute> {
  return {
    lead: {
      owner: 'Lead Reviewer',
      window: 'Sprint now',
      actions: recommendations.lead.slice(0, 2).map((item) => item.message)
    },
    manager: {
      owner: 'Engineering Manager',
      window: 'This week',
      actions: recommendations.manager.slice(0, 2).map((item) => item.message)
    },
    executive: {
      owner: 'Delivery Leadership',
      window: 'This month',
      actions: recommendations.executive.slice(0, 2).map((item) => item.message)
    },
    security: {
      owner: 'Security Operations',
      window: 'Before release',
      actions: recommendations.security.slice(0, 2).map((item) => item.message)
    }
  };
}

export function buildQualityPulse(insights: DashboardInsights): QualityPulse {
  const { commitRiskCards, bottlenecks, opportunities } = insights;

  const riskBuckets = commitRiskCards.reduce(
    (acc: BucketCounts, risk: CommitRisk) => {
      acc[classifyByLevel(risk.level)] += 1;
      return acc;
    },
    { low: 0, medium: 0, high: 0 }
  );

  const severityBuckets = bottlenecks.reduce(
    (acc: { medium: number; high: number; critical: number }, stage: Bottleneck) => {
      if (stage.status === 'critical') {
        acc.critical += 1;
      } else if (stage.status === 'high') {
        acc.high += 1;
      } else {
        acc.medium += 1;
      }
      return acc;
    },
    { medium: 0, high: 0, critical: 0 }
  );

  const totalRisk = Math.max(1, commitRiskCards.length);
  const averageRiskScore =
    commitRiskCards.reduce((acc: number, item: CommitRisk) => acc + item.score, 0) / totalRisk;

  const riskPenalty = clamp(Math.round(averageRiskScore * 0.2));
  const concentrationPenalty = (riskBuckets.high / totalRisk) * 20;
  const severityPenalty = riskBuckets.high * 8 + severityBuckets.high * 4 + severityBuckets.critical * 10;
  const bonus = Math.min(12, opportunities.length * 2);

  const overallScore = clamp(
    Math.round(100 - concentrationPenalty - riskPenalty - severityPenalty + bonus)
  );

  const securitySignals = countSecuritySignals(commitRiskCards);
  const recommendations = buildRecommendations(
    commitRiskCards,
    bottlenecks,
    opportunities,
    riskBuckets,
    severityBuckets,
    securitySignals
  );

  return {
    overallScore,
    riskBuckets,
    bottleneckBuckets: severityBuckets,
    opportunityCount: opportunities.length,
    topRiskCommitId: topReasoningCommit(commitRiskCards),
    topOpportunityTitle: topOpportunityTitle(opportunities),
    topBottleneckName: topBottleneck(bottlenecks),
    securitySignalCount: securitySignals.length,
    recommendations,
    actionRoutes: toActionRoutes(recommendations)
  };
}
