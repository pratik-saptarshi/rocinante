use crate::errors::AnalyzerError;
use crate::types::Principal;

pub fn decode_principal(token: &str) -> Result<Principal, AnalyzerError> {
    let parts: Vec<&str> = token.split(':').collect();
    if parts.len() != 2 {
        return Err(AnalyzerError::InvalidToken);
    }
    let roles = parts[1]
        .split(',')
        .map(str::trim)
        .filter(|r| !r.is_empty())
        .map(ToString::to_string)
        .collect::<Vec<_>>();
    Ok(Principal {
        user: parts[0].to_string(),
        roles,
    })
}

pub fn require_admin(principal: &Principal) -> Result<(), AnalyzerError> {
    if principal.roles.iter().any(|role| role == "admin") {
        Ok(())
    } else {
        Err(AnalyzerError::PermissionDenied(principal.user.clone()))
    }
}
