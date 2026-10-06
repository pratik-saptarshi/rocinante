# Frontend UX Remediation Roadmap — 2026-10-05

**Status (updated 2026-10-06):** Phase 0 planning is complete. Phase 1 implementation has run in the open `fix/frontend-ux-phase-1` worktree, and local validation results and environment limitations are recorded in the [Phase 1 validation note](frontend-ux-phase-1-closeout-2026-10-06.md). Phase 1 remains open: hosted same-head UI Playwright and aggregate results are unverified. Branch protection is verified at main `d4bc8e7`: the protected branch requires `test` and `codeql`, and aggregate `test` includes `ui-playwright`. Related Beads remain open. Phases 2–4 remain planned.
**Source review:** [Frontend UX adversarial panel](../reviews/2026-10-05-frontend-user-experience/review_panel_report.md) · [process transcript](../reviews/2026-10-05-frontend-user-experience/review_panel_process.md)
**Panel result:** 3.7/10 mean; 10 P2, 3 P3; no P0/P1. Six targeted source checks confirmed UX-04 through UX-09. The independent Opus judge phase was unavailable and is disclosed in the source report. The per-finding plan integration is in the [frontend UX traceability ledger](frontend-ux-plan-traceability-2026-10-05.md).

## Objective

Remediate verified trust, data-state, control, and accessibility defects in the React/Vite browser preview. Validate the two conditional findings before deciding whether they need code changes. Keep each implementation milestone small, testable, and reviewable through a protected pull request.

The scope is the browser preview in `ui/`. This roadmap does not establish that the separate Rust desktop shell has the same defects or authorize changing its native behavior or retained command/payload contracts.

## Inputs and current constraints

- Review findings and evidence: [review report](../reviews/2026-10-05-frontend-user-experience/review_panel_report.md), [process transcript](../reviews/2026-10-05-frontend-user-experience/review_panel_process.md), [HTML report](../reviews/2026-10-05-frontend-user-experience/review_panel_report.html), and its `state/` records.
- UI source of truth: `ui/src/`; UI entry and test contracts are summarized in `ui/codemap.md`.
- Current package manager pin: pnpm `12.9.1`.
- Hosted `ui-quality` runs frozen install, `pnpm exec tsc -b`, `pnpm exec vitest run`, and `pnpm run build`. The separate `ui-playwright` job installs Chromium and runs `pnpm run test:e2e`; Phase 1 makes the aggregate `test` depend on it and fail closed on failure/skip. Bead `BI-5u3.1.3` tracks this aggregate gate. Branch API confirmed that protected `main` at `d4bc8e7` requires `test` and `codeql`; the required aggregate `test` depends on `ui-playwright`. Do not claim hosted E2E acceptance until the same-head job passes.
- No standalone UI lint script is defined in `ui/package.json`. Do not report a separate UI lint pass unless a lint command is added and run.
- This workspace already contains unrelated dirty native-shell, macOS, and readiness work. Keep it intact. Do not use that dirty branch as the UX implementation base.
- The user reported that the native application did not come up cleanly during a macOS acceptance attempt. That incident is outside this frontend review. Track it through the existing [desktop parity record](desktop-parity-matrix.html) and [readiness tracker](bead-issue-tracker.html); do not claim native acceptance based on browser-preview work.
- This roadmap records the frontend review scope and plan. Refresh `main`, matching branches, and required checks before each implementation milestone; do not carry old CI results forward as same-head evidence.

## Plan-integrator disposition

The panel marks UX-01–UX-11 as source/dependency-supported defects or concrete behavior concerns, with UX-03 qualified because intended preview behavior is not settled. UX-12 is conditional on audience/contract intent; UX-13 has no demonstrated viewport or task impairment.

| Finding | Priority | Evidence | Plan relation | Actionability | Groundedness | Filter | Context | Action category | Governance gate | Bead / milestone | Planned disposition |
|---|---|---|---|---:|---:|---|---|---|---|---|---|
| UX-01 | P2 | [VERIFIED] | Gap | 0.95 | 0.95 | Pass | full | Bundle | none | BI-5u3.1.1 / M1 | Preserve explicit empty/import/reset state. |
| UX-02 | P2 | [VERIFIED] | Gap | 0.87 | 0.95 | Pass | full | Bundle | none | BI-5u3.3.1 / M2 | Label static section provenance. |
| UX-03 | P2 | [PARTIAL] | Gap | 0.82 | 0.80 | Pass with caveat | partial | Bundle with caveat | Conditional scope-expansion gate if adding behavior | BI-5u3.4.1 / M3 | Clarify preview intent; do not add an audit engine or data source by assumption. |
| UX-04 | P2 | [VERIFIED] | Gap | 0.95 | 0.95 | Pass | full | Bundle | none | BI-5u3.1.2 / M1 | Validate schema before committing imported state. |
| UX-05 | P2 | [VERIFIED] | Gap | 0.95 | 0.95 | Pass | full | Bundle | none | BI-5u3.3.2 / M2 | Derive warning/routes from active telemetry. |
| UX-06 | P2 | [VERIFIED] | Gap | 0.95 | 0.95 | Pass | full | Bundle | none | BI-5u3.3.3 / M2 | Unify severity classification. |
| UX-07 | P2 | [VERIFIED] | Gap | 0.95 | 0.95 | Pass | full | Bundle | none | BI-5u3.3.4 / M2 | Align explanation and scoring formula. |
| UX-08 | P2 | [WEB-VERIFIED] | Gap | 0.92 | 0.92 | Pass | full | Bundle | none | BI-5u3.4.2 / M3 | Meet default-state text contrast and verify interaction states. |
| UX-09 | P2 | [VERIFIED] | Gap | 0.90 | 0.90 | Pass | full | Bundle | none | BI-5u3.4.3 / M3 | Name the actual switch input. |
| UX-10 | P2 | [VERIFIED] | Gap | 0.85 | 0.85 | Pass | full | Bundle | none | BI-5u3.4.4 / M3 | Expose accessible result/status semantics. |
| UX-11 | P3 | [VERIFIED] | Gap | 0.80 | 0.85 | Pass | full | Bundle | none | BI-5u3.4.5 / M3 | Add navigable page/tab semantics. |
| UX-12 | P3 | [DISPUTED] | New concern | 0.55 | 0.70 | Pass with caveat | partial | Defer / decision spike | Human decision before copy change | BI-5u3.5.1 / M4 | Resolve intended browser-preview audience and bridge wording; do not alter copy before decision. |
| UX-13 | P3 | [UNVERIFIED] | New concern | 0.45 | 0.45 | Flag for human review | partial | Defer / validation spike | none | BI-5u3.5.2 / M4 | Test representative viewports/tasks; make no layout change unless impairment is reproduced. |

**Classification totals:** Total 13 · Must-fix (P0/P1): 0 · Bundle into milestones: 11 · Defer as validation/decision spikes: 2 · Informational: 0. Actionability filter: 12 pass, 1 flag, 0 drop.

**[WARNING] Context partial:** UX-03 lacks a settled control contract, UX-12 lacks the intended preview audience/copy contract, and UX-13 lacks rendered viewport/task evidence. Their plans preserve those limits and defer any speculative behavior or layout changes. All P2 findings are planned for the first three implementation PRs. No specialist verification was triggered because there are no P0/P1 findings; the panel’s compressed-run limitation remains a caveat.

## Scope decisions

1. **Keep browser and native work separate.** The browser findings do not establish native shell defects, and browser validation cannot close the separately reported desktop startup instability. If a writable open branch is found for this exact frontend scope at execution time, reuse it. Otherwise start a clean worktree from freshly verified `origin/main`.
2. **Do not implement new product functionality by assumption.** The repository documents React/Vite as a browser compatibility preview, which is the current scope. Run Full Audit, SEO, and Field/Lab controls without an implemented contract must be visibly preview-only or unavailable. Implement real audit or telemetry behavior only after its data source and owner decision are defined.
3. **Keep input compatibility stable.** Validate preview payloads without changing existing public command or payload shapes. Reject malformed nested values before replacing active UI state.
4. **Require same-head evidence.** A milestone is complete only when its focused local checks pass and all applicable hosted required checks terminate green on the PR head being merged.
5. **Be explicit about E2E hosting.** The hosted Playwright job is separate from `ui-quality`; require its result in the aggregate gate through `BI-5u3.1.3`; refresh branch-protection requirements during each execution because the base branch may change.

## Phases and PR journey

### Phase 0 — Establish a clean baseline and preview contract (complete — 2026-10-05)

**Bead:** `BI-5u3` (program epic)
**Completed snapshot (local 2026-10-05):** planning was based on `main` SHA `eb83be9da64057dc71838b33edc01a9a2769b0fa`. The reported native app startup instability and platform acceptance remain separate from this browser-preview workstream; this roadmap records no native acceptance result.
**Preview contract:** the repository documents React/Vite as a browser compatibility preview, separate from the production desktop transport. This documented boundary is the current scope. Controls without an implemented contract must be visibly preview-only or unavailable until a behavior contract exists.
**Exit:** the baseline and preview boundary are recorded. Phase 1 was the next planned work and has since run; see its implementation and validation snapshot below. Hosted check results are not evidence for frontend acceptance unless they apply to the exact PR head.

<a id="phase-1"></a>
### Phase 1 — implementation run; hosted gates and completion pending

**Protect payload state and recovery (PR 1)**

**Milestone:** [BI-5u3.1](#phase-1)
**Findings:** UX-01, UX-04
**Commit slices:**
- `fix(ui): preserve explicit empty telemetry state`
- `fix(ui): validate dashboard payload before state updates`
- `test(ui): cover import state and last-good recovery`

**Acceptance criteria:**
- Explicit empty arrays display a deliberate empty state; missing, empty, sample, and imported modes are distinguishable. Empty and partial imports do not show sample-derived recommendations or hard-coded action-route instructions; the score is unavailable unless imported telemetry produces at least one commit-risk and one bottleneck record; empty arrays mean no observations, not perfect quality. Empty-state visual and security copy does not refer to a sample window.
- Empty and partial imports without bottleneck records show an unavailable top-bottleneck metric and explanation, rather than the sample-mode `review` fallback. Sample and populated imported modes retain their current labels. Bead `BI-5u3.1.4` tracks the PR review follow-up.
- Empty and partial imports without commit-risk records show an unavailable Top Risk Commit trace with neutral status and no blank identifier. Sample mode and populated imported commit IDs retain their existing behavior. Bead `BI-5u3.1.5` tracks this PR review follow-up.
- Blank Apply does not silently reset data. Reset to Sample is explicit and labels the resulting data as sample.
- Nested shape and numeric validation runs before replacing active state. The object-valued `stages[].name` case is rejected with a visible, accessible error.
- Invalid JSON or schema leaves the last valid view usable; valid existing payloads continue to work.
- The browser does not crash or render an object as a React child for invalid inputs.

**Test plan:**
- Unit tests for missing, empty, valid, and malformed records and finite numeric boundaries.
- Component tests for blank Apply, explicit reset, imported/sample/empty labels, accessible validation errors, and last-good state.
- Unit/component tests for empty, partial, sample, and populated top-bottleneck labels and explanation copy; assert that imported no-data never says `review`.
- Unit/component/browser tests for empty and partial Top Risk Commit traces; assert unavailable copy/status when no commit-risk record exists.
- Extend `ui/e2e/app.spec.ts` with empty/import/reset and malformed-import recovery flows.
- Run from `ui/` using pnpm `12.9.1`: `pnpm install --frozen-lockfile`, `pnpm exec tsc -b`, `pnpm exec vitest run`, `pnpm run build`, and `pnpm run test:e2e`.
- Hosted `ui-quality`, `ui-playwright`, and aggregate `test` must pass at the same commit. The `ui-playwright` job and aggregate dependency have been added; their same-head hosted results and branch-protection required-check status remain unverified.

**PR gate:** keep this focused on React preview import/state. Merge through protected PR after same-head checks pass; then base Phase 2 on the merged main tip.

**Local implementation snapshot (2026-10-06):** Payload validation preserves omitted collections separately from explicitly empty arrays. Demo seed data is only used by the no-payload sample/reset view; imported partial payloads leave omitted collections empty and show a partial state. Component and unit coverage covers missing/partial/empty semantics, last-good recovery, and suppressing sample recommendations/action routes for imports. Empty-state, top-risk, and bottleneck copy stays source-neutral when no records are present. A browser test now applies valid data, enters malformed JSON, checks the announced error, and confirms the prior dashboard remains visible. The aggregate `test` job depends on `ui-playwright` and fails if that job fails or is skipped; a workflow contract prevents adding an unreviewed conditional skip. Local results and limitations are recorded in [the Phase 1 validation note](frontend-ux-phase-1-closeout-2026-10-06.md).

The first hosted run after making Playwright an aggregate dependency exposed a stale Rust assertion for the aggregate dependency list. The assertion now includes `ui-playwright`, and the focused Rust contract test passes locally. A further PR review found absent imported commits still appear as a blank Top Risk Commit with a good status; the local follow-up now reports unavailable and has regression coverage. Same-head hosted aggregate and Security checks must pass after these corrections.

Compatibility note: public envelope and field names remain stable, including the legacy/null envelope form. Validation now rejects incomplete rows and malformed/non-finite limits that older imports may have passed through; callers must provide complete records. The last-good view is retained on rejection. See the closeout note for details.

The pinned pnpm 12.9.1 executable was unavailable locally (host pnpm reported 12.8.1 and hung); existing `node_modules/.bin` tools were used for TypeScript, Vitest, and build. The repository actionlint command passes with its configured ignore for the known `vulnerability-alerts` scope diagnostic; unignored actionlint reports that diagnostic. Local Playwright is blocked because the sandbox rejects binding `127.0.0.1:4173` with `EPERM`. Hosted `ui-playwright` and aggregate `test` are pending. Branch protection was verified at base `d4bc8e7`: protected `main` requires `test` and `codeql`, with `ui-playwright` transitively required through `test`. Keep `BI-5u3.1`, `BI-5u3.1.1`, `BI-5u3.1.2`, `BI-5u3.1.3`, `BI-5u3.1.4`, and `BI-5u3.1.5` open until their hosted criteria are met.

### Phase 2

**Make metrics and recommendations data-grounded (PR 2)**

**Milestone:** [BI-5u3.3](#phase-2)
**Findings:** UX-02, UX-05, UX-06, UX-07
**Dependency:** Phase 1 merged.
**Commit slices:**
- `fix(ui): label static dashboard provenance`
- `fix(ui): derive recommendations from active telemetry`
- `fix(ui): unify stage severity and score explanations`
- `test(ui): cover data-grounded dashboard guidance`

**Acceptance criteria:**
- Every static section clearly identifies sample/example provenance, including after telemetry import; sections derived from imported telemetry identify that source.
- Healthy complete input does not show unconditional red Manager warnings or unrelated routes. Empty input has an honest empty/healthy state.
- Any retained canned A-124 route is labeled illustrative and cannot be mistaken for an action derived from the current payload.
- Views that show the same stage status use the same classifier and configured latency threshold.
- Explainability text matches score components. Opportunity signals are not described as changing the score unless the formula actually includes them.

**Test plan:**
- Unit-test score components, opportunity invariance/contribution, severity boundaries, healthy/critical/empty recommendations.
- Component-test source labels and route/recommendation records against current input.
- Playwright verifies healthy and critical payloads produce the expected guidance and no false/sample action.
- Run UI typecheck, full Vitest, production build, and Playwright; require same-head hosted `ui-quality`.

**PR gate:** keep scoring/recommendation changes derived from the preview’s current input model. Do not port them to the Rust shell or alter native command contracts in this PR.

### Phase 3

**Make controls and dashboard interactions accessible (PR 3)**

**Milestone:** [BI-5u3.4](#phase-3)
**Findings:** UX-03, UX-08, UX-09, UX-10, UX-11
**Dependency:** Phase 2 merged.
**Commit slices:**
- `fix(ui): clarify preview-only controls`
- `fix(ui): meet primary text contrast requirements`
- `fix(a11y): name switches and announce dashboard status`
- `fix(a11y): add page landmarks and tab relationships`
- `test(ui): cover accessible dashboard interaction`

**Acceptance criteria:**
- No enabled control appears to perform a data operation while silently changing nothing. When intent is intentionally illustrative, label it preview-only/unavailable; implement real behavior only after its data contract is defined.
- Normal-size enabled primary button text meets at least 4.5:1 contrast in default contained and outlined states; check hover, focus, disabled, and theme variants separately.
- Field/Lab input has a meaningful accessible name on the interactive switch input, not only the MUI wrapper; checked state is clear.
- Admin/baseline results, successful updates, and errors expose appropriate accessible status semantics; textarea errors are programmatically associated.
- Main content has a main landmark, meaningful heading hierarchy, and SEO tab/panel ID and ARIA relationships.

**Test plan:**
- Testing Library checks role/name/checked for switch, actual control results, success/error status roles, input error relationships, main landmark, heading levels, and tab panels.
- Add a deterministic contrast check for tokens plus browser-computed foreground/background contrast for rendered button states. Do not claim a whole-app WCAG audit from this check.
- Run keyboard-only task paths and a screen-reader spot check for announcements and heading/tab navigation.
- Extend Playwright for pointer/keyboard control flows. Run UI typecheck, Vitest, build, Playwright, and same-head required CI.

**PR gate:** use the smallest changes that accurately describe preview behavior. Do not add a real audit engine or a second data source under this accessibility PR.

### Phase 4

**Resolve audience and test responsive task fit**

**Milestone:** [BI-5u3.5](#phase-4)
**Findings:** UX-12, UX-13
**Dependency:** Phase 3 merged. This phase is a validation gate, not a pre-approved redesign.

**Acceptance criteria:**
- A product decision records the browser preview audience and whether bridge errors are contract-test guidance or end-user recovery copy.
- Browser fallback messaging accurately names the browser preview and gives a next action if the decision establishes that the current Tauri wording misleads. If current wording is intentional for the harness, document rationale and close without code.
- Responsive evidence covers 320, 360, 768, 1024, and 1280 CSS-pixel widths plus 200% zoom.
- Representative tasks cover sample load, telemetry import, recommendation reading, and admin/baseline status. Evidence records overflow, clipped controls/text, focus, and section-order friction.
- No layout changes are approved unless testing reproduces a concrete user problem. If none is reproduced, close UX-13 with evidence and no code change.

**Test plan:**
- Add/extend Playwright viewport scenarios; save screenshots, traces, and task notes as review artifacts.
- Check no horizontal page overflow, clipped content, unreachable controls, or lost visible focus.
- If a code fix is justified, create a separate scoped PR 4 with its own acceptance criteria and same-head checks; otherwise record closure evidence in the roadmap and Beads.

## Cross-PR validation matrix

| Gate | Phase 1 | Phase 2 | Phase 3 | Phase 4 |
|---|---:|---:|---:|---:|
| `pnpm install --frozen-lockfile` using pin 12.9.1 | ✓ | ✓ | ✓ | ✓ |
| `pnpm exec tsc -b` | ✓ | ✓ | ✓ | ✓ |
| `pnpm exec vitest run` | ✓ | ✓ | ✓ | ✓ |
| `pnpm run build` | ✓ | ✓ | ✓ | ✓ |
| `pnpm run test:e2e` | ✓ | ✓ | ✓ | ✓ |
| Hosted UI required checks on exact PR head | ✓ | ✓ | ✓ | If code PR |
| Manual accessibility/viewport evidence | — | — | keyboard + AT | viewport + task |
| No public payload or native contract changes | ✓ | ✓ | ✓ | ✓ |

The check marks above are **planned gates**, not completed results. Phase 1 local typecheck, Vitest, and production build results are recorded in the validation note; local Playwright could not run because sandbox networking denied the web-server bind. The hosted `ui-playwright` job and aggregate dependency have been added under `BI-5u3.1.3`, but same-head hosted results remain unverified. The protected `main` branch requires `test` and `codeql`; since required `test` depends on `ui-playwright`, the browser job is transitively required, though it is not a separate branch-protection context. Report local results separately from hosted CI, keep Phase 1 Beads open, and leave Phases 2–4 planned.

### Hosted Playwright validation gate

`BI-5u3.1.3` owns the hosted browser-test enabler: a UI-triggered job must install the pinned pnpm/browser tooling, run `pnpm run test:e2e`, publish actionable failure output, and record which required workflow context contains the Playwright job. This heading is the stable `spec_id` target for that issue. The `ui-playwright` job and aggregate dependency have been added; same-head hosted results remain unverified. The protected `main` branch requires `test` and `codeql`; since required `test` depends on `ui-playwright`, the browser job is transitively required, though it is not a separate branch-protection context.

## Progressive commit and PR policy

For each milestone:

1. Refresh `main`, matching PR branches, and required check configuration before creating or updating a branch. Preserve unrelated dirty work and use a clean worktree.
2. Inspect existing PR branches for matching scope and write access. Reuse a writable branch only when it contains this UX scope. If a same-scope branch is not writable/outdated, follow the existing preference to close an unnecessary branch rather than use a complicated workaround.
3. Create milestone commits using the Conventional Commit slices above. Push the scoped branch and open one focused PR against `main`; do not merge until every required check is terminal and green on that exact head.
4. Merge one milestone before basing the next PR, avoiding a long dependent stack. If an implementation change is unnecessary after validation, record its evidence instead of opening an empty PR.
5. Update Beads states and this roadmap only after the implementation and required validation evidence exist. Never mark hosted checks green based on local results or pending runs.

## Finding-to-Bead traceability

| Finding / enabler | Bead | Milestone | Planned status |
|---|---|---|---|
| UX-01 | `BI-5u3.1.1` | M1 (`BI-5u3.1`) | Implementation present; local checks passed; same-head hosted gate pending; bead remains open and remote sync unverified |
| UX-02 | `BI-5u3.3.1` | M2 (`BI-5u3.3`) | Open / planned |
| UX-03 | `BI-5u3.4.1` | M3 (`BI-5u3.4`) | Open / planned; preview intent caveat |
| UX-04 | `BI-5u3.1.2` | M1 (`BI-5u3.1`) | Implementation present; local type/unit/build checks passed; same-head hosted Playwright gate pending; bead remains open and remote sync unverified; original failure path was statically traced, not runtime reproduced |
| UX-05 | `BI-5u3.3.2` | M2 (`BI-5u3.3`) | Open / planned |
| UX-06 | `BI-5u3.3.3` | M2 (`BI-5u3.3`) | Open / planned |
| UX-07 | `BI-5u3.3.4` | M2 (`BI-5u3.3`) | Open / planned |
| UX-08 | `BI-5u3.4.2` | M3 (`BI-5u3.4`) | Open / planned; default button state only |
| UX-09 | `BI-5u3.4.3` | M3 (`BI-5u3.4`) | Open / planned |
| UX-10 | `BI-5u3.4.4` | M3 (`BI-5u3.4`) | Open / planned; AT behavior untested |
| UX-11 | `BI-5u3.4.5` | M3 (`BI-5u3.4`) | Open / planned |
| UX-12 | `BI-5u3.5.1` | M4 (`BI-5u3.5`) | Open / validation only; disputed |
| UX-13 | `BI-5u3.5.2` | M4 (`BI-5u3.5`) | Open / validation only; impact unverified |
| Hosted E2E enabler | `BI-5u3.1.3` | M1 (`BI-5u3.1`) | Open / planned; hosted Playwright pending |
| PR review: no sample bottleneck for imported no-data | `BI-5u3.1.4` | M1 (`BI-5u3.1`) | Fix and regression coverage present locally; hosted same-head checks pending |
| PR review: no blank or low-risk top-risk trace for imported no-data | `BI-5u3.1.5` | M1 (`BI-5u3.1`) | Fix and regression coverage present locally; hosted same-head checks pending |

Beads are repository-local and currently unsynced. The database was initialized because no `.beads` workspace existed. The initialization could not resolve GitHub DNS while checking the configured Dolt remote, so remote issue history and sync status are not verified. The `BI-...` IDs above are created locally; verify remote sync and collision status before treating them as shared team issues. The tracked JSONL snapshot preserves issue content, acceptance criteria, spec links, labels, and dependencies while omitting owner, creator, and timestamp metadata; the local Beads database remains authoritative for those fields.

## Review integration decisions

- **UX-03:** Preserve the finding that controls do not produce their apparent data/action change, but do not assume the preview needs a full audit engine. Product intent is missing; the bead accepts either a defined behavior or a clear preview-only/unavailable state.
- **UX-04:** Validate malformed input before state replacement; add a render boundary only if implementation design supports it as defense in depth.
- **UX-08:** Scope remediation to the measured default button pair and verify other relevant states; do not claim an overall WCAG pass from one contrast fix.
- **Independent judge limitation:** The panel lacked its required independent Opus Phase 14 judge. Preserve the compressed-run caveat and do not treat the synthesis as independent approval.

## Dissent Ledger

| Finding | Positions | Evidence summary | Escalation result | Decision owner |
|---|---|---|---|---|
| UX-12 | Product trust saw likely misleading Tauri-specific recovery wording; usability noted that the compatibility harness may intentionally expose Tauri validation. | Source confirms the browser-preview/native boundary and Tauri-specific wording, but not the intended audience or copy contract. | No independent Opus judge was available. Defer copy changes until the audience and contract are recorded. | Product owner |
| UX-13 | Reviewers raised the fixed 340px column and section order; none established clipping, task failure, or excessive scrolling. | Source confirms layout facts only. No viewport, zoom, or representative task evidence exists. | Retain as an unverified validation spike; authorize no layout change without a reproduced impairment. | UX/product reviewer |

## Action Items

| Priority | Owner | Action | Source finding |
|---|---|---|---|
| P2 | UI implementer | Deliver Phase 1 for explicit empty/import/reset state, schema validation, and honest no-data risk/bottleneck semantics; retain the last good view on invalid input. | UX-01, UX-04, BI-5u3.1.4, BI-5u3.1.5 |
| P2 | CI/UI maintainer | Run hosted Playwright on the exact PR head and inspect its artifacts; preserve its fail-closed dependency in the required aggregate `test` gate before claiming hosted E2E. | BI-5u3.1.3 |
| P2 | UI implementer | Deliver Phase 2 with input-derived recommendations, shared severity rules, and score explanations tied to computed contributors. | UX-02, UX-05, UX-06, UX-07 |
| P2 | Product owner and UI implementer | Decide whether the controls are illustrative/unavailable or have a defined behavior before any new operation/data source is implemented; complete the Phase 3 accessibility fixes. | UX-03, UX-08, UX-09, UX-10, UX-11 |
| P3 | Product owner | Record the browser-preview audience and bridge-copy contract before changing UX-12 wording. | UX-12 |
| P3 | UX/product reviewer | Run viewport, zoom, and representative-task validation; close UX-13 without code if no impairment is reproduced, otherwise scope a separate PR. | UX-13 |
| P2 | Beads maintainer | Verify locally created IDs against the remote board and sync only after connectivity returns; do not report remote sync before confirmation. | BI-5u3 |

## Plan-integrator traceability

The disposition table above is canonical; the dedicated [frontend UX traceability ledger](frontend-ux-plan-traceability-2026-10-05.md) summarizes each finding’s relationship, evidence, filter result, category, and bead. The root epic is `BI-5u3`; milestone beads are `BI-5u3.1`, `BI-5u3.3`, `BI-5u3.4`, and `BI-5u3.5`. Finding beads retain their refined acceptance criteria and test plans in the [dated Beads issue snapshot](frontend-ux-remediation-beads-2026-10-05.jsonl). UX integration history is in [`frontend-ux-plan-integration_log-2026-10-05.jsonl`](frontend-ux-plan-integration_log-2026-10-05.jsonl).

## Finding specifications and stable Bead anchors

Each heading below provides the fragment target referenced by its finding Bead `spec_id`. These sections state the planned scope and evidence limits; they do not mark findings implemented or validated.

### UX-01

**Preserve explicit empty telemetry.** Keep missing, empty, sample, and imported data states distinct; blank Apply must not reset the current view. Evidence: `ui/src/insight-engine.ts:146-153` and `ui/src/App.tsx:190-213`. Validation is planned in Phase 1.

### UX-02

**Show the source of static results.** Identify every static audit/security/performance section as sample or example data, including after telemetry import. Evidence: `ui/src/App.tsx:704,737-744,763-768,787-788` and `ui/src/dashboard-content.ts:35-55`. Validation is planned in Phase 2.

### UX-03

**Make control behavior honest.** Run Full Audit, SEO, and Field/Lab controls without an implemented contract must be visibly preview-only or unavailable. Do not add an audit engine or new data source by assumption. Evidence: `ui/src/App.tsx:705-714,733-745,787-798`; expected product behavior remains qualified. Validation is planned in Phase 3.

### UX-04

**Reject invalid telemetry before state replacement.** Validate nested values such as object-valued `stages[].name` and preserve the last good view. The reported React-child failure is a static source trace, not a runtime reproduction. Evidence: `ui/src/App.tsx:197-203,360-363,135-145`, `ui/src/insight-engine.ts:126`, and `ui/src/domain/quality-pulse.ts:178`. Validation is planned in Phase 1.

### UX-05

**Derive recommendations from active input.** Healthy, empty, and critical datasets must produce truthful warnings/routes; any illustrative route must be labeled as sample. Evidence: `ui/src/domain/quality-pulse.ts:75-107,138-160` and `ui/src/App.tsx:365-390`. Validation is planned in Phase 2.

### UX-06

**Use one stage-severity rule.** Align dashboard views on a shared configurable latency classification, or explicitly distinguish their criteria. Evidence: `ui/src/App.tsx:178-187,441-451,595-607` and `ui/src/insight-engine.ts:107-131`. Validation is planned in Phase 2.

### UX-07

**Keep score explanations aligned with the formula.** Opportunity signals must not be described as boosting the score unless the implemented formula includes them. Evidence: `ui/src/dashboard-explainability.ts:35-43` and `ui/src/domain/quality-pulse.ts:163-180`. Validation is planned in Phase 2.

### UX-08

**Meet primary-button text contrast.** Review evidence calculates 4.2884:1 for the default enabled button text. Verify other interaction and theme states after a fix; this finding is not a whole-app WCAG audit. Evidence: `ui/src/main.tsx:6-12` and the installed MUI palette/button styles. Validation is planned in Phase 3.

### UX-09

**Name the switch input.** Put an accessible name on the interactive Field/Lab switch input or associate its visible label; verify checked meaning. Evidence: `ui/src/App.tsx:790-798` and installed MUI Switch/SwitchBase prop forwarding. Rendered AT behavior remains untested. Validation is planned in Phase 3.

### UX-10

**Expose results and errors accessibly.** Use appropriate status/error semantics for admin, baseline, import, and reset outcomes, and associate payload errors with the textarea. Evidence: `ui/src/admin-bridge-panel.tsx:34-36` and `ui/src/App.tsx:190-213,463-490,544-546`. AT announcement behavior remains untested. Validation is planned in Phase 3.

### UX-11

**Add navigable page and tab structure.** Provide a main landmark, meaningful headings, and explicit tab/panel relationships. Evidence: `ui/src/App.tsx:249-268,280,315,342,733-745` and installed MUI Typography mapping. Validation is planned in Phase 3.

### UX-12

**Resolve browser-preview audience before changing bridge wording.** Tauri-specific fallback text and the documented browser-preview boundary are both source-visible, but whether the wording misleads is disputed and depends on audience intent. Evidence: `ui/src/tauri-admin.ts:146-150,182-188` and `ui/codemap.md:3-7,16-26`. Phase 4 is validation-only.

### UX-13

**Validate responsive task fit before changing layout.** Record viewport/task evidence at the planned widths and zoom; make a layout change only if a concrete impairment is reproduced. Source layout facts alone do not establish harm. Evidence: `ui/src/App.tsx:248-267,365-390,454-547,551-686`. Phase 4 is validation-only.

**Final Recommendation:** Applied with caveats. Human review remains required before changing UX-03 control behavior or UX-12 copy, and evidence is required before any UX-13 layout change. Phase 1 implementation is in progress on PR #114 with local unit/type/build evidence; the new bottleneck fallback fix is covered by tests. Its hosted Playwright/aggregate gate remains pending; browser execution, accessibility validation, and later phases are not complete.
