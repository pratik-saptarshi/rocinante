# ui/src/

## Responsibility
React dashboard implementation, contract normalization, and deterministic
insight/view-model generation.

## Design
The folder uses a composition-first pattern:
- `App.tsx` is the shell and state owner.
- `admin-bridge-panel.tsx` isolates the command button cluster and result
  display from the shell.
- `dashboard-content.ts` centralizes display copy and reusable finding groups.
- `dashboard-contract.ts` normalizes payload envelopes and render limits.
- `dashboard-explainability.ts` and `dashboard-visuals.ts` derive presentation
  cards from the insight model.
- `insight-engine.ts` transforms telemetry into domain-shaped view models.
- `admin-bridge-contract.ts` and `tauri-admin.ts` define the command bridge.
- `domain/` contains pure helpers for risk, opportunity, and quality logic.

## Flow
1. `main.tsx` mounts `App`.
2. `App.tsx` loads sample insights and displays the dashboard shell.
3. JSON payload input passes through `readPayload` and `readLimits`.
4. Insight engine produces commit risk, bottleneck, and opportunity cards.
5. Contract helpers build admin payloads and invoke the desktop bridge.

## Integration
- Consumed by `ui/e2e/app.spec.ts` and the `ui/src/*.test.tsx` / `ui/src/*.test.ts`
  suites.
- Shared browser test setup lives in `ui/src/test/`.
- Calls into `src-tauri` only through `tauri-admin.ts`.
