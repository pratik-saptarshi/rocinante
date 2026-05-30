use repo_analyzer_core::auth::{decode_principal, require_admin};

#[test]
fn admin_role_is_accepted() {
    let p = decode_principal("alice:reader,admin").expect("token");
    assert!(require_admin(&p).is_ok());
}

#[test]
fn non_admin_is_rejected() {
    let p = decode_principal("bob:reader").expect("token");
    assert!(require_admin(&p).is_err());
}
