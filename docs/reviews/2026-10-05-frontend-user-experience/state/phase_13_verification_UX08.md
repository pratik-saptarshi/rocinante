# Phase 13 — Targeted verification: UX-08

Verification agent: Accessibility Reviewer (gpt-6-astra substitute; Opus unavailable). Standard tier. Theme/component trace and numerical calculation; no rendered inspection.

**[VR_CONFIRMED] — High confidence, P2: Default primary action text is approximately 4.288:1, below WCAG 2.2 AA 1.4.3’s 4.5:1 requirement for normal-size text.**

- `ui/src/main.tsx:6-12` sets primary `#5577cc`, light mode, and white paper. Installed MUI is 9.4.0 (`ui/node_modules/@mui/material/package.json:3`).
- MUI’s `contrastThreshold` defaults to 3 and its palette selects white contrast text for this primary (`ui/node_modules/@mui/material/styles/createPalette.js:211,226-230`). Contained buttons use `primary.contrastText` over `primary.main`; outlined buttons use `primary.main` as text (`Button/Button.js:167-172`).
- Default button text is 14px and small button text 13px (`styles/createTypography.js:25,29,74`; `Button/Button.js:218-241`), both normal-size text. The contained and outlined default backgrounds are the primary blue and white paper respectively.

For `#5577cc`, WCAG channel linearization gives relative luminance `L = 0.1948457977`; contrast with white is `(1 + 0.05) / (L + 0.05) = 4.2884134003:1`, below 4.5:1. This covers enabled, unhovered default-state button text, including Apply Payload/Reset to Sample and Run Full Audit.

**Limit:** No browser computed-style inspection, screenshots, tests, or edits. This is not a complete WCAG audit. Criterion: [WCAG 2.2 SC 1.4.3](https://www.w3.org/TR/wcag/#contrast-minimum).
