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
