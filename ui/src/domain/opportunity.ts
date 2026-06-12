export interface Signal {
  id: string;
  area: string;
  title: string;
  impact: number;
  effort: number;
  confidence: number;
}

export interface Opportunity {
  id: string;
  area: string;
  title: string;
  impact: number;
  effort: number;
  confidence: number;
  priorityScore: number;
}

export function rankOpportunities(signals: Signal[], limit = 3): Opportunity[] {
  return signals
    .map((signal) => {
      const safeEffort = Math.max(signal.effort, 1);
      const priorityScore = Number(((signal.impact * signal.confidence) / safeEffort * 100).toFixed(2));
      return {
        id: signal.id,
        area: signal.area,
        title: signal.title,
        impact: signal.impact,
        effort: signal.effort,
        confidence: signal.confidence,
        priorityScore
      };
    })
    .sort((a, b) => b.priorityScore - a.priorityScore)
    .slice(0, limit);
}
