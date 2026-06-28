use repo_analyzer_core::app_support::{app_state, build_app};
use repo_analyzer_core::auth::issue_test_token;
use repo_analyzer_core::storage::{IngestionBackendConfig, IngestionBackendKind};
use serde_json::json;
use std::sync::Mutex;
use tauri::ipc::InvokeBody;
use tauri::test::{get_ipc_response, mock_builder, mock_context, noop_assets, INVOKE_KEY};
use tauri::webview::InvokeRequest;
use tauri::WebviewWindowBuilder;

#[test]
fn registered_release_baseline_handlers_roundtrip_through_tauri_ipc() {
    let backend = IngestionBackendConfig {
        kind: IngestionBackendKind::BadgerSidecar,
        strict_badger_required: true,
        endpoint: Some("unix:///var/run/badger.sock".to_string()),
    };
    let mut state = app_state();
    state.ingestion_backend = Mutex::new(backend);

    let app = build_app(mock_builder(), state)
        .build(mock_context(noop_assets()))
        .expect("build app");
    let webview = WebviewWindowBuilder::new(&app, "main", Default::default())
        .build()
        .expect("build webview");

    let token = issue_test_token("alice", &["admin"], 3600);
    let seed_request = InvokeRequest {
        cmd: "reseed_release_baseline".into(),
        callback: tauri::ipc::CallbackFn(0),
        error: tauri::ipc::CallbackFn(1),
        url: "tauri://localhost".parse().expect("url"),
        body: InvokeBody::from(json!({
            "token": token,
            "repo_name": "repo-a",
            "baseline_complexity": 19.25
        })),
        headers: Default::default(),
        invoke_key: INVOKE_KEY.to_string(),
    };

    let seed_response = get_ipc_response(&webview, seed_request)
        .expect("seed through registered handler")
        .deserialize::<f64>()
        .expect("deserialize seed response");
    assert_eq!(seed_response, 19.25);

    let query_request = InvokeRequest {
        cmd: "query_release_baseline".into(),
        callback: tauri::ipc::CallbackFn(0),
        error: tauri::ipc::CallbackFn(1),
        url: "tauri://localhost".parse().expect("url"),
        body: InvokeBody::from(json!({
            "token": token,
            "repo_name": "repo-a"
        })),
        headers: Default::default(),
        invoke_key: INVOKE_KEY.to_string(),
    };

    let query_response = get_ipc_response(&webview, query_request)
        .expect("query through registered handler")
        .deserialize::<Option<f64>>()
        .expect("deserialize query response");
    assert_eq!(query_response, Some(19.25));
}
