# Phase 5 — Debate, round 1: accessibility reviewer

| Peer finding | Position | Source-grounded reasoning |
|---|---|---|
| Empty/partial telemetry becomes sample data | Endorse, P2, high confidence | insight-engine.ts:147, :150, :151 use array length; App.tsx:202 presents combined result without identifying substituted collections. |
| Fixed results lack provenance | Endorse, P2, with wording correction | dashboard-content.ts:37 and :47-49 contains specific assertions. SEO is titled “Example guidance” at App.tsx:744; avoid claiming every finding is unlabeled. Fixed SEO metrics :737-743 still lack provenance. |
| Audit and selectors do not change data | Endorse, P2 | Audit no handler at App.tsx:705-713. Field/Lab changes checked state :792; metric source fixed :787-788. SEO changes tab/title but not values/findings. Say “does not change data,” not “nothing changes.” |
| Fixed width and ordering | Partly endorse, P3 | 340px at App.tsx:262; detail follows forms :454-547/:551 onward. Recommendations/routing already appear :365-390. No overflow or zoom issue established without rendered checks. |
| Browser admin controls fail in normal browser | Endorse misleading affordance, P2 | tauri-admin.ts:137-150 and :182-188; enabled buttons at admin-bridge-panel.tsx:29. Compatibility harness is intentional; disclose availability and harness boundary. |

The reviewer also noted the role-specific and fixed-width impacts should remain bounded. No new issue.
