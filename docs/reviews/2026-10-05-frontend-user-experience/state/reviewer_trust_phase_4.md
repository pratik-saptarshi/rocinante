# Phase 4 — Private reflection: data trust

| Finding | Confidence | Decision |
|---|---|---|
| T1 — Empty telemetry becomes sample activity | High | Retain P2; strongest. insight-engine.ts:147, :150, :151 replace empty arrays with seeds; App.tsx:202 applies the result. Distinguish intentional initial sample mode from explicit empty arrays. |
| T2 — Undisclosed synthetic audit/security assertions | High for behavior; Medium for impact | Retain P2. Fixed findings at dashboard-content.ts:37, :47-49; rendered at App.tsx:704, :763. “Example guidance” at :744 is partial cue; no persistent provenance for fixed scores. |
| T3 — Inert/cosmetic controls | High | Retain P2. Audit has no handler at App.tsx:705-713; SEO changes heading but not data at :733-744; field/lab only state at :787-797. SEO cue “Example” makes this the weakest subclaim. |
| T4 — Browser bridge fallback | High for normal browser failure; Medium for expected recovery | Narrow to P2/P3 disclosure concern. Enabled controls are intentional contract harness. Keep only inaccurate capability/recovery explanation (tauri-admin.ts:182-187 vs ui/codemap.md:5-7, 24-25); do not claim native shell is broken. |

T1 and T2 remain distinct (import semantics vs presentation provenance). T2/T3 may share a remediation work item but concern different failures. Blank Apply joins T1. No new finding.
