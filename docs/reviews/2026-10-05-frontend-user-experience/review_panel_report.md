> ⚠️ **COMPRESSED RUN — Phase 14 independent Opus judge unavailable; Phase 14.5 omitted.**
>
> This review completed three independent perspectives, blind finals, a completeness audit, claim verification, tier refinement, and six targeted verifications. The primary reviewer wrote an orchestrator synthesis in place of the independent Opus judge. Treat the ruling as medium confidence and do not treat this as a fully completed Overseer judge gate.

# Review Panel Report

**Work reviewed:** React/Vite browser preview frontend (`ui/`)  |  **Date:** 2026-10-05
**Panel:** 3 reviewers + completeness auditor + primary verification/synthesis
**Verdict:** Revise before relying on the preview for user decisions  |  **Confidence:** Medium
**Auto-detected signals:** React, TypeScript, accessibility, dashboard UX
**Review mode:** Precise (source-level frontend review)
**Data flow trace:** Standard | import, score/guidance, controls, and admin bridge paths traced
**Panel execution context:** Local dirty-tree HEAD `e9d6d3ab6d398d6c9c786e7b7108d9d926d0b1bb` on `fix/weighted-rollup-aggregation`; the worktree had unrelated dirty files and `ui/` had no local modifications. **Reproducible UI source:** `ui/` matches reachable `main` commit `eb83be9da64057dc71838b33edc01a9a2769b0fa` byte-for-byte. The panel did not run on `main`, and this does not claim whole-tree equivalence.

## Executive Summary

The preview has useful dashboard structure, but several displayed results and controls do not reliably correspond to the currently supplied data. The most important issues are silent sample-data substitution, false critical guidance, contradictory severity labels, an import shape that can statically reach an uncaught render failure, and an explanation that claims opportunity signals raise a score whose formula ignores them. The default primary button color also falls below the normal-text contrast threshold, and the Field/Lab switch lacks an accessible name on its interactive input. Blind reviewers scored the preview 4/10, 4/10, and 3/10 (mean **3.7/10**, median 4). This is a source-based readiness assessment, not a user study.

**Correlation notice:** The three reviewers and supporting reviewers used the same gpt-6-astra substitute family because Opus was unavailable. They reviewed independently before debate, but model-family correlation may narrow blind spots.

## Scope & Limitations

Reviewed the current React/Vite browser preview and its import, insight, Quality Pulse, explanation, admin-bridge, and MUI rendering paths. The repository documents this UI as a browser compatibility preview and describes a separate Rust native shell. The native shell itself was not reviewed.

No tests, app launch, browser screenshots, responsive viewport checks, keyboard task study, rendered accessibility-tree inspection, screen-reader check, or user research were performed. Findings marked verified mean that source or installed dependency behavior supports the claim; they do not imply observed runtime impact. UX-04 is a statically traced failure path, not a runtime reproduction. UX-08 is scoped to enabled, unhovered default button states and is not a complete WCAG audit. UX-10/11 describe markup semantics, not exact assistive-technology output. UX-12 depends on intended audience; UX-13 has no demonstrated task impairment.

The interactive HTML companion was assembled by the primary reviewer because the required Opus HTML agent was unavailable; it follows the review dashboard structure and links to the recorded evidence.

Epistemic labels: `[VERIFIED]` source/dependency path supports the claim; `[PARTIAL]` only part of the claim is established; `[UNVERIFIED]` impact needs rendered/task evidence; `[WEB-VERIFIED]` external criterion checked. `[COMPRESSED]` marks this report’s incomplete independent judge gate.
Defect type: `[EXISTING_DEFECT]` describes current source behavior.

## Score Summary

| Reviewer | Persona | Intensity | Initial | Final | Recommendation |
|---|---|---:|---:|---:|---|
| UX Usability | Task flow and interaction critic | Adversarial | — | 4/10 | Improve data truthfulness, action clarity, and dashboard hierarchy before relying on the preview. |
| UX Accessibility | Accessibility and interaction critic | Adversarial | — | 4/10 | Address contrast, switch naming, and status feedback; verify rendered semantics. |
| UX Trust | Product trust and provenance critic | Adversarial | — | 3/10 | Make data provenance and preview/runtime boundaries explicit. |

The source record preserves each blind final; initial numerical scores were not separately recorded. Mean: 3.67/10, shown as 3.7/10.

## Consensus Points

- Empty or absent input can become sample data, and the UI does not consistently distinguish sample results from imported results.
- Several controls appear actionable while their behavior is absent or limited to changing local selection state.
- Command/baseline outputs and successful payload updates have no explicit status announcement in the markup.
- The browser preview needs clearer provenance and action feedback before users should rely on displayed recommendations.

## Disagreement Points

1. **UX-12, Tauri wording:** Product trust saw a likely mismatch between browser-visible recovery wording and the documented preview/native boundary; usability noted that the compatibility harness may intentionally expose Tauri-specific validation. Ruling: keep as conditional P3; verify audience and intended copy before calling it misleading or changing the contract.
2. **UX-13, layout:** Reviewers noted a fixed 340px right column and detailed sections below forms. No one established clipping, task failure, or excessive scrolling. Ruling: retain only as an unverified P3 validation question.
3. **Accessibility impact:** Source proves missing input naming, absent live-region markup, and specific default-state contrast. The exact user experience with assistive technology and other interaction states remains untested. Keep the source-level defects, qualify their impact, and do not claim a complete accessibility audit.

## Completeness Audit Findings

The completeness auditor added five cross-module issues beyond the initial reviewer set: UX-04 malformed import render path, UX-05 false manager warnings/static routes, UX-06 conflicting stage severity, UX-07 score/explanation mismatch, and UX-08 primary contrast. Targeted verification confirmed UX-04 through UX-09 from source and installed MUI behavior. These remain P2 because they can materially mislead or interrupt a preview session, but evidence does not establish a P0/P1 outage or safety impact.

## Coverage Gaps

- No real browser run or malformed-payload reproduction.
- No mobile, narrow-window, zoom, or reflow checks.
- No keyboard-only task completion or screen-reader testing.
- No validation with intended dashboard users to resolve preview audience, provenance expectations, or which controls are intentionally illustrative.
- No review of the separate native Rust shell.
- Remote branch freshness against `main` was not checked.

## Action Items

1. **[P2] [VERIFIED][COMPRESSED] [EXISTING_DEFECT] UX-01 — Preserve the difference between missing and explicitly empty telemetry.** `ui/src/insight-engine.ts:146-153` substitutes sample seeds when arrays are empty/missing; `ui/src/App.tsx:190-213` also makes blank Apply reset displayed insights. Keep a clear sample/imported/empty state, make reset intent explicit, and avoid replacing valid current results when an import is invalid. **Tier:** Standard. **Raised by:** usability, accessibility, trust. **Verify:** empty arrays, missing arrays, blank Apply, and reset state transitions.
2. **[P2] [VERIFIED][COMPRESSED] [EXISTING_DEFECT] UX-02 — Label the provenance of every static audit/security/performance result.** Several values and findings are hard-coded (`App.tsx:704,737-744,763-768,787-788`; `dashboard-content.ts:35-55`) and remain static after telemetry import. “Example guidance” at `App.tsx:744` is a partial cue, not persistent provenance for all sections. **Tier:** Standard. **Raised by:** trust; auditor confirmed related static guidance. **Verify:** imported state and sample state visibly identify each result’s source.
3. **[P2] [PARTIAL][COMPRESSED] [EXISTING_DEFECT] UX-03 — Make action and selector behavior match its label.** Enabled Run Full Audit has no handler (`App.tsx:705-714`); SEO tabs change selected title but not underlying metrics/findings (`:733-745`); Field/Lab toggles state while data remains fixed (`:787-798`). Disable, clearly mark preview-only, or implement the represented operation. **Tier:** Light. **Raised by:** all three reviewers. **Verify:** each control’s resulting data/action.
4. **[P2] [VERIFIED][COMPRESSED] [EXISTING_DEFECT] UX-04 — Validate payload shape before committing imported telemetry.** A valid JSON object with an object-valued `stages[].name` passes the cast path, reaches Quality Pulse, and is rendered as a React child (`App.tsx:197-203,360-363`; `insight-engine.ts:126`; `quality-pulse.ts:178`; `MetricItem` `App.tsx:135-145`). No error boundary is present at `main.tsx:15-21`. Validate types/ranges and preserve the last valid view while identifying invalid fields. This is a static failure path, not a runtime reproduction. **Tier:** Standard. **Raised by:** completeness auditor. **Verify:** malformed-but-valid JSON cases and retained last-good state.
5. **[P2] [VERIFIED][COMPRESSED] [EXISTING_DEFECT] UX-05 — Derive warnings, routes, and next steps from active data.** `quality-pulse.ts:75-107,138-160` can produce unconditional red manager warnings and sample routes even when complete telemetry is healthy; `App.tsx:365-390` presents them as active recommendations (the sample route includes A-124 in the Team Lead view). Add a real healthy state and tie every recommendation to the active records. **Tier:** Standard. **Raised by:** completeness auditor. **Verify:** healthy, empty, and critical datasets.
6. **[P2] [VERIFIED][COMPRESSED] [EXISTING_DEFECT] UX-06 — Use one stage-severity rule across views.** Job Observability uses fixed 1,000/2,000ms thresholds (`App.tsx:178-187`), while `insight-engine.ts:107-131` uses configurable `latencyP95Ms`. For queue 0, latency 750ms, and configured ceiling 200ms, one view can call the stage good while another calls it critical. Share the classifier or clearly distinguish the criteria. **Tier:** Standard. **Raised by:** completeness auditor. **Verify:** boundary values in every affected view.
7. **[P2] [VERIFIED][COMPRESSED] [EXISTING_DEFECT] UX-07 — Align score explanations with the scoring formula.** `dashboard-explainability.ts:35-43` says opportunity signals “are boosting the score,” but `quality-pulse.ts:163-180` calculates the score from risk and bottleneck counts only. Remove the claimed contribution or implement it in the formula. **Tier:** Standard. **Raised by:** completeness auditor. **Verify:** explanation and score use the same component calculation.
8. **[P2] [WEB-VERIFIED][COMPRESSED] [EXISTING_DEFECT] UX-08 — Adjust the primary color or text treatment to meet normal-text contrast.** Default `#5577cc` with white text calculates to 4.2884:1; outlined primary text on white has the same ratio. WCAG 2.2 SC 1.4.3 sets 4.5:1 for normal-size text. Evidence is scoped to default enabled button states; no rendered checks were run. **Tier:** Standard. **Verify:** all button states, focus rings, and theme variants. [WCAG 2.2 SC 1.4.3](https://www.w3.org/TR/wcag/#contrast-minimum).
9. **[P2] [VERIFIED][COMPRESSED] [EXISTING_DEFECT] UX-09 — Put the switch name on its interactive input or associate a visible label.** `App.tsx:790-798` puts `aria-label` on MUI Switch, but installed MUI 9.4 forwards it to the wrapper while the input receives separate props. “Field Data” and “Lab Data” are unassociated sibling text. **Tier:** Standard. **Raised by:** targeted accessibility verification. **Verify:** rendered accessibility tree and actual keyboard/screen-reader operation after the fix.
10. **[P2] [VERIFIED][COMPRESSED] [EXISTING_DEFECT] UX-10 — Announce asynchronous results and successful updates.** Admin and baseline results are plain captions (`admin-bridge-panel.tsx:34-36`; `App.tsx:544-546`); successful payload Apply/Reset do not announce an outcome (`App.tsx:190-213`). Invalid JSON uses `role="alert"` but lacks explicit textarea helper/error linkage (`:487-490`). Add status semantics and connect input errors to the textarea. **Tier:** Standard. **Verify:** status updates and errors with keyboard and assistive technology.
11. **[P3] [VERIFIED][COMPRESSED] [EXISTING_DEFECT] UX-11 — Give the page a navigable heading and tab structure.** The root has no main landmark, section headings use subtitle variants that installed MUI maps to `h6`, and SEO tabs lack explicit tab/panel IDs and relations (`App.tsx:249-268,280,315,342,733-745`; installed `Typography.js:101-112`). Add a coherent heading hierarchy, main landmark, and tab-panel relationships. **Tier:** Standard. **Verify:** rendered DOM and screen-reader navigation.
12. **[P3] [PARTIAL][COMPRESSED] [EXISTING_DEFECT] UX-12 — Clarify browser-preview versus native-runtime recovery messaging.** `tauri-admin.ts:146-150,182-188` mentions a Tauri runtime, while `ui/codemap.md:3-7,16-26` documents a browser compatibility preview and separate native shell. The harness may intentionally test this boundary, so agree on audience and copy before changing behavior. **Tier:** Deep. **Verify:** intended audience and expected preview contract.
13. **[P3] [UNVERIFIED][COMPRESSED] [EXISTING_DEFECT] UX-13 — Validate layout and task ordering at narrow viewports.** Source defines a 340px right column and places detailed role content after payload/admin/baseline forms (`App.tsx:248-267,454-547,551-686`), although role recommendations also appear near the top (`:365-390`). No impact was demonstrated. **Tier:** Deep. **Verify:** responsive reflow and representative tasks at narrow widths and zoom.

## Verification Summary

| Phase | Result |
|---|---|
| Phase 8 completeness audit | Added UX-04 through UX-08. |
| Phase 9 command execution | Skipped: no reviewer-suggested verification commands were run; this was a static review. |
| Phase 10 claim verification | UX-01 through UX-11 source claims verified or qualified; UX-12 partial; UX-13 impact unverified; no misattributed citation identified. |
| Phase 11 severity verification | Skipped because no P0/P1 findings were raised. |
| Phase 12a/12b tier assignment | Draft completed; advisor refined UX-02, UX-11, UX-12. |
| Phase 13 targeted verification | Six source/dependency checks, UX-04 through UX-09: all confirmed. |
| Phase 14 judge | Independent Opus judge unavailable; primary wrote a disclosed synthesis. |
| Phase 14.5 | Omitted because the synthesis introduced no P0/P1 claims. |

## Detailed Reviews

- [Data-flow and completeness audit](state/phase_8_audit.md)
- [Claim verification table](state/phase_10_claim_verification.md)
- [Confidence-based tier draft](state/phase_12a_tier_assignment.md)
- [Tier refinement](state/phase_12b_tier_refinement.md)
- [Targeted verification: UX-04](state/phase_13_verification_UX04.md), [UX-05](state/phase_13_verification_UX05.md), [UX-06](state/phase_13_verification_UX06.md), [UX-07](state/phase_13_verification_UX07.md), [UX-08](state/phase_13_verification_UX08.md), [UX-09](state/phase_13_verification_UX09.md)
- [Orchestrator synthesis](state/phase_14_judge_analysis.md)
- [Chronological panel process and transcript](review_panel_process.md)
