# Phase 3 — Independent review: usability and task flow

Reviewer model: gpt-6-astra (Opus requested by overseer skill was unavailable). Static source review only; no browser rendering, native-host validation, tests, or edits.

**UX readiness: 4/10 for an understandable, task-oriented preview.** The dashboard provides useful role-specific summaries, but its enabled controls and presentation often imply capabilities or data provenance that the rendered interface does not explain.

### U1 — P2: “Run Full Audit” is enabled but cannot run anything

Evidence: ui/src/App.tsx:704-714. The score is the literal 85; the contained button has no onClick, submit form, navigation, or disabled state.

Behavior: Activating the prominently styled primary action produces no application response.

Impact: Users cannot complete the explicitly advertised audit task or distinguish an unsupported feature from a malfunction.

Remedy: For this preview, label the section “Sample accessibility report” and replace the active action with an explained unavailable state. If auditing becomes supported, show progress, completion, and a run timestamp.

Confidence: High, direct component source.

### U2 — P2: Scope and data-source selectors change their own state without changing results

Evidence: ui/src/App.tsx:153-154, 733-744, 787-797.

Behavior: “Current Page” / “Site-Wide” changes only the parenthetical guidance heading; every metric and finding uses the same constants. “Field Data” / “Lab Data” updates fieldData, which is consumed only by the switch’s checked prop. The performance score and recommendations remain constant. “Lab Data” also stays bold in both states.

Impact: Users are invited to compare different scopes or measurement sources, but the interface presents an unchanged result without explaining why.

Remedy: Supply distinct, clearly marked example datasets for each selection or remove these selectors until meaningful. Make the currently selected source explicit in the metric heading.

Confidence: High.

### U3 — P2: The landing screen does not explain that it is a preview with sample results

Evidence: ui/src/App.tsx:157, 279-310, 461-485, 704, 737-743, 763, 787. Preview role is documented in ui/codemap.md:3-6.

Behavior: Initial insights are built without imported data; audit/SEO/security/performance results include literal scores and an unconditional “General Site Security: High.” “Reset to Sample” appears later, but there is no persistent preview/data-source disclosure alongside the initial results.

Impact: A newcomer must infer which results are examples, imported measurements, or operational capabilities. Imported telemetry updates one group of results while the website audit panels remain static, increasing ambiguity.

Remedy: Add a visible preview banner and source label above the first summary: sample/imported, selected dataset, and which panels it affects. Label the static website panels as sample reference material.

Confidence: High for missing disclosure and mixed sources; misunderstanding is an inference.

### U4 — P2 initially: Audience-specific detail is below diagnostics and admin forms

Evidence: Selector at ui/src/App.tsx:289-302; common explainability/trend/jobs at :394-452; payload/admin forms at :454-547; focus panels at :551-686.

Behavior: Choosing Manager, Executive, or Security changes some introductory/action text near the selector, but substantial role-specific detail appears after diagnostics and developer operation forms. There is no section navigation or jump-to-focus action.

Remedy: Put selected role focus near its summary; move advanced tools behind a disclosure or separate view.

Confidence: High for order, medium for impact before user testing.

### U5 — P3: Dashboard remains a narrow side panel on large screens

Evidence: ui/src/App.tsx:248-267 sets a full-height flex container, right alignment, and 340 pixel Paper width. No responsive width/breakpoint rule is supplied.

Behavior: Standalone preview constrains the dashboard to a narrow column; explanations and labels can wrap more.

Remedy: If the sidebar is deliberate, frame it as a companion-panel preview. Otherwise use a responsive layout. This is not a verified overflow defect.

Confidence: High for fixed layout, medium for visual severity without rendering.

### U6 — P2: Applying an empty payload silently discards the imported dataset

Evidence: ui/src/App.tsx:190-195 resets insights to sample data when the input is empty; separate Reset action at :209-213 and buttons at :483-485.

Behavior: After importing data, clearing the editor and pressing Apply switches the dashboard to samples without an error or source-change confirmation.

Remedy: Disable Apply for empty input or ask for a telemetry payload. Keep sample restoration on the explicit Reset action and announce the source.

Confidence: High. It replaces in-memory displayed insights, not persisted source data.

Positive evidence: Audience controls genuinely update recommendations, ownership, timing, and focus content (:168-169, :365-389, :551-686); malformed JSON preserves insights and shows role=alert (:197-205, :487-490); status uses text as well as color (:111-115). The absent browser bridge is expected in this documented compatibility harness and is not a broken native integration. No claims about contrast, focus, clipping, or native UX were made.
