# Frontend UX Plan Review Traceability — 2026-10-05

**Status (updated 2026-10-06):** Phase 0 planning is integrated. Phase 1 local typecheck, Vitest (13 files / 79 tests), production build, CI workflow contract, actionlint-with-repository-ignore checks, and discovery of all 8 browser tests pass; local browser execution is blocked by sandbox `EPERM` on `127.0.0.1:4173`. Hosted CI/aggregate/Playwright, Security including CodeQL, and Dependency Review passed on PR #114 code head `71bf9e5990428ac0603eeb370837bd954c4ef1f2`; run IDs are in the validation note. Protected `main` branch requirements were verified at base `d4bc8e7`: `test` and `codeql`; the aggregate `test` requires `ui-playwright` transitively. Phase 1 implementation child Beads are closed locally; milestone `BI-5u3.1` remains open pending protected merge. Remote Beads sync is unverified. See the [Phase 1 validation note](frontend-ux-phase-1-closeout-2026-10-06.md).
**Source:** [dated adversarial panel report](../reviews/2026-10-05-frontend-user-experience/review_panel_report.md), [process transcript](../reviews/2026-10-05-frontend-user-experience/review_panel_process.md), and companion state files.
**Plan:** [Frontend UX remediation roadmap](remediation-roadmap-2026-10-05.md).

The review scored 3.7/10, classified UX-01–UX-10 as P2 and UX-11–UX-13 as P3, with no P0/P1. Six targeted source checks confirmed UX-04–UX-09. There was no independent Opus Phase 14 judge and no browser, user, viewport, keyboard, or assistive-technology testing.

## Finding dispositions

| Finding | Cross-reference | Evidence and caveat | Actionability / groundedness | Filter / category | Finding bead | Milestone | Planned disposition |
|---|---|---|---:|---|---|---|---|
| UX-01 | Gap | Verified source behavior: empty/missing arrays and blank Apply can select sample data; Phase 1 implementation distinguishes missing, partial, explicitly empty, sample, and imported states | 0.95 / 0.95 | Pass / bundle | BI-5u3.1.1 | BI-5u3.1 | Local and hosted checks passed on `71bf9e5`; child bead closed locally, remote sync unverified |
| UX-02 | Gap | Verified static content lacks consistent provenance after import | 0.87 / 0.95 | Pass / bundle | BI-5u3.3.1 | BI-5u3.3 | Label each static section or derive it from input; open |
| UX-03 | Gap | Source paths verified; intended browser-preview control behavior unresolved | 0.82 / 0.80 | Pass with caveat / bundle with caveat | BI-5u3.4.1 | BI-5u3.4 | Require a visible result or preview-only/unavailable state; open |
| UX-04 | Gap | Schema-invalid object name was statically traced, not runtime reproduced; Phase 1 validates before replacement and adds component/browser recovery coverage | 0.95 / 0.95 | Pass / bundle | BI-5u3.1.2 | BI-5u3.1 | Local type/unit/build and hosted Playwright/aggregate passed on `71bf9e5`; child bead closed locally, remote sync unverified |
| UX-05 | Gap | Source trace confirms warnings/routes can be misleading for healthy input | 0.95 / 0.95 | Pass / bundle | BI-5u3.3.2 | BI-5u3.3 | Derive warning and routes from active telemetry; open |
| UX-06 | Gap | Source trace confirms inconsistent severity thresholds across views | 0.95 / 0.95 | Pass / bundle | BI-5u3.3.3 | BI-5u3.3 | Use a shared severity rule; open |
| UX-07 | Gap | Source trace confirms opportunity explanation does not match score formula | 0.95 / 0.95 | Pass / bundle | BI-5u3.3.4 | BI-5u3.3 | Align displayed explanation with actual score components; open |
| UX-08 | Gap | Default enabled button text calculates below target; other rendered states untested | 0.92 / 0.92 | Pass / bundle | BI-5u3.4.2 | BI-5u3.4 | Correct measured contrast and check interaction/theme states; open |
| UX-09 | Gap | MUI source trace shows accessible name does not reach switch input; AT tree untested | 0.90 / 0.90 | Pass / bundle | BI-5u3.4.3 | BI-5u3.4 | Name the actual input and verify checked meaning; open |
| UX-10 | Gap | Markup lacks live status semantics; exact AT announcements untested | 0.85 / 0.85 | Pass / bundle | BI-5u3.4.4 | BI-5u3.4 | Add status/error relationships and AT spot check; open |
| UX-11 | Gap | Source markup lacks main/heading/tab relationships; rendered navigation untested | 0.80 / 0.85 | Pass / bundle | BI-5u3.4.5 | BI-5u3.4 | Add navigable landmarks, headings, and tab-panel relations; open |
| UX-12 | New concern | Disputed whether Tauri bridge wording misleads; audience/contract unknown | 0.55 / 0.70 | Pass with caveat / defer | BI-5u3.5.1 | BI-5u3.5 | Record audience/copy decision before any wording change; open, validation only |
| UX-13 | New concern | Layout facts are source-visible, but no viewport/task impairment was demonstrated | 0.45 / 0.45 | Flag / defer | BI-5u3.5.2 | BI-5u3.5 | Run viewport/task validation; change layout only if harm is reproduced; open |
| Hosted E2E enabler | Validation gap | Separate `ui-playwright` job exists; aggregate `test` previously did not depend on it | — | Implemented enabler | BI-5u3.1.3 | BI-5u3.1 | Aggregate requires successful Playwright result; hosted Playwright and aggregate passed on `71bf9e5`; child bead closed locally |
| PR #114 CI follow-up: Rust assertion omitted the new aggregate UI gate | Correction | First hosted run failed `ci_workflow_has_aggregate_test_gate` because its expected dependency list lacked `ui-playwright`; the assertion is corrected and the focused Rust test passes locally | 0.95 / 1.00 | Pass / must-fix for Phase 1 | BI-5u3.1.3 | BI-5u3.1 | Corrected CI, aggregate, Security, and Dependency Review passed on `71bf9e5` |
| PR #114 follow-up: imported empty data labels a missing bottleneck as `review` | Correction | Current PR comment confirmed sample fallback leaked into the imported Quality Pulse metric and explainability trace; the fix returns unavailable when sample fallbacks are disabled | 0.90 / 0.95 | Pass / must-fix for Phase 1 | BI-5u3.1.4 | BI-5u3.1 | Unit, component, browser assertions and hosted UI/aggregate checks passed on `71bf9e5`; child bead closed locally |
| PR #114 follow-up: missing imported commits are presented as low risk | Correction | Current PR comment confirmed the empty risk identifier and good status implied an assessment without observations; the fix shows unavailable with neutral status | 0.95 / 0.95 | Pass / must-fix for Phase 1 | BI-5u3.1.5 | BI-5u3.1 | Unit, component, browser assertions and hosted UI/aggregate checks passed on `71bf9e5`; child bead closed locally |
| PR #114 follow-up: absent risk/stage observations appear healthy | Correction | Current PR comment confirmed empty arrays showed green no-risk/zero-pressure results; the fix uses unavailable neutral cards without records | 0.95 / 0.95 | Pass / must-fix for Phase 1 | BI-5u3.1.6 | BI-5u3.1 | Unit, component, browser assertions and hosted UI/aggregate checks passed on `71bf9e5`; child bead closed locally |
| PR #114 follow-up: populated imports are still labeled as awaiting telemetry | Correction | Current PR comment confirms `buildRoutes(false)` suppressed every imported route; routes now derive per audience from actual records and report healthy stages/no security findings as observed results | 0.95 / 0.95 | Pass / must-fix for Phase 1 | BI-5u3.1.8 | BI-5u3.1 | Local type/unit/build and hosted UI/Playwright/aggregate/Security checks passed on `71bf9e5`; child bead closed locally |

**Classification:** 13 findings; 0 must-fix, 11 bundle, 2 defer, 0 informational. Actionability filter: 12 pass, 1 flag, 0 dropped. UX-03, UX-12, and UX-13 have partial context and keep explicit decision or evidence gates.

## Dissent and governance

- **UX-03:** expected preview behavior is unsettled. Human review is required before adding control behavior or a data source; a visible preview-only/unavailable state is acceptable under the current contract.
- **UX-12:** the audience and intended bridge-error copy are disputed. Record a product decision before changing the wording.
- **UX-13:** responsive harm is unverified. Test representative viewports and tasks; change layout only if a concrete impairment is reproduced.
- **UX-04:** the invalid nested value is a statically traced failure path, not a runtime reproduction.
- No P0/P1 finding qualified for specialist verification under the integrator governance gate. The source panel’s compressed-run limitation remains disclosed.

**Recommendation:** Phase 1 and Phase 2 may proceed within the documented browser-preview contract. Require human review before changing UX-03 behavior or UX-12 copy, and evidence before a UX-13 layout change. This planning record does not close findings or claim implementation.

## Action items

| Priority | Owner | Action | Bead |
|---|---|---|---|
| P2 | Implementer | Preserve explicit empty/import/reset states and reject malformed payloads without discarding the last valid view | BI-5u3.1 |
| P2 | Implementer | Ground provenance, warnings, routes, severity, and score explanations in active input | BI-5u3.3 |
| P2 | Implementer | Make controls, text contrast, form names, status updates, landmarks, and tab relations accessible | BI-5u3.4 |
| P3 | Product owner and implementer | Record preview audience/copy intent and test responsive task fit before deciding on code changes | BI-5u3.5 |
| P2 | CI owner | Require hosted Playwright success in aggregate CI and verify its required-check status | BI-5u3.1.3 |

The complete acceptance criteria, test plans, cross-PR gates, and Beads dependencies are maintained in the [roadmap](remediation-roadmap-2026-10-05.md) and [dated Beads issue snapshot](frontend-ux-remediation-beads-2026-10-05.jsonl). The snapshot omits owner, creator, and timestamp metadata; remote Beads sync remains unverified. UX-specific integration history is append-only in [the dated integration log](frontend-ux-plan-integration_log-2026-10-05.jsonl).

## Implementation review follow-up

The Phase 1 quality review found no blocker in the empty-state copy fix. Its optional note identified an awkward fallback when a populated risk record has no reason factors. The UI now states that no risk factors were provided for that commit, and the unit test covers both the trend rationale and ranking rationale. This was an informational polish follow-up, not an additional panel finding or Beads issue.

The live PR review then found a P2 correction: imported empty/partial telemetry inherited the sample `review` bottleneck label in both the metric and explainability trace. The Phase 1 implementation now represents the absent value as unavailable, keeps the sample fallback for sample mode, and tests both empty and partial data. Bead `BI-5u3.1.4` records the acceptance criteria and test plan; local and hosted checks passed on `71bf9e5`.

The initial hosted CI run also failed its Rust aggregate-job contract because the expected `needs` list had not been updated for `ui-playwright`. The assertion now matches the workflow and its focused Rust test passes locally with the pinned prebuilt DuckDB runtime. Corrected hosted CI and Security passed on `71bf9e5`.

A later PR review found that a missing imported commit still appeared as a blank top-risk identifier with a good status. The trace now shows an unavailable label and neutral status when no commit-risk records are present. Bead `BI-5u3.1.5` records the acceptance criteria and tests; local and hosted checks passed on `71bf9e5`.

The next PR review found empty commit/stage trend lines showed green zero/healthy values despite missing records. They now show Unavailable with neutral status when no observations exist. Bead `BI-5u3.1.6` records the acceptance criteria and tests; local and hosted checks passed on `71bf9e5`.

The next PR review found that complete imported payloads were still presented as awaiting telemetry because the route builder treated all imported modes as empty. Bead `BI-5u3.1.8` records the refined per-audience criteria. Routes now derive lead actions from commit-risk records, manager actions from observed stage pressure, executive actions from opportunity records, and security results from dependency/automation signals. Audiences without their relevant imported records remain in the awaiting state; sample routes are unchanged. Local typecheck, all 79 UI tests, build, and hosted UI/Playwright/aggregate/Security checks passed on `71bf9e5`; the child bead is closed locally.
