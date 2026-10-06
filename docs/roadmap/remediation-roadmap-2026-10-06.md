# Frontend UX Remediation Roadmap — 2026-10-06

**Status (2026-10-06):** Phase 1 and Phase 2 are complete. PR #112's nine review threads are resolved; physical macOS tray/foreground acceptance remains unverified. Phase 3 implementation merged via PR #119 at `94d0bfc60325d8a80dcbedbe4e1ec33790d3a7f1` from head `15148ad6c671ed4e0403dcc297f210eb016ff713`; exact-head CI `37438201414`, Security `37438201595`, and Dependency Review `37438201425` passed. UI quality passed with pinned pnpm 12.9.1, TypeScript, 95 unit tests, and production build; PR #119 had 10 Playwright tests. Phase 4 UX-12/13 implementation merged via PR #121 at `17e19cb69f5bd24de72abd42cb51853ee05c14aa` from head `d782e4ff48514816c7f4a9cb49086558a8f9ec24`; exact-head CI `37440607409`, Security `37440607270`, and Dependency Review `37440607276` passed. Its UI and 11 Playwright tests passed, including the five viewport widths, but review found the overflow assertions ran only before import/admin state changes. Follow-up PR work adds after-transition checks; validation is pending. Phase 3 keyboard/screen-reader checks and Phase 4 200% zoom/full task validation remain open.

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
- The 2026-10-06 Beads snapshot contains 27 unique issues: 17 open and 10 closed. `bd lint` passed with 16 open issues checked. Children `.1.9` and `.1.10` have local closure evidence tied to `a334cf9`; `.1.3` hosted Playwright evidence is recorded against the same code head in the merged PR docs. The Phase 1 parent is closed. Remote Beads synchronization remains unverified.
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
| PR #118 review follow-up: duplicate stage names overwrite severity lookup | VERIFIED | 0.90 | Full | Bundle in Phase 3; associate severity by row identity/order and add a mixed healthy/critical duplicate-name regression | `BI-5u3.4.6` / M3 follow-up |
| PR #119 review: pending status hidden while live region is busy | VERIFIED | 0.95 | Full | Remove `aria-busy` from the admin and baseline status live regions; test that pending text remains exposed | `BI-5u3.4.4` / M3 |
| PR #119 review: Action Routing incorrectly exposed as H2 peer | VERIFIED | 0.95 | Full | Make Action Routing an H3 nested under Quality Pulse and assert heading level | `BI-5u3.4.5` / M3 |
| PR #119 hosted CI: baseline pending test used a response shape the bridge does not return | VERIFIED | 0.98 | Full | Change the fake bridge result to a numeric baseline and require a fresh exact-head UI run | `BI-5u3.4.4` / M3 verification |
| PR #121 review: attribute the correct Playwright count to each head | VERIFIED | 0.99 | Full | Record 10 Playwright tests for PR #119; attribute the 11th viewport test to PR #121 and append an explicit correction | M3/M4 evidence |
| PR #121 review: repeat viewport overflow check after telemetry import and admin fallback | VERIFIED | 0.99 | Full | Assert document width after each state transition at all five viewport widths | `BI-5u3.5.2` / M4 |
| PR #121 review: reconcile stale completion and dependency wording throughout roadmap | VERIFIED | 0.98 | Full | Update worklist table, coherence/actions/final recommendation to reflect merged PRs and open human evidence | `BI-5u3.4`/`.5` |

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

**Exit gate (complete):** `BI-5u3.1` is closed after implementation follow-ups `.1.9` and `.1.10`, Phase 1 documentation closeout, review resolution, and terminal-green exact-head hosted checks. The Phase 1 test/check evidence above is historical and does not replace validation of later heads.

## Phase 2 — Ground metrics, provenance, and recommendations (PR 2)

**Milestone:** `BI-5u3.3`. **Findings:** UX-02, UX-05, UX-06, UX-07. **Dependency:** Phase 1 merged.

**Commit slices:** `fix(ui): label dashboard data provenance`; `fix(ui): derive recommendations from active telemetry`; `fix(ui): align severity and score explanations`; `test(ui): cover grounded metrics and guidance`.

**Acceptance criteria**

- Each fixed/example section identifies its source; imported-derived sections identify the active imported data.
- Complete healthy data never shows unconditional critical warnings or unrelated sample routes. Empty data has a truthful empty/unavailable state.
- No route or recommendation is presented as derived from current data unless its source record supports it; any retained example is clearly labeled illustrative.
- Views use the same stage-severity classifier and configured latency threshold. Duplicate stage names will be covered by `BI-5u3.4.6` in Phase 3 so every row retains its own computed severity.
- Score explanations name only formula inputs. Opportunity-only changes cannot be described as raising the score unless the score formula actually includes them.

**Test plan**

- Unit-test severity boundaries and each score component, including opportunity-only input changes. Phase 3 adds a duplicate-name component regression and verifies row-specific status.
- Component-test provenance and routes for empty, healthy, and critical payloads.
- Playwright verifies healthy and critical payload guidance, no false critical route, and clear sample labeling.
- Run frozen install, typecheck, complete Vitest suite, production build, Playwright, and exact-head hosted required checks.

**Delivery:** one focused branch/PR based on the latest merged `main`, with reviewable Conventional Commit slices. Do not move these changes into the native shell or alter command contracts.

## Phase 3 — Make controls and accessibility semantics honest (PR 3)

**Milestone:** `BI-5u3.4` plus review follow-up `BI-5u3.4.6`. **Findings:** UX-03, UX-08–UX-11 and duplicate-stage severity. **Dependency:** Phase 2 merged.

**Commit slices:** `fix(ui): clarify preview-only controls`; `fix(a11y): meet primary text contrast`; `fix(a11y): name switches and announce results`; `fix(a11y): add landmarks and tab relationships`; `test(a11y): cover dashboard interactions`.

**Acceptance criteria**

- Every enabled control performs the represented operation. Unavailable audit and Field/Lab source controls are disabled and carry clear preview limitations; the SEO page/site selector is removed while neither scope has distinct source data. No inert control reports a completed action. A real audit engine or new data source requires a separately defined contract.
- Default normal-size primary text meets WCAG 2.2 SC 1.4.3 4.5:1 contrast for contained and outlined controls; hover, focus, disabled, and theme variants are separately measured.
- The disabled Field/Lab switch input is associated with its visible label and reports its unavailable state; no data-mode change is implied.
- Import/reset state and admin/baseline pending and final outcomes expose suitable status or alert semantics. Pending text remains announced while commands run: do not set `aria-busy` on the live status region. Payload errors stay associated with the textarea; busy commands disable repeat submissions and one live region updates without focus movement.
- Exactly one main landmark and a descriptive H1 precede nested H2/H3 section headings. Because the inert SEO selector is removed, no tab/panel relationship is needed; if real page/site data is added later, its tabs must have unique IDs and matching panel relationships.

**Test plan**

- Testing Library asserts disabled preview controls, visible labels and switch name/state, admin and baseline pending/final/error status without `aria-busy`, textarea error associations, main landmark and heading levels (including Action Routing H3), plus duplicate-name row severity. SEO selector removal is asserted explicitly.
- Contrast tests calculate from computed button foreground/background in Testing Library and Playwright, including enabled contained/outlined actions after hover/focus. Current claims cover exercised colors only; a full color-system audit remains out of scope.
- Playwright checks landmarks, disabled actions, accessible switch naming, and computed contrast. Record keyboard-only operation and a screen-reader spot check for status/heading navigation before closing the phase; those human checks are pending.
- Run frozen install, typecheck, complete Vitest suite, production build, Playwright, and exact-head hosted required checks.

**Delivery:** PR #119 merged through auto-merge at `94d0bfc` after exact-head CI `37438201414`, Security `37438201595`, and Dependency Review `37438201425` passed on code head `15148ad`. The review fixes for live-region `aria-busy`, heading nesting, and duplicate-stage severity are included. The implementation delivery is complete, but keep `BI-5u3.4` and its human-check children open until keyboard-only and screen-reader checks are recorded. Native shell and command payload contracts stayed out of scope.

## Phase 4 — Decide preview wording and validate responsive task fit

**Milestone:** `BI-5u3.5`. **Findings:** UX-12, UX-13. **Dependency:** Phase 3 implementation merged; manual accessibility checks remain a separate Phase 3 closeout gate.

**Delivery:** PR #121, branch `fix/ux-phase4-preview-fit-2026-10-06`, starts from merge `94d0bfc`. It records the preview audience and updates fallback copy to match the browser-preview/native-host boundary. It adds a Playwright task/overflow check at 320, 360, 768, 1024, and 1280 CSS-pixel widths. The first hosted viewport run passed with no overflow at those widths, but PR #121 review found it checked only the initial state. PR #122 repeats the overflow assertion after telemetry import and admin fallback; await those checks before closing UX-13.

**Acceptance criteria**

- The decision note records React/Vite as the browser preview and headless validation harness, and Rust eframe/winit as the supported desktop host.
- Browser fallback copy identifies the unavailable desktop command runtime and says admin commands are not available in the browser preview; the Tauri compatibility adapter does not imply a production Tauri runtime.
- At 320, 360, 768, 1024, and 1280 CSS px, the main surface has no horizontal overflow; payload import and admin fallback tasks remain reachable and produce truthful status.
- Record clipping, control reachability, keyboard focus, and task/section-order friction. Apply the smallest layout adjustment only if browser evidence shows impairment.
- Manually validate representative tasks at 200% zoom and keyboard-only operation; record screen-reader announcement/heading observations with Phase 3 checks.

**Test plan:** Playwright runs import and admin fallback tasks at all five CSS widths and asserts document width does not exceed the viewport. Hosted UI-quality and Playwright runs are required on the exact PR head. Record browser artifacts/task notes. Manual 200% zoom, keyboard, and screen-reader checks remain human evidence. If no responsive defect is reproduced, close UX-13 without layout changes; otherwise implement a narrow responsive fix with a regression test in a reviewable PR.

## Progressive commit and PR gates

1. **Phase 1 complete:** PR #114 code and PR #116 documentation/Beads closeout merged; exact-head hosted gates passed and all UX PR #114 review threads are resolved.
2. **PR #117 closeout complete:** current GitHub reports all 9 PR #112 review threads resolved. The physical macOS interaction gate remains distinct.
3. **Phase 2 complete:** PR #118 merged as `ce1e05d`; CI `37430967217`, Security `37430967218`, and Dependency Review `37430967207` passed on its exact head.
4. **Phase 3 implementation merged:** PR #119 merged at `94d0bfc` after exact-head CI `37438201414`, Security `37438201595`, and Dependency Review `37438201425` passed. All three PR #119 review threads are resolved. Keep the Phase 3 milestone open until manual keyboard and screen-reader evidence is recorded.
5. **Phase 4 follow-up in progress:** PR #121 merged at `17e19cb` after exact-head gates passed. PR #122 branch `fix/ux-phase4-review-followups-2026-10-06` reconciles review evidence and rechecks overflow after import/admin transitions at each tested width. Auto-merge is enabled; do not close UX-13 until exact-head checks and remaining manual zoom/task evidence are recorded.
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
| `BI-5u3.4` + `.4.1`–`.4.5` | M3 UX-03/08/09/10/11 plus `BI-5u3.4.6`: honest unavailable controls, AA contrast, meaningful switch name, async status announcements, main/heading semantics; remove nonfunctional SEO tabs pending real scope data; preserve per-row severity for duplicate stage names | DOM/accessibility assertions, computed contrast, keyboard and browser checks, duplicate-name mixed-severity regression; manual screen-reader spot check | Open; PR #119 merged with exact-head hosted checks green; manual keyboard and screen-reader evidence remains open |
| `BI-5u3.4.6` | PR #118 follow-up: keep severity attached to each duplicate-named telemetry stage row | Mixed healthy/critical duplicate-name Testing Library regression; full UI and same-head hosted checks | Closed; row-specific regression and all exact-head PR #119 required checks passed |
| `BI-5u3.5` + `.5.1`–`.5.2` | M4 UX-12/13: document audience/copy contract; validate responsive task fit before code | Decision note; viewport/zoom screenshots and task results | Open; Phase 3 implementation is merged. UX-12 decision/copy is validated; UX-13 initial five-width run passed but after-transition overflow evidence and 200% zoom/manual tasks remain open |

Detailed per-issue acceptance criteria and test plans are stored in the Beads descriptions and the current [UX Beads snapshot](frontend-ux-remediation-beads-2026-10-06.jsonl). The local Beads IDs and history are not confirmed synchronized to a remote tracker.

## Coherence check, caveats, and action items

- **No contradictory completion claim:** Phase 1–3 implementation PRs are merged with exact-head hosted checks; all UX PR #114 threads and all 9 current PR #112 threads are resolved. Phase 3 human keyboard/screen-reader acceptance remains open. Phase 4 PR #121 is merged; PR #122 is adding after-transition viewport checks and awaits exact-head hosted validation.
- **UX decisions and validation:** UX-12 audience/copy decision is recorded and implemented in PR #121. UX-13 has an initial five-width pass; PR #121 review correctly noted overflow must be rechecked after import/admin transitions. PR #122 carries that regression; 200% zoom and human task checks remain pending.
- **Boundary preserved:** implementation targets `ui/`; native runtime and public payload contracts stay out of scope.
- **Context warning:** Physical macOS Show/Quit and foreground restoration remain unverified independently of the resolved documentation threads. Keyboard-only and screen-reader Phase 3 checks also remain pending.
- **Dissent ledger:** UX-12 reviewers disagreed on Tauri-specific wording; the repository codemap resolved the audience contract as browser preview/headless harness with Rust eframe/winit desktop. UX-13 remains open for expanded responsive/zoom evidence. The independent judge stage was unavailable.

| Priority | Owner | Action | Source |
|---|---|---|---|
| P2 | Reviewer/maintainer | Record keyboard-only review and screen-reader spot check before closing Phase 3 | BI-5u3.4 |
| P2 | Implementer | Run PR #122 after-transition overflow checks at five widths; address any reproduced overflow before merge | UX-13 |
| P3 | UX/product reviewer | Complete representative recommendation/baseline tasks at 200% zoom and record focus/order observations | BI-5u3.5.2 |

**Final Recommendation:** Phases 1–3 implementation and Phase 4 audience/copy delivery are merged through protected PRs with exact-head automated gates green. PR #122 is validating overflow after telemetry import and admin fallback at five widths. Keep Phase 3 open for keyboard-only/screen-reader evidence and Phase 4 open for 200% zoom and the remaining responsive task observations. Physical macOS Show/Quit acceptance remains a separate open gate. Remote Beads synchronization is unverified; the panel's independent judge stage was unavailable.

**Integration history:** PR #119 merged at `94d0bfc` after CI `37438201414`, Security `37438201595`, and Dependency Review `37438201425` passed on exact head `15148ad`; 10 Playwright tests passed. PR #121 merged at `17e19cb` after CI `37440607409`, Security `37440607270`, and Dependency Review `37440607276` passed on exact head `d782e4f`; 11 Playwright tests passed, including initial five-width coverage. Post-merge review corrected the test-count attribution and added required overflow rechecks after import/admin transitions; that follow-up is under PR #122.
