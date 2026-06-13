import { fireEvent, render, screen, waitFor, within } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import App from './App';

describe('Optimization sidebar layout', () => {
  it('renders key sidepanel sections and primary action', () => {
    render(<App />);

    expect(
      screen.getByText(/The Web Companion: Optimization Hub/i)
    ).toBeInTheDocument();

    expect(
      screen.getByRole('button', { name: /Run Full Audit/i })
    ).toBeInTheDocument();

    expect(
      screen.getByText(/WCAG 2.1\/2.2 AA Accessibility Audit/i)
    ).toBeInTheDocument();

    expect(
      screen.getByText(/SEO, GEO & AEO Performance/i)
    ).toBeInTheDocument();

    expect(
      screen.getByText(/Security & Drupal Review/i)
    ).toBeInTheDocument();

    expect(
      screen.getByText(/Page Performance Metrics/i)
    ).toBeInTheDocument();

    expect(screen.getByRole('tab', { name: /Current Page/i })).toBeInTheDocument();
    expect(screen.getByRole('tab', { name: /Site-Wide/i })).toBeInTheDocument();
    expect(screen.getByLabelText(/Run Full Audit/i)).toBeInTheDocument();
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

  it('renders trend and risk visuals from the shared insight helper', () => {
    render(<App />);

    const trendRiskSection = screen.getByTestId('trend-risk-section');
    expect(screen.getByText(/Trend & Risk View/i)).toBeInTheDocument();
    expect(screen.getByText(/PR Risk Trajectory/i)).toBeInTheDocument();
    expect(screen.getByText(/Bottleneck Pressure/i)).toBeInTheDocument();
    expect(within(trendRiskSection).getByText(/A-124 score 100/i)).toBeInTheDocument();
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

  it('renders the full admin command bridge surface for Tauri runtime parity', () => {
    render(<App />);

    expect(screen.getByRole('button', { name: /Ingest Event/i })).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /Promote Lifecycle/i })).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /Query Aggregates/i })).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /Committer Scores/i })).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /Rank PRs/i })).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /Update Scoring Weights/i })).toBeInTheDocument();
  });

  it('applies custom payload JSON to refresh risk/bottleneck/opportunity outputs', () => {
    render(<App />);

    const jsonInput = screen.getByLabelText(/Telemetry payload JSON/i);
    const customPayload = {
      commits: [{ id: 'custom-999', files: 25, changedLines: 800, dependencyChanges: 1, testTouch: false, failedAutomations: 1 }],
      stages: [{ name: 'build', queueDepth: 8, throughput: 8, avgLatencyMs: 1500 }],
      signals: [{ id: 'custom-op-1', area: 'infra', title: 'Cache invalidation map', impact: 5, effort: 2, confidence: 0.9 }]
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

    expect(screen.getByRole('alert')).toHaveTextContent(/Invalid JSON payload/i);
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

  it(
    'falls back to sample data when payload field is intentionally emptied',
    () => {
      render(<App />);

      fireEvent.change(screen.getByLabelText(/Telemetry payload JSON/i), {
        target: {
          value: JSON.stringify({
            commits: [{ id: 'temp', files: 2, changedLines: 10, dependencyChanges: 0, testTouch: true, failedAutomations: 0 }]
          })
        }
      });
      fireEvent.click(screen.getByRole('button', { name: /Apply Payload/i }));
      expect(screen.getByTestId('snapshot-risk-count')).toHaveTextContent('1');

      fireEvent.change(screen.getByLabelText(/Telemetry payload JSON/i), { target: { value: '   ' } });
      fireEvent.click(screen.getByRole('button', { name: /Apply Payload/i }));
      expect(screen.getByTestId('snapshot-risk-count')).toHaveTextContent('3');
    },
    10000
  );

  it(
    'applies payload envelope with nested limits and removes security matches',
    () => {
      render(<App />);

      const envelopePayload = {
        payload: {
          commits: [{ id: 'safe-001', files: 1, changedLines: 40, dependencyChanges: 0, testTouch: true, failedAutomations: 0 }],
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

      fireEvent.change(screen.getByLabelText(/Telemetry payload JSON/i), { target: { value: JSON.stringify(envelopePayload) } });
      fireEvent.click(screen.getByRole('button', { name: /Apply Payload/i }));

      expect(screen.getByTestId('snapshot-risk-count')).toHaveTextContent('1');
      expect(screen.getByTestId('snapshot-opportunity-count')).toHaveTextContent('1');

      fireEvent.click(screen.getByRole('button', { name: 'Security' }));
      expect(screen.getAllByText(/No critical security signals in sample window/i).length).toBeGreaterThanOrEqual(1);
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
});
