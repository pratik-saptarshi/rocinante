# Review Panel Process — Frontend UX

**Date:** 2026-10-05
**Panel local dirty-tree HEAD:** `e9d6d3ab6d398d6c9c786e7b7108d9d926d0b1bb` on `fix/weighted-rollup-aggregation`
**Protocol:** Overseer v3.2.0, one source-only panel run
**Model availability:** Opus unavailable. Reviewers and support reviewers used gpt-6-astra substitutes. The independent Phase 14 judge could not be run; the primary reviewer’s disclosed synthesis is recorded in place of that gate. This is marked as a compressed run.

This is the chronological process record. The agent and verification outputs below are copied verbatim from the state files. The primary report provides the findings and recommendations in consolidated form.

## Persona Profiles Registry

### UX Usability reviewer — task-flow and interaction critic
- **Role/expertise:** Frontend usability; clarity of actions, task flow, data and control expectations.
- **Reasoning strategy:** Follow the user path through import, dashboard interpretation, and actions; separate observed behavior from assumptions about product intent.
- **Agreement intensity:** Adversarial; independent before debate.
- **Phases:** 3, 4, 5 round 1, 7.

### UX Accessibility reviewer — accessibility and interaction critic
- **Role/expertise:** Accessible names, semantics, status feedback, keyboard interaction, and contrast.
- **Reasoning strategy:** Trace JSX and installed MUI behavior; qualify claims needing browser or assistive-technology evidence.
- **Agreement intensity:** Adversarial; independent before debate.
- **Phases:** 3, 4, 5 round 1, 7.

### UX Trust reviewer — product trust and provenance critic
- **Role/expertise:** Data provenance, confidence, recommendation accuracy, and product boundary messaging.
- **Reasoning strategy:** Compare interface claims with supplied data, computed paths, static examples, and documented preview/native boundary.
- **Agreement intensity:** Adversarial; independent before debate.
- **Phases:** 3, 4, 5 round 1, 7.

### Completeness auditor
- **Role/expertise:** Cross-module frontend edge-case review.
- **Reasoning strategy:** Search for schema edges, constants, static guidance, and overlooked data paths.
- **Phases:** 8.

### Tier refinement advisor
- **Role/expertise:** Domain-neutral verification effort calibration.
- **Reasoning strategy:** Refine draft tiers based on evidence complexity and assign a suitable verification persona.
- **Phases:** 12b.

### Phase 13 verification specialists
- **UX-04 — Code Reviewer, Standard:** JSON parsing through data model, projection, render path, and error boundaries.
- **UX-05 — UX Reviewer, Standard:** Healthy telemetry through recommendations and routes.
- **UX-06 — Code Reviewer, Standard:** Compare severity classifiers and configuration.
- **UX-07 — Code Reviewer, Standard:** Compare explanation text and scoring formula.
- **UX-08 — Accessibility Reviewer, Standard:** Resolve theme/component colors, text size, contrast arithmetic, and criterion.
- **UX-09 — Accessibility Reviewer, Standard:** Trace MUI props to the actual interactive input.
- All used the gpt-6-astra substitute and performed read-only analysis.

### Primary orchestrator synthesis
- **Role/expertise:** Reconcile evidence, severity, disputes, and action order.
- **Reasoning strategy:** Read verification artifacts, avoid severity inflation, and disclose unavailable independent judge gate.
- **Phases:** 10, 12a, report synthesis. This is not an independent Supreme Judge.

## Phase 1 — Setup

**Context brief:** Review current React/Vite browser preview frontend in `ui/` for usability and user experience. Focus on data trust, control behavior, accessible interaction, and displayed guidance. Repository documentation describes the UI as a browser compatibility preview and a separate Rust eframe/winit native shell. Do not conflate the two.

**Review mode:** Precise source review. Each code claim needs source evidence. Runtime, user, assistive-technology, or viewport impacts without dynamic evidence remain qualified.

**Persona selection:** Three independent perspectives covered task flow, accessibility, and trust/provenance. A completeness auditor scanned for additional cross-module issues. No P0/P1 finding emerged.

**Panel execution context:** The panel ran against local dirty-tree HEAD `e9d6d3ab6d398d6c9c786e7b7108d9d926d0b1bb` on branch `fix/weighted-rollup-aggregation`. The worktree contained unrelated dirty files; `ui/` had no local modifications.

**Reproducible UI source revision:** The reviewed `ui/` subtree matches reachable `main` commit `eb83be9da64057dc71838b33edc01a9a2769b0fa` byte-for-byte, verified with `git diff --exit-code eb83be9da64057dc71838b33edc01a9a2769b0fa e9d6d3ab6d398d6c9c786e7b7108d9d926d0b1bb -- ui`. The panel itself ran in the local dirty-tree context, not on `main`; this subtree comparison does not establish whole-repository tree equivalence.

## Phase 2 — Data-flow trace

Four paths were followed from source to UI:

1. **Payload import:** Apply JSON in `App.tsx` → parse/cast → defaults and transformations in `insight-engine.ts` → Quality Pulse and displayed insights.
2. **Score and explanations:** commits/stages/signals → separate insight collections → Quality Pulse buckets and score → dashboard metric and explanation. Opportunity signals are counted, but the traced score formula uses risk and bottleneck counts.
3. **Controls and guidance:** audit, SEO tabs, and Field/Lab toggle → event/state handling → displayed metric/finding content. Some controls change selection state without changing the represented data or invoking the implied action.
4. **Admin bridge:** runtime detection → Tauri-facing fallback text and command results → captions/status markup. Intended audience for this preview message was not established.

No runtime trace, test, browser session, or screenshot was used.


## Phase 3 — Independent reviews

<!-- Verbatim state transcript: state/reviewer_usability_phase_3.md -->

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

<!-- Verbatim state transcript: state/reviewer_accessibility_phase_3.md -->

# Phase 3 — Independent review: accessibility and interaction

Reviewer model: gpt-6-astra (Opus requested by overseer skill was unavailable). Static source review only; no browser, assistive-technology, or layout tests. The supported native desktop UI is outside this review.

Overall accessibility and interaction score: 4/10.

### AX-1 — P2: Admin and baseline results are not announced

Evidence: ui/src/admin-bridge-panel.tsx:34 and ui/src/App.tsx:544 render changing results as caption text. Async handlers at App.tsx:215, 221, and 233 set only the result, with no pending state or focus/status management.

Impact: A screen-reader user activating a command receives no explicit announcement that it began, completed, or failed.

Remedy: Add a role=status/polite live region for progress and outcomes, associate errors with relevant inputs, and serialize conflicting operations while pending.

Confidence: High for absent explicit status semantics; no actual screen-reader output tested.

### AX-2 — P2: Field/Lab switch has no accessible name on its actual input

Evidence: ui/src/App.tsx:792 puts aria-label directly on MUI Switch; adjacent text at :791 and :793 is not a label. Installed MUI 9.4 source forwards the prop to the wrapper (ui/node_modules/@mui/material/Switch/Switch.js:308; internal/SwitchBase.js:180), while input props are separately created at :205.

Impact: Keyboard focus reaches a switch without a programmatic label identifying the setting.

Remedy: Put the label on slotProps.input or use an associated FormControlLabel. Prefer explicit Field/Lab choices if both alternatives need naming.

Confidence: High from app and installed-library source; exact assistive-technology output untested.

### AX-3 — P2: Controls advertise actions without performing them

Evidence: ui/src/App.tsx:705-713 renders enabled Run Full Audit without a click handler. fieldData is initialized at :154 and only read/updated by the switch at :792; performance values at :787-788 do not depend on it.

Impact: Click/Enter on the audit button produces no result. Changing the selector changes appearance without changing metrics.

Remedy: Implement an outcome or mark the preview affordance unavailable; label sample metrics and provide separate data or remove the ineffective selector.

Confidence: High.

### AX-4 — P2: Successful payload updates have no completion message

Evidence: ui/src/App.tsx:190-207 updates insights; controls are at :463-486, after affected summaries at :314-452. Only invalid JSON has role=alert at :488. Reset has no status at :209-213.

Impact: A keyboard/screen-reader user is not told that the data changed or where the refreshed summary is.

Remedy: Announce a short success summary and provide a “View updated summary” link or focus target. The practical scroll burden is viewport-dependent and unmeasured.

Confidence: High for absent success feedback; medium for impact.

### AX-5 — P3: SEO tabs have no panel relationship

Evidence: ui/src/App.tsx:733-745 defines tabs without IDs/aria-controls and renders metrics/guidance without a labelled tabpanel.

Impact: Selected tab and content are not programmatically connected.

Remedy: Add IDs, aria-controls, role=tabpanel, and aria-labelledby; or use a labelled filter if selection only changes shared example content.

Confidence: High.

### AX-6 — P3: Long dashboard has flat heading semantics and no main landmark

Evidence: ui/src/App.tsx:249 and :258 are generic containers; page title :280 and section titles :315/:342/:395 use subtitle variants without semantic overrides. Installed MUI maps subtitle1/subtitle2 to h6 (ui/node_modules/@mui/material/Typography/Typography.js:101-109); ui/src/main.tsx:6-13 supplies no override.

Impact: Heading navigation has no hierarchy between page title, sections, and subsections; there is no main landmark shortcut.

Remedy: Use main, one h1, section h2s, and nested h3s while retaining visual variants. Avoid empty title headings at App.tsx:570 and :668.

Confidence: High for markup; screen-reader impact inferred.

No claim was made about contrast, mobile clipping, or focus-ring visibility without rendered validation.

<!-- Verbatim state transcript: state/reviewer_trust_phase_3.md -->

# Phase 3 — Independent review: product mental model and data trust

Reviewer model: gpt-6-astra (Opus requested by overseer skill was unavailable). Static source review only; no files/tests changed. Scope is the React browser preview, not the native application.

UX readiness: 3/10 for an externally presented preview.

### T1 — P2: Empty telemetry silently becomes invented activity

Evidence: ui/src/insight-engine.ts:146-153; sample records at :66-81; import path ui/src/App.tsx:197-203.

Behavior: Applying {"commits":[],"stages":[],"signals":[]} selects all three sample datasets because fallback uses .length. Partial real payload similarly combines real data with synthetic stages or opportunities.

Impact: Users cannot represent “no activity” and can receive recommendations about nonexistent commits, queues, or work.

Remedy: Reserve sample generation for explicit demo initialization/reset. Preserve supplied empty arrays, distinguish missing sections, and disclose provenance per section for partial imports.

Confidence: High, direct code path.

### T2 — P2: Synthetic security and audit assertions lack persistent provenance

Evidence: initial sample state ui/src/App.tsx:157; header :279-284; hardcoded accessibility score :701-714; “General Site Security: High” :760-767; fixed findings ui/src/dashboard-content.ts:35-55.

Behavior: A fresh visit displays risk recommendations, score 85, “CSP: Missing,” and enabled HSTS. “Reset to Sample” at App.tsx:483-485 is a clue, but does not identify the source of each result. These panels stay static after importing telemetry.

Remedy: Show a persistent Browser preview · Sample data label, identify imported versus sample sections, label fixed audit panels “Illustrative example — no audit performed,” and show source/target/time when real results exist.

Confidence: High for presentation; user misinterpretation is an inference.

### T3 — P2: Visible controls promise operations or data changes they do not perform

Evidence: App.tsx:705-713 has no Run Full Audit handler; :733-744 switches tabs while metrics remain fixed; :787-797 changes fieldData without changing outputs.

Behavior: Audit action does nothing; selecting scope/source changes no underlying values or findings.

Remedy: Implement observable results where supported. Otherwise explain/disable the affordance or render a labelled noninteractive example.

Confidence: High.

### T4 — P2 initially: Browser admin controls imply an unavailable operation and obsolete recovery route

Evidence: enabled admin buttons ui/src/admin-bridge-panel.tsx:27-31; baseline actions ui/src/App.tsx:537-542; runtime lookup ui/src/tauri-admin.ts:146-150; fallback :182-187; architectural boundary ui/codemap.md:3-6, 23-26.

Behavior: In ordinary browser use, admin operations fail with a Tauri-only message. Current documentation describes a separate Rust native shell and compatibility-only preview adapter.

Remedy: Display runtime capability before interaction; keep controls on a developer/testing surface and explain the true browser/native boundary.

Confidence: High for fallback, medium for intended audience.

Follow-up question (not defect): Is this preview intended for internal contract development only or stakeholder demonstrations? The design choice affects prominence, but unlabelled sample results remain misleading without explanation.


## Phase 4 — Private reflections

<!-- Verbatim state transcript: state/reviewer_usability_phase_4.md -->

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

<!-- Verbatim state transcript: state/reviewer_accessibility_phase_4.md -->

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

<!-- Verbatim state transcript: state/reviewer_trust_phase_4.md -->

# Phase 4 — Private reflection: data trust

| Finding | Confidence | Decision |
|---|---|---|
| T1 — Empty telemetry becomes sample activity | High | Retain P2; strongest. insight-engine.ts:147, :150, :151 replace empty arrays with seeds; App.tsx:202 applies the result. Distinguish intentional initial sample mode from explicit empty arrays. |
| T2 — Undisclosed synthetic audit/security assertions | High for behavior; Medium for impact | Retain P2. Fixed findings at dashboard-content.ts:37, :47-49; rendered at App.tsx:704, :763. “Example guidance” at :744 is partial cue; no persistent provenance for fixed scores. |
| T3 — Inert/cosmetic controls | High | Retain P2. Audit has no handler at App.tsx:705-713; SEO changes heading but not data at :733-744; field/lab only state at :787-797. SEO cue “Example” makes this the weakest subclaim. |
| T4 — Browser bridge fallback | High for normal browser failure; Medium for expected recovery | Narrow to P2/P3 disclosure concern. Enabled controls are intentional contract harness. Keep only inaccurate capability/recovery explanation (tauri-admin.ts:182-187 vs ui/codemap.md:5-7, 24-25); do not claim native shell is broken. |

T1 and T2 remain distinct (import semantics vs presentation provenance). T2/T3 may share a remediation work item but concern different failures. Blank Apply joins T1. No new finding.


## Phase 5 — Debate, round 1

One recorded debate round; later rounds were not needed for this bounded review.

<!-- Verbatim state transcript: state/reviewer_usability_phase_5_round1.md -->

# Phase 5 — Debate, round 1: usability reviewer

| Peer claim | Position | Evidence and qualification |
|---|---|---|
| Empty/partial imports use sample seeds | Endorse, P2 | insight-engine.ts:146-153 tests each array’s .length and substitutes a seed independently. App.tsx:198-203 accepts this without source feedback. Merge blank Apply into broader import-integrity issue, with separate acceptance case. |
| Static results lack section provenance | Endorse, P2 | dashboard-content.ts:35-55 hardcodes findings/counts/header states/timings; App.tsx:704, :737-744, :763-768, :787-788 renders them. “Example guidance” at :744 is a limited cue; does not explain scores/neighboring sections. |
| Admin/baseline output lacks live status | Endorse as source issue, P2 accessibility candidate | admin-bridge-panel.tsx:34-36 and App.tsx:544-546 are plain captions. This proves no explicit announcement support, not that every screen reader is silent. |
| Main landmark/headings/tab linkage | Partial endorsement | App.tsx:249-268 generic containers; section titles :315-401 subtitle semantics. Tabs :733-744 lack explicit ID/aria-controls/tabpanel. Verify rendered MUI tree before claims about exact heading tree. Keep tab relation with meaning of tabs to avoid double count. |
| Browser bridge controls enabled | Endorse disclosure issue, reject “broken integration” framing | tauri-admin.ts:146-153 and :182-188; admin-bridge-panel.tsx:27-32. Compatibility harness purpose is documented. Explain availability/message; do not claim supported native integration is broken. |

Changes: U4 remains P3 because top role recommendations are at App.tsx:365-389; U5 remains conditional P3 with no verified overflow; U1/U2 consolidated; U6 merged into import integrity. Priorities are provenance, empty-data semantics, honest controls, accessible feedback. No new finding.

<!-- Verbatim state transcript: state/reviewer_accessibility_phase_5_round1.md -->

# Phase 5 — Debate, round 1: accessibility reviewer

| Peer finding | Position | Source-grounded reasoning |
|---|---|---|
| Empty/partial telemetry becomes sample data | Endorse, P2, high confidence | insight-engine.ts:147, :150, :151 use array length; App.tsx:202 presents combined result without identifying substituted collections. |
| Fixed results lack provenance | Endorse, P2, with wording correction | dashboard-content.ts:37 and :47-49 contains specific assertions. SEO is titled “Example guidance” at App.tsx:744; avoid claiming every finding is unlabeled. Fixed SEO metrics :737-743 still lack provenance. |
| Audit and selectors do not change data | Endorse, P2 | Audit no handler at App.tsx:705-713. Field/Lab changes checked state :792; metric source fixed :787-788. SEO changes tab/title but not values/findings. Say “does not change data,” not “nothing changes.” |
| Fixed width and ordering | Partly endorse, P3 | 340px at App.tsx:262; detail follows forms :454-547/:551 onward. Recommendations/routing already appear :365-390. No overflow or zoom issue established without rendered checks. |
| Browser admin controls fail in normal browser | Endorse misleading affordance, P2 | tauri-admin.ts:137-150 and :182-188; enabled buttons at admin-bridge-panel.tsx:29. Compatibility harness is intentional; disclose availability and harness boundary. |

The reviewer also noted the role-specific and fixed-width impacts should remain bounded. No new issue.

<!-- Verbatim state transcript: state/reviewer_trust_phase_5_round1.md -->

# Phase 5 — Debate, round 1: trust reviewer

- Empty Apply: endorse behavior and merge with T1. App.tsx:190-194 resets to samples; explicit Reset at :209-213 and buttons :479-485. Preserve separate acceptance cases for blank input and valid empty arrays.
- Command/baseline announcements: endorse. Plain captions at admin-bridge-panel.tsx:34-36 and App.tsx:544-546; payload validation errors already have role=alert at :486-489, so do not claim errors are wholly unannounced. Missing input association remains plausible at :463-477.
- Audience ordering: narrow to P3 information-architecture recommendation. Role-specific recommendations are already before admin controls at App.tsx:365-389.
- Browser admin actions: withdraw claim that enabled controls are inherently defective because this is a documented compatibility harness. Retain a narrower P2/P3 capability-message concern: fallback says desktop Tauri runtime at tauri-admin.ts:182-187 while ui/codemap.md:5-7, 24-25 describes separate Rust native host.
- Changes: T1 absorbs blank Apply; admin finding loses unconditional disable requirement; payload alert acknowledged. No severity escalation or new issue.


## Phase 7 — Blind final assessments

<!-- Verbatim state transcript: state/reviewer_usability_phase_7.md -->

# Phase 7 — Blind final assessment: usability

Independent browser-preview UX readiness score: 4/10. This is an expert judgment, not a measured usability result or native-host assessment.

| Priority | Finding | Severity | Evidence |
|---|---|---|---|
| 1 | Empty or partial imports silently introduce sample data | P2 | ui/src/insight-engine.ts:146-153 substitutes samples for empty/absent arrays; ui/src/App.tsx:190-203 applies results without source feedback; empty editor also switches to samples. |
| 2 | Displayed results lack clear provenance | P2 | ui/src/App.tsx:704, :737-744, :763-768, :787-788 renders fixed claims alongside imported insights; ui/src/dashboard-content.ts:35-55 supplies literal findings. |
| 3 | Enabled controls imply unsupported operations | P2 | Run Full Audit no action at ui/src/App.tsx:705-713; SEO tabs change only heading :733-744; Field/Lab changes only switch state :154, :787-797. |

Withdrawals: U4 becomes P3 because top role recommendations exist at App.tsx:365-389. U5 is conditional P3 with no overflow defect established. Empty Apply replaces displayed in-memory insights; it does not delete persisted data.

Verdict: Useful as a developer demonstration, but it needs honest control availability, explicit data provenance, and faithful empty-data handling before an unfamiliar user can reliably interpret and operate it.

<!-- Verbatim state transcript: state/reviewer_accessibility_phase_7.md -->

# Phase 7 — Blind final assessment: accessibility

Independent readiness score: 4/10 for the React browser preview.

| Priority | Finding | Severity | Evidence |
|---|---|---|---|
| 1 | Explicit empty telemetry becomes sample data | P2 | ui/src/insight-engine.ts:147, :150, :151 substitute seeds for supplied empty arrays. |
| 2 | Enabled controls promise unimplemented behavior | P2 | Run Full Audit has no handler at ui/src/App.tsx:705; Field/Lab changes checked state at :792 while values stay fixed at :787-788. |
| 3 | Command outcomes lack accessible status feedback | P2 | Results appear as plain captions at ui/src/admin-bridge-panel.tsx:34 and ui/src/App.tsx:544, with no live-region/status semantics. |

Withdrawals: None of the core findings. Exact screen-reader announcements, viewport clipping, and command races are untested. Unnamed switch-input and flat-heading findings remain source-confirmed but assistive-technology effects require a rendered check.

Verdict: Useful as an internal harness, but it needs explicit data provenance, honest action states, and accessible feedback before presenting it as dependable preview.

<!-- Verbatim state transcript: state/reviewer_trust_phase_7.md -->

# Phase 7 — Blind final assessment: data trust

Independent final score: 3/10 for browser-preview UX/trust readiness. Qualitative source review, not a measured usability score.

| ID | Finding | Severity | Evidence and confidence |
|---|---|---|---|
| T1 | Empty imports become synthetic activity | P2 | ui/src/insight-engine.ts:147-153 replaces supplied empty arrays with samples; ui/src/App.tsx:190-203 also makes blank/primitive input restore samples. Confirmed behavior; misunderstanding is inferred. |
| T2 | Fixed audit/security findings lack persistent provenance | P2 | ui/src/App.tsx:704, :763, :787; ui/src/dashboard-content.ts:35-55 contains fixed scores/security claims/findings. Sample cues exist elsewhere, but sections do not identify their source. |
| T3 | Audit and data controls have no corresponding operation | P2 | ui/src/App.tsx:705-713 enabled audit button without handler; :733-744 and :787-797 change scope/switch state without changing metrics. |

Withdrawn/narrowed: Enabled admin controls are not inherently defective; retain only the inaccurate runtime explanation as a secondary concern (tauri-admin.ts:182-187 vs ui/codemap.md:5-7, :16-17, :24-25). Do not claim payload errors are unannounced; App.tsx:486-489 has role=alert.

Verdict: Suitable as an explained internal harness; it needs explicit data provenance, faithful empty-state handling, and honest control affordances before presenting it as a self-explanatory preview.


## Phase 8 — Completeness audit

<!-- Verbatim state transcript: state/phase_8_audit.md -->

# Phase 8 — Completeness audit

Auditor: gpt-6-astra substitute (Opus unavailable). Read-only static source review; no tests, screenshots, or app launch.

1. **P2 — Malformed telemetry can crash the preview after Apply.** App.tsx:197 parses JSON and casts without schema validation. An object-valued stage.name survives insight-engine.ts:126, becomes topBottleneckName at domain/quality-pulse.ts:178, and reaches a direct React child at App.tsx:360 and :145. Render occurs outside the Apply handler catch. main.tsx:15 has no error boundary. A syntactically valid payload can remove the usable dashboard. Remedy: validate element types and finite ranges before replacing state; retain previous valid insights and identify bad fields. Confidence high, source-derived.

2. **P2 — Healthy populated telemetry still produces false critical recommendations and unrelated routes.** domain/quality-pulse.ts:98-107 unconditionally emits two red manager warnings and uses “review” if no critical/high stage exists. buildRoutes at :138 onward does not receive telemetry and hardcodes A-124, review, and sample opportunities. Routes are displayed beside calculated recommendations in App.tsx:373. Remedy: derive routes/recommendations from active data and add healthy state. Confidence high.

3. **P2 — Stage severity contradicts across sections.** App.tsx:178-187 uses fixed 1,000/2,000ms thresholds; insight-engine.ts:107-131 uses supplied latencyP95Ms and 3x thereof. For queue depth 0, latency 500, configured ceiling 100, Job Observability says good while bottleneck model says critical. Remedy: one shared severity classifier or clearly named distinct criteria. Confidence high.

4. **P2 — Explainability claims a score contribution that does not exist.** dashboard-explainability.ts:35-42 says opportunity signals “are boosting the score”; domain/quality-pulse.ts:173 computes score only from high-risk commits and critical/high bottlenecks. Remedy: derive explanation from actual components or describe opportunities separately. Confidence high.

5. **P2 — Primary button text contrast is below AA for normal-size text.** main.tsx:9 sets #5577cc; App.tsx:480 uses a small contained button. Installed MUI selects white contrast text and uses primary as background. Static calculation is 4.288:1. W3C WCAG 2.2 SC 1.4.3 requires 4.5:1 for normal text; [official criterion](https://www.w3.org/TR/wcag/#contrast-minimum). Remedy: darken the theme primary token and validate each interaction state. Source/color confidence high; no rendered inspection.


## Phase 10 — Claim verification

<!-- Verbatim state transcript: state/phase_10_claim_verification.md -->

# Phase 10 — Claim verification

Verifier: primary reviewer, source read-back using code graph and direct excerpts; no tests or runtime inspection. “Verified” means the cited implementation establishes the stated behavior, not that a human observed it in a browser.

| ID | Claim | Verdict | Citation check / limits |
|---|---|---|---|
| UX-01 | Empty/missing payload arrays select sample seeds; blank Apply resets to sample | [VERIFIED] | App.tsx:190-203, 209-213; insight-engine.ts:146-153. Confirmed; no persisted data is modified. |
| UX-02 | Fixed audit/security/performance assertions lack a persistent per-section source label | [VERIFIED] | App.tsx:704, 737-744, 763-768, 787-788; dashboard-content.ts:35-55. SEO says “Example guidance” at :744, so “no cue anywhere” would be inaccurate. |
| UX-03 | Enabled audit action has no handler; scope/source selectors do not change represented values | [VERIFIED] | App.tsx:705-714, 733-745, 787-798; fieldData appears only in initialization and switch path. SEO changes selected tab/title but not metric/finding data. |
| UX-04 | Valid schema-invalid stage name may crash rendering | [VERIFIED, static path] | App.tsx:197-203 casts parsed content; insight-engine.ts:107-131 preserves stage name; quality-pulse.ts:178 propagates it; App.tsx:360-363 and MetricItem :135-145 render it as child. main.tsx:15-21 has no error boundary. Runtime reproduction intentionally not run. |
| UX-05 | Healthy telemetry can still yield red manager warning and static sample route | [VERIFIED] | domain/quality-pulse.ts:75-107, :138-160; App.tsx:365-390. `criticalStage` fallback and unconditional severity are explicit; routes hardcoded independently of input. |
| UX-06 | Stage status differs under configured latency threshold | [VERIFIED] | App.tsx:178-187 uses constants; dashboard-contract.ts:17-25 parses latencyP95Ms; insight-engine.ts:107-131 applies that config. Example: queueDepth=0, avgLatencyMs=500, latencyP95Ms=100 gives App good and model critical. |
| UX-07 | Explanation credits opportunities for score while formula excludes them | [VERIFIED] | dashboard-explainability.ts:35-43 vs domain/quality-pulse.ts:163-180, especially :173. |
| UX-08 | Primary blue with white text is 4.288:1 and below 4.5:1 normal text AA criterion | [WEB-VERIFIED] | main.tsx:6-12 sets #5577cc and white paper; installed MUI v9.4 palette code selects white contrast text at that ratio; App.tsx:480 uses small contained button. Calculated ratio 4.2884:1. W3C WCAG 2.2 SC 1.4.3 confirms 4.5:1 normal-text threshold: https://www.w3.org/TR/wcag/#contrast-minimum. |
| UX-09 | Switch accessible label is on MUI wrapper, not actual input | [VERIFIED, dependency implementation] | App.tsx:790-798 supplies only aria-label on Switch and no associated label. Installed MUI Switch.js:302-328 forwards other props to SwitchBase and separately supplies input role; SwitchBase.js:171-208 sends forwarded props to root, input slot has separate slotProps. MUI version is installed 9.4.0. Exact assistive output not tested. |
| UX-10 | Command/baseline state and successful payload update have no explicit live announcement | [VERIFIED, markup] | admin-bridge-panel.tsx:34-36 and App.tsx:544-546 are plain Typography; App.tsx:190-203 and :209-213 have no success message. Invalid JSON does have role=alert at :487-490, but textarea lacks error/helper linkage. No actual screen-reader run. |
| UX-11 | Flat heading levels, missing main landmark and no explicit tabs/panel relation | [VERIFIED, markup] | App.tsx root and Paper :249-268 generic; titles :280, :315, :342 etc use subtitle1/2. Installed MUI Typography maps both to h6 (Typography.js:101-112). App.tsx:733-745 has no explicit IDs, aria-controls, or tabpanel. Rendered AT impact not tested. |
| UX-12 | Browser bridge error suggests a Tauri desktop runtime although current codemap describes a compatibility preview and separate Rust native shell | [PARTIAL] | tauri-admin.ts:146-150, :182-188 and ui/codemap.md:3-7, 16-26 support the architectural mismatch. It is intentional contract-preview behavior; user-facing impact depends on audience. |
| UX-13 | Fixed 340px column / role detail position causes usability problems | [UNVERIFIED impact] | App.tsx:253-267 and :454-547 before :551-686 establish layout/order; top role recommendations also occur at :365-390. No viewport or task testing was performed, so keep as conditional P3 design follow-up, not an established defect. |

No hallucinated or misattributed code citation identified in the final set. No P0 or P1 claims were made.


## Phase 12 — Tier assignment

<!-- Verbatim state transcript: state/phase_12a_tier_assignment.md -->

# Phase 12a — Confidence-based verification tier draft

| ID | Claim | Draft tier | Signal |
|---|---|---|---|
| UX-01 | Empty/missing imports and blank Apply use sample data | Standard | High reviewer confidence and cross-file input/data flow. |
| UX-02 | Fixed audit/security metrics lack section-level provenance | Light | High confidence; fixed literals and render path are directly checkable. |
| UX-03 | Audit/SEO/Field-Lab controls do not perform implied data operation | Light | All reviewers agree; simple source/render paths. |
| UX-04 | Schema-invalid object stage name can crash rendering | Standard | Single auditor discovery; requires tracing across parser, insight model, pulse, and React render. |
| UX-05 | Healthy input still produces red critical recommendations and static routes | Standard | Single auditor discovery; cross-file computed vs hardcoded path. |
| UX-06 | Stage urgency thresholds conflict | Standard | Single auditor discovery; cross-file config/classifier comparison. |
| UX-07 | Opportunity explanation contradicts score formula | Standard | Single auditor discovery; cross-file explanation/formula comparison. |
| UX-08 | Primary button contrast is 4.288:1 against white | Standard | External WCAG criterion plus color calculation and MUI palette behavior; criterion web-verified. |
| UX-09 | MUI Switch accessible name is attached to wrapper rather than input | Standard | Single reviewer discovery; depends on installed component slot forwarding. |
| UX-10 | Async statuses and payload success have no live-region announcements | Standard | High markup confidence; specific screen-reader behavior remains untested. |
| UX-11 | Heading hierarchy/main/tab-panel semantics are flat/missing | Light | Markup and installed MUI mapping are directly checkable; runtime announcement not tested. |
| UX-12 | Browser/Tauri recovery wording mismatches preview/native architecture | Standard | Severity disputed by reviewers; cross-check implementation and codemap. |
| UX-13 | Fixed narrow panel/order hurts responsive/task usability | Deep | Impact requires rendered viewport/task validation; retain only as conditional design question. |

Tier definitions follow Overseer: Light = local fact/read; Standard = multiple files/import flow; Deep = runtime/external or unresolved intent. The reviewer team did not execute a browser, screen reader, or task test.

<!-- Verbatim state transcript: state/phase_12b_tier_refinement.md -->

# Phase 12b — Tier refinement advisor

Advisor: gpt-6-astra substitute (Opus unavailable). Read-only; no tests or edits.

Recommended overrides: **UX-02 Light → Standard**, **UX-11 Light → Standard**, and **UX-12 Standard → Deep**. Keep the remaining draft tiers.

| Finding | Final tier | Verification persona | Reasoning |
|---|---|---|---|
| UX-01 Empty/partial imports and blank Apply | Standard | Code Reviewer | Blank Apply is a local handler fact, but collection fallback requires tracing the payload reader, insight engine, and displayed state. The combined finding needs cross-file verification. |
| UX-02 Static results lack provenance | Standard | UX Reviewer | Static values can be identified locally. Establishing that users cannot distinguish their provenance requires reviewing the surrounding labels, shared content, and state transitions. A single constant does not establish the full UX claim. |
| UX-03 No-op controls | Light | Code Reviewer | The relevant controls and state usage are visible in App.tsx: handler absence and state that does not affect the displayed measurements are bounded source facts. Escalation is unnecessary unless the claim expands to expected product behavior. |
| UX-04 Object-valued stage name crashes rendering | Standard | Code Reviewer | Requires tracing JSON acceptance through the model, Quality Pulse, MetricItem, and installed rendering behavior, plus checking error boundaries. This is established cross-file logic; runtime reproduction would corroborate it but is not necessary to understand the failure path. |
| UX-05 False red recommendations and static routes | Standard | UX Reviewer | Requires comparing valid healthy input, domain classification, recommendation construction, and rendered guidance. The issue concerns whether displayed instructions accurately represent the supplied records. |
| UX-06 Conflicting latency severity thresholds | Standard | Code Reviewer | Requires comparing two classification paths and following the configurable limit. A concrete source-derived input can demonstrate the contradiction without running the application. |
| UX-07 Opportunity explanation contradicts score formula | Standard | Code Reviewer | The explanation and score formula live in separate modules. Verification needs to establish whether any opportunity contribution exists along the complete score path. |
| UX-08 Primary contrast ratio and WCAG threshold | Standard | Accessibility Reviewer | Requires resolving theme tokens, installed MUI foreground/background selection, text sizing, and contrast arithmetic. The official threshold has already been checked, so no unresolved external question remains that would justify Deep. Scope the conclusion to the verified default state. |
| UX-09 Switch label forwarding | Standard | Accessibility Reviewer | The label’s presence in JSX is insufficient; verification must follow installed MUI prop forwarding to determine whether it names the interactive input. If that dependency trace remains ambiguous, escalate to Deep for accessibility-tree inspection. |
| UX-10 Missing status announcements | Standard | Accessibility Reviewer | Requires following result updates to their output components and checking live-region semantics, focus changes, and dependency defaults. The claim spans multiple components even though the absence is source-verifiable. |
| UX-11 Flat headings and missing tab association | Standard | Accessibility Reviewer | Tab association omissions are locally visible, but the actual heading elements depend on MUI’s variant mapping and theme configuration. The combined finding therefore exceeds a single-file fact. |
| UX-12 Tauri messaging versus the preview boundary | Deep | Product/UX Reviewer | Source can establish the message and documented Rust/browser boundary. Whether the message misleads the intended audience remains an unresolved intent question. Deep verification should resolve that expectation; further code reading alone cannot establish it. |
| UX-13 Layout impact | Deep | Usability Reviewer | Fixed dimensions are source facts, but task obstruction, reflow, scrolling burden, and viewport impact require rendered viewport/task validation. Retain Deep and keep the impact conditional until that evidence exists. |

No severity changes or additional findings were proposed. No edits or tests were performed.


## Phase 13 — Targeted verification

<!-- Verbatim state transcript: state/phase_13_verification_UX04.md -->

# Phase 13 — Targeted verification: malformed telemetry

Persona: Code Reviewer; Standard tier. Reviewer: gpt-6-astra substitute. [VR_CONFIRMED]

The reviewer traced a valid payload with stages:[{name:{},queueDepth:0,throughput:1,avgLatencyMs:1}]. App.tsx:198 parses JSON; :202 passes a TypeScript assertion but no runtime validator; dashboard-contract.ts:33 returns the payload. insight-engine.ts:150 accepts the nonempty stage array and :126 preserves stage.name. quality-pulse.ts:178 returns that object as topBottleneckName. App.tsx:362 passes it into MetricItem, whose Typography renders value directly at :145.

The catch surrounds parse and synchronous insight construction, but the React reconciliation happens later. main.tsx:15-21 has no error boundary. Installed React rejects an object child. Therefore the exact invalid-child failure path is source-confirmed; no runtime reproduction was performed, and the browser-visible failure screen is not claimed.

<!-- Verbatim state transcript: state/phase_13_verification_UX05.md -->

# Phase 13 — Targeted verification: recommendations from healthy telemetry

Persona: Code Reviewer; Standard tier. Reviewer: gpt-6-astra substitute. [VR_CONFIRMED]

A complete healthy payload avoids sample-array fallback: all nonempty arrays are used at insight-engine.ts:146-159; safe one-file/test-touched commit can score 3 (lines 88-104), and queue 0/latency 100 stage is healthy under default ceiling (107-130, 154). buildQualityPulse derives actual buckets and score (domain/quality-pulse.ts:163-185), then buildRecommendations finds no critical/high stage at :80 but manager copy uses “review” fallback and severity bad unconditionally at :98-108.

buildRoutes takes no insights at :138. Team Lead routing hardcodes A-124 at :140-143; manager route hardcodes review queue text at :145-148. App.tsx:365-390 renders calculated recommendations and route strings.

Wording correction: A-124 is the Team Lead route, not the Manager route. A healthy summary can coexist with error-styled critical Manager recommendations and an unrelated Team Lead route. Static trace only; no tests/browser run.

<!-- Verbatim state transcript: state/phase_13_verification_UX06.md -->

# Phase 13 — Targeted verification: stage severity thresholds

Persona: Code Reviewer; Standard tier. Reviewer: gpt-6-astra substitute. [VR_CONFIRMED]

Concrete valid input: stage review, queueDepth 0, throughput 10, avgLatencyMs 750; limits.latencyP95Ms 200.

App.tsx:197-202 parses and passes readLimits. dashboard-contract.ts:10-25 accepts latencyP95Ms=200; :28-33 preserves stage payload. insight-engine.ts:150, :154-158 uses supplied stage and ceiling; stageToBottleneck at :107-114 reports critical because 750 >= 3*200. Job Observability independently uses fixed thresholds at App.tsx:178-187, so 0 queue / 750ms is good. App.tsx:441-451 renders that label; Manager Radar at :595-607 renders critical.

The discrepancy is source-confirmed. Whether two policies were intended is undocumented; the user interface does not explain separate criteria. Static trace only; no runtime validation.

<!-- Verbatim state transcript: state/phase_13_verification_UX07.md -->

# Phase 13 — Targeted verification: UX-07

Verification agent: Code Reviewer (gpt-6-astra substitute; Opus unavailable). Standard tier. Static inspection only.

**[VR_CONFIRMED] — Opportunity signals do not contribute to the displayed overall score, but the explanation explicitly says they do.**

- `ui/src/dashboard-explainability.ts:40` emits, whenever `pulse.opportunityCount > 0`: “{count} opportunity signal(s) are boosting the score.”
- `ui/src/insight-engine.ts:146` separately transforms commits into `commitRiskCards`, stages into `bottlenecks`, and signals into `opportunities`. Opportunity values do not modify commit or bottleneck classifications.
- `ui/src/domain/quality-pulse.ts:164-173` derives risk buckets only from commit risk cards and bottleneck buckets only from bottlenecks. At line 173, `overallScore` is `Math.max(45, 100 - riskBuckets.high * 10 - bottleneckBuckets.critical * 15 - bottleneckBuckets.high * 5)`. `opportunityCount` is returned separately.
- `ui/src/App.tsx:165` passes the Quality Pulse to `buildExplainabilityTraces`; `:352` displays `qualityPulse.overallScore`; `:407` renders the explanation detail.

Holding commits and stages constant while changing opportunity count or values cannot boost this score, despite the displayed statement.

**Limit:** Source inspection only; no tests, execution, or edits.

<!-- Verbatim state transcript: state/phase_13_verification_UX08.md -->

# Phase 13 — Targeted verification: UX-08

Verification agent: Accessibility Reviewer (gpt-6-astra substitute; Opus unavailable). Standard tier. Theme/component trace and numerical calculation; no rendered inspection.

**[VR_CONFIRMED] — High confidence, P2: Default primary action text is approximately 4.288:1, below WCAG 2.2 AA 1.4.3’s 4.5:1 requirement for normal-size text.**

- `ui/src/main.tsx:6-12` sets primary `#5577cc`, light mode, and white paper. Installed MUI is 9.4.0 (`ui/node_modules/@mui/material/package.json:3`).
- MUI’s `contrastThreshold` defaults to 3 and its palette selects white contrast text for this primary (`ui/node_modules/@mui/material/styles/createPalette.js:211,226-230`). Contained buttons use `primary.contrastText` over `primary.main`; outlined buttons use `primary.main` as text (`Button/Button.js:167-172`).
- Default button text is 14px and small button text 13px (`styles/createTypography.js:25,29,74`; `Button/Button.js:218-241`), both normal-size text. The contained and outlined default backgrounds are the primary blue and white paper respectively.

For `#5577cc`, WCAG channel linearization gives relative luminance `L = 0.1948457977`; contrast with white is `(1 + 0.05) / (L + 0.05) = 4.2884134003:1`, below 4.5:1. This covers enabled, unhovered default-state button text, including Apply Payload/Reset to Sample and Run Full Audit.

**Limit:** No browser computed-style inspection, screenshots, tests, or edits. This is not a complete WCAG audit. Criterion: [WCAG 2.2 SC 1.4.3](https://www.w3.org/TR/wcag/#contrast-minimum).

<!-- Verbatim state transcript: state/phase_13_verification_UX09.md -->

# Phase 13 — Targeted verification: UX-09

Verification agent: Accessibility Reviewer (gpt-6-astra substitute; Opus unavailable). Standard tier. Installed MUI source trace only.

**[VR_CONFIRMED] — High confidence, P2: The Field/Lab switch’s supplied `aria-label` reaches the SwitchBase wrapper, not the native input with `role="switch"`; adjacent text is not an associated label.**

- Installed MUI is 9.4.0 (`ui/node_modules/@mui/material/package.json:3`). `ui/src/App.tsx:792` supplies `aria-label` to `<Switch>` without `slotProps.input`, `aria-labelledby`, or an ID/label association.
- In `ui/node_modules/@mui/material/Switch/Switch.mjs:254-255,295-301`, the attribute is collected in `...other` and sent to SwitchBase. Input props come separately from `slotProps.input` (`:264,318-319`), where the native input receives `role: 'switch'`.
- `internal/SwitchBase.mjs:108-113,168-176` retains the attribute in root props and sends it to a span root; the input slot receives separate forwarded props (`:164-167,198-201`). `useSlot.mjs:62` forwards remaining attributes to the root slot.
- “Field Data” and “Lab Data” are sibling Typography elements at `App.tsx:790-797`, without IDs, `htmlFor`, or an enclosing label.

**Remedy:** Put an accessible name on the input (for example `slotProps={{ input: { 'aria-label': 'Use field data' } }}`) or associate a visible label; make the name explain what checked means.

**Limit:** No rendered accessibility tree or assistive-technology test was performed. The source-confirmed issue is the missing input-name association.


## Phase 14 — Synthesis

<!-- Verbatim state transcript: state/phase_14_judge_analysis.md -->

# Phase 14 — Orchestrator synthesis (independent Opus judge unavailable)

This is the primary reviewer’s synthesis of the recorded panel, completeness audit, source verification, tier refinement, and targeted verification. It is **not** an independent Supreme Judge result: the Overseer skill requires Opus for that phase, and Opus was unavailable in this environment. No new high-severity findings are introduced.

## Ruling

The React/Vite preview should be revised before it is presented as a trustworthy, accessible dashboard for user-supplied telemetry. The most serious confirmed UX risks are misleading data/state behavior (sample substitution, static critical guidance, conflicting severity, and a score explanation that contradicts the formula), an uncaught render path from malformed-but-valid JSON, and accessibility defects in contrast and switch naming. Keep all findings at P2 or P3: source evidence supports real defects, but no P0/P1 impact is established and there was no runtime, user, screen-reader, or viewport validation.

**Score:** 3.7/10 mean across blind finals (4/10, 4/10, 3/10); median 4/10. This is a qualitative readiness score, not a usability study. **Verdict:** Revise before relying on the preview for user decisions; acceptable only as an internal harness with clear limitations.

## Evidence and severity

- UX-01–UX-11 are supported by source paths and/or installed MUI behavior. UX-04 is a statically traced crash path, not a runtime reproduction. UX-08 is scoped to default enabled button states. UX-10 and UX-11 establish markup semantics, not exact assistive-technology announcements.
- UX-12 is partial: browser bridge text names a Tauri runtime while the UI documentation describes a browser compatibility preview and a separate Rust native shell. Whether this misleads depends on the preview’s intended audience.
- UX-13 remains an unverified usability hypothesis: layout/order facts are visible in source, but no viewport or task test showed obstruction.
- No issue warrants P0/P1. Do not state that the application fails WCAG overall; the evidence establishes a specific default-state text contrast failure and one unnamed switch input.

## Consensus and disputes

Reviewers converged on empty-data/sample ambiguity, controls that imply unavailable behavior, and absent explicit status feedback. The completeness audit supplied UX-04 through UX-08, which were subsequently traced; it also prompted targeted UX-07 through UX-09 checks. Product trust and usability differed on the Tauri/browser message: classify it as a conditional audience/message issue, not a broken native integration. Reviewers also treated fixed layout as a concern; retain it only as P3 conditional pending rendered validation.

## Coverage and confidence

Static code/data-flow coverage is good for import, score, guidance, control, and bridge paths. Coverage is incomplete for responsive layouts, screen readers, keyboard task completion, actual browser error rendering, and real user expectations. The three reviewers and support agents used the same gpt-6-astra substitute model family; independent prompts reduce direct cross-talk but do not eliminate correlated-model bias. Overall confidence is **Medium**.

## Recommended order

1. Correct data integrity and trust behavior: distinguish empty input from missing input, preserve prior state on invalid payload, validate payload shapes, and derive recommendations and routes from active telemetry.
2. Unify metric severity and score explanations with the actual scoring/classification functions; make nonfunctional controls disabled or label them as preview-only.
3. Fix default text contrast, name the actual switch input, and provide semantic status/error feedback.
4. Clarify browser preview versus native-runtime messaging with the intended audience; validate responsive layouts and task ordering in a browser before treating UX-12/13 as closed.

No tests or edits were performed. Phase 14.5 is omitted because this synthesis introduced no P0/P1 claims; an independent Phase 14/14.5 judge gate remains incomplete.


## Phase 6 — Round summary

The reviewers converged on sample-data ambiguity, actions/selectors whose behavior does not match their apparent purpose, and missing explicit status feedback. The completeness auditor added UX-04 through UX-08. The browser/Tauri wording remained conditional on audience intent; the layout concern remained unverified without rendered viewport/task evidence.

## Phase 9 — Verification commands

Skipped. No reviewer-suggested verification command was executed. This was a static review; tests and app launch were not requested or run.

## Phase 11 — Severity verification

Skipped because no P0/P1 claim was raised. All final findings are P2 or P3, with impact limits recorded in the report.

## Phase 14.5 — Post-judge verification

Omitted because the primary synthesis introduced no P0/P1 finding. Since an independent Opus judge was unavailable, the independent judge and post-judge gate remain incomplete.

## Phase 15 — Outputs

- Primary structured report: `review_panel_report.md`
- This chronological process log: `review_panel_process.md`
- Interactive dashboard: `review_panel_report.html`, assembled by the primary reviewer because the required Opus HTML agent was unavailable; it includes the report data and links to the state-backed transcripts.
