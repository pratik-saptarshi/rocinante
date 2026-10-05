# rocinante-desktop-shell

## Responsibility

GTK-free native desktop host for repository selection, authenticated analysis,
saved-metric views, URL lifecycle, the admin command bridge and release-baseline
controls, the ported React companion insight slice, and static accessibility,
SEO, Drupal security, and performance reference panels.

## Modules

- `src/lib.rs` owns native application state, navigation, window/tray behavior,
  scan and saved-metric tasks, notification delivery, and the eframe UI. The
  first UI frame requests a visible, unminimized window and macOS activation. Tray Show sends visibility/focus commands before waking the UI loop; installed Darwin lifecycle acceptance passes, while strict mode reports visible=true, activation_policy=0 (Regular), active=false, and frontmost=false in this automation session.
- `src/dashboard.rs` builds deterministic summaries from actual repository
  metrics; it does not infer risk from generic metric values.
- The Dashboard release-baseline controls use the shared `rocinante-storage`
  authorization and persistence adapter and execute off the UI thread.
- The admin command bridge offers all nine shared actions through a command
  selector and editable JSON payload, using the same shared services as Tauri.
- `src/dashboard_insights.rs` ports the companion's sample commit-risk,
  bottleneck, opportunity, quality-pulse, audience recommendation/routing,
  explainability, trend/PR-risk, job-observability, limit, JSON envelope, and
  apply/reset semantics, quality snapshot, and audience-specific focus sections. Sample insights are labeled so they are not presented
  as scan results.
- `src/dashboard_reference.rs` holds the React reference metrics/findings and
  SEO/performance selectors. Dashboard panels are labeled as static sample
  audit data; Run Full Audit retains the source's no-op behavior.
- `src/deep_link.rs` parses the repository URL scheme and implements the
  bounded per-user inbox used to forward events to a running instance.
- `src/macos_url.rs` registers the macOS AppleEvent URL handler.
- `packaging/` contains per-user installation and platform URL registration.

## Validation

- `tests/native_shell_contract.rs` covers state, lifecycle, navigation,
  shortcuts, paths, and window profile.
- `tests/dashboard_view.rs` and unit tests in `dashboard_insights.rs` cover
  actual metric summaries and the ported insight payload contract.
- `tests/dashboard_reference.rs` covers reference fixtures, selector defaults,
  and rendering/source labels for the sample audit panels.
- `tests/deep_link_inbox.rs`, `tests/macos_installation.rs`, and
  `tests/windows_registration.rs` cover inbox, installer, protocol, and
  acceptance wiring contracts. The installed Darwin acceptance exercises
  cold/warm/restart URL delivery, close-to-tray, Show/Quit action handling,
  window visibility, and a successful native notification request. It requests activation through `NSRunningApplication` and `NSApplication`. The acceptance helper reports visibility, frontmost state, activation policy, and active state; strict mode reported a Regular, visible app that was not active or frontmost in this automation session. It does not prove visible notification delivery or physical tray-menu clicks. OS
  acceptance scripts exercise installed lifecycle behavior.
- Hosted Linux/Windows lifecycle evidence, visible notification delivery,
  physical tray-menu clicks, the import selection decision, Linux/Windows tray
  runtime, and visual launch checks remain in BI-049.
- A read-only `AXIsProcessTrusted` probe returned false for the current macOS
  automation process; no permission was changed. Physical tray-menu delivery
  requires an Accessibility-authorized interactive acceptance run.
