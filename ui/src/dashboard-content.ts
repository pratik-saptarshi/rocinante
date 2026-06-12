import type { StakeholderAudience } from './domain/quality-pulse';

export type AuditStatus = 'good' | 'medium' | 'bad';

export interface DashboardFinding {
  id: string;
  text: string;
  status: AuditStatus;
}

export interface AudienceHighlight {
  tone: string;
  guidance: string;
}

export const dashboardAudienceHighlights: Record<StakeholderAudience, AudienceHighlight> = {
  lead: {
    tone: 'Team leads: prioritize blocked PR hotspots and coaching cues.',
    guidance: 'Prioritize review-ready commits before opening the next work cycle.'
  },
  manager: {
    tone: 'Managers: monitor cycle time, reviewer load, and handoff stability.',
    guidance: 'Uncover queue pressure to rebalance approval throughput and merge cadence.'
  },
  executive: {
    tone: 'Executives: monitor strategic quality and delivery predictability.',
    guidance: 'Use the opportunity list to stabilize throughput and defect risk over time.'
  },
  security: {
    tone: 'Security: prioritize policy drift and dependency risk signals.',
    guidance: 'Surface release and dependency risks before they enter long-running branches.'
  }
};

export const dashboardFindingGroups: Record<'accessibility' | 'seo' | 'security' | 'performance', DashboardFinding[]> = {
  accessibility: [
    { id: 'alt-text', text: 'Missing image alt text (7 instances)', status: 'bad' },
    { id: 'contrast', text: 'Low color contrast on buttons', status: 'medium' },
    { id: 'keyboard', text: 'Keyboard navigation traps found', status: 'medium' }
  ],
  seo: [
    { id: 'h1', text: 'Improve H1 and title tag consistency', status: 'medium' },
    { id: 'faq', text: 'Add FAQ schema', status: 'medium' },
    { id: 'qa', text: 'Optimize for long-tail, question-based queries', status: 'medium' }
  ],
  security: [
    { id: 'csp', text: 'Content-Security-Policy: Missing', status: 'bad' },
    { id: 'frame', text: 'X-Frame-Options: SAMEORIGIN', status: 'good' },
    { id: 'hsts', text: 'Strict-Transport-Security: Enabled', status: 'good' }
  ],
  performance: [
    { id: 'lcp', text: 'Largest Contentful Paint (LCP): 2.9s', status: 'medium' },
    { id: 'cls', text: 'Cumulative Layout Shift (CLS): 0.05', status: 'good' },
    { id: 'tbt', text: 'Total Blocking Time (TBT): 350ms', status: 'medium' }
  ]
};
