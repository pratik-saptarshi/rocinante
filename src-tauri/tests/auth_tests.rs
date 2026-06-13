use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use hmac::{Hmac, KeyInit, Mac};
use repo_analyzer_core::auth::{decode_principal, issue_test_token, require_admin};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

fn sign_token_segment(header: &str, payload: &str, secret: &str) -> String {
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).expect("hmac key");
    mac.update(format!("{}.{}", header, payload).as_bytes());
    let signature = mac.finalize().into_bytes();
    URL_SAFE_NO_PAD.encode(signature)
}

#[test]
fn admin_role_is_accepted() {
    let token = issue_test_token("alice", &["reader", "admin"], 3600);
    let p = decode_principal(&token).expect("token");
    assert!(require_admin(&p).is_ok());
}

#[test]
fn non_admin_is_rejected() {
    let token = issue_test_token("bob", &["reader"], 3600);
    let p = decode_principal(&token).expect("token");
    assert!(require_admin(&p).is_err());
}

#[test]
fn rejects_unsigned_token() {
    let token = "bad.header.payload";
    assert!(decode_principal(token).is_err());
}

#[test]
fn rejects_expired_token() {
    let token = issue_test_token("alice", &["admin"], -3600);
    assert!(decode_principal(&token).is_err());
}

#[test]
fn rejects_wrong_audience() {
    let header = URL_SAFE_NO_PAD.encode(r#"{"alg":"HS256","typ":"JWT"}"#.as_bytes());
    let payload = URL_SAFE_NO_PAD.encode(
        r#"{"user":"alice","roles":["admin"],"iss":"rocinante-console","aud":"other","exp":9999999999}"#,
    );
    let signature = sign_token_segment(&header, &payload, "dev-secret-key");
    let token = format!("{}.{}.{}", header, payload, signature);
    assert!(decode_principal(&token).is_err());
}

#[test]
fn rejects_wrong_issuer() {
    let header = URL_SAFE_NO_PAD.encode(r#"{"alg":"HS256","typ":"JWT"}"#.as_bytes());
    let payload = URL_SAFE_NO_PAD.encode(
        r#"{"user":"alice","roles":["admin"],"iss":"other-service","aud":"repo-analyzer","exp":9999999999}"#,
    );
    let signature = sign_token_segment(&header, &payload, "dev-secret-key");
    let token = format!("{}.{}.{}", header, payload, signature);
    assert!(decode_principal(&token).is_err());
}

#[test]
fn rejects_wrong_signature() {
    let token = issue_test_token("alice", &["admin"], 3600);
    let parts: Vec<&str> = token.split('.').collect();
    assert_eq!(parts.len(), 3);
    let tampered = format!("{}.{}.{}", parts[0], parts[1], "invalidsig");
    assert!(decode_principal(&tampered).is_err());
}
