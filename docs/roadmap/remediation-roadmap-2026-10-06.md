# Frontend UX Remediation Roadmap — 2026-10-06

**Status (2026-10-06):** Phase 1 and Phase 2 are complete. PR #114 and documentation closeout PR #116 merged with exact-head checks green; PR #117 closed all five outstanding PR #112 documentation threads, and a current GitHub refresh reports all 9/9 PR #112 review threads resolved. PR #118 merged Phase 2 at `ce1e05d8459ee12392c474365bb8e120780ecb37`; exact-head CI (`37430967217`), Security (`37430967218`), and Dependency Review (`37430967207`) passed. Phase 3 is being implemented on `fix/ux-phase3-controls-accessibility-2026-10-06`, branched from that main head. Hosted checks, keyboard-only acceptance, and screen-reader spot checks for Phase 3 are pending. PR #114 merged into `main` as `b204f83b2d69655002018038ff59e1b14f87f37a`; code head `a334cf900b61f5273d4005a05cdf71b79e7cc6b7` passed local TypeScript, 13 Vitest files / 84 tests, production build, and diff checks, plus hosted aggregate CI (`37419778717`), Security (`37419778678`), and Dependency Review (`37419778673`). The later merge head passed CI (`37423517717`), Security (`37423517696`), and Dependency Review (`37423517727`). Documentation/Beads closeout PR #116 merged as `6fcc8d75b491898a7a07dc4875fb4b4555578107`; exact head `5172c28f9c86feb3eb246c279a955fed594fe0ee` passed CI (`37424524116`), Security (`37424524195`), and Dependency Review (`37424524140`). All Phase 1 review threads are resolved. PR #112 merged as `035c290662e2a4c79e247de6036886ea90e7bf60`. PR #117 reconciled its documentation follow-up, and the latest GitHub query shows all 9 review threads resolved; the manual physical tray interaction remains a separate acceptance gate.

**Inputs:** [2026-10-05 UX adversarial panel report](../reviews/2026-10-05-frontend-user-experience/review_panel_report.md), [panel process and evidence](../reviews/2026-10-05-frontend-user-experience/review_panel_process.md), and the current local Beads hierarchy.

## Purpose and scope

Address the panel’s React/Vite browser-preview findings through sequenced, reviewable milestones. Keep browser-preview changes separate from the Rust native shell and its command/payload compatibility contracts. Do not infer that the panel reviewed native-shell usability.

The panel scored the preview 3.7/10 and raised 10 P2 and 3 P3 findings, with no P0/P1 findings. It reviewed source and installed dependencies; it did not run the UI, observe rendered accessibility output, or conduct user research. The independent Opus judge stage was unavailable, so the report is explicitly compressed and medium-confidence. UX-12 is disputed and UX-13 is unverified; neither authorizes speculative code changes.

## Current state and evidence boundary

- The root worktree is dirty on `fix/weighted-rollup-aggregation`. Preserve it; do UI implementation in a clean isolated worktree.
- The panel reviewed local source at `e9d6d3a`. PR #114 delivered Phase 1 code and merged on 2026-10-06. Because two documentation review threads remained open at merge, PR #116 is the Phase 1 documentation closeout vehicle.
- Local Beads records `BI-5u3.1.1`–`.1.8` as closed against PR #114 code head `71bf9e5`; their close notes record local typecheck, 13 Vitest files / 79 tests, production build, Playwright discovery, and hosted CI/security/dependency-review passes on that earlier code head. These records do not close the parent milestone.
- Live GitHub was refreshed on 2026-10-06. PR #114 merged as `b204f83b2d69655002018038ff59e1b14f87f37a` after branch head `b15118411dea0d2104eecffca74d17c6f66200ab` passed CI (`37423517717`), Security (`37423517696`), and Dependency Review (`37423517727`). Docs closeout PR #116 merged as `6fcc8d75b491898a7a07dc4875fb4b4555578107`; its three exact-head workflows passed and all PR #114 threads are resolved. Auto-merge completed the protected flow.
- PR #112 is merged as `035c290`; all nine review threads are currently resolved according to the live GitHub thread query. Physical macOS tray Show/Quit, foreground restoration, warm URL delivery, and saved-state restart remain separate human/installed-app gates; documentation follow-up does not claim those interactions were physically witnessed.
- The user-reported unstable startup has not been tied to a specific binary or cause. The [dated macOS shutdown remediation plan](macos-native-shutdown-remediation-2026-10-06.md) defines a current-build reproduction and evidence sequence; the [decision record](../decisions/decision-2026-10-06.md) keeps the hard-exit candidate gated on installed-app acceptance. DuckDB remains prebuilt-only.
- The isolated checkout `/private/tmp/rocinante-ux-phase-plan` preserves the review history; its three documentation files were published as PR #116. Code validation on code head `a334cf9` passed: TypeScript, Vitest (13 files / 84 tests), production build, and `git diff --check`. Playwright discovery finds all 8 tests; local browser execution remains blocked by sandbox `EPERM` binding `127.0.0.1:4173`.
- The 2026-10-06 Beads snapshot contains 26 unique issues: 16 open and 10 closed. `bd lint` passed with 16 open issues checked. Children `.1.9` and `.1.10` have local closure evidence tied to `a334cf9`; `.1.3` hosted Playwright evidence is recorded against the same code head in the merged PR docs. The Phase 1 parent is closed. Remote Beads synchronization remains unverified.
- `BI-5u3.1.3` and all Phase 1 child records are closed; hosted Playwright passed on the exact code head as part of CI `37419778717` and was reconfirmed by the later PR merge head.
- UI package pin is pnpm `12.9.1`. No standalone UI lint script is defined; do not report a separate UI lint result unless one is added and run.

## Plan-integrator classification

The panel’s stated P2/P3 severities map to effective P2/P3 under the integrator rules. Since none is P0/P1, no finding is classified as an immediate must-fix by severity. Verified, actionable P2 findings are bundled into implementation milestones. Disputed or unverified P3 concerns are deferred to validation spikes. The added PR #114 comment is treated as a P2 implementation blocker for the open PR, with partial context until its current source path and CI are refreshed.

| Finding | Epistemic label | Actionability | Context | Disposition | Bead / milestone |
|---|---|---:|---|---|---|
| UX-01 missing vs explicit-empty telemetry | VERIFIED | 0.95 | Full | Bundle; implemented on prior #114 head, verify latest head | `BI-5u3.1.1` / M1 |
| UX-02 static section provenance | VERIFIED | 0.87 | Full | Bundle; source labels and active-data provenance | `BI-5u3.3.1` / M2 |
| UX-03 controls imply unavailable actions | PARTIAL | 0.82 | Partial | Bundle with caveat; establish preview control intent, then implement or label | `BI-5u3.4.1` / M3 |
| UX-04 malformed import can reach render | VERIFIED | 0.95 | Full | Bundle; validate before state replacement | `BI-5u3.1.2` / M1 |
| UX-05 false manager warnings/routes | VERIFIED | 0.95 | Full | Bundle; derive recommendations from active records | `BI-5u3.3.2` / M2 |
| UX-06 inconsistent stage severity | VERIFIED | 0.95 | Full | Bundle; share the classifier and configured threshold | `BI-5u3.3.3` / M2 |
| UX-07 score explanation/formula mismatch | VERIFIED | 0.95 | Full | Bundle; align formula and explanation | `BI-5u3.3.4` / M2 |
| UX-08 default primary text contrast | WEB-VERIFIED | 0.92 | Full | Bundle; fix measured contrast and test relevant states | `BI-5u3.4.2` / M3 |
| UX-09 switch input naming | VERIFIED | 0.90 | Full | Bundle; name the actual interactive input | `BI-5u3.4.3` / M3 |
| UX-10 async/result status semantics | VERIFIED | 0.85 | Full | Bundle; expose success/error/status accessibly | `BI-5u3.4.4` / M3 |
| UX-11 landmarks, headings, tab relationships | VERIFIED | 0.80 | Full | Bundle; improve rendered navigation semantics | `BI-5u3.4.5` / M3 |
| UX-12 browser/native recovery wording | DISPUTED | 0.55 | Partial | Defer; decide preview audience/contract before copy changes | `BI-5u3.5.1` / M4 |
| UX-13 responsive layout/task fit | UNVERIFIED | 0.45 | Partial | Defer; test viewports/tasks and change layout only if impairment reproduces | `BI-5u3.5.2` / M4 |
| PR #114 review: security signal outside display limit | VERIFIED | 0.90 | Full | Fix is on PR #114; classify the full dataset, retain bounded details, disclose omitted count, and test recommendations/routes beyond the card cap | `BI-5u3.1.9` / M1 |
| PR #114 review: critical manager stage hidden by truncation | VERIFIED | 0.95 | Full | Rank critical stages ahead of high stages by status and impact before selecting two manager actions | `BI-5u3.1.10` / M1 |
| PR #118 review follow-up: duplicate stage names overwrite severity lookup | VERIFIED | 0.90 | Full | Bundle in Phase 3; associate severity by row identity/order and add a mixed healthy/critical duplicate-name regression | `BI-5u3.7` / M3 follow-up |

**Disposition totals:** panel findings: 0 must-fix, 11 bundled, 2 deferred, 0 informational. PR review supplement: 2 P2 bundled follow-ups; both fixes are implemented and independently reviewed, with exact-head hosted gates and one review thread still blocking Phase 1 close. No governance veto was triggered by the panel’s P0/P1 rules. Scope remains the browser preview.

## Phase 0 — Refresh baseline and contract

**Bead:** `BI-5u3` (epic). **Entry:** preserve the dirty root worktree. **Exit:** before each milestone, record current `origin/main`, open PR status/head, write access, branch protection, and exact-head checks. Confirm whether the browser UI is a compatibility harness or user-facing dashboard and what Run Full Audit, SEO, and Field/Lab controls promise. Do not reuse stale remote facts.

## Phase 1 — Protect imports and data truthfulness (PR #114)

**Milestone:** `BI-5u3.1`. **Findings:** UX-01, UX-04, and PR review supplements `BI-5u3.1.9` and `BI-5u3.1.10`. Keep all implementation and review follow-ups on existing PR #114.

**Commit slices on PR #114:** `fix(ui): preserve security signals beyond display limit`; `fix(ui): cap rendered security signal details`; `test(ui): cover hidden security recommendations`; `docs: refresh Phase 1 closeout evidence`; `docs: clarify Phase 1 commit evidence`; `fix(ui): prioritize critical manager stages`.

**Acceptance criteria**

- Missing, explicitly empty, sample, and imported telemetry states remain distinct. Blank Apply does not silently replace visible data; explicit Reset to Sample is visibly labeled.
- Nested payload fields and numeric values are validated before state replacement. Invalid JSON or schema preserves the last valid view and presents an accessible error.
- Data used to detect security signals is not truncated by a presentation/display limit. A security record beyond that limit still contributes to counts, recommendations, and routes; unrelated records do not inflate the count. The Security panel renders at most three detail cards and reports how many matching signals are omitted, while the ordinary risk-card limit remains unchanged.
- Manager recommendations and manager action routes rank critical stages before high stages, then order equal-severity stages by impact and deterministic tie-breaks before truncating to two actions.
- Existing valid payload shapes and native-shell/public contracts remain compatible.

**Test plan**

- Unit tests: absent vs empty vs populated records; malformed nested object and non-finite/range values; security signal before and after the display limit; non-security record after the limit.
- Component tests: with `limits.risks: 1`, confirm hidden security records still drive counts, recommendations, and routes; confirm the Security panel caps at three cards and reports the remaining count.
- Component/unit tests: place two high stages before a critical stage in payload order and confirm the critical stage leads both manager outputs; verify impact ordering and deterministic ties.
- Component tests: blank Apply, explicit reset, visible source/state labels, accessible validation, and last-good state after rejected imports.
- Playwright: sample/import/empty/reset, malformed import recovery, and hidden-beyond-limit security signal.
- Run frozen pnpm install with `12.9.1`, `pnpm exec tsc -b`, `pnpm exec vitest run`, `pnpm run build`, and `pnpm run test:e2e`. Require same-head hosted UI/Playwright and aggregate checks; inspect review threads and merge only via protected PR after all required checks are green.

**Exit gate:** close `BI-5u3.1` only after `BI-5u3.1.9` and `.10` are implemented and reviewed, all known review threads are resolved, and exact-head required CI is terminal green. Local TypeScript, 84 Vitest tests, production build, and independent spec/quality reviews pass; checks for current head `a334cf9` are pending. Enable auto-merge only after required checks are green and review blockers remain resolved. Until then Phase 2 must not start.

## Phase 2 — Ground metrics, provenance, and recommendations (PR 2)

**Milestone:** `BI-5u3.3`. **Findings:** UX-02, UX-05, UX-06, UX-07. **Dependency:** Phase 1 merged.

**Commit slices:** `fix(ui): label dashboard data provenance`; `fix(ui): derive recommendations from active telemetry`; `fix(ui): align severity and score explanations`; `test(ui): cover grounded metrics and guidance`.

**Acceptance criteria**

- Each fixed/example section identifies its source; imported-derived sections identify the active imported data.
- Complete healthy data never shows unconditional critical warnings or unrelated sample routes. Empty data has a truthful empty/unavailable state.
- No route or recommendation is presented as derived from current data unless its source record supports it; any retained example is clearly labeled illustrative.
- Views use the same stage-severity classifier and configured latency threshold; duplicate stage names retain each row's own computed severity and do not inherit a neighbor's status.
- Score explanations name only formula inputs. Opportunity-only changes cannot be described as raising the score unless the score formula actually includes them.

**Test plan**

- Unit-test severity boundaries and each score component, including opportunity-only input changes; regress duplicate stage names with different statuses and verify each rendered row keeps its own status.
- Component-test provenance and routes for empty, healthy, and critical payloads.
- Playwright verifies healthy and critical payload guidance, no false critical route, and clear sample labeling.
- Run frozen install, typecheck, complete Vitest suite, production build, Playwright, and exact-head hosted required checks.

**Delivery:** one focused branch/PR based on the latest merged `main`, with reviewable Conventional Commit slices. Do not move these changes into the native shell or alter command contracts.

## Phase 3 — Make controls and accessibility semantics honest (PR 3)

**Milestone:** `BI-5u3.4`. **Findings:** UX-03, UX-08–UX-11. **Dependency:** Phase 2 merged.

**Commit slices:** `fix(ui): clarify preview-only controls`; `fix(a11y): meet primary text contrast`; `fix(a11y): name switches and announce results`; `fix(a11y): add landmarks and tab relationships`; `test(a11y): cover dashboard interactions`.

**Acceptance criteria**

- Every enabled control performs the represented operation. Unavailable audit and Field/Lab source controls are disabled and carry clear preview limitations; the SEO page/site selector is removed while neither scope has distinct source data. No inert control reports a completed action. A real audit engine or new data source requires a separately defined contract.
- Default normal-size primary text meets WCAG 2.2 SC 1.4.3 4.5:1 contrast for contained and outlined controls; hover, focus, disabled, and theme variants are separately measured.
- The disabled Field/Lab switch input is associated with its visible label and reports its unavailable state; no data-mode change is implied.
- Import/reset state and admin/baseline pending and final outcomes expose suitable status or alert semantics. Payload errors stay associated with the textarea; busy commands disable repeat submissions and one live region updates without focus movement.
- Exactly one main landmark and a descriptive H1 precede nested H2/H3 section headings. Because the inert SEO selector is removed, no tab/panel relationship is needed; if real page/site data is added later, its tabs must have unique IDs and matching panel relationships.

**Test plan**

- Testing Library asserts disabled preview controls, visible labels and switch name/state, admin pending/final/error status, textarea error associations, main landmark and heading levels, plus duplicate-name row severity. SEO selector removal is asserted explicitly.
- Contrast tests calculate from computed button foreground/background in Testing Library and Playwright, including enabled contained/outlined actions after hover/focus. Current claims cover exercised colors only; a full color-system audit remains out of scope.
- Playwright checks landmarks, disabled actions, accessible switch naming, and computed contrast. Record keyboard-only operation and a screen-reader spot check for status/heading navigation before closing the phase; those human checks are pending.
- Run frozen install, typecheck, complete Vitest suite, production build, Playwright, and exact-head hosted required checks.

**Delivery:** focused PR #119 (or next available number) from the refreshed Phase 2 main head, with reviewable code/test/doc commits. Enable auto-merge only after same-head required workflows pass and review blockers are resolved. Keep the native shell and command payload contracts out of scope.

## Phase 4 — Decide preview wording and validate responsive task fit

**Milestone:** `BI-5u3.5`. **Findings:** UX-12, UX-13. **Dependency:** Phase 3 merged. This is a validation gate, not pre-approval for redesign.

**Commit/PR rule:** this phase has no implementation commit by default. If validation proves a defect, create a separate scoped PR with a specific Conventional Commit and regression test named in that reproduced defect’s follow-up issue.

**Acceptance criteria**

- A decision note records intended preview audience and whether Tauri-specific bridge errors are compatibility-harness guidance or user-facing recovery copy.
- Change wording only if it contradicts that documented audience/contract; otherwise record the rationale and close without code.
- Gather browser evidence at 320, 360, 768, 1024, and 1280 CSS pixels and at 200% zoom for sample load, import, recommendation reading, and admin/baseline status tasks.
- Record horizontal overflow, clipping, control reachability, keyboard focus, and task/section-order friction. Change layout only if a concrete task impairment is reproduced.

**Test plan:** Playwright viewport runs with screenshots/traces and a concise task record. Run the complete UI checks if test code or application code changes. If no defect is reproduced, close the validation issues with evidence and no code PR. If a defect is reproduced, write a narrow acceptance scope and open a separate Phase 4 PR.

## Progressive commit and PR gates

1. **Phase 1 complete:** PR #114 code and PR #116 documentation/Beads closeout merged; exact-head hosted gates passed and all UX PR #114 review threads are resolved.
2. **PR #117 closeout complete:** current GitHub reports all 9 PR #112 review threads resolved. The physical macOS interaction gate remains distinct.
3. **Phase 2 complete:** PR #118 merged as `ce1e05d`; CI `37430967217`, Security `37430967218`, and Dependency Review `37430967207` passed on its exact head.
4. **PR 3 / Phase 3 in progress:** branch `fix/ux-phase3-controls-accessibility-2026-10-06` was based on `ce1e05d`. Implement controls, contrast, input naming, live statuses, landmarks/headings, and the duplicate-stage severity regression in small Conventional Commit slices. Open one focused PR and request auto-merge after checks and review pass on the same head.
5. **Phase 4:** after Phase 3 merges, complete decision and viewport evidence. Open another PR only if validation proves a code defect.
6. At each phase start, refresh open PR heads, ownership/write access, and checks. Reuse an existing writable branch only when its exact scope matches; otherwise use a clean branch from verified latest `main`. Merge only when local validation, required hosted checks, and review are satisfactory on the same commit. Record outcomes; never mark a planned check as passed.

## Beads worklist and finding traceability

| Bead | Scope / refined acceptance | Validation | State in local Beads |
|---|---|---|---|
| `BI-5u3` | Epic: all panel findings and follow-up review blockers map to issues, milestones, and evidence; preserve preview/native boundary | `bd lint`; verify graph and finding-to-test mapping | Open; Beads database is local and remote sync is unverified |
| `BI-5u3.1` | M1: explicit import states, schema safety, security signals survive display truncation | Unit, component, Playwright, same-head hosted checks | Closed after PR #114 and docs closeout PR #116 merged; same-head hosted checks passed and review threads resolved |
| `BI-5u3.6` | M0 prerequisite: reconcile PR #112 documentation threads with merged shutdown source/evidence; leave physical tray acceptance pending | Roadmap/docs contracts, relevant Rust/doc checks, same-head CI/Security/Dependency Review; verify thread states | Closed after PR #117; all 9/9 PR #112 threads now resolved; physical tray acceptance remains open |
| `.1.1` / `.1.2` | UX-01 distinct missing/empty/sample/import states; UX-04 validate malformed schema before render/state update | Import-state and malformed-payload tests | Closed; exact code head `a334cf9` passed hosted CI and local UI checks |
| `.1.3` | Hosted Playwright gate with distinct result and failure evidence | UI-triggered CI success/failure behavior | Closed; hosted Playwright and aggregate CI passed on exact code head `a334cf9` |
| `.1.4`–`.1.8` | M1 follow-ups: do not claim imported bottleneck/trend/top-risk data when absent; derive routes from full active telemetry | Empty/healthy/security-rich component and E2E cases | Closed; exact code head `a334cf9` passed hosted CI and local UI checks |
| `.1.9` | New PR review follow-up: full-dataset security classification independent of card display limit | Security record after limit still counted/routed; non-security control case | Closed locally against `a334cf9`; tracker and closeout evidence merged in PR #116; exact-head hosted checks passed |
| `.1.10` | PR review follow-up: rank critical manager stages before the two-action limit | Later critical stage wins over earlier highs in recommendation and route; impact ordering and deterministic ties | Closed locally against `a334cf9`; tracker and closeout evidence merged in PR #116; exact-head hosted checks passed |
| `BI-5u3.3` + `.3.1`–`.3.4` | M2 UX-02/05/06/07: provenance, data-derived warnings/routes, consistent severity, truthful formula explanation | Score/severity unit tests; healthy/empty/critical component + E2E tests | Closed by PR #118 `ce1e05d`; exact-head hosted checks green |
| `BI-5u3.4` + `.4.1`–`.4.5` | M3 UX-03/08/09/10/11: honest unavailable controls, AA contrast, meaningful switch name, async status announcements, main/heading semantics; remove nonfunctional SEO tabs pending real scope data | DOM/accessibility assertions, computed contrast, keyboard and browser checks; manual screen-reader spot check | Open; Phase 3 implementation branch active, hosted and human checks pending |
| `BI-5u3.5` + `.5.1`–`.5.2` | M4 UX-12/13: document audience/copy contract; validate responsive task fit before code | Decision note; viewport/zoom screenshots and task results | Open; blocked on Phase 3 |

Detailed per-issue acceptance criteria and test plans are stored in the Beads descriptions and the current [UX Beads snapshot](frontend-ux-remediation-beads-2026-10-06.jsonl). The local Beads IDs and history are not confirmed synchronized to a remote tracker.

## Coherence check, caveats, and action items

- **No contradictory completion claim:** Phase 1 and Phase 2 are merged with exact-head hosted checks; all UX PR #114 threads and all 9 current PR #112 threads are resolved. Phase 3 code is in progress and must not be marked complete until same-head CI and review pass.
- **No speculative UX change:** disputed UX-12 and unverified UX-13 remain validation-only.
- **Boundary preserved:** implementation targets `ui/`; native runtime and public payload contracts stay out of scope.
- **Context warning:** Physical macOS Show/Quit and foreground restoration remain unverified independently of the resolved documentation threads. Keyboard-only and screen-reader Phase 3 checks also remain pending.
- **Dissent ledger:** UX-12 usability vs trust reviewers disagree about browser-visible Tauri wording; decision owner is product owner. UX-13 impact was not demonstrated; decision owner is UX/product reviewer. Independent judge stage was unavailable.

| Priority | Owner | Action | Source |
|---|---|---|---|
| P2 | Implementer | Finish Phase 3 code and regression tests, then open focused PR and enable auto-merge after same-head required checks/review pass | UX-03/08–11, duplicate-stage regression |
| P2 | Reviewer/maintainer | Perform keyboard-only review and screen-reader spot check before closing Phase 3 | BI-5u3.4 |
| P2 | Product owner | Record preview audience/control promises before UX-12 wording decisions | UX-12 |
| P3 | UX/product reviewer | Run responsive viewport/task validation and change layout only if impairment reproduces | UX-13 |
| P3 | UX/product reviewer | Run responsive task validation; authorize layout work only with reproduced impairment | UX-13 |

**Final Recommendation:** Applied with caveats. Phases 1 and 2 are complete; Phase 3 code is in progress on a branch from verified main. Its automated and hosted validation is pending, as are keyboard-only and screen-reader checks. Physical macOS Show/Quit acceptance remains a separate open gate. UX-12 still needs a product audience/wording decision, UX-13 remains unverified, remote Beads synchronization is unverified, and the panel's independent judge stage was unavailable.

**Integration history:** The 13 panel findings were appended on 2026-10-05. PR #114 follow-up `PR114-R08` was recorded separately. This revision adds `PR112-R09`, correcting the live merged state and tracking its five unresolved comments in prerequisite bead `BI-5u3.6`.
