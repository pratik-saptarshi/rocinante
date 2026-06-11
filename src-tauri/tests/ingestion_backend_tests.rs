use repo_analyzer_core::storage::{IngestionBackendConfig, IngestionBackendKind};

#[test]
fn strict_mode_rejects_transitional_sled_backend() {
    let cfg = IngestionBackendConfig {
        kind: IngestionBackendKind::SledTransitional,
        strict_badger_required: true,
        endpoint: None,
    };

    let err = cfg.validate().expect_err("expected strict-mode rejection");
    assert!(err
        .to_string()
        .contains("Badger sidecar backend is required in strict mode"));
}

#[test]
fn strict_mode_accepts_badger_sidecar_with_endpoint() {
    let cfg = IngestionBackendConfig {
        kind: IngestionBackendKind::BadgerSidecar,
        strict_badger_required: true,
        endpoint: Some("unix:///var/run/badger.sock".to_string()),
    };

    assert!(cfg.validate().is_ok());
}

#[test]
fn badger_sidecar_requires_endpoint() {
    let cfg = IngestionBackendConfig {
        kind: IngestionBackendKind::BadgerSidecar,
        strict_badger_required: false,
        endpoint: None,
    };

    let err = cfg
        .validate()
        .expect_err("expected endpoint validation failure");
    assert!(err
        .to_string()
        .contains("Badger sidecar endpoint is required"));
}

#[test]
fn badger_sidecar_rejects_invalid_endpoint_scheme() {
    let cfg = IngestionBackendConfig {
        kind: IngestionBackendKind::BadgerSidecar,
        strict_badger_required: false,
        endpoint: Some("tcp://127.0.0.1:9000".to_string()),
    };

    let err = cfg
        .validate()
        .expect_err("expected invalid scheme validation failure");
    assert!(err
        .to_string()
        .contains("Badger sidecar endpoint must start with inproc:// or unix://"));
}

#[test]
fn badger_sidecar_rejects_empty_endpoint_with_whitespace() {
    let cfg = IngestionBackendConfig {
        kind: IngestionBackendKind::BadgerSidecar,
        strict_badger_required: false,
        endpoint: Some("   ".to_string()),
    };

    let err = cfg
        .validate()
        .expect_err("expected whitespace endpoint validation failure");
    assert!(err.to_string().contains("Badger sidecar endpoint is required"));
}
