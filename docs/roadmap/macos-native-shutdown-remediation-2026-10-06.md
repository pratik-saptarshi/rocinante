# macOS Native Shutdown and Launch Acceptance — 2026-10-06

**Status:** The current source candidate passed the final PR #112 hosted CI, Security, and Dependency Review checks. Physical menu interaction and an interactive installed-app startup on the current candidate remain unverified. Keep the desktop release acceptance gate open.

## Verified source and hosted evidence

- PR #112 merged as `035c290662e2a4c79e247de6036886ea90e7bf60`; tested head: `bb2d1700e4ae0c1663df140336f2f2ededbd0963`.
- CI `37417428474`, Security `37417428663`, and Dependency Review `37417428598` completed successfully on that head.
- The explicit macOS quit path saves shell state, flushes storage, drops the tray icon, and calls `std::process::exit(0)`. Linux and Windows close the eframe viewport; ordinary close-to-tray continues to hide the window.
- The latest recorded physical Show attempt after `2fac448` timed out without a witnessed Show callback and predates the current hard-exit candidate.
- The user reported that the app was unstable and did not come up cleanly. No error text, crash report, or reproducible current-head launch failure has been collected, so cause and affected build are undetermined.
- DuckDB must remain a checksum-verified prebuilt binary. Do not enable a source-build feature or compile it from scratch.

## Remediation and acceptance sequence

1. Build/install the signed macOS application from the current merged source using the repository's normal packaging and Launch Services path. Record commit, artifact checksum, macOS version, and launch method.
2. If launch fails, preserve the exact dialog or console output, unified log/crash report, bundle signature verification, executable architecture, DuckDB binary checksum, loader name, and Frameworks-relative `LC_RPATH`. Classify a Launch Services error separately from a pre-main dyld/AppKit failure. Do not attribute the user's report to the quit change without matching build and evidence.
3. On a clean successful launch, test cold and warm URL delivery, close-to-tray, physical tray Show with a witnessed callback and frontmost visible window, physical tray Quit with its callback witness and process exit, then restart and confirm saved state.
4. Repeat the complete acceptance on the same exact build after any source or packaging change. Keep each observed result and failure trace attached to that build.

## Exit criteria

- The current installed app launches without an unexplained crash or dialog.
- Physical Show and Quit each produce their expected callback witnesses; Show restores and foregrounds the app, and Quit exits after persisted state is flushed.
- Cold/warm URL delivery and saved-state restart pass after current-source installation.
- DuckDB loader and signature checks pass for the bundled prebuilt binary without a source build.
- The accepted build has current local and hosted package/CI/security/dependency results. A script timeout or successful CI run alone cannot close the physical interaction gate.

Until all exit criteria are met, mark interactive macOS shutdown and launch readiness pending.
