import { expect, test } from '@playwright/test';

test.describe('frontend behavior', () => {
  test('renders the optimization dashboard shell and primary controls', async ({ page }) => {
    await page.goto('/');

    await expect(page.getByText('The Web Companion: Optimization Hub')).toBeVisible();
    await expect(page.getByRole('button', { name: 'Run Full Audit' })).toBeVisible();
    await expect(page.getByText('WCAG 2.1/2.2 AA Accessibility Audit')).toBeVisible();
    await expect(page.getByRole('tab', { name: 'Current Page' })).toBeVisible();
    await expect(page.getByRole('tab', { name: 'Site-Wide' })).toBeVisible();
    await expect(page.getByRole('button', { name: 'Team Lead' })).toBeVisible();
  });

  test('switches stakeholder views and updates the quality pulse copy', async ({ page }) => {
    await page.goto('/');

    await page.getByRole('button', { name: 'Security' }).click();

    await expect(page.getByText('Security Focus')).toBeVisible();
    await expect(page.getByText('Security Operations')).toBeVisible();
    await expect(page.getByText('Security-Weighted Commit Signals')).toBeVisible();
    await expect(page.getByText(/Security-sensitive signals from/i)).toBeVisible();
  });

  test('applies payload envelopes and resets to sample data', async ({ page }) => {
    await page.goto('/');

    const jsonInput = page.getByLabel('Telemetry payload JSON');
    await jsonInput.fill(
      JSON.stringify({
        payload: {
          commits: [
            {
              id: 'browser-001',
              files: 2,
              changedLines: 61,
              dependencyChanges: 0,
              testTouch: true,
              failedAutomations: 0
            }
          ],
          stages: [{ name: 'review', queueDepth: 1, throughput: 12, avgLatencyMs: 200 }],
          signals: [{ id: 'browser-op-1', area: 'infra', title: 'Cache invalidation', impact: 4, effort: 2, confidence: 0.9 }]
        },
        limits: { risks: 1, opportunities: 1, severityThreshold: 4, latencyP95Ms: 1000 }
      })
    );

    await page.getByRole('button', { name: 'Apply Payload' }).click();

    await expect(page.getByTestId('snapshot-risk-count')).toHaveText('1');
    await expect(page.getByTestId('snapshot-opportunity-count')).toHaveText('1');
    await expect(page.getByText('browser-001 score 100')).toBeVisible();

    await page.getByRole('button', { name: 'Reset to Sample' }).click();
    await expect(page.getByTestId('snapshot-risk-count')).toHaveText('3');
  });

  test('surfaces the admin bridge fallback in desktop-absent browsers', async ({ page }) => {
    await page.goto('/');

    await page.getByRole('button', { name: 'Ingest Event' }).click();

    await expect(page.getByTestId('admin-bridge-result')).toContainText('Tauri runtime not detected');
  });
});
