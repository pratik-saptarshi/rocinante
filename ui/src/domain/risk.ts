export type RiskLevel = 'low' | 'medium' | 'high';

export interface CommitEvent {
  id: string;
  files: number;
  changedLines: number;
  dependencyChanges: number;
  testTouch: boolean;
  failedAutomations: number;
}

export interface CommitRisk {
  id: string;
  score: number;
  level: RiskLevel;
  reasons: string[];
}

function clampScore(value: number): number {
  if (value < 0) return 0;
  if (value > 100) return 100;
  return Math.round(value);
}

function classify(score: number): RiskLevel {
  if (score >= 75) return 'high';
  if (score >= 45) return 'medium';
  return 'low';
}

export function assessCommitRisks(events: CommitEvent[], limit = 3): CommitRisk[] {
  const scored = events.map((event) => {
    const reasons: string[] = [];
    let score = 0;

    if (event.changedLines > 200) {
      score += Math.min(55, Math.round(event.changedLines / 20));
      reasons.push('High churn');
    } else {
      score += Math.round(event.changedLines / 20);
    }

    if (event.files > 8) {
      score += 25;
      reasons.push('High file spread');
    }

    if (event.dependencyChanges > 0) {
      score += event.dependencyChanges * 10;
      reasons.push('Dependency risk');
    }

    if (event.failedAutomations > 0) {
      score += event.failedAutomations * 20;
      reasons.push('Automation failures');
    }

    if (!event.testTouch) {
      score += 12;
      reasons.push('No tests touched');
    }

    const normalized = clampScore(score);
    const level = classify(normalized);
    return { id: event.id, score: normalized, level, reasons };
  });

  return scored
    .sort((a, b) => b.score - a.score)
    .slice(0, limit);
}
