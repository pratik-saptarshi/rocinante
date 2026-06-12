import { fireEvent, render, screen, within } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import App from './App';

describe('dashboard explainability panel', () => {
  it('renders deterministic decomposition traces from sample insights', () => {
    render(<App />);

    const explainabilitySection = screen.getByTestId('explainability-section');

    expect(within(explainabilitySection).getByText(/Explainability Panel/i)).toBeInTheDocument();
    expect(within(explainabilitySection).getByText(/Score Decomposition/i)).toBeInTheDocument();
    expect(within(explainabilitySection).getByText(/Top Risk Commit/i)).toBeInTheDocument();
    expect(within(explainabilitySection).getByText(/Top Bottleneck/i)).toBeInTheDocument();
    expect(within(explainabilitySection).getByText(/Opportunity Lift/i)).toBeInTheDocument();
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

describe('dashboard trend and risk view', () => {
  it('renders deterministic trend and risk cards from sample insights', () => {
    render(<App />);

    const trendRiskSection = screen.getByTestId('trend-risk-section');

    expect(within(trendRiskSection).getByText(/Trend & Risk View/i)).toBeInTheDocument();
    expect(within(trendRiskSection).getByText(/Risk Trend/i)).toBeInTheDocument();
    expect(within(trendRiskSection).getByText(/2 high-risk commit\(s\) out of 3/i)).toBeInTheDocument();
    expect(within(trendRiskSection).getByText(/1 critical \/ 2 high bottleneck\(s\)/i)).toBeInTheDocument();
    expect(within(trendRiskSection).getByText(/3 actionable opportunity\(s\)/i)).toBeInTheDocument();
  });

  it('updates trend and risk cards when payload changes', () => {
    render(<App />);

    fireEvent.change(screen.getByLabelText(/Telemetry payload JSON/i), {
      target: {
        value: JSON.stringify({
          commits: [
            {
              id: 'custom-999',
              files: 25,
              changedLines: 800,
              dependencyChanges: 1,
              testTouch: false,
              failedAutomations: 1
            }
          ],
          stages: [{ name: 'build', queueDepth: 9, throughput: 4, avgLatencyMs: 1400 }],
          signals: [
            { id: 'custom-op-1', area: 'infra', title: 'Reduce release coupling', impact: 5, effort: 3, confidence: 0.8 }
          ]
        })
      }
    });
    fireEvent.click(screen.getByRole('button', { name: /Apply Payload/i }));

    const trendRiskSection = screen.getByTestId('trend-risk-section');
    expect(within(trendRiskSection).getByText(/1 high-risk commit\(s\) out of 1/i)).toBeInTheDocument();
    expect(within(trendRiskSection).getByText(/0 medium-risk commit\(s\)/i)).toBeInTheDocument();
    expect(within(trendRiskSection).getByText(/build/i)).toBeInTheDocument();
  });
});
