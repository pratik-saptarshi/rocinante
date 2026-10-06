# Phase 4 — Private reflection: usability

Re-read cited App.tsx sections; no other reviewers’ findings consulted.

| Finding | Confidence | Decision |
|---|---|---|
| U1 — Audit action has no behavior | High | Retain P2. App.tsx:705-713 supplies neither action handler nor disabled state; score is literal at :704. Strongest claim. |
| U2 — Selectors do not change represented results | High | Retain P2, consolidate with U1 as misleading preview controls. SEO changes its heading (:733-744); field/lab changes only switch state (:154, :787-797). Do not call SEO entirely inert. |
| U3 — Missing sample/preview provenance | High | Retain P2. Header and initial summaries provide no source disclosure (:279-310, :314-364); static scores/security claims at :704, :737-743, :763, :787. “Reset to Sample” and “Example guidance” are partial cues, so avoid claiming sample labeling is absent everywhere. |
| U4 — Audience details follow advanced tools | Medium | Reclassify to P3. Role-specific recommendations already appear near the top (:365-389); impact is not measured. |
| U5 — Narrow standalone layout | Medium | Retain as conditional P3 advisory. 340px right-aligned width is certain (:253-266), but could be intended side-panel design. Withdraw as defect if confirmed design requirement. No overflow claim. |
| U6 — Empty Apply switches to samples | High | Retain P2. Replaces displayed in-memory data (:190-195), duplicating explicit Reset (:209-213, :483-485). Do not imply persisted data is deleted. |

Revised grouping: three P2 groups (unsupported control affordances; unclear provenance; empty Apply behavior), two P3 recommendations. No P0/P1. No new issue. The unbound Settings icon at :286 is decorative SVG rather than a declared button, so “broken action” would be weaker. Score remains 4/10 as expert judgment, not measurement.
