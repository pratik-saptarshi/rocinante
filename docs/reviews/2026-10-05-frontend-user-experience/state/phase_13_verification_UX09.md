# Phase 13 — Targeted verification: UX-09

Verification agent: Accessibility Reviewer (gpt-6-astra substitute; Opus unavailable). Standard tier. Installed MUI source trace only.

**[VR_CONFIRMED] — High confidence, P2: The Field/Lab switch’s supplied `aria-label` reaches the SwitchBase wrapper, not the native input with `role="switch"`; adjacent text is not an associated label.**

- Installed MUI is 9.4.0 (`ui/node_modules/@mui/material/package.json:3`). `ui/src/App.tsx:792` supplies `aria-label` to `<Switch>` without `slotProps.input`, `aria-labelledby`, or an ID/label association.
- In `ui/node_modules/@mui/material/Switch/Switch.mjs:254-255,295-301`, the attribute is collected in `...other` and sent to SwitchBase. Input props come separately from `slotProps.input` (`:264,318-319`), where the native input receives `role: 'switch'`.
- `internal/SwitchBase.mjs:108-113,168-176` retains the attribute in root props and sends it to a span root; the input slot receives separate forwarded props (`:164-167,198-201`). `useSlot.mjs:62` forwards remaining attributes to the root slot.
- “Field Data” and “Lab Data” are sibling Typography elements at `App.tsx:790-797`, without IDs, `htmlFor`, or an enclosing label.

**Remedy:** Put an accessible name on the input (for example `slotProps={{ input: { 'aria-label': 'Use field data' } }}`) or associate a visible label; make the name explain what checked means.

**Limit:** No rendered accessibility tree or assistive-technology test was performed. The source-confirmed issue is the missing input-name association.
