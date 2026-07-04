# ui/src/domain/

## Responsibility
Pure domain helpers for classifying quality pulse data into risks,
opportunities, and bottleneck summaries.

## Design
Functional, side-effect-free utilities:
- `risk.ts`: commit risk scoring and classification.
- `opportunity.ts`: opportunity ranking heuristics.
- `bottleneck.ts`: queue/latency bottleneck detection.
- `quality-pulse.ts`: stakeholder-specific action routes and rollups.

## Flow
1. `insight-engine.ts` produces the base insight model.
2. Domain helpers reshape the model into audience-specific guidance.
3. `App.tsx` renders the derived recommendations and routing copy.

## Integration
- Imported by `ui/src/insight-engine.ts` and `ui/src/App.tsx`.
- Intentionally isolated from the runtime bridge and browser test harness.
