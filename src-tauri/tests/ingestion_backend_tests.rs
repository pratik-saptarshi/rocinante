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

    let err = cfg.validate().expect_err("expected endpoint validation failure");
    assert!(err
        .to_string()
        .contains("Badger sidecar endpoint is required"));
}
