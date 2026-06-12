import { describe, expect, it } from 'vitest';
import { assessCommitRisks, type CommitEvent } from './domain/risk';

describe('commit risk scoring', () => {
  it('computes high risk for high churn and dependency additions', () => {
    const events: CommitEvent[] = [
      {
        id: 'c-1',
        files: 2,
        changedLines: 120,
        dependencyChanges: 0,
        testTouch: true,
        failedAutomations: 0
      },
      {
        id: 'c-2',
        files: 9,
        changedLines: 500,
        dependencyChanges: 2,
        testTouch: false,
        failedAutomations: 1
      }
    ];

    const ranked = assessCommitRisks(events, 2);

    expect(ranked).toHaveLength(2);
    expect(ranked[0].id).toBe('c-2');
    expect(ranked[0].level).toBe('high');
    expect(ranked[0].score).toBeGreaterThan(80);
    expect(ranked[0].reasons.length).toBeGreaterThanOrEqual(2);
  });

  it('classifies low risk commits', () => {
    const ranked = assessCommitRisks(
      [
        {
          id: 'c-3',
          files: 1,
          changedLines: 8,
          dependencyChanges: 0,
          testTouch: true,
          failedAutomations: 0
        }
      ],
      1
    );

    expect(ranked[0].level).toBe('low');
    expect(ranked[0].score).toBeLessThan(45);
    expect(ranked[0].reasons).toHaveLength(0);
  });
});
