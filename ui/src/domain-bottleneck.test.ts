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

  it('drops low-pressure stages while preserving medium bottlenecks', () => {
    const stages: PipelineStage[] = [
      { name: 'archive', queueDepth: 1, throughput: 20, avgLatencyMs: 300 },
      { name: 'review', queueDepth: 3, throughput: 4, avgLatencyMs: 1000 }
    ];

    const items = detectBottlenecks(stages, { severityThreshold: 0.7, latencyP95Ms: 1000 });

    expect(items).toHaveLength(1);
    expect(items[0].name).toBe('review');
    expect(items[0].status).toBe('medium');
    expect(items[0].impact).toBeGreaterThan(0.7);
    expect(items[0].impact).toBeLessThan(1.0);
  });

  it('uses the default thresholds when options are omitted', () => {
    const stages: PipelineStage[] = [
      { name: 'edge', queueDepth: 6, throughput: 5, avgLatencyMs: 1000 }
    ];

    const items = detectBottlenecks(stages);

    expect(items).toHaveLength(1);
    expect(items[0].name).toBe('edge');
    expect(items[0].impact).toBeGreaterThanOrEqual(1.2);
    expect(items[0].status).toBe('high');
  });
});
