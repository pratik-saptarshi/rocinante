# macOS Native Shutdown Remediation — 2026-10-05

## Objective

Make the installed macOS shell quit without aborting or remaining resident,
preserve the tray-app close-to-hide behavior, and keep manual tray-menu and
foreground checks explicitly separate from automated action-routing evidence.

## Evidence and diagnosis

Three Rocinante crash reports from 2026-10-05 were inspected:

- `rocinante-desktop-shell-2026-10-05-122042.ips`
- `rocinante-desktop-shell-2026-10-05-123312.ips`
- `rocinante-desktop-shell-2026-10-05-123532.ips`

All three record `SIGABRT` on the main thread. Their stacks pass through
AppKit `NSApplication terminate:`, Winit's
`ApplicationDelegate.app_will_terminate`, and the shell's macOS termination
call from `RocinanteApp::ui`. The synchronous AppKit call re-entered Winit's
window shutdown while eframe was still inside its UI callback; the panic then
crossed the Objective-C callback boundary and aborted the process.

The first remediation, pushed as `b0f8ad0`, deferred `terminate:` by one AppKit
run-loop turn. It removed the direct re-entrancy path, but hosted CI run
`37356919269` still failed `macos-url-dispatch`: the acceptance Quit request
left the bundled process resident until the 45-second timeout. The UI path had
already queued an eframe viewport close before scheduling delayed AppKit
termination. Winit could begin leaving its event loop before the delayed
selector was processed. This is the control-flow diagnosis being addressed by
the follow-up; the hosted failure itself only establishes that the process
did not exit cleanly.

## Chosen remediation

The follow-up commit `8d049fd` makes the macOS tray Quit action schedule AppKit
termination directly after the current UI pass, without first asking eframe to
close its viewport. For a macOS close request whose policy is Exit, the shell
cancels the pending viewport close and schedules the same deferred AppKit
request. This keeps Winit's event loop alive long enough for AppKit to deliver
the termination callback outside the active UI callback. Non-macOS explicit
Quit continues to close the eframe viewport.

This preserves the application behavior and IPC/payload surface. DuckDB remains
the SHA-256-verified prebuilt shared library; this remediation does not add any
DuckDB source-build path.

## Validation and exit gates

| Gate | Evidence | State |
|---|---|---|
| Rust formatting | `rtk cargo fmt --manifest-path src-tauri/Cargo.toml --all -- --check` | Passed locally after `8d049fd` |
| Desktop-shell tests | `rtk cargo test --manifest-path src-tauri/Cargo.toml --locked -p rocinante-desktop-shell` | 52 passed across 9 suites |
| Warning-denied Clippy | `rtk cargo clippy --manifest-path src-tauri/Cargo.toml --locked -p rocinante-desktop-shell --all-targets --all-features -- -D warnings` | Passed locally |
| Installed macOS lifecycle | `rtk proxy bash scripts/test-macos-url-dispatch.sh` | Passed twice locally after `8d049fd`: cold/warm URL delivery, automated tray action routing, notification request, minimize/restore, clean Quit, and saved-state restart |
| Hosted CI on `8d049fd` | CI run `37358596816` | Running at the time this record was prepared; do not claim hosted success until terminal |
| Hosted Security on `8d049fd` | Security run `37358596821` | Running at the time this record was prepared; do not claim hosted success until terminal |
| Hosted Dependency Review on `8d049fd` | Run `37358596837` | Passed |
| Physical macOS tray selection and foreground | Manual Show/Quit with witnessed callbacks and frontmost window | Open; automated action injection is not physical menu evidence. AppKit's activation request has returned `accepted=false` in scripted runs. |

## Remaining work

1. Require CI and Security to finish green on the current PR head. If macOS
   acceptance fails again, inspect its job log and add the observed witness
   state before changing the shutdown path.
2. Keep the matrix, readiness roadmap, decision record, README, codemaps, BOM,
   test plan, and publish checklist aligned with the terminal run results.
3. Resolve the parity-matrix review comment only after the matrix records the
   previously timed-out physical Show attempt and the corrected automated
   shutdown evidence, and after same-head required checks pass.
4. Keep physical Show/Quit and foreground acceptance open until a person
   completes the manual flow and the script records both callbacks, frontmost
   state, process exit, and restart evidence.

PR #112 remains open on `fix/weighted-rollup-aggregation`; do not merge outside
the protected pull request flow.

## Superseding status update — 2026-10-05

The direct deferred-AppKit candidate in `8d049fd` did not clear hosted CI:
run `37358596816` again failed the macOS URL-dispatch job because the process
remained resident after the automated Quit request. Therefore the selector
ordering diagnosis above was not sufficient to explain the hosted behavior.
The follow-up diagnostics commit `5f68c95` makes the next acceptance run report
whether the Quit control was consumed, its callback witness, the last app
state, and the process command/state. CI run `37359612876` was queued when this
update was prepared; Security run `37359612735` was in progress and Dependency
Review run `37359612983` passed.

A new local candidate removes the tray icon before sending eframe's viewport
Close command and removes the AppKit `terminate:` call from the UI path. Its
first installed lifecycle run passed on this Mac after formatting, 52
desktop-shell tests, and warning-denied Clippy passed. This candidate has not
been pushed or hosted-validated. Promote it only after the diagnostic run
clarifies whether the previous Quit action reached the app, then require a new
same-head macOS CI pass.

This supersedes the prior local hypothesis that scheduling AppKit termination
directly from the Quit callback alone resolves the hosted timeout. The crash
reports still establish the synchronous termination re-entrancy defect; the
resident-process failure is a separate acceptance result and remains open.

## Current branch status — `fe5b2eb` (2026-10-05)

The tray-release candidate has now been pushed to PR #112 as
`fe5b2eb00c85d5d27780c686ee20809661421b55`. It drops the tray icon and asks
eframe to close its viewport; the acceptance script retains diagnostics for
the control receipt, Quit callback marker, saved shell state, and process state
if the hosted job fails. This avoids the AppKit `terminate:` call that caused
the original abort and the deferred variants that left the process resident
on the hosted runner.

On this candidate, the full serial Rust workspace passes 270 tests across 64
suites, workspace formatting passes, and warning-denied workspace Clippy
passes. The installed macOS lifecycle passes locally, including cold/warm URL
delivery, automated tray routing, notification request, minimize/restore,
clean Quit, and saved-state restart. Dependency Review run `37360802366` was
queued and CI run `37360802367` was pending at the time of this update; no
current-head hosted success is claimed. The earlier diagnostics run for
`5f68c95` remains queued, Security run `37359612735` passed, and Dependency
Review run `37359612983` passed. Manual physical Show/Quit and foreground
acceptance remain open.

## Eframe event-loop exit correction — `b95a8c1` (2026-10-05)

The hosted macOS log for `fe5b2eb` recorded both
`quit_control_received=true` and `quit_action_started=true`, but the process
remained resident after the tray icon was dropped and the root viewport was
closed. This rules out the acceptance control path as the cause. The shell was
using eframe's default `NativeOptions::run_and_return=true`, which returns after
the main window closes and continues execution in the caller. The shell now
sets `run_and_return: false`, which tells eframe to exit the native application
when its root window closes. The close-to-tray path still cancels the close
request and hides the window; explicit Quit drops the tray icon before closing.
See the pinned [eframe 0.36.2 `run_and_return` contract](https://docs.rs/eframe/0.36.2/eframe/struct.NativeOptions.html#structfield.run_and_return).

The change is on PR #112 at `b95a8c136b7b15d68f84e9a6e1817c180cf21f9f`.
The remote branch also includes a formatting correction for a blank line that
failed hosted `cargo fmt` on `fe5b2eb`.

| Gate | Evidence | State |
|---|---|---|
| Rust formatting | `rtk cargo fmt --manifest-path src-tauri/Cargo.toml --all -- --check` | Passed locally on the same source change; hosted run for `b95a8c1` pending |
| Desktop-shell tests | `rtk cargo test --manifest-path src-tauri/Cargo.toml --locked -p rocinante-desktop-shell -- --test-threads=1` | 52 passed across 9 suites locally |
| Workspace tests | `rtk cargo test --manifest-path src-tauri/Cargo.toml --locked --workspace -- --test-threads=1` | 270 passed across 64 suites locally |
| Warning-denied Clippy | `rtk cargo clippy --manifest-path src-tauri/Cargo.toml --locked --workspace --all-targets --all-features -- -D warnings` | Passed locally |
| Hosted CI / Security / Dependency Review | Current head `b95a8c1` | Pending; no same-head hosted success is claimed |
| Installed macOS lifecycle | `rtk bash scripts/test-macos-url-dispatch.sh` | Inconclusive locally: Launch Services refused the temporary bundle (`-10822`); an unbundled binary smoke run emitted Launch Services errors and did not record startup. The hosted packaged-app run is required. |
| Physical macOS tray selection and foreground | Manual Show/Quit with witnessed callbacks and frontmost window | Open; no physical interaction is claimed |

Do not close the shutdown gate or resolve review feedback until the hosted
macOS lifecycle and same-head required checks complete successfully. If the
process still remains resident, inspect its termination state and the eframe
event-loop path before making another change.

At 2026-10-05 19:33 UTC, the current-head CI run was pending and Security and
Dependency Review were queued. [GitHub Status](https://www.githubstatus.com/)
reported an active Actions degraded-performance incident beginning at 19:11
UTC that day. Treat the queue delay as external status, not as a pass or code
failure; retain the gate until the actual same-head runs finish.

The prior runs queued for source head `b95a8c1` were cancelled after the documentation commit moved the branch to `2e6c8e4`. Fresh required runs on the final source/documentation tree are still required.