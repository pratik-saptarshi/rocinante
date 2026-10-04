use repo_analyzer_core::app_support::{app_state, build_app};
fn main() {
    build_app(tauri::Builder::default(), app_state())
        .run(tauri::generate_context!())
        .expect("tauri runtime failed");
}
