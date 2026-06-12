import { fireEvent, render, screen, within } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import App from './App';

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
