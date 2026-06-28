export interface InsightLimits {
  risks?: number;
  opportunities?: number;
  severityThreshold?: number;
  latencyP95Ms?: number;
}

export interface InsightCommit {
  id: string;
  files: number;
  changedLines: number;
  dependencyChanges: number;
  testTouch: boolean;
  failedAutomations: number;
}

export interface InsightStage {
  name: string;
  queueDepth: number;
  throughput: number;
  avgLatencyMs: number;
}

export interface InsightSignal {
  id: string;
  area: string;
  title: string;
  impact: number;
  effort: number;
  confidence: number;
}

export interface InsightPayload {
  commits?: InsightCommit[];
  stages?: InsightStage[];
  signals?: InsightSignal[];
}

export interface CommitRiskCard {
  id: string;
  score: number;
  level: 'high' | 'medium' | 'good';
  reasons: string[];
}

export interface BottleneckCard {
  name: string;
  status: 'critical' | 'high' | 'medium' | 'good';
  impact: number;
  rationale: string;
}

export interface OpportunityCard {
  id: string;
  title: string;
  priorityScore: number;
}

export interface DashboardInsights {
  commitRiskCards: CommitRiskCard[];
  bottlenecks: BottleneckCard[];
  opportunities: OpportunityCard[];
  stages: InsightStage[];
}

const defaultCommitSeed: InsightCommit[] = [
  { id: 'A-124', files: 16, changedLines: 280, dependencyChanges: 1, testTouch: false, failedAutomations: 1 },
  { id: 'B-245', files: 9, changedLines: 110, dependencyChanges: 0, testTouch: true, failedAutomations: 0 },
  { id: 'C-381', files: 4, changedLines: 28, dependencyChanges: 0, testTouch: true, failedAutomations: 0 }
];

const defaultStageSeed: InsightStage[] = [
  { name: 'review', queueDepth: 10, throughput: 9, avgLatencyMs: 1100 },
  { name: 'build', queueDepth: 5, throughput: 12, avgLatencyMs: 850 },
  { name: 'release', queueDepth: 4, throughput: 18, avgLatencyMs: 420 }
];

const defaultSignalSeed: InsightSignal[] = [
  { id: 'op-1', area: 'tests', title: 'Trim flaky tests', impact: 5, effort: 2, confidence: 0.9 },
  { id: 'op-2', area: 'deps', title: 'Reduce dependency churn', impact: 4, effort: 3, confidence: 0.8 },
  { id: 'op-3', area: 'ui', title: 'Split dashboard rendering concerns', impact: 3, effort: 2, confidence: 0.75 }
];

function clampScore(value: number): number {
  return Math.max(0, Math.min(100, Math.round(value)));
}

function scoreCommit(commit: InsightCommit): CommitRiskCard {
  const rawScore =
    commit.changedLines / 8 +
    commit.files * 2 +
    commit.dependencyChanges * 8 +
    commit.failedAutomations * 20 +
    (commit.testTouch ? 0 : 10);
  const score = clampScore(rawScore);
  const level = score >= 80 ? 'high' : score >= 50 ? 'medium' : 'good';
  const reasons = [
    ...(commit.dependencyChanges > 0 ? ['Dependency risk'] : []),
    ...(commit.failedAutomations > 0 ? ['Automation failures'] : []),
    ...(commit.files >= 12 ? ['Large diff surface'] : []),
    ...(commit.testTouch ? [] : ['Missing test coverage'])
  ];

  return { id: commit.id, score, level, reasons };
}

function stageToBottleneck(stage: InsightStage, latencyCeiling: number): BottleneckCard {
  if (stage.queueDepth >= 10 || stage.avgLatencyMs >= latencyCeiling * 3) {
    return {
      name: stage.name,
      status: 'critical',
      impact: stage.queueDepth + 5,
      rationale: `Critical stage(s): ${stage.name} is exceeding the tolerated queue window.`
    };
  }

  if (stage.queueDepth >= 4 || stage.avgLatencyMs >= latencyCeiling) {
    return {
      name: stage.name,
      status: 'high',
      impact: stage.queueDepth + 2,
      rationale: `Critical stage(s): ${stage.name} is approaching the queue pressure ceiling.`
    };
  }

  return {
    name: stage.name,
    status: 'good',
    impact: Math.max(1, stage.queueDepth),
    rationale: `${stage.name} stays within the healthy execution window.`
  };
}

function signalToOpportunity(signal: InsightSignal): OpportunityCard {
  return {
    id: signal.id,
    title: signal.title,
    priorityScore: clampScore(signal.impact * 16 + signal.confidence * 10 - signal.effort * 3)
  };
}

function limitList<T>(items: T[], limit?: number): T[] {
  return typeof limit === 'number' && Number.isFinite(limit) ? items.slice(0, Math.max(0, Math.floor(limit))) : items;
}

export function buildDashboardInsights(payload: InsightPayload = {}, limits: InsightLimits = {}): DashboardInsights {
  const commits = (payload.commits?.length ? payload.commits : defaultCommitSeed)
    .map(scoreCommit)
    .sort((left, right) => right.score - left.score || left.id.localeCompare(right.id));
  const stages = payload.stages?.length ? payload.stages : defaultStageSeed;
  const signals = (payload.signals?.length ? payload.signals : defaultSignalSeed)
    .map(signalToOpportunity)
    .sort((left, right) => right.priorityScore - left.priorityScore || left.id.localeCompare(right.id));
  const latencyCeiling = limits.latencyP95Ms ?? 1_000;

  return {
    commitRiskCards: limitList(commits, limits.risks),
    bottlenecks: stages.map((stage) => stageToBottleneck(stage, latencyCeiling)),
    opportunities: limitList(signals, limits.opportunities),
    stages
  };
}
