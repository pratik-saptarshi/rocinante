export interface PipelineStage {
  name: string;
  queueDepth: number;
  throughput: number;
  avgLatencyMs: number;
}

export interface Bottleneck {
  name: string;
  impact: number;
  status: 'medium' | 'high' | 'critical';
  rationale: string;
}

export function detectBottlenecks(
  stages: PipelineStage[],
  options?: { latencyP95Ms?: number; severityThreshold?: number }
): Bottleneck[] {
  const p95 = options?.latencyP95Ms ?? 1000;
  const threshold = options?.severityThreshold ?? 1.2;

  const measured = stages
    .map((stage) => {
      const pressure = (stage.queueDepth / Math.max(stage.throughput, 1)) * (stage.avgLatencyMs / p95);
      const score = Number((pressure * 100).toFixed(2));
      const status: Bottleneck['status'] = score >= 200 ? 'critical' : score >= 120 ? 'high' : 'medium';
      const rationale = `pressure=${pressure.toFixed(2)} via queue=${stage.queueDepth}, throughput=${stage.throughput}, latency=${stage.avgLatencyMs}ms`;
      return { name: stage.name, impact: pressure, status, rationale, score };
    })
    .filter((item) => item.impact >= threshold);

  return measured
    .sort((a, b) => b.impact - a.impact)
    .map(({ name, impact, status, rationale }) => ({
      name,
      impact: Number(impact.toFixed(2)),
      status,
      rationale
    }));
}
