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
