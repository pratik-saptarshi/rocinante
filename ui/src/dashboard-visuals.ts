import type { DashboardInsights } from './insight-engine';

export type VisualTone = 'good' | 'medium' | 'bad';

export interface TrendLineCard {
  id: string;
  label: string;
  value: string;
  tone: VisualTone;
  rationale: string;
}

export interface PrRiskRankingCard {
  id: string;
  title: string;
  score: number;
  tone: VisualTone;
  rationale: string;
}

export interface DashboardVisuals {
  summary: string;
  trendLines: TrendLineCard[];
  prRiskRankings: PrRiskRankingCard[];
}

function toneFromRiskScore(score: number): VisualTone {
  if (score >= 80) return 'bad';
  if (score >= 50) return 'medium';
  return 'good';
}

function toneFromOpportunityScore(score: number): VisualTone {
  if (score >= 70) return 'good';
  if (score >= 45) return 'medium';
  return 'bad';
}

export function buildDashboardVisuals(insights: DashboardInsights): DashboardVisuals {
  const sortedRisks = [...insights.commitRiskCards].sort((left, right) => right.score - left.score);
  const sortedBottlenecks = [...insights.bottlenecks].sort((left, right) => right.impact - left.impact);
  const sortedOpportunities = [...insights.opportunities].sort((left, right) => right.priorityScore - left.priorityScore);

  const topRisk = sortedRisks[0];
  const criticalRisks = sortedRisks.filter((risk) => risk.level === 'high').length;
  const pressureStages = insights.bottlenecks.filter((stage) => stage.status === 'critical' || stage.status === 'high').length;
  const topStage = sortedBottlenecks[0];
  const topOpportunity = sortedOpportunities[0];

  return {
    summary: topRisk
      ? `${criticalRisks} high-risk commits and ${pressureStages} pressured stages`
      : 'No risk signals available',
    trendLines: [
      {
        id: 'risk-trajectory',
        label: 'PR Risk Trajectory',
        value: topRisk ? `${criticalRisks} high-risk commits` : 'No high-risk commits',
        tone: topRisk ? toneFromRiskScore(topRisk.score) : 'good',
        rationale: topRisk
          ? `Top risk ${topRisk.id} is driven by ${topRisk.reasons.slice(0, 2).join(', ') || 'sample data'}.`
          : 'The current sample window has no elevated commit risks.'
      },
      {
        id: 'bottleneck-pressure',
        label: 'Bottleneck Pressure',
        value: `${pressureStages} pressured stages`,
        tone: topStage ? topStage.status : 'good',
        rationale: topStage
          ? `Highest-pressure stage ${topStage.name} needs ${topStage.status === 'critical' ? 'immediate' : 'near-term'} attention.`
          : 'The current sample window has no pressured stages.'
      },
      {
        id: 'opportunity-velocity',
        label: 'Opportunity Velocity',
        value: `${insights.opportunities.length} actionable opportunities`,
        tone: topOpportunity ? toneFromOpportunityScore(topOpportunity.priorityScore) : 'good',
        rationale: topOpportunity
          ? `Lead opportunity ${topOpportunity.title} should unblock the next cycle.`
          : 'No opportunities surfaced in the current payload.'
      }
    ],
    prRiskRankings: sortedRisks.slice(0, 3).map((risk) => ({
      id: risk.id,
      title: `${risk.id} score ${risk.score}`,
      score: risk.score,
      tone: risk.level === 'high' ? 'bad' : risk.level === 'medium' ? 'medium' : 'good',
      rationale: risk.reasons.length ? risk.reasons.join(', ') : 'No named risk factors'
    }))
  };
}
