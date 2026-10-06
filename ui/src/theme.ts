import { createTheme } from '@mui/material';

export const appTheme = createTheme({
  palette: {
    mode: 'light',
    primary: { main: '#3659a7' },
    background: { default: '#f4f4f4', paper: '#ffffff' }
  },
  shape: { borderRadius: 8 }
});
