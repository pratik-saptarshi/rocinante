use repo_analyzer_core::app_support::{app_state, build_app};
use repo_analyzer_core::storage::{IngestionBackendConfig, IngestionBackendKind};
use std::sync::Mutex;

fn main() {
    let backend = IngestionBackendConfig {
        kind: IngestionBackendKind::BadgerSidecar,
        strict_badger_required: true,
        endpoint: Some("unix:///var/run/badger.sock".to_string()),
    };
    backend
        .validate()
        .expect("invalid strict ingestion backend configuration");

    let mut state = app_state();
    state.ingestion_backend = Mutex::new(backend);

    build_app(tauri::Builder::default(), state)
        .run(tauri::generate_context!())
        .expect("tauri runtime failed");
}
