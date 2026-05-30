# Rocinante Tauri Repo Analyzer

Rust + Tauri on-prem repository analysis application with a bead-based plugin engine.

## Scope

- Recursive scanning of local Git repositories
- Modular bead plugin pipeline
- Built-in beads:
  - Code quality checks
  - Cyclomatic complexity estimation
  - Contribution velocity
  - PR approval fidelity checks
- RBAC-restricted admin command surface
- Local-only telemetry storage (no external egress)

## Run tests

```bash
cargo test --manifest-path src-tauri/Cargo.toml
```

## Repository setup

For secure GitHub repository bootstrap and branch protections, see:

- `docs/github-setup.md`
