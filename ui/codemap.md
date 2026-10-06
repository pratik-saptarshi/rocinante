# ui/

## Responsibility

React/Vite browser preview for dashboard, insight visualization, and the
headless UI validation harness. It is not the supported native desktop host;
desktop operations run in the Rust eframe/winit shell.

## Design

- `src/App.tsx` composes the dashboard and local state.
- `src/admin-bridge-panel.tsx` contains preview controls for admin payloads.
- `src/insight-engine.ts` and `src/domain/` build deterministic view models.
- `src/dashboard-contract.ts` and `src/admin-bridge-contract.ts` normalize
  dashboard and command compatibility payloads.
- `src/tauri-admin.ts` retains a browser/runtime adapter for compatibility
  tests. It does not imply a packaged Tauri runtime.

## Flow

1. `src/main.tsx` boots the React tree and theme.
2. `src/App.tsx` renders the preview dashboard.
3. Contract helpers validate and normalize JSON payloads.
4. Admin preview helpers dispatch only when a compatible runtime is present;
   supported desktop behavior is implemented through the native Rust shell.
5. Vitest and Playwright validate browser behavior and fallback handling.

## Integration and Validation

- The Rust service workspace is authoritative for desktop scans, storage, and
  admin operations; this UI remains a preview and test surface.
- `ui/package.json` pins pnpm `12.9.1`.
- CI runs pinned install, typecheck, unit tests, and production build in `ui-quality`; `UI Playwright` installs Chromium, runs `pnpm run test:e2e`, and uploads the HTML report and retry traces.
- Browser suites live under `ui/src/`, `ui/src/test/`, and `ui/e2e/`.
- Payload parsing validates JSON and nested telemetry before replacing the last-good view; explicit empty collections remain empty, and omitted collections stay empty in imported partial payloads. Sample telemetry is limited to the initial/reset sample view.
