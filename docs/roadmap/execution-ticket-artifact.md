# Single-Source Execution Ticket Artifact

## Source of Truth (canonical grounding)
- `docs/roadmap/feature-backlog.html`
- `docs/roadmap/beads.html`
- `docs/roadmap/bead-issue-tracker.html`
- `docs/roadmap/test-plan.html`
- `docs/roadmap/plan-review-traceability.html`

This artifact is the authoritative execution index for implementation, TDD/BDD,
tracking, and gating. All ticket states and AC are grounded in the five files
above and must be kept in sync by updating those sources first.

## Current status mapping

- `F-008A` through `F-008F`: Completed.
- `F-015`: Completed.
- `F-016`: Completed.
- `F-017`: Completed.
- `F-018`: Completed.
- `F-019`: Completed.
- `F-020`: Completed.
- `F-021`: Completed.
- `F-022`: Completed.
- `F-023`: Completed.
- `F-024`: Completed.
- `F-028`: Completed.
- `F-029`: Completed.
- `F-030`: Completed.
- `F-031`: In Progress.
- `F-032`: Completed.

## Remaining feature hierarchy

1. Control-plane convergence
   - `F-031` frontend structural decomposition
   - `FE-009` command schema and backend contract convergence
2. Dashboard and operational insight
   - `F-025` release baseline management UI
   - `F-026` job observability
3. Scale and history
   - `F-027` bulk import utility
4. Untriaged backlog tail
   - `F-033`

## Roadmap Completion Snapshot

- Completed features: `F-001` … `F-014`, `F-008A`, `F-008B`, `F-008C`, `F-008D`,
  `F-008E`, `F-008F`, `F-015`, `F-016`, `F-017`, `F-018`, `F-019`, `F-020`,
  `F-021`, `F-022`, `F-023`, `F-024`, `F-028`, `F-029`, `F-030`, `F-032` (34)
- In progress features: `F-031` (1)
- New backlog: `F-025` … `F-027`, `F-033` (4)
- Completion ratio: `34 / 39 = 87.2%`

## Capability-level execution anchors

### Capability C1: Storage Integrity and Throughput
- `F-008A` / `F-008B` / `F-008C` / `F-008D` / `F-008E` / `F-008F`

### Capability C2: Security and Trust
- `F-018` / `F-019` / `F-020` / `F-021` / `F-022` / `F-023`

### Capability C3: Sanitization Correctness
- `F-017` / `F-029` / `F-030`

### Capability C4: UX Architecture and Contract Safety
- `F-015` / `F-016` / `F-024` / `F-031` / `FE-009` / `F-032`

## Key feature AC and test mapping

- `F-018` ↔ `T-010`
- `F-020` ↔ `T-012`
- `F-021` ↔ `T-013`
- `F-022` ↔ `T-025`
- `F-023` ↔ `T-011`
- `F-024` ↔ `T-026`
- `F-031` / `FE-009` ↔ `T-FE-011`, `T-023`
- `FE-009` command failures and parity ↔ `T-021`, `T-023`

## Frontend Delivery Strand

- `F-015` Admin UI integration: Completed.
- `F-016` Rich dashboard visualizations: Completed.
- `F-017` Expanded sanitizer rules: Completed.
- `F-018` Signed scoring-config integrity verification: Completed.
- `F-024` Explainability panel: Completed.
- `F-031` Frontend structural decomposition: In Progress.

