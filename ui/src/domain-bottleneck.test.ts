import { describe, expect, it } from 'vitest';
import { detectBottlenecks, type PipelineStage } from './domain/bottleneck';

describe('bottleneck detection', () => {
  it('orders stages by normalized queue pressure', () => {
    const stages: PipelineStage[] = [
      { name: 'scan', queueDepth: 2, throughput: 45, avgLatencyMs: 420 },
      { name: 'review', queueDepth: 11, throughput: 5, avgLatencyMs: 2400 },
      { name: 'ci', queueDepth: 45, throughput: 30, avgLatencyMs: 1100 }
    ];

    const items = detectBottlenecks(stages, { severityThreshold: 1.2, latencyP95Ms: 1200 });

    expect(items).toHaveLength(2);
    expect(items[0].name).toBe('review');
    expect(items[0].impact).toBeGreaterThan(items[1].impact);
    expect(items[0].status).toBe('critical');
    expect(items[1].status).toBe('high');
  });
});
