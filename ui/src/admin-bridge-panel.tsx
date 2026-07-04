import { Button, Stack, TextField, Typography } from '@mui/material';
import { ADMIN_BRIDGE_ACTIONS } from './admin-bridge-contract';
import type { AdminBridgeCommand } from './tauri-admin';

export interface AdminBridgePanelProps {
  adminToken: string;
  adminResult: string;
  onAdminTokenChange: (token: string) => void;
  onRunAdminCommand: (command: AdminBridgeCommand) => void;
}

export function AdminBridgePanel({
  adminToken,
  adminResult,
  onAdminTokenChange,
  onRunAdminCommand
}: AdminBridgePanelProps) {
  return (
    <Stack spacing={1}>
      <TextField
        fullWidth
        label="Admin token"
        size="small"
        value={adminToken}
        onChange={(event) => onAdminTokenChange(event.target.value)}
      />
      <Stack direction="row" spacing={1} sx={{ flexWrap: 'wrap' }}>
        {ADMIN_BRIDGE_ACTIONS.map(({ command, label }) => (
          <Button key={command} size="small" variant="outlined" onClick={() => onRunAdminCommand(command)}>
            {label}
          </Button>
        ))}
      </Stack>
      <Typography variant="caption" data-testid="admin-bridge-result">
        {adminResult}
      </Typography>
    </Stack>
  );
}
