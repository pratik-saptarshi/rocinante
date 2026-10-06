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
    await expect(page.getByTestId('quality-pulse-section').getByText(/Security-sensitive signals from/i).first()).toBeVisible();
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
    await expect(page.getByText('browser-001 score 12 (good)')).toBeVisible();
    await expect(page.getByTestId('telemetry-data-state')).toHaveText('Imported telemetry is displayed.');

    await page.getByRole('button', { name: 'Reset to Sample' }).click();
    await expect(page.getByTestId('snapshot-risk-count')).toHaveText('3');
    await expect(page.getByTestId('telemetry-data-state')).toHaveText('Sample telemetry is displayed.');
  });

  test('keeps last-good data after an invalid import and represents an empty import explicitly', async ({ page }) => {
    await page.goto('/');
    const jsonInput = page.getByLabel('Telemetry payload JSON');

    await jsonInput.fill(JSON.stringify({
      commits: [{ id: 'browser-good', files: 1, changedLines: 8, dependencyChanges: 0, testTouch: true, failedAutomations: 0 }],
      stages: [{ name: 'review', queueDepth: 1, throughput: 8, avgLatencyMs: 200 }],
      signals: []
    }));
    await page.getByRole('button', { name: 'Apply Payload' }).click();
    await expect(page.getByTestId('snapshot-risk-count')).toHaveText('1');

    await jsonInput.fill(JSON.stringify({
      commits: [{ id: 'bad', files: 1, changedLines: 8, dependencyChanges: 0, testTouch: true, failedAutomations: 0 }],
      stages: [{ name: { label: 'review' }, queueDepth: 1, throughput: 8, avgLatencyMs: 200 }],
      signals: []
    }));
    await page.getByRole('button', { name: 'Apply Payload' }).click();
    await expect(page.getByRole('alert')).toContainText('stages[0].name');
    await expect(page.getByText('browser-good score 3 (good)')).toBeVisible();
    await expect(page.getByTestId('snapshot-risk-count')).toHaveText('1');

    await jsonInput.fill(JSON.stringify({ commits: [], stages: [], signals: [] }));
    await page.getByRole('button', { name: 'Apply Payload' }).click();
    await expect(page.getByTestId('telemetry-data-state')).toHaveText('This imported payload contains no telemetry records.');
    await expect(page.getByTestId('snapshot-risk-count')).toHaveText('0');
  });

  test('announces malformed JSON and keeps the last-good dashboard visible', async ({ page }) => {
    await page.goto('/');
    const jsonInput = page.getByLabel('Telemetry payload JSON');

    await jsonInput.fill(JSON.stringify({
      commits: [{ id: 'last-good-json', files: 2, changedLines: 15, dependencyChanges: 0, testTouch: true, failedAutomations: 0 }],
      stages: [],
      signals: []
    }));
    await page.getByRole('button', { name: 'Apply Payload' }).click();
    await expect(page.getByTestId('snapshot-risk-count')).toHaveText('1');
    await expect(page.getByText('last-good-json score 6 (good)')).toBeVisible();

    await jsonInput.fill('{ commits: [');
    await page.getByRole('button', { name: 'Apply Payload' }).click();

    await expect(page.getByRole('alert')).toContainText('Invalid telemetry payload');
    await expect(page.getByTestId('snapshot-risk-count')).toHaveText('1');
    await expect(page.getByText('last-good-json score 6 (good)')).toBeVisible();
    await expect(jsonInput).toHaveAttribute('aria-invalid', 'true');
  });

  test('surfaces the admin bridge fallback in desktop-absent browsers', async ({ page }) => {
    await page.goto('/');

    await page.getByRole('button', { name: 'Ingest Event' }).click();

    await expect(page.getByTestId('admin-bridge-result')).toContainText('Tauri runtime not detected');
  });

  test('surfaces the admin bridge payload when the browser shim is available', async ({ page }) => {
    await page.addInitScript(() => {
      (globalThis as typeof globalThis & {
        __TAURI__?: { core?: { invoke?: (cmd: string, args: unknown) => Promise<unknown> } };
      }).__TAURI__ = {
        core: {
          invoke: async (cmd, args) => ({ cmd, args })
        }
      };
    });

    await page.goto('/');

    await page.getByRole('button', { name: 'Ingest Event' }).click();

    await expect(page.getByTestId('admin-bridge-result')).toContainText('ingest_event');
    await expect(page.getByTestId('admin-bridge-result')).toContainText('ui-bridge-001');
  });

  test('loads and reseeds release baselines through the browser shim', async ({ page }) => {
    await page.addInitScript(() => {
      (globalThis as typeof globalThis & {
        __TAURI__?: { core?: { invoke?: (cmd: string, args: unknown) => Promise<unknown> } };
      }).__TAURI__ = {
        core: {
          invoke: async (cmd, args) => {
            if (cmd === 'query_release_baseline') {
              return 9.75;
            }
            if (cmd === 'reseed_release_baseline') {
              return 12.25;
            }
            return { cmd, args };
          }
        }
      };
    });

    await page.goto('/');

    await expect(page.getByTestId('baseline-management-section')).toBeVisible();
    await page.getByRole('button', { name: 'Load Baseline' }).click();
    await expect(page.getByTestId('baseline-management-result')).toContainText(
      'OK query_release_baseline: 9.75'
    );

    await page.getByLabel('Baseline complexity').fill('12.25');
    await page.getByRole('button', { name: 'Reseed Baseline' }).click();
    await expect(page.getByTestId('baseline-management-result')).toContainText(
      'OK reseed_release_baseline: 12.25'
    );
  });
});
