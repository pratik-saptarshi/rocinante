# Phase 7 — Blind final assessment: usability

Independent browser-preview UX readiness score: 4/10. This is an expert judgment, not a measured usability result or native-host assessment.

| Priority | Finding | Severity | Evidence |
|---|---|---|---|
| 1 | Empty or partial imports silently introduce sample data | P2 | ui/src/insight-engine.ts:146-153 substitutes samples for empty/absent arrays; ui/src/App.tsx:190-203 applies results without source feedback; empty editor also switches to samples. |
| 2 | Displayed results lack clear provenance | P2 | ui/src/App.tsx:704, :737-744, :763-768, :787-788 renders fixed claims alongside imported insights; ui/src/dashboard-content.ts:35-55 supplies literal findings. |
| 3 | Enabled controls imply unsupported operations | P2 | Run Full Audit no action at ui/src/App.tsx:705-713; SEO tabs change only heading :733-744; Field/Lab changes only switch state :154, :787-797. |

Withdrawals: U4 becomes P3 because top role recommendations exist at App.tsx:365-389. U5 is conditional P3 with no overflow defect established. Empty Apply replaces displayed in-memory insights; it does not delete persisted data.

Verdict: Useful as a developer demonstration, but it needs honest control availability, explicit data provenance, and faithful empty-data handling before an unfamiliar user can reliably interpret and operate it.
