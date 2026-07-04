import { fireEvent, render, screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';
import { AdminBridgePanel } from './admin-bridge-panel';

describe('AdminBridgePanel', () => {
  it('renders the bridge controls from the shared command catalog', () => {
    const onAdminTokenChange = vi.fn();
    const onRunAdminCommand = vi.fn();

    render(
      <AdminBridgePanel
        adminToken="alice:admin"
        adminResult="No admin command executed yet."
        onAdminTokenChange={onAdminTokenChange}
        onRunAdminCommand={onRunAdminCommand}
      />
    );

    expect(screen.getByLabelText('Admin token')).toHaveValue('alice:admin');
    expect(screen.getByRole('button', { name: 'Ingest Event' })).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Update Scoring Weights' })).toBeInTheDocument();
    expect(screen.getByTestId('admin-bridge-result')).toHaveTextContent('No admin command executed yet.');
  });

  it('emits token changes and command clicks through callbacks', () => {
    const onAdminTokenChange = vi.fn();
    const onRunAdminCommand = vi.fn();

    render(
      <AdminBridgePanel
        adminToken="alice:admin"
        adminResult="ready"
        onAdminTokenChange={onAdminTokenChange}
        onRunAdminCommand={onRunAdminCommand}
      />
    );

    fireEvent.change(screen.getByLabelText('Admin token'), { target: { value: 'bob:admin' } });
    fireEvent.click(screen.getByRole('button', { name: 'Query Aggregates' }));

    expect(onAdminTokenChange).toHaveBeenCalledWith('bob:admin');
    expect(onRunAdminCommand).toHaveBeenCalledWith('query_aggregates');
  });
});
