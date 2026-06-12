import { describe, expect, it } from 'vitest';
import { dashboardAudienceHighlights, dashboardFindingGroups } from './dashboard-content';

describe('dashboard content', () => {
  it('centralizes the role-specific copy', () => {
    expect(dashboardAudienceHighlights.lead).toEqual({
      tone: 'Team leads: prioritize blocked PR hotspots and coaching cues.',
      guidance: 'Prioritize review-ready commits before opening the next work cycle.'
    });
    expect(dashboardAudienceHighlights.manager.guidance).toContain('queue pressure');
    expect(dashboardAudienceHighlights.executive.tone).toContain('Executives');
    expect(dashboardAudienceHighlights.security.guidance).toContain('release and dependency risks');
  });

  it('groups the section findings for reuse in App', () => {
    expect(dashboardFindingGroups.accessibility).toHaveLength(3);
    expect(dashboardFindingGroups.seo).toHaveLength(3);
    expect(dashboardFindingGroups.security[0]).toEqual(
      expect.objectContaining({ id: 'csp', text: expect.stringContaining('Content-Security-Policy'), status: 'bad' })
    );
    expect(dashboardFindingGroups.performance[0]).toEqual(
      expect.objectContaining({ id: 'lcp', text: expect.stringContaining('Largest Contentful Paint') })
    );
  });
});
