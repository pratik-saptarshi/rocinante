# Phase 4 — Private reflection: accessibility

| Finding | Confidence | Decision |
|---|---|---|
| AX-1 — Unannounced command results | High | Retain P2. Plain caption containers at admin-bridge-panel.tsx:34 and App.tsx:544; handlers set strings at App.tsx:218, :226, :245. Narrow concurrency assertion: overlapping requests are possible with async transport, but no race was reproduced; ordinary fallback resolves promptly. |
| AX-2 — Unnamed Field/Lab input | High | Retain P2. App.tsx:792 prop reaches SwitchBase wrapper (installed MUI Switch.js:308; SwitchBase.js:180), while input has separate props at :205. No associated label. Exact announcement untested. |
| AX-3 — Ineffective controls | High | Retain P2. Button lacks handler at App.tsx:705; metrics at :787-788 do not depend on the switch state at :792. Distinct defects on one control can share a remediation without dropping acceptance criteria. |
| AX-4 — Silent payload success | High for missing notification; Medium for practical impact | Retain P2 with qualification. Apply/Reset update state without completion text at App.tsx:190, :209; invalid JSON has role=alert at :488. A user can inspect results manually; do not claim results are inaccessible. |
| AX-5 — Tabs without panel relationships | High | Retain P3. App.tsx:733-745 has no explicit IDs, aria-controls, or labelled tabpanel. This does not prove keyboard tab selection fails. |
| AX-6 — Flat headings/no main landmark | High for markup; Medium for severity | Retain P3. Root generic containers at App.tsx:249, :258; repeated subtitle variants at :280, :315, :342 and :95 map to h6 in installed MUI Typography.js:108. Avoid claiming all heading navigation fails; structure exists but conveys no hierarchy. |

New refinement: JSON error has role=alert but is not associated with the textarea; merge into feedback remediation rather than add a separate finding. No screen-reader, layout, or focus test performed.
