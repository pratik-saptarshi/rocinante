# Phase 3 — Independent review: product mental model and data trust

Reviewer model: gpt-6-astra (Opus requested by overseer skill was unavailable). Static source review only; no files/tests changed. Scope is the React browser preview, not the native application.

UX readiness: 3/10 for an externally presented preview.

### T1 — P2: Empty telemetry silently becomes invented activity

Evidence: ui/src/insight-engine.ts:146-153; sample records at :66-81; import path ui/src/App.tsx:197-203.

Behavior: Applying {"commits":[],"stages":[],"signals":[]} selects all three sample datasets because fallback uses .length. Partial real payload similarly combines real data with synthetic stages or opportunities.

Impact: Users cannot represent “no activity” and can receive recommendations about nonexistent commits, queues, or work.

Remedy: Reserve sample generation for explicit demo initialization/reset. Preserve supplied empty arrays, distinguish missing sections, and disclose provenance per section for partial imports.

Confidence: High, direct code path.

### T2 — P2: Synthetic security and audit assertions lack persistent provenance

Evidence: initial sample state ui/src/App.tsx:157; header :279-284; hardcoded accessibility score :701-714; “General Site Security: High” :760-767; fixed findings ui/src/dashboard-content.ts:35-55.

Behavior: A fresh visit displays risk recommendations, score 85, “CSP: Missing,” and enabled HSTS. “Reset to Sample” at App.tsx:483-485 is a clue, but does not identify the source of each result. These panels stay static after importing telemetry.

Remedy: Show a persistent Browser preview · Sample data label, identify imported versus sample sections, label fixed audit panels “Illustrative example — no audit performed,” and show source/target/time when real results exist.

Confidence: High for presentation; user misinterpretation is an inference.

### T3 — P2: Visible controls promise operations or data changes they do not perform

Evidence: App.tsx:705-713 has no Run Full Audit handler; :733-744 switches tabs while metrics remain fixed; :787-797 changes fieldData without changing outputs.

Behavior: Audit action does nothing; selecting scope/source changes no underlying values or findings.

Remedy: Implement observable results where supported. Otherwise explain/disable the affordance or render a labelled noninteractive example.

Confidence: High.

### T4 — P2 initially: Browser admin controls imply an unavailable operation and obsolete recovery route

Evidence: enabled admin buttons ui/src/admin-bridge-panel.tsx:27-31; baseline actions ui/src/App.tsx:537-542; runtime lookup ui/src/tauri-admin.ts:146-150; fallback :182-187; architectural boundary ui/codemap.md:3-6, 23-26.

Behavior: In ordinary browser use, admin operations fail with a Tauri-only message. Current documentation describes a separate Rust native shell and compatibility-only preview adapter.

Remedy: Display runtime capability before interaction; keep controls on a developer/testing surface and explain the true browser/native boundary.

Confidence: High for fallback, medium for intended audience.

Follow-up question (not defect): Is this preview intended for internal contract development only or stakeholder demonstrations? The design choice affects prominence, but unlabelled sample results remain misleading without explanation.
