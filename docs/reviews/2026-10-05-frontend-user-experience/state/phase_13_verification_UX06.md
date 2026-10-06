# Phase 13 — Targeted verification: stage severity thresholds

Persona: Code Reviewer; Standard tier. Reviewer: gpt-6-astra substitute. [VR_CONFIRMED]

Concrete valid input: stage review, queueDepth 0, throughput 10, avgLatencyMs 750; limits.latencyP95Ms 200.

App.tsx:197-202 parses and passes readLimits. dashboard-contract.ts:10-25 accepts latencyP95Ms=200; :28-33 preserves stage payload. insight-engine.ts:150, :154-158 uses supplied stage and ceiling; stageToBottleneck at :107-114 reports critical because 750 >= 3*200. Job Observability independently uses fixed thresholds at App.tsx:178-187, so 0 queue / 750ms is good. App.tsx:441-451 renders that label; Manager Radar at :595-607 renders critical.

The discrepancy is source-confirmed. Whether two policies were intended is undocumented; the user interface does not explain separate criteria. Static trace only; no runtime validation.
