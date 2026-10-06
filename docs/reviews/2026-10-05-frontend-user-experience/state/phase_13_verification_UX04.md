# Phase 13 — Targeted verification: malformed telemetry

Persona: Code Reviewer; Standard tier. Reviewer: gpt-6-astra substitute. [VR_CONFIRMED]

The reviewer traced a valid payload with stages:[{name:{},queueDepth:0,throughput:1,avgLatencyMs:1}]. App.tsx:198 parses JSON; :202 passes a TypeScript assertion but no runtime validator; dashboard-contract.ts:33 returns the payload. insight-engine.ts:150 accepts the nonempty stage array and :126 preserves stage.name. quality-pulse.ts:178 returns that object as topBottleneckName. App.tsx:362 passes it into MetricItem, whose Typography renders value directly at :145.

The catch surrounds parse and synchronous insight construction, but the React reconciliation happens later. main.tsx:15-21 has no error boundary. Installed React rejects an object child. Therefore the exact invalid-child failure path is source-confirmed; no runtime reproduction was performed, and the browser-visible failure screen is not claimed.
