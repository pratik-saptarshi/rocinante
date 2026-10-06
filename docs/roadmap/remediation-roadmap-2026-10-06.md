# Frontend UX Remediation Roadmap — 2026-10-06

**Status (2026-10-06):** Phase 1 is complete. PR #114 merged into `main` as `b204f83b2d69655002018038ff59e1b14f87f37a`; code head `a334cf900b61f5273d4005a05cdf71b79e7cc6b7` passed local TypeScript, 13 Vitest files / 84 tests, production build, and diff checks, plus hosted aggregate CI (`37419778717`), Security (`37419778678`), and Dependency Review (`37419778673`). The later merge head passed CI (`37423517717`), Security (`37423517696`), and Dependency Review (`37423517727`). Documentation/Beads closeout PR #116 merged as `6fcc8d75b491898a7a07dc4875fb4b4555578107`; exact head `5172c28f9c86feb3eb246c279a955fed594fe0ee` passed CI (`37424524116`), Security (`37424524195`), and Dependency Review (`37424524140`). All Phase 1 review threads are resolved. PR #112 merged as `035c290662e2a4c79e247de6036886ea90e7bf60`; five P2 inline review threads remain unresolved after merge. Their evidence/documentation closeout is the next gate before Phase 2.

**Inputs:** [2026-10-05 UX adversarial panel report](../reviews/2026-10-05-frontend-user-experience/review_panel_report.md), [panel process and evidence](../reviews/2026-10-05-frontend-user-experience/review_panel_process.md), and the current local Beads hierarchy.

## Purpose and scope

Address the panel’s React/Vite browser-preview findings through sequenced, reviewable milestones. Keep browser-preview changes separate from the Rust native shell and its command/payload compatibility contracts. Do not infer that the panel reviewed native-shell usability.

The panel scored the preview 3.7/10 and raised 10 P2 and 3 P3 findings, with no P0/P1 findings. It reviewed source and installed dependencies; it did not run the UI, observe rendered accessibility output, or conduct user research. The independent Opus judge stage was unavailable, so the report is explicitly compressed and medium-confidence. UX-12 is disputed and UX-13 is unverified; neither authorizes speculative code changes.

## Current state and evidence boundary

- The root worktree is dirty on `fix/weighted-rollup-aggregation`. Preserve it; do UI implementation in a clean isolated worktree.
- The panel reviewed local source at `e9d6d3a`. PR #114 delivered Phase 1 code and merged on 2026-10-06. Because two documentation review threads remained open at merge, PR #116 is the Phase 1 documentation closeout vehicle.
- Local Beads records `BI-5u3.1.1`–`.1.8` as closed against PR #114 code head `71bf9e5`; their close notes record local typecheck, 13 Vitest files / 79 tests, production build, Playwright discovery, and hosted CI/security/dependency-review passes on that earlier code head. These records do not close the parent milestone.
- Live GitHub was refreshed on 2026-10-06. PR #114 merged as `b204f83b2d69655002018038ff59e1b14f87f37a` after branch head `b15118411dea0d2104eecffca74d17c6f66200ab` passed CI (`37423517717`), Security (`37423517696`), and Dependency Review (`37423517727`). Docs closeout PR #116 merged as `6fcc8d75b491898a7a07dc4875fb4b4555578107`; its three exact-head workflows passed and all PR #114 threads are resolved. Auto-merge completed the protected flow.
- PR #112 is merged as `035c290`. GitHub still reports five unresolved P2 review threads (`PRRT_kwDOSr9EN86pJmZK`, `PRRT_kwDOSr9EN86pLAJi`, `PRRT_kwDOSr9EN86pSwNa`, `PRRT_kwDOSr9EN86pS49n`, and `PRRT_kwDOSr9EN86pS_Ru`). They require accurate timed-out manual tray evidence; current macOS quit source/validation in root and scoped codemaps; readiness checklist/test-plan status; and the Markdown bill of materials. The final reviewed PR #112 head was `bb2d170`; CI `37417428474`, Security `37417428663`, and Dependency Review `37417428598` passed on that head. Those automated checks do not prove physical Show/Quit or foreground behavior. Resolve each thread only after its document is reconciled with merged source and that limitation remains explicit.
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
- Views use the same stage-severity classifier and configured latency threshold.
- Score explanations name only formula inputs. Opportunity-only changes cannot be described as raising the score unless the score formula actually includes them.

**Test plan**

- Unit-test severity boundaries and each score component, including opportunity-only input changes.
- Component-test provenance and routes for empty, healthy, and critical payloads.
- Playwright verifies healthy and critical payload guidance, no false critical route, and clear sample labeling.
- Run frozen install, typecheck, complete Vitest suite, production build, Playwright, and exact-head hosted required checks.

**Delivery:** one focused branch/PR based on the latest merged `main`, with reviewable Conventional Commit slices. Do not move these changes into the native shell or alter command contracts.

## Phase 3 — Make controls and accessibility semantics honest (PR 3)

**Milestone:** `BI-5u3.4`. **Findings:** UX-03, UX-08–UX-11. **Dependency:** Phase 2 merged.

**Commit slices:** `fix(ui): clarify preview-only controls`; `fix(a11y): meet primary text contrast`; `fix(a11y): name switches and announce results`; `fix(a11y): add landmarks and tab relationships`; `test(a11y): cover dashboard interactions`.

**Acceptance criteria**

- Every enabled control either performs the represented operation or clearly identifies preview-only/unavailable behavior. A real audit engine or new data source requires a separately defined contract.
- Default normal-size primary text meets WCAG 2.2 SC 1.4.3 4.5:1 contrast for contained and outlined controls; hover, focus, disabled, and theme variants are separately measured.
- Field/Lab switch’s interactive input exposes a meaningful accessible name and accurate checked state.
- Import/reset/admin/baseline outcomes expose suitable status or alert semantics. Errors are associated with the relevant control without stale or duplicate announcements.
- Main landmark, heading hierarchy, and keyboard-operable tab/panel name and ID relationships are present and unique.

**Test plan**

- Testing Library asserts control outcomes, role/name/state, status and alert behavior, input error associations, main landmark, headings, and tab relationships.
- Contrast tests check theme tokens and rendered button-state colors; scope the claim to the states checked.
- Playwright exercises pointer and keyboard flows. Record a keyboard-only pass and a screen-reader spot check for announcements/navigation.
- Run frozen install, typecheck, complete Vitest suite, production build, Playwright, and exact-head hosted required checks.

**Delivery:** one focused PR after Phase 2 merges. Do not add product behavior beyond the browser preview contract established in Phase 0.

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
2. **Milestone 0 / PR #117:** from verified current `main` (`6fcc8d7` was latest at review), reconcile the five outstanding PR #112 threads in a focused documentation follow-up. Record the manual Show timeout and current hard-exit source; quote exact hosted checks but keep physical macOS Show/Quit acceptance open. Run documentation contracts, relevant Rust format/contract checks, and required hosted PR checks before resolving threads.
3. **PR 2 / Phase 2:** only after PR #117's checks pass and five review threads are resolved, branch from the updated `main`; commit provenance, recommendation, severity, and score work in small Conventional Commit slices; open one focused PR.
4. **PR 3 / Phase 3:** after PR 2 merges, branch from updated `main`; commit control and accessibility work in reviewable slices; open one focused PR.
5. **Phase 4:** after PR 3 merges, complete decision and viewport evidence. Open a fourth PR only if validation proves a code defect.
6. At each phase start, refresh open PR heads, ownership/write access, and checks. Reuse an existing writable branch only when its exact scope matches; otherwise use a clean branch from verified latest `main`. Merge only when local validation, required hosted checks, and review are satisfactory on the same commit. Record outcomes; never mark a planned check as passed.

## Beads worklist and finding traceability

| Bead | Scope / refined acceptance | Validation | State in local Beads |
|---|---|---|---|
| `BI-5u3` | Epic: all panel findings and follow-up review blockers map to issues, milestones, and evidence; preserve preview/native boundary | `bd lint`; verify graph and finding-to-test mapping | Open; Beads database is local and remote sync is unverified |
| `BI-5u3.1` | M1: explicit import states, schema safety, security signals survive display truncation | Unit, component, Playwright, same-head hosted checks | Closed after PR #114 and docs closeout PR #116 merged; same-head hosted checks passed and review threads resolved |
| `BI-5u3.6` | M0 prerequisite: reconcile the five unresolved PR #112 threads with merged shutdown source and evidence; leave physical tray acceptance pending | Roadmap/docs contracts, relevant Rust/doc checks, same-head CI/Security/Dependency Review; verify all five GitHub thread states | Open; blocks `BI-5u3.3` |
| `.1.1` / `.1.2` | UX-01 distinct missing/empty/sample/import states; UX-04 validate malformed schema before render/state update | Import-state and malformed-payload tests | Closed; exact code head `a334cf9` passed hosted CI and local UI checks |
| `.1.3` | Hosted Playwright gate with distinct result and failure evidence | UI-triggered CI success/failure behavior | Closed; hosted Playwright and aggregate CI passed on exact code head `a334cf9` |
| `.1.4`–`.1.8` | M1 follow-ups: do not claim imported bottleneck/trend/top-risk data when absent; derive routes from full active telemetry | Empty/healthy/security-rich component and E2E cases | Closed; exact code head `a334cf9` passed hosted CI and local UI checks |
| `.1.9` | New PR review follow-up: full-dataset security classification independent of card display limit | Security record after limit still counted/routed; non-security control case | Closed locally against `a334cf9`; tracker and closeout evidence merged in PR #116; exact-head hosted checks passed |
| `.1.10` | PR review follow-up: rank critical manager stages before the two-action limit | Later critical stage wins over earlier highs in recommendation and route; impact ordering and deterministic ties | Closed locally against `a334cf9`; tracker and closeout evidence merged in PR #116; exact-head hosted checks passed |
| `BI-5u3.3` + `.3.1`–`.3.4` | M2 UX-02/05/06/07: provenance, data-derived warnings/routes, consistent severity, truthful formula explanation | Score/severity unit tests; healthy/empty/critical component + E2E tests | Open; blocked by `BI-5u3.6` and completion of Phase 1 |
| `BI-5u3.4` + `.4.1`–`.4.5` | M3 UX-03/08/09/10/11: honest controls, contrast, switch naming, status semantics, landmarks/tabs | DOM/accessibility assertions, contrast, keyboard, screen-reader spot check, E2E | Open |
| `BI-5u3.5` + `.5.1`–`.5.2` | M4 UX-12/13: document audience/copy contract; validate responsive task fit before code | Decision note; viewport/zoom screenshots and task results | Open |

Detailed per-issue acceptance criteria and test plans are stored in the Beads descriptions and the current [UX Beads snapshot](frontend-ux-remediation-beads-2026-10-06.jsonl). The local Beads IDs and history are not confirmed synchronized to a remote tracker.

## Coherence check, caveats, and action items

- **No contradictory completion claim:** Phase 1 code and docs are merged with exact-head hosted evidence on `a334cf9`, `b151184`, and `5172c28`; all UX PR #114 threads are resolved. PR #112 is merged but has five unresolved P2 threads; its CI/security/dependency checks passed on exact head `bb2d170`.
- **No speculative UX change:** disputed UX-12 and unverified UX-13 remain validation-only.
- **Boundary preserved:** implementation targets `ui/`; native runtime and public payload contracts stay out of scope.
- **Context warning:** Physical macOS Show/Quit and foreground restoration remain unverified. The PR #112 automated checks passed on `bb2d170`, but do not establish a successful physical tray interaction. Keep this acceptance gate open independently of the documentation-thread closeout.
- **Dissent ledger:** UX-12 usability vs trust reviewers disagree about browser-visible Tauri wording; decision owner is product owner. UX-13 impact was not demonstrated; decision owner is UX/product reviewer. Independent judge stage was unavailable.

| Priority | Owner | Action | Source |
|---|---|---|---|
| P2 | Implementer/reviewer | Reconcile root/scoped codemaps, publish checklist, test plan, BOM, and parity matrix with current macOS quit implementation and actual manual evidence; validate on focused follow-up PR #117 | PR #112 threads |
| P2 | Reviewer/maintainer | Resolve all five PR #112 threads only after evidence matches merged source and documentation checks pass; start Phase 2 only then | User sequencing instruction |
| P2 | Product owner | Record browser preview audience/control promises before UX-03 behavior or UX-12 copy decisions | UX-03/12 |
| P2 | Implementer | Begin Phase 2 only from updated `main` after Phase 1 merge | UX-02/05/06/07 |
| P3 | UX/product reviewer | Run responsive task validation; authorize layout work only with reproduced impairment | UX-13 |

**Final Recommendation:** Applied with caveats. The panel’s actionable findings are mapped into ordered phases and refined Beads with acceptance and test criteria. Phase 1 is complete. Phase 0 is the immediate gated closeout for five unresolved comments on merged PR #112. Phase 2 must wait until that focused follow-up passes same-head checks and the threads are resolved. Physical macOS Show/Quit acceptance remains a separate open gate; remote Beads synchronization is unverified; UX-03/UX-12 product intent is unresolved; UX-13 impact is unverified; and the independent panel judge stage was unavailable.

**Integration history:** The 13 panel findings were appended on 2026-10-05. PR #114 follow-up `PR114-R08` was recorded separately. This revision adds `PR112-R09`, correcting the live merged state and tracking its five unresolved comments in prerequisite bead `BI-5u3.6`.
