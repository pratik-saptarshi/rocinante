import { expect, test } from '@playwright/test';

test.describe('frontend behavior', () => {
  test('renders the optimization dashboard shell and primary controls', async ({ page }) => {
    await page.goto('/');

    await expect(page.getByText('The Web Companion: Optimization Hub')).toBeVisible();
    await expect(page.getByRole('button', { name: 'Run Full Audit' })).toBeDisabled();
    await expect(page.getByText('Audit execution is unavailable in this preview.')).toBeVisible();
    await expect(page.getByText('WCAG 2.1/2.2 AA Accessibility Audit')).toBeVisible();
    await expect(page.getByText(/Page-specific and site-wide selection is unavailable/i)).toBeVisible();
    await expect(page.getByRole('switch', { name: 'Field and lab data selection unavailable in this preview' })).toBeDisabled();
    await expect(page.getByRole('button', { name: 'Team Lead' })).toBeVisible();
  });


  test('exposes semantic landmarks and keeps preview-only actions unavailable', async ({ page }) => {
    await page.goto('/');

    await expect(page.getByRole('main')).toHaveCount(1);
    await expect(page.getByRole('heading', { level: 1, name: 'The Web Companion: Optimization Hub' })).toBeVisible();
    await expect(page.getByRole('heading', { level: 2 }).first()).toBeVisible();
    await expect(page.getByRole('button', { name: 'Run Full Audit' })).toBeDisabled();
    await expect(page.getByRole('switch', { name: 'Field and lab data selection unavailable in this preview' })).toBeDisabled();
    await expect(page.getByRole('tab')).toHaveCount(0);
  });

  test('uses sufficient computed contrast for enabled primary and outlined actions', async ({ page }) => {
    await page.goto('/');
    const parseCssColor = (color: string) => {
      const channels = color.match(/[0-9.]+/g)?.map(Number);
      if (!channels || channels.length < 3) throw new Error(`Unexpected CSS color: ${color}`);
      const alpha = channels.length > 3 ? channels[3] : 1;
      return channels.slice(0, 3).map((channel) => channel * alpha + 255 * (1 - alpha));
    };
    const luminance = (color: number[]) => {
      const linear = color.map((channel) => {
        const value = channel / 255;
        return value <= 0.04045 ? value / 12.92 : ((value + 0.055) / 1.055) ** 2.4;
      });
      return 0.2126 * linear[0] + 0.7152 * linear[1] + 0.0722 * linear[2];
    };
    const contrast = (foreground: string, background: string) => {
      const values = [luminance(parseCssColor(foreground)), luminance(parseCssColor(background))]
        .sort((left, right) => right - left);
      return (values[0] + 0.05) / (values[1] + 0.05);
    };
    const colors = async (name: string) => page.getByRole('button', { name }).evaluate((element) => {
      const style = getComputedStyle(element);
      return { foreground: style.color, background: style.backgroundColor };
    });

    for (const name of ['Apply Payload', 'Reset to Sample']) {
      const button = page.getByRole('button', { name });
      const states = [await colors(name)];
      await button.focus();
      expect(await button.evaluate((element) => element.matches(':focus-visible'))).toBeTruthy();
      states.push(await colors(name));
      await button.hover();
      states.push(await colors(name));
      for (const state of states) {
        expect(contrast(state.foreground, state.background === 'rgba(0, 0, 0, 0)' ? 'rgb(255, 255, 255)' : state.background))
          .toBeGreaterThanOrEqual(4.5);
      }
    }
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
    const qualityPulse = page.getByTestId('quality-pulse-section');
    await expect(qualityPulse.getByText('Current import')).toBeVisible();
    await expect(qualityPulse.getByText(/browser-001: good risk \(score 12\)/)).toBeVisible();
    await expect(qualityPulse.getByText(/A-124|Sprint now|sample window/i)).toHaveCount(0);
    await page.getByRole('button', { name: 'Executive' }).click();
    await expect(qualityPulse.getByText(/Cache invalidation \(score/)).toBeVisible();
    await page.getByRole('button', { name: 'Security' }).click();
    await expect(qualityPulse.getByText(/No dependency or automation-failure signals were found/)).toBeVisible();

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
    const qualityPulse = page.getByTestId('quality-pulse-section');
    await expect(qualityPulse.getByText(/trim flaky tests|sample window|high-risk commit A-124/i)).toHaveCount(0);
    await expect(qualityPulse.getByText('Awaiting telemetry')).toHaveCount(1);
    await expect(qualityPulse.getByTestId('pulse-score')).toHaveText('Unavailable');
    await expect(qualityPulse.getByTestId('pulse-top-bottleneck')).toHaveText('Unavailable');
    await expect(page.getByTestId('explainability-section').getByText(/Top Bottleneck: Unavailable — No bottleneck records are available for this import\./)).toBeVisible();
    await expect(page.getByTestId('explainability-section').getByText(/Top Risk Commit: Unavailable — No commit-risk records are available for this import\./)).toBeVisible();
    const trendRisk = page.getByTestId('trend-risk-section');
    await expect(trendRisk.getByText(/PR Risk Trajectory: Unavailable — No commit-risk records are available\./)).toBeVisible();
    await expect(trendRisk.getByText(/Bottleneck Pressure: Unavailable — No bottleneck records are available\./)).toBeVisible();
    await page.getByRole('button', { name: 'Security' }).click();
    await expect(page.getByText('No critical security signals are available.')).toBeVisible();
    await expect(page.getByText(/sample window/i)).toHaveCount(0);
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

    await expect(page.getByTestId('admin-bridge-result')).toContainText('Desktop command runtime not detected');
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
  test('keeps dashboard tasks within common viewport widths', async ({ page }) => {
    for (const width of [320, 360, 768, 1024, 1280]) {
      await page.setViewportSize({ width, height: 900 });
      await page.goto('/');

      await expect(page.getByRole('main')).toBeVisible();
      await expect(page.getByRole('heading', { level: 1, name: 'The Web Companion: Optimization Hub' })).toBeVisible();

      const expectViewportFit = async (state: string) => {
        const documentWidth = await page.evaluate(() => document.documentElement.scrollWidth);
        expect(documentWidth, `page width at ${width}px viewport ${state}`).toBeLessThanOrEqual(width);
      };
      await expectViewportFit('on initial render');

      await page.getByLabel('Telemetry payload JSON').fill(JSON.stringify({
        commits: [{ id: `viewport-${width}`, files: 1, changedLines: 8, dependencyChanges: 0, testTouch: true, failedAutomations: 0 }],
        stages: [{ name: 'review', queueDepth: 1, throughput: 8, avgLatencyMs: 200 }],
        signals: []
      }));
      await page.getByRole('button', { name: 'Apply Payload' }).click();
      await expect(page.getByTestId('snapshot-risk-count')).toHaveText('1');
      await expectViewportFit('after telemetry import');

      const adminAction = page.getByRole('button', { name: 'Ingest Event' });
      await adminAction.scrollIntoViewIfNeeded();
      await adminAction.click();
      await expect(page.getByTestId('admin-bridge-result')).toContainText('Desktop command runtime not detected');
      await expectViewportFit('after admin fallback');
    }
  });

});
