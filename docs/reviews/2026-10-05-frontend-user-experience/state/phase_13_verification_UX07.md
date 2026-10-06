# Phase 13 — Targeted verification: UX-07

Verification agent: Code Reviewer (gpt-6-astra substitute; Opus unavailable). Standard tier. Static inspection only.

**[VR_CONFIRMED] — Opportunity signals do not contribute to the displayed overall score, but the explanation explicitly says they do.**

- `ui/src/dashboard-explainability.ts:40` emits, whenever `pulse.opportunityCount > 0`: “{count} opportunity signal(s) are boosting the score.”
- `ui/src/insight-engine.ts:146` separately transforms commits into `commitRiskCards`, stages into `bottlenecks`, and signals into `opportunities`. Opportunity values do not modify commit or bottleneck classifications.
- `ui/src/domain/quality-pulse.ts:164-173` derives risk buckets only from commit risk cards and bottleneck buckets only from bottlenecks. At line 173, `overallScore` is `Math.max(45, 100 - riskBuckets.high * 10 - bottleneckBuckets.critical * 15 - bottleneckBuckets.high * 5)`. `opportunityCount` is returned separately.
- `ui/src/App.tsx:165` passes the Quality Pulse to `buildExplainabilityTraces`; `:352` displays `qualityPulse.overallScore`; `:407` renders the explanation detail.

Holding commits and stages constant while changing opportunity count or values cannot boost this score, despite the displayed statement.

**Limit:** Source inspection only; no tests, execution, or edits.
