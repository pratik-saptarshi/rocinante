# Phase 3 — Independent review: accessibility and interaction

Reviewer model: gpt-6-astra (Opus requested by overseer skill was unavailable). Static source review only; no browser, assistive-technology, or layout tests. The supported native desktop UI is outside this review.

Overall accessibility and interaction score: 4/10.

### AX-1 — P2: Admin and baseline results are not announced

Evidence: ui/src/admin-bridge-panel.tsx:34 and ui/src/App.tsx:544 render changing results as caption text. Async handlers at App.tsx:215, 221, and 233 set only the result, with no pending state or focus/status management.

Impact: A screen-reader user activating a command receives no explicit announcement that it began, completed, or failed.

Remedy: Add a role=status/polite live region for progress and outcomes, associate errors with relevant inputs, and serialize conflicting operations while pending.

Confidence: High for absent explicit status semantics; no actual screen-reader output tested.

### AX-2 — P2: Field/Lab switch has no accessible name on its actual input

Evidence: ui/src/App.tsx:792 puts aria-label directly on MUI Switch; adjacent text at :791 and :793 is not a label. Installed MUI 9.4 source forwards the prop to the wrapper (ui/node_modules/@mui/material/Switch/Switch.js:308; internal/SwitchBase.js:180), while input props are separately created at :205.

Impact: Keyboard focus reaches a switch without a programmatic label identifying the setting.

Remedy: Put the label on slotProps.input or use an associated FormControlLabel. Prefer explicit Field/Lab choices if both alternatives need naming.

Confidence: High from app and installed-library source; exact assistive-technology output untested.

### AX-3 — P2: Controls advertise actions without performing them

Evidence: ui/src/App.tsx:705-713 renders enabled Run Full Audit without a click handler. fieldData is initialized at :154 and only read/updated by the switch at :792; performance values at :787-788 do not depend on it.

Impact: Click/Enter on the audit button produces no result. Changing the selector changes appearance without changing metrics.

Remedy: Implement an outcome or mark the preview affordance unavailable; label sample metrics and provide separate data or remove the ineffective selector.

Confidence: High.

### AX-4 — P2: Successful payload updates have no completion message

Evidence: ui/src/App.tsx:190-207 updates insights; controls are at :463-486, after affected summaries at :314-452. Only invalid JSON has role=alert at :488. Reset has no status at :209-213.

Impact: A keyboard/screen-reader user is not told that the data changed or where the refreshed summary is.

Remedy: Announce a short success summary and provide a “View updated summary” link or focus target. The practical scroll burden is viewport-dependent and unmeasured.

Confidence: High for absent success feedback; medium for impact.

### AX-5 — P3: SEO tabs have no panel relationship

Evidence: ui/src/App.tsx:733-745 defines tabs without IDs/aria-controls and renders metrics/guidance without a labelled tabpanel.

Impact: Selected tab and content are not programmatically connected.

Remedy: Add IDs, aria-controls, role=tabpanel, and aria-labelledby; or use a labelled filter if selection only changes shared example content.

Confidence: High.

### AX-6 — P3: Long dashboard has flat heading semantics and no main landmark

Evidence: ui/src/App.tsx:249 and :258 are generic containers; page title :280 and section titles :315/:342/:395 use subtitle variants without semantic overrides. Installed MUI maps subtitle1/subtitle2 to h6 (ui/node_modules/@mui/material/Typography/Typography.js:101-109); ui/src/main.tsx:6-13 supplies no override.

Impact: Heading navigation has no hierarchy between page title, sections, and subsections; there is no main landmark shortcut.

Remedy: Use main, one h1, section h2s, and nested h3s while retaining visual variants. Avoid empty title headings at App.tsx:570 and :668.

Confidence: High for markup; screen-reader impact inferred.

No claim was made about contrast, mobile clipping, or focus-ring visibility without rendered validation.
