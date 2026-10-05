pub mod admin;
pub mod budget_guard;
pub mod ci_gate;
pub mod command_compat;
pub mod fix_proposal;
pub mod git_providers;
pub mod incident_feedback;
pub mod onprem;
pub mod risk_contract;
pub mod roadmap_coherence;
pub mod scoring;
pub mod storage;
pub mod team_policies;
pub mod triage;
pub mod verifier;

/// Compatibility alias for the former Tauri command helper module.
pub use command_compat as tauri_commands;
pub use rocinante_analysis::{auth, engine, errors, git, plugins, telemetry, types};
