import { fireEvent, render, screen, waitFor, within } from '@testing-library/react';
import { afterEach, describe, expect, it, vi } from 'vitest';
import App from './App';
import { ThemeProvider } from '@mui/material';
import { setAdminInvokeForTesting } from './tauri-admin';
import { appTheme } from './theme';


describe('Phase 3 accessibility and control behavior', () => {
  it('provides a main landmark, a page heading, and honest preview controls', () => {
    render(<App />);

    expect(screen.getAllByRole('main')).toHaveLength(1);
    expect(screen.getByRole('heading', { level: 1, name: 'The Web Companion: Optimization Hub' })).toBeInTheDocument();
    expect(screen.getAllByRole('heading', { level: 2 }).length).toBeGreaterThan(0);
    expect(screen.getByRole('button', { name: 'Run Full Audit' })).toBeDisabled();
    expect(screen.getByText('Audit execution is unavailable in this preview.')).toBeVisible();
    expect(screen.queryByRole('tab')).not.toBeInTheDocument();
    expect(screen.getByText(/Page-specific and site-wide selection is unavailable/i)).toBeVisible();
  });

  it('associates the unavailable field and lab selector with its visible label', () => {
    render(<App />);

    const selector = screen.getByRole('switch', {
      name: 'Field and lab data selection unavailable in this preview'
    });
    expect(selector).toBeDisabled();
    expect(selector).not.toBeChecked();
    fireEvent.keyDown(selector, { key: ' ' });
    expect(selector).not.toBeChecked();
  });

  it('keeps duplicate stage severities attached to their own telemetry rows', () => {
    render(<App />);
    fireEvent.change(screen.getByLabelText(/Telemetry payload JSON/i), {
      target: {
        value: JSON.stringify({
          commits: [],
          stages: [
            { name: 'duplicate', queueDepth: 0, throughput: 10, avgLatencyMs: 100 },
            { name: 'duplicate', queueDepth: 10, throughput: 2, avgLatencyMs: 3000 }
          ],
          signals: []
        })
      }
    });
    fireEvent.click(screen.getByRole('button', { name: /Apply Payload/i }));

    const observability = screen.getByTestId('job-observability-section');
    const healthyRow = within(observability).getByText(/duplicate: queue 0, throughput 10, lag 100ms/i).closest('li');
    const criticalRow = within(observability).getByText(/duplicate: queue 10, throughput 2, lag 3000ms/i).closest('li');
    expect(healthyRow).toHaveTextContent('good');
    expect(criticalRow).toHaveTextContent('bad');
  });

  it('announces pending and completed admin bridge results through one status region', async () => {
    let resolveCommand: ((value: unknown) => void) | undefined;
    setAdminInvokeForTesting(() => new Promise((resolve) => { resolveCommand = resolve; }));
    render(<App />);

    const result = screen.getByTestId('admin-bridge-result');
    expect(result).toHaveAttribute('role', 'status');
    fireEvent.click(screen.getByRole('button', { name: 'Ingest Event' }));
    expect(result).toHaveTextContent('Running ingest_event');
    expect(result).toHaveAttribute('aria-busy', 'true');

    resolveCommand?.({ accepted: true });
    await waitFor(() => expect(result).toHaveTextContent('OK ingest_event'));
    expect(result).toHaveAttribute('aria-busy', 'false');
  });

  it('announces bridge errors as final status text', async () => {
    setAdminInvokeForTesting(async () => { throw new Error('bridge offline'); });
    render(<App />);

    fireEvent.click(screen.getByRole('button', { name: 'Ingest Event' }));
    await waitFor(() => expect(screen.getByTestId('admin-bridge-result')).toHaveTextContent('ERR ingest_event: bridge offline'));
  });

  it('keeps enabled primary and outlined button text above 4.5:1 contrast', () => {
    render(<ThemeProvider theme={appTheme}><App /></ThemeProvider>);
    const parseRgb = (value: string) => {
      const channels = value.match(/[\\d.]+/g)?.slice(0, 3).map(Number);
      if (!channels || channels.length !== 3) throw new Error(`Unexpected computed color: ${value}`);
      return channels.map((channel) => {
        const normalized = channel / 255;
        return normalized <= 0.04045 ? normalized / 12.92 : ((normalized + 0.055) / 1.055) ** 2.4;
      });
    };
    const contrast = (foreground: string, background: string) => {
      const luminance = (color: string) => {
        const [red, green, blue] = parseRgb(color);
        return 0.2126 * red + 0.7152 * green + 0.0722 * blue;
      };
      const values = [luminance(foreground), luminance(background)].sort((left, right) => right - left);
      return (values[0] + 0.05) / (values[1] + 0.05);
    };

    const contained = screen.getByRole('button', { name: 'Apply Payload' });
    const outlined = screen.getByRole('button', { name: 'Reset to Sample' });
    const containedStyle = getComputedStyle(contained);
    const outlinedStyle = getComputedStyle(outlined);
    expect(contrast(containedStyle.color, containedStyle.backgroundColor)).toBeGreaterThanOrEqual(4.5);
    expect(contrast(outlinedStyle.color, 'rgb(255, 255, 255)')).toBeGreaterThanOrEqual(4.5);
  });
});

afterEach(() => {
  setAdminInvokeForTesting(null);
});

describe('dashboard explainability panel', () => {
  it('renders deterministic decomposition traces from sample insights', () => {
    render(<App />);

    const explainabilitySection = screen.getByTestId('explainability-section');

    expect(within(explainabilitySection).getByText(/Explainability Traces/i)).toBeInTheDocument();
    expect(within(explainabilitySection).getByText(/Score Decomposition/i)).toBeInTheDocument();
    expect(within(explainabilitySection).getByText(/Top Risk Commit/i)).toBeInTheDocument();
    expect(within(explainabilitySection).getByText(/Top Bottleneck/i)).toBeInTheDocument();
    expect(within(explainabilitySection).getByText(/Opportunity Signals/i)).toBeInTheDocument();
  });

  it('updates explainability traces when payload changes', () => {
    render(<App />);

    fireEvent.change(screen.getByLabelText(/Telemetry payload JSON/i), {
      target: {
        value: JSON.stringify({
          commits: [
            {
              id: 'safe-1',
              files: 1,
              changedLines: 8,
              dependencyChanges: 0,
              testTouch: true,
              failedAutomations: 0
            }
          ],
          stages: [{ name: 'scan', queueDepth: 1, throughput: 20, avgLatencyMs: 300 }],
          signals: [
            { id: 'op-1', area: 'infra', title: 'Reduce release coupling', impact: 5, effort: 3, confidence: 0.8 }
          ]
        })
      }
    });
    fireEvent.click(screen.getByRole('button', { name: /Apply Payload/i }));

    const explainabilitySection = screen.getByTestId('explainability-section');
    expect(within(explainabilitySection).getByText(/Overall score 100\/100/i)).toBeInTheDocument();
    expect(within(explainabilitySection).getByText(/safe-1/i)).toBeInTheDocument();
    expect(within(explainabilitySection).getByText(/Reduce release coupling/i)).toBeInTheDocument();
  });
});

describe('Optimization sidebar layout', () => {
  it('renders key sidepanel sections and primary action', () => {
    render(<App />);

    expect(screen.getByText(/The Web Companion: Optimization Hub/i)).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /Run Full Audit/i })).toBeInTheDocument();
    expect(screen.getByText(/WCAG 2.1\/2.2 AA Accessibility Audit/i)).toBeInTheDocument();
    expect(screen.getByText(/SEO, GEO & AEO Performance/i)).toBeInTheDocument();
    expect(screen.getByText(/Security & Drupal Review/i)).toBeInTheDocument();
    expect(screen.getByText(/Page Performance Metrics/i)).toBeInTheDocument();
    expect(screen.queryByRole('tab')).not.toBeInTheDocument();
    expect(screen.getByRole('button', { name: /Run Full Audit/i })).toBeDisabled();
    expect(screen.getByText(/Page-specific and site-wide selection is unavailable/i)).toBeInTheDocument();
    expect(screen.getByRole('switch', { name: /Field and lab data selection unavailable/i })).toBeDisabled();
    expect(screen.getByText(/Answer Engine Optimization/i)).toBeInTheDocument();
    expect(screen.getByText(/General Site Security/i)).toBeInTheDocument();
    expect(screen.getByText(/Field Data/i)).toBeInTheDocument();
    expect(screen.getByText(/Lab Data/i)).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Team Lead' })).toBeInTheDocument();
    expect(screen.getByText(/Team leads: prioritize blocked PR hotspots/i)).toBeInTheDocument();
  });

  it('shows lead insights by default', () => {
    render(<App />);

    expect(screen.getByText(/Team Lead Focus/i)).toBeInTheDocument();
    expect(screen.getByText(/Top Commit Risks/i)).toBeInTheDocument();
  });

  it('shows quality pulse with role-specific recommendations', () => {
    render(<App />);

    const qualityPulseSection = screen.getByTestId('quality-pulse-section');

    expect(screen.getByText(/Quality Pulse/i)).toBeInTheDocument();
    expect(screen.getByText(/Action Routing/i)).toBeInTheDocument();
    expect(screen.getByText(/Lead Reviewer/i)).toBeInTheDocument();
    expect(screen.getByText(/Sprint now/i)).toBeInTheDocument();
    expect(screen.getByTestId('pulse-score')).toHaveTextContent(/\/100/);
    expect(
      within(qualityPulseSection).getByText(
        /Focus first on high-risk commit A-124 before expanding the next cycle\./i
      )
    ).toBeInTheDocument();
    expect(screen.getByTestId('pulse-top-bottleneck')).toBeInTheDocument();
  });

  it('does not show sample recommendations or action routes for an empty import', () => {
    render(<App />);

    fireEvent.change(screen.getByLabelText(/Telemetry payload JSON/i), {
      target: { value: JSON.stringify({ commits: [], stages: [], signals: [] }) }
    });
    fireEvent.click(screen.getByRole('button', { name: /Apply Payload/i }));

    const qualityPulseSection = screen.getByTestId('quality-pulse-section');
    expect(within(qualityPulseSection).queryByText(/A-124|trim flaky tests|sample window/i)).not.toBeInTheDocument();
    expect(within(qualityPulseSection).getByText('Awaiting telemetry')).toBeInTheDocument();
    expect(within(qualityPulseSection).getByTestId('pulse-score')).toHaveTextContent('Unavailable');
    expect(within(qualityPulseSection).getByTestId('pulse-top-bottleneck')).toHaveTextContent('Unavailable');
    expect(within(qualityPulseSection).queryByText(/high-risk commit/i)).not.toBeInTheDocument();
    expect(within(screen.getByTestId('explainability-section')).getByText(/Top Bottleneck: Unavailable — No bottleneck records are available for this import\./)).toBeInTheDocument();
    expect(within(screen.getByTestId('explainability-section')).getByText(/Top Risk Commit: Unavailable — No commit-risk records are available for this import\./)).toBeInTheDocument();
    const trendRiskSection = screen.getByTestId('trend-risk-section');
    expect(within(trendRiskSection).getByText(/PR Risk Trajectory: Unavailable — No commit-risk records are available\./)).toBeInTheDocument();
    expect(within(trendRiskSection).getByText(/Bottleneck Pressure: Unavailable — No bottleneck records are available\./)).toBeInTheDocument();

    fireEvent.click(screen.getByRole('button', { name: 'Security' }));
    expect(screen.getByText('No critical security signals are available.')).toBeInTheDocument();
    expect(screen.queryByText(/sample window/i)).not.toBeInTheDocument();

    fireEvent.change(screen.getByLabelText(/Telemetry payload JSON/i), {
      target: { value: JSON.stringify({ commits: [{ id: 'partial-import', files: 1, changedLines: 1, dependencyChanges: 0, testTouch: true, failedAutomations: 0 }], signals: [] }) }
    });
    fireEvent.click(screen.getByRole('button', { name: /Apply Payload/i }));
    expect(within(screen.getByTestId('quality-pulse-section')).getByTestId('pulse-top-bottleneck')).toHaveTextContent('Unavailable');
    expect(within(screen.getByTestId('explainability-section')).getByText(/Top Bottleneck: Unavailable — No bottleneck records are available for this import\./)).toBeInTheDocument();
  });

  it('derives visible action routing from populated imported telemetry', () => {
    render(<App />);
    fireEvent.change(screen.getByLabelText(/Telemetry payload JSON/i), {
      target: {
        value: JSON.stringify({
          commits: [{ id: 'route-commit', files: 4, changedLines: 520, dependencyChanges: 1, testTouch: false, failedAutomations: 1 }],
          stages: [{ name: 'route-review', queueDepth: 12, throughput: 4, avgLatencyMs: 2400 }],
          signals: [{ id: 'route-signal', area: 'build', title: 'Shorten route build', impact: 5, effort: 2, confidence: 0.9 }]
        })
      }
    });
    fireEvent.click(screen.getByRole('button', { name: /Apply Payload/i }));

    const qualityPulseSection = screen.getByTestId('quality-pulse-section');
    expect(within(qualityPulseSection).getByText('Current import')).toBeInTheDocument();
    expect(within(qualityPulseSection).getByText(/route-commit: high risk \(score .*Dependency risk/)).toBeInTheDocument();
    expect(within(qualityPulseSection).queryByText(/A-124|This week|Sprint now|sample window/i)).not.toBeInTheDocument();

    fireEvent.click(screen.getByRole('button', { name: 'Manager' }));
    expect(within(qualityPulseSection).getByText(/route-review: critical pressure/)).toBeInTheDocument();
    fireEvent.click(screen.getByRole('button', { name: 'Executive' }));
    expect(within(qualityPulseSection).getByText(/Shorten route build \(score/)).toBeInTheDocument();
    fireEvent.click(screen.getByRole('button', { name: 'Security' }));
    expect(within(qualityPulseSection).getByText(/route-commit: review dependency risk and automation failures/)).toBeInTheDocument();
  });

  it('keeps full security counts and routes when detail cards are display-limited', () => {
    render(<App />);
    fireEvent.change(screen.getByLabelText(/Telemetry payload JSON/i), {
      target: {
        value: JSON.stringify({
          limits: { risks: 1 },
          commits: [
            { id: 'visible-non-security', files: 24, changedLines: 900, dependencyChanges: 0, testTouch: true, failedAutomations: 0 },
            ...Array.from({ length: 5 }, (_, index) => ({
              id: `hidden-security-${index + 1}`,
              files: 1,
              changedLines: 8,
              dependencyChanges: 1,
              testTouch: true,
              failedAutomations: 0
            }))
          ],
          stages: [],
          signals: []
        })
      }
    });
    fireEvent.click(screen.getByRole('button', { name: /Apply Payload/i }));

    expect(screen.getByTestId('snapshot-risk-count')).toHaveTextContent('1');
    expect(screen.getByTestId('pulse-security-count')).toHaveTextContent('5');
    expect(screen.getAllByText(/visible-non-security/i).length).toBeGreaterThan(0);

    fireEvent.click(screen.getByRole('button', { name: 'Security' }));

    const qualityPulseSection = screen.getByTestId('quality-pulse-section');
    expect(
      within(qualityPulseSection).getByText(
        'Security-sensitive signals from hidden-security-1 should be reviewed before release.'
      )
    ).toBeInTheDocument();
    expect(within(qualityPulseSection).getByText('hidden-security-1: review dependency risk.')).toBeInTheDocument();
    expect(within(qualityPulseSection).getByText('hidden-security-2: review dependency risk.')).toBeInTheDocument();

    const securityDetailSection = screen.getByTestId('security-detail-section');
    const securityDetailList = within(securityDetailSection).getByRole('list');
    expect(within(securityDetailList).getAllByRole('listitem')).toHaveLength(3);
    expect(within(securityDetailSection).getByText('hidden-security-1: Dependency risk')).toBeInTheDocument();
    expect(within(securityDetailSection).getByText('hidden-security-3: Dependency risk')).toBeInTheDocument();
    expect(within(securityDetailSection).queryByText(/hidden-security-[45]/)).not.toBeInTheDocument();
    expect(within(securityDetailSection).getByTestId('security-signals-overflow')).toHaveTextContent(
      '2 more security signals not shown.'
    );
  });

  it('renders trend and risk visuals from the shared insight helper', () => {
    render(<App />);

    const trendRiskSection = screen.getByTestId('trend-risk-section');
    expect(screen.getByText(/Trend & Risk View/i)).toBeInTheDocument();
    expect(screen.getByText(/PR Risk Trajectory/i)).toBeInTheDocument();
    expect(screen.getByText(/Bottleneck Pressure/i)).toBeInTheDocument();
    expect(within(trendRiskSection).getByText(/A-124 score 100/i)).toBeInTheDocument();
  });

  it('renders explainability traces from the shared quality pulse', () => {
    render(<App />);
    const explainabilitySection = screen.getByTestId('explainability-section');
    expect(screen.getByText(/Explainability Traces/i)).toBeInTheDocument();
    expect(screen.getByText(/Trace Decomposition/i)).toBeInTheDocument();
    expect(within(explainabilitySection).getByText(/Score Decomposition/i)).toBeInTheDocument();
    expect(within(explainabilitySection).getByText(/Top Risk Commit/i)).toBeInTheDocument();
    expect(within(explainabilitySection).getByText(/Top Bottleneck/i)).toBeInTheDocument();
    expect(within(explainabilitySection).getByText(/Opportunity Signals/i)).toBeInTheDocument();
  });

  it('renders job observability metrics from the shared stage telemetry', () => {
    render(<App />);
    const jobObservabilitySection = screen.getByTestId('job-observability-section');
    expect(screen.getByText(/Job Observability/i)).toBeInTheDocument();
    expect(screen.getByTestId('job-observability-stage-count')).toHaveTextContent('3');
    expect(within(jobObservabilitySection).getByText(/review: queue 10, throughput 9, lag 1100ms/i)).toBeInTheDocument();
    expect(within(jobObservabilitySection).getByText(/build: queue 5, throughput 12, lag 850ms/i)).toBeInTheDocument();
    expect(within(jobObservabilitySection).getByText(/release: queue 4, throughput 18, lag 420ms/i)).toBeInTheDocument();
  });

  it('switches to manager insights when selected', () => {
    render(<App />);

    fireEvent.click(screen.getByRole('button', { name: 'Manager' }));
    const qualityPulseSection = screen.getByTestId('quality-pulse-section');

    expect(screen.getByText(/Manager Focus/i)).toBeInTheDocument();
    expect(screen.getByText(/Engineering Manager/i)).toBeInTheDocument();
    expect(screen.getByText(/This week/i)).toBeInTheDocument();
    expect(screen.getByText(/Bottleneck Radar/i)).toBeInTheDocument();
    expect(
      within(qualityPulseSection).getByText(
        /Critical stage\(s\): review need additional reviewer capacity\./i
      )
    ).toBeInTheDocument();
  });

  it('switches to executive insights when selected', () => {
    render(<App />);

    fireEvent.click(screen.getByRole('button', { name: 'Executive' }));
    const qualityPulseSection = screen.getByTestId('quality-pulse-section');
    const recommendationList = within(qualityPulseSection).getAllByRole('list')[0];

    expect(screen.getByText(/Executive Focus/i)).toBeInTheDocument();
    expect(screen.getByText(/Delivery Leadership/i)).toBeInTheDocument();
    expect(screen.getByText(/This month/i)).toBeInTheDocument();
    expect(screen.getByText(/Top Improvement Opportunities/i)).toBeInTheDocument();
    expect(within(recommendationList).getByText(/Top opportunity: Trim flaky tests/i)).toBeInTheDocument();
  });

  it('switches to security insights when selected', () => {
    render(<App />);

    fireEvent.click(screen.getByRole('button', { name: 'Security' }));

    expect(screen.getByText(/Security Focus/i)).toBeInTheDocument();
    expect(screen.getByText(/Security Operations/i)).toBeInTheDocument();
    expect(
      screen.getByText(/Security-sensitive signals from A-124 should be reviewed before release\./i)
    ).toBeInTheDocument();
    expect(screen.getByText(/Security-Weighted Commit Signals/i)).toBeInTheDocument();
    expect(screen.getAllByText(/Security-sensitive signals from/i).length).toBeGreaterThanOrEqual(1);
  });

  it('shows a default quality snapshot from sample data', () => {
    render(<App />);

    expect(screen.getByTestId('snapshot-risk-count')).toHaveTextContent('3');
    expect(screen.getByTestId('snapshot-bottleneck-count')).toHaveTextContent('1 critical, 2 high');
    expect(screen.getByTestId('snapshot-opportunity-count')).toHaveTextContent('3');
    expect(screen.getByText(/Team Lead Focus/i)).toBeInTheDocument();
  });

  it('renders admin bridge controls and shows runtime fallback message', async () => {
    render(<App />);

    expect(screen.getByText(/Admin Command Bridge/i)).toBeInTheDocument();
    fireEvent.click(screen.getByRole('button', { name: /Ingest Event/i }));
    await waitFor(() =>
      expect(screen.getByTestId('admin-bridge-result')).toHaveTextContent(/Tauri runtime not detected/i)
);
});

  it('renders admin bridge payload details when a Tauri shim is present', async () => {
    const invoke = vi.fn().mockImplementation(async (cmd, args) => ({ cmd, args }));
    setAdminInvokeForTesting(invoke);

    render(<App />);

    fireEvent.click(screen.getByRole('button', { name: /Ingest Event/i }));

    await waitFor(() => {
      expect(screen.getByTestId('admin-bridge-result')).toHaveTextContent(/OK ingest_event:/i);
      expect(screen.getByTestId('admin-bridge-result')).toHaveTextContent(/ui-bridge-001/i);
    });
    expect(invoke).toHaveBeenCalledWith('ingest_event', expect.any(Object));
  });

  it('renders the full admin command bridge surface for Tauri runtime parity', () => {
    render(<App />);

    expect(screen.getByRole('button', { name: /Ingest Event/i })).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /Promote Lifecycle/i })).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /Query Aggregates/i })).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /Committer Scores/i })).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /Rank PRs/i })).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /Update Scoring Weights/i })).toBeInTheDocument();
  });

  it('renders release baseline management controls', () => {
    render(<App />);

    expect(screen.getByTestId('baseline-management-section')).toBeInTheDocument();
    expect(screen.getByLabelText(/Baseline repo/i)).toHaveValue('repo-a');
    expect(screen.getByLabelText(/Baseline complexity/i)).toHaveValue('18.5');
    expect(screen.getByRole('button', { name: /Load Baseline/i })).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /Reseed Baseline/i })).toBeInTheDocument();
  });

  it('loads and reseeds release baseline values through the admin bridge', async () => {
    const invoke = vi.fn().mockImplementation(async (cmd, args) => {
      if (cmd === 'query_release_baseline') {
        return 9.75;
      }
      if (cmd === 'reseed_release_baseline') {
        return 12.25;
      }
      return { cmd, args };
    });
    setAdminInvokeForTesting(invoke);

    render(<App />);

    fireEvent.click(screen.getByRole('button', { name: /Load Baseline/i }));
    await waitFor(() =>
      expect(screen.getByTestId('baseline-management-result')).toHaveTextContent(
        /OK query_release_baseline: 9.75/i
      )
    );

    fireEvent.change(screen.getByLabelText(/Baseline complexity/i), { target: { value: '12.25' } });
    fireEvent.click(screen.getByRole('button', { name: /Reseed Baseline/i }));
    await waitFor(() =>
      expect(screen.getByTestId('baseline-management-result')).toHaveTextContent(
        /OK reseed_release_baseline: 12.25/i
      )
    );

    expect(invoke).toHaveBeenCalledWith(
      'query_release_baseline',
      expect.objectContaining({ repoName: 'repo-a', token: 'alice:admin' })
    );
    expect(invoke).toHaveBeenCalledWith(
      'reseed_release_baseline',
      expect.objectContaining({
        repoName: 'repo-a',
        token: 'alice:admin',
        baselineComplexity: 12.25
      })
    );
  });

  it('applies custom payload JSON to refresh risk/bottleneck/opportunity outputs', () => {
    render(<App />);

    const jsonInput = screen.getByLabelText(/Telemetry payload JSON/i);
    const customPayload = {
      commits: [
        { id: 'custom-999', files: 25, changedLines: 800, dependencyChanges: 1, testTouch: false, failedAutomations: 1 }
      ],
      stages: [{ name: 'build', queueDepth: 8, throughput: 8, avgLatencyMs: 1500 }],
      signals: [
        { id: 'custom-op-1', area: 'infra', title: 'Cache invalidation map', impact: 5, effort: 2, confidence: 0.9 }
      ]
    };

    fireEvent.change(jsonInput, { target: { value: JSON.stringify(customPayload) } });
    fireEvent.click(screen.getByRole('button', { name: /Apply Payload/i }));

    expect(screen.getByTestId('snapshot-risk-count')).toHaveTextContent('1');
    expect(screen.getByTestId('snapshot-bottleneck-count')).toHaveTextContent('0 critical, 1 high');
    expect(screen.getByTestId('snapshot-opportunity-count')).toHaveTextContent('1');
    expect(screen.getAllByText(/custom-999 score 100/i).length).toBeGreaterThanOrEqual(1);
  });

  it('shows payload validation errors for malformed JSON', () => {
    render(<App />);

    fireEvent.change(screen.getByLabelText(/Telemetry payload JSON/i), { target: { value: 'oops: bad json' } });
    fireEvent.click(screen.getByRole('button', { name: /Apply Payload/i }));

    expect(screen.getByRole('alert')).toHaveTextContent(/Invalid telemetry payload/i);
  });

  it('resets to sample data when payload input is cleared', () => {
    render(<App />);

    fireEvent.change(screen.getByLabelText(/Telemetry payload JSON/i), {
      target: {
        value:
          '{"commits":[{"id":"custom-1","files":2,"changedLines":60,"dependencyChanges":0,"testTouch":true,"failedAutomations":0}]}'
      }
    });
    fireEvent.click(screen.getByRole('button', { name: /Apply Payload/i }));
    expect(screen.getByTestId('snapshot-risk-count')).toHaveTextContent('1');

    fireEvent.click(screen.getByRole('button', { name: /Reset to Sample/i }));
    expect(screen.getByTestId('snapshot-risk-count')).toHaveTextContent('3');
    expect(
      screen.getByText(/Focus first on high-risk commit A-124 before expanding the next cycle\./i)
    ).toBeInTheDocument();
  });

  it('does not reset to sample data when Apply is blank and keeps the last good view on invalid schema', () => {
    render(<App />);
    const input = screen.getByLabelText(/Telemetry payload JSON/i);
    const lastGoodPayload = {
      commits: [{ id: 'last-good', files: 2, changedLines: 10, dependencyChanges: 0, testTouch: true, failedAutomations: 0 }],
      stages: [{ name: 'review', queueDepth: 1, throughput: 12, avgLatencyMs: 200 }],
      signals: []
    };

    fireEvent.change(input, { target: { value: JSON.stringify(lastGoodPayload) } });
    fireEvent.click(screen.getByRole('button', { name: /Apply Payload/i }));
    expect(screen.getByTestId('snapshot-risk-count')).toHaveTextContent('1');
    expect(screen.getByTestId('telemetry-data-state')).toHaveTextContent(/Imported telemetry/i);

    fireEvent.change(input, { target: { value: '   ' } });
    fireEvent.click(screen.getByRole('button', { name: /Apply Payload/i }));
    expect(screen.getByTestId('snapshot-risk-count')).toHaveTextContent('1');
    expect(screen.getByRole('alert')).toHaveTextContent(/Enter a telemetry payload/i);

    fireEvent.change(input, {
      target: {
        value: JSON.stringify({
          commits: lastGoodPayload.commits,
          stages: [{ name: { label: 'review' }, queueDepth: 1, throughput: 12, avgLatencyMs: 200 }],
          signals: []
        })
      }
    });
    fireEvent.click(screen.getByRole('button', { name: /Apply Payload/i }));
    expect(screen.getByTestId('snapshot-risk-count')).toHaveTextContent('1');
    expect(screen.getAllByText(/last-good score/i).length).toBeGreaterThan(0);
    expect(screen.getByRole('alert')).toHaveTextContent(/stages\[0\]\.name/);
    expect(input).toHaveAttribute('aria-invalid', 'true');
    expect(input).toHaveAttribute('aria-describedby', 'telemetry-payload-error');
  });

  it('shows an explicit empty imported state without substituting sample telemetry', () => {
    render(<App />);
    const input = screen.getByLabelText(/Telemetry payload JSON/i);
    fireEvent.change(input, { target: { value: JSON.stringify({ commits: [], stages: [], signals: [] }) } });
    fireEvent.click(screen.getByRole('button', { name: /Apply Payload/i }));

    expect(screen.getByTestId('telemetry-data-state')).toHaveTextContent(/no telemetry records/i);
    expect(screen.getByTestId('snapshot-risk-count')).toHaveTextContent('0');
    expect(screen.getByTestId('snapshot-bottleneck-count')).toHaveTextContent('0 critical, 0 high');
    expect(screen.getByTestId('snapshot-opportunity-count')).toHaveTextContent('0');
  });

  it('does not call a partial explicitly empty payload a complete no-records import', () => {
    render(<App />);
    const input = screen.getByLabelText(/Telemetry payload JSON/i);
    fireEvent.change(input, { target: { value: JSON.stringify({ commits: [] }) } });
    fireEvent.click(screen.getByRole('button', { name: /Apply Payload/i }));

    expect(screen.getByTestId('telemetry-data-state')).toHaveTextContent(/some telemetry collections are omitted/i);
    expect(screen.getByTestId('telemetry-data-state')).not.toHaveTextContent(/contains no telemetry records/i);
    expect(screen.getByTestId('snapshot-risk-count')).toHaveTextContent('0');
    expect(screen.getByTestId('snapshot-bottleneck-count')).toHaveTextContent('0 critical, 0 high');
    expect(screen.getByTestId('snapshot-opportunity-count')).toHaveTextContent('0');
  });

  it('distinguishes missing fields and reset sample data from imported data', () => {
    render(<App />);
    expect(screen.getByTestId('telemetry-data-state')).toHaveTextContent(/Sample telemetry/i);
    const input = screen.getByLabelText(/Telemetry payload JSON/i);
    fireEvent.change(input, { target: { value: '{}' } });
    fireEvent.click(screen.getByRole('button', { name: /Apply Payload/i }));
    expect(screen.getByTestId('telemetry-data-state')).toHaveTextContent(/fields are missing/i);
    expect(screen.getByTestId('snapshot-risk-count')).toHaveTextContent('0');
    expect(screen.getByTestId('snapshot-bottleneck-count')).toHaveTextContent('0 critical, 0 high');
    expect(screen.getByTestId('snapshot-opportunity-count')).toHaveTextContent('0');
    fireEvent.click(screen.getByRole('button', { name: /Reset to Sample/i }));
    expect(screen.getByTestId('telemetry-data-state')).toHaveTextContent(/Sample telemetry/i);
  });

  it(
    'applies payload envelope with nested limits and removes security matches',
    () => {
      render(<App />);

      const envelopePayload = {
        payload: {
          commits: [
            { id: 'safe-001', files: 1, changedLines: 40, dependencyChanges: 0, testTouch: true, failedAutomations: 0 }
          ],
          stages: [{ name: 'review', queueDepth: 1, throughput: 20, avgLatencyMs: 300 }],
          signals: [{ id: 'op-1', area: 'tests', title: 'Trim flaky tests', impact: 4, effort: 6, confidence: 0.7 }]
        },
        limits: {
          risks: 1,
          opportunities: 1,
          severityThreshold: 5,
          latencyP95Ms: 1000
        }
      };

      fireEvent.change(screen.getByLabelText(/Telemetry payload JSON/i), {
        target: { value: JSON.stringify(envelopePayload) }
      });
      fireEvent.click(screen.getByRole('button', { name: /Apply Payload/i }));

      expect(screen.getByTestId('snapshot-risk-count')).toHaveTextContent('1');
      expect(screen.getByTestId('snapshot-opportunity-count')).toHaveTextContent('1');

      fireEvent.click(screen.getByRole('button', { name: 'Security' }));
      expect(screen.getAllByText(/No critical security signals are available/i).length).toBeGreaterThanOrEqual(1);
    },
    10000
  );

  it('updates quality pulse recommendations for security-empty payloads', () => {
    render(<App />);

    const payloadWithoutSignals = {
      commits: [
        {
          id: 'dry-1',
          files: 22,
          changedLines: 780,
          dependencyChanges: 1,
          testTouch: false,
          failedAutomations: 2
        }
      ],
      stages: [{ name: 'review', queueDepth: 12, throughput: 4, avgLatencyMs: 1500 }],
      signals: []
    };

    fireEvent.change(screen.getByLabelText(/Telemetry payload JSON/i), {
      target: { value: JSON.stringify(payloadWithoutSignals) }
    });
    fireEvent.click(screen.getByRole('button', { name: /Apply Payload/i }));

    expect(
      screen.getByText(/Focus first on high-risk commit dry-1 before expanding the next cycle\./i)
    ).toBeInTheDocument();
    fireEvent.click(screen.getByRole('button', { name: 'Security' }));
    expect(
      screen.getByText(/Security-sensitive signals from dry-1 should be reviewed before release\./i)
    ).toBeInTheDocument();
  });

  it('keeps static audit sections labeled as examples after telemetry import', () => {
    render(<App />);
    const provenanceLabels = ['accessibility', 'seo', 'security', 'performance'].map((section) =>
      screen.getByTestId(`provenance-${section}`)
    );
    for (const label of provenanceLabels) {
      expect(label).toHaveTextContent(/static example content; not derived from imported telemetry/i);
    }

    const input = screen.getByLabelText(/Telemetry payload JSON/i);
    fireEvent.change(input, {
      target: { value: JSON.stringify({ commits: [], stages: [], signals: [] }) }
    });
    fireEvent.click(screen.getByRole('button', { name: /Apply Payload/i }));

    for (const label of provenanceLabels) {
      expect(label).toBeVisible();
      expect(label).toHaveTextContent(/static example content; not derived from imported telemetry/i);
    }
  });

  it('uses configured latency severity in Job Observability', () => {
    render(<App />);
    fireEvent.change(screen.getByLabelText(/Telemetry payload JSON/i), {
      target: {
        value: JSON.stringify({
          payload: {
            commits: [{ id: 'safe-commit', files: 1, changedLines: 8, dependencyChanges: 0, testTouch: true, failedAutomations: 0 }],
            stages: [{ name: 'slow-stage', queueDepth: 0, throughput: 10, avgLatencyMs: 750 }],
            signals: []
          },
          limits: { latencyP95Ms: 200 }
        })
      }
    });
    fireEvent.click(screen.getByRole('button', { name: /Apply Payload/i }));

    const observability = screen.getByTestId('job-observability-section');
    expect(within(observability).getByText('bad')).toBeVisible();
    expect(within(observability).getByText(/slow-stage: queue 0, throughput 10, lag 750ms/i)).toBeVisible();
    fireEvent.click(screen.getByRole('button', { name: 'Manager' }));
    expect(screen.getByText(/slow-stage \(critical\) impact 5/i)).toBeVisible();
  });
});
