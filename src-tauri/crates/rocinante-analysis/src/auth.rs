use crate::errors::AnalyzerError;
use crate::types::Principal;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use hmac::{Hmac, KeyInit, Mac};
use rocinante_core::auth_claims::PrincipalClaims;
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};

type HmacSha256 = Hmac<Sha256>;

const JWT_ALG_HEADER: &str = r#"{"alg":"HS256","typ":"JWT"}"#;
const DEFAULT_ISSUER: &str = "rocinante-console";
const DEFAULT_AUDIENCE: &str = "repo-analyzer";
#[allow(dead_code)]
const CLAIM_TOKEN_TTL_SECONDS: i64 = 900;

fn token_secret() -> String {
    std::env::var("RUNICIPAL_TOKEN_SECRET").unwrap_or_else(|_| "dev-secret-key".to_string())
}

pub fn require_configured_token_secret() -> Result<(), AnalyzerError> {
    match std::env::var("RUNICIPAL_TOKEN_SECRET") {
        Ok(secret) if secret.len() >= 32 && secret != "dev-secret-key" => Ok(()),
        _ => Err(AnalyzerError::Integrity(
            "RUNICIPAL_TOKEN_SECRET must be configured with at least 32 bytes to scan repositories"
                .into(),
        )),
    }
}

fn now_ts() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

fn encode_base64_url(bytes: &[u8]) -> String {
    URL_SAFE_NO_PAD.encode(bytes)
}

fn decode_base64_url(payload: &str) -> Result<Vec<u8>, AnalyzerError> {
    URL_SAFE_NO_PAD
        .decode(payload)
        .map_err(|_| AnalyzerError::InvalidToken)
}

fn expected_signature(header: &str, payload: &str) -> Result<String, AnalyzerError> {
    let mut mac = HmacSha256::new_from_slice(token_secret().as_bytes())
        .map_err(|_| AnalyzerError::InvalidToken)?;
    mac.update(format!("{}.{}", header, payload).as_bytes());
    let signature = mac.finalize().into_bytes();
    Ok(encode_base64_url(&signature))
}

fn verify_signature(header: &str, payload: &str, signature: &str) -> bool {
    expected_signature(header, payload)
        .ok()
        .is_some_and(|expected| expected == signature)
}

pub fn decode_principal(token: &str) -> Result<Principal, AnalyzerError> {
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        return Err(AnalyzerError::InvalidToken);
    }

    let header = decode_base64_url(parts[0])?;
    if std::str::from_utf8(&header).map_err(|_| AnalyzerError::InvalidToken)? != JWT_ALG_HEADER {
        return Err(AnalyzerError::InvalidToken);
    }

    let payload = decode_base64_url(parts[1])?;
    let claims: PrincipalClaims =
        serde_json::from_slice(&payload).map_err(|_| AnalyzerError::InvalidToken)?;
    if !verify_signature(parts[0], parts[1], parts[2]) {
        return Err(AnalyzerError::InvalidToken);
    }

    claims
        .into_principal_if_valid(now_ts(), DEFAULT_ISSUER, DEFAULT_AUDIENCE)
        .ok_or(AnalyzerError::InvalidToken)
}

pub fn issue_test_token(user: &str, roles: &[&str], ttl_seconds: i64) -> String {
    let claims = PrincipalClaims {
        user: user.to_string(),
        roles: roles.iter().map(|role| (*role).to_string()).collect(),
        iss: DEFAULT_ISSUER.to_string(),
        aud: DEFAULT_AUDIENCE.to_string(),
        exp: now_ts() + ttl_seconds,
    };

    let header = encode_base64_url(JWT_ALG_HEADER.as_bytes());
    let payload = encode_base64_url(
        &serde_json::to_vec(&claims).expect("serialize principal claims for test token"),
    );
    let signature = expected_signature(&header, &payload).expect("sign test token");
    format!("{}.{}.{}", header, payload, signature)
}

pub fn require_admin(principal: &Principal) -> Result<(), AnalyzerError> {
    if rocinante_core::authorization::principal_is_admin(principal) {
        Ok(())
    } else {
        Err(AnalyzerError::PermissionDenied(principal.user.clone()))
    }
}
