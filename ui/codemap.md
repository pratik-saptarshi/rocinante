# ui/

## Responsibility
Frontend application package for the dashboard, insight visualization, and
desktop-command bridge. Also owns the browser validation harness.

## Design
Built as a Vite + React + TypeScript package with MUI-based layout and a
separation between:
- dashboard composition (`src/App.tsx`)
- extracted bridge shell (`src/admin-bridge-panel.tsx`)
- deterministic view-model builders (`src/insight-engine.ts`, `src/domain/`)
- contract adapters (`src/dashboard-contract.ts`, `src/admin-bridge-contract.ts`)
- runtime bridge (`src/tauri-admin.ts`)

## Flow
1. `src/main.tsx` boots the React tree and applies the theme.
2. `src/App.tsx` renders the dashboard shell and binds local state.
3. `src/admin-bridge-panel.tsx` isolates command dispatch controls from the
   dashboard shell.
4. Payload contracts normalize incoming JSON envelopes into insight inputs.
5. Admin bridge helpers build typed command payloads and dispatch them when
   the desktop runtime is present.
6. Browser tests exercise the same shell and fallback behavior headlessly.

## Integration
- Depends on `src-tauri/` through the Tauri invoke bridge.
- Validated by Vitest and Playwright suites under `ui/src/`, `ui/src/test/`,
  and `ui/e2e/`.
