import { describe, expect, it } from 'vitest';
import { rankOpportunities, type Signal } from './domain/opportunity';

describe('opportunity ranking', () => {
  it('prioritizes opportunities by ROI and impact', () => {
    const signals: Signal[] = [
      { id: 'o1', area: 'tests', title: 'Reduce flaky tests', impact: 4, effort: 5, confidence: 0.9 },
      { id: 'o2', area: 'ci', title: 'Parallelize lint and tests', impact: 5, effort: 3, confidence: 0.7 },
      { id: 'o3', area: 'release', title: 'Add branch quality gates', impact: 3, effort: 2, confidence: 0.6 }
    ];

    const ranked = rankOpportunities(signals, 2);

    expect(ranked).toHaveLength(2);
    expect(ranked[0].id).toBe('o2');
    expect(ranked[0].priorityScore).toBeGreaterThan(ranked[1].priorityScore);
  });
});
