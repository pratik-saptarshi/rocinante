import { assessCommitRisks, type CommitEvent } from './domain/risk';
import { detectBottlenecks, type PipelineStage } from './domain/bottleneck';
import { rankOpportunities, type Signal } from './domain/opportunity';
import { defaultInsightConfig, sampleCommits, sampleSignals, sampleStages } from './insight-data';

export type RawInput = string | number | boolean | null | undefined | Record<string, unknown> | unknown[];

export interface InsightPayload {
  commits?: Array<Partial<CommitEvent> & Record<string, RawInput>>;
  stages?: Array<Partial<PipelineStage> & Record<string, RawInput>>;
  signals?: Array<Partial<Signal> & Record<string, RawInput>>;
}

export interface InsightLimits {
  risks?: number;
  opportunities?: number;
  severityThreshold?: number;
  latencyP95Ms?: number;
}

export interface DashboardInsights {
  commitRiskCards: ReturnType<typeof assessCommitRisks>;
  bottlenecks: ReturnType<typeof detectBottlenecks>;
  opportunities: ReturnType<typeof rankOpportunities>;
}

function toPositiveInt(value: unknown): number {
  if (typeof value !== 'number' || Number.isNaN(value)) {
    return 0;
  }
  return Math.max(0, Math.floor(value));
}

function toCommitEvent(raw: Partial<CommitEvent>): CommitEvent {
  return {
    id: String(raw.id ?? ''),
    files: toPositiveInt(raw.files),
    changedLines: toPositiveInt(raw.changedLines),
    dependencyChanges: toPositiveInt(raw.dependencyChanges),
    testTouch: Boolean(raw.testTouch),
    failedAutomations: toPositiveInt(raw.failedAutomations)
  };
}

function toStage(raw: Partial<PipelineStage>): PipelineStage {
  return {
    name: String(raw.name ?? 'un-named stage'),
    queueDepth: toPositiveInt(raw.queueDepth),
    throughput: toPositiveInt(raw.throughput),
    avgLatencyMs: toPositiveInt(raw.avgLatencyMs)
  };
}

function toSignal(raw: Partial<Signal>): Signal {
  const effort = Math.max(0.1, toPositiveInt(raw.effort));
  const rawConfidence = raw.confidence;
  const normalizedConfidence =
    typeof rawConfidence === 'number' ? Math.min(1, Math.max(0, rawConfidence)) : 0;

  return {
    id: String(raw.id ?? ''),
    area: String(raw.area ?? 'general'),
    title: String(raw.title ?? 'Untitled opportunity'),
    impact: toPositiveInt(raw.impact),
    effort,
    confidence: normalizedConfidence
  };
}

export function buildDashboardInsights(payload?: InsightPayload, limits?: InsightLimits): DashboardInsights {
  const commitsSource = payload?.commits?.length ? payload.commits : sampleCommits;
  const stagesSource = payload?.stages?.length ? payload.stages : sampleStages;
  const signalsSource = payload?.signals?.length ? payload.signals : sampleSignals;

  const riskLimit = Math.max(1, toPositiveInt(limits?.risks ?? defaultInsightConfig.riskLimit));
  const opportunityLimit = Math.max(1, toPositiveInt(limits?.opportunities ?? defaultInsightConfig.opportunityLimit));
  const severityThreshold = limits?.severityThreshold ?? defaultInsightConfig.severityThreshold;
  const latencyP95Ms = toPositiveInt(limits?.latencyP95Ms ?? defaultInsightConfig.latencyP95Ms);

  return {
    commitRiskCards: assessCommitRisks(commitsSource.map(toCommitEvent), riskLimit),
    bottlenecks: detectBottlenecks(stagesSource.map(toStage), {
      severityThreshold,
      latencyP95Ms: latencyP95Ms || defaultInsightConfig.latencyP95Ms
    }),
    opportunities: rankOpportunities(signalsSource.map(toSignal), opportunityLimit)
  };
}
