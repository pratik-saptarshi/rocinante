# Phase 8 — Completeness audit

Auditor: gpt-6-astra substitute (Opus unavailable). Read-only static source review; no tests, screenshots, or app launch.

1. **P2 — Malformed telemetry can crash the preview after Apply.** App.tsx:197 parses JSON and casts without schema validation. An object-valued stage.name survives insight-engine.ts:126, becomes topBottleneckName at domain/quality-pulse.ts:178, and reaches a direct React child at App.tsx:360 and :145. Render occurs outside the Apply handler catch. main.tsx:15 has no error boundary. A syntactically valid payload can remove the usable dashboard. Remedy: validate element types and finite ranges before replacing state; retain previous valid insights and identify bad fields. Confidence high, source-derived.

2. **P2 — Healthy populated telemetry still produces false critical recommendations and unrelated routes.** domain/quality-pulse.ts:98-107 unconditionally emits two red manager warnings and uses “review” if no critical/high stage exists. buildRoutes at :138 onward does not receive telemetry and hardcodes A-124, review, and sample opportunities. Routes are displayed beside calculated recommendations in App.tsx:373. Remedy: derive routes/recommendations from active data and add healthy state. Confidence high.

3. **P2 — Stage severity contradicts across sections.** App.tsx:178-187 uses fixed 1,000/2,000ms thresholds; insight-engine.ts:107-131 uses supplied latencyP95Ms and 3x thereof. For queue depth 0, latency 500, configured ceiling 100, Job Observability says good while bottleneck model says critical. Remedy: one shared severity classifier or clearly named distinct criteria. Confidence high.

4. **P2 — Explainability claims a score contribution that does not exist.** dashboard-explainability.ts:35-42 says opportunity signals “are boosting the score”; domain/quality-pulse.ts:173 computes score only from high-risk commits and critical/high bottlenecks. Remedy: derive explanation from actual components or describe opportunities separately. Confidence high.

5. **P2 — Primary button text contrast is below AA for normal-size text.** main.tsx:9 sets #5577cc; App.tsx:480 uses a small contained button. Installed MUI selects white contrast text and uses primary as background. Static calculation is 4.288:1. W3C WCAG 2.2 SC 1.4.3 requires 4.5:1 for normal text; [official criterion](https://www.w3.org/TR/wcag/#contrast-minimum). Remedy: darken the theme primary token and validate each interaction state. Source/color confidence high; no rendered inspection.
