import { assessCommitRisks, type CommitEvent } from './domain/risk';
import { detectBottlenecks, type PipelineStage } from './domain/bottleneck';
import { rankOpportunities, type Signal } from './domain/opportunity';

export const sampleCommits: CommitEvent[] = [
  { id: 'A-121', files: 9, changedLines: 460, dependencyChanges: 2, testTouch: false, failedAutomations: 1 },
  { id: 'A-122', files: 3, changedLines: 90, dependencyChanges: 0, testTouch: true, failedAutomations: 0 },
  { id: 'A-123', files: 6, changedLines: 190, dependencyChanges: 1, testTouch: true, failedAutomations: 0 },
  { id: 'A-124', files: 14, changedLines: 340, dependencyChanges: 0, testTouch: false, failedAutomations: 2 }
];

export const sampleStages: PipelineStage[] = [
  { name: 'scan', queueDepth: 2, throughput: 45, avgLatencyMs: 420 },
  { name: 'review', queueDepth: 11, throughput: 5, avgLatencyMs: 2400 },
  { name: 'ci', queueDepth: 45, throughput: 30, avgLatencyMs: 1100 },
  { name: 'deploy', queueDepth: 2, throughput: 4, avgLatencyMs: 3600 }
];

export const sampleSignals: Signal[] = [
  { id: 'o1', area: 'tests', title: 'Fix flaky test clusters on UI suite', impact: 5, effort: 6, confidence: 0.84 },
  { id: 'o2', area: 'ci', title: 'Enable parallel test shards on high-touch branches', impact: 5, effort: 4, confidence: 0.68 },
  { id: 'o3', area: 'review', title: 'Add reviewer SLA automation for SLA drifts', impact: 4, effort: 5, confidence: 0.9 },
  { id: 'o4', area: 'security', title: 'Gate dependency updates through staged canary', impact: 5, effort: 3, confidence: 0.75 }
];

export const defaultInsightConfig = {
  riskLimit: 3,
  opportunityLimit: 3,
  severityThreshold: 1.2,
  latencyP95Ms: 1200
} as const;

export const sampleDerivedInsights = {
  commitRiskCards: assessCommitRisks(sampleCommits, defaultInsightConfig.riskLimit),
  bottlenecks: detectBottlenecks(sampleStages, {
    severityThreshold: defaultInsightConfig.severityThreshold,
    latencyP95Ms: defaultInsightConfig.latencyP95Ms
  }),
  opportunities: rankOpportunities(sampleSignals, defaultInsightConfig.opportunityLimit)
};
