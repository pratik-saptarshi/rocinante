use crate::types::Principal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrincipalClaims {
    pub user: String,
    pub roles: Vec<String>,
    pub iss: String,
    pub aud: String,
    pub exp: i64,
}

impl PrincipalClaims {
    /// Validate the application claims against adapter-supplied time and token context.
    pub fn into_principal_if_valid(
        self,
        now: i64,
        expected_issuer: &str,
        expected_audience: &str,
    ) -> Option<Principal> {
        if self.iss != expected_issuer
            || self.aud != expected_audience
            || self.exp <= now
            || self.user.trim().is_empty()
        {
            return None;
        }

        Some(Principal {
            user: self.user,
            roles: self.roles,
        })
    }
}
