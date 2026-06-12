use thiserror::Error;

#[derive(Debug, Error)]
pub enum AnalyzerError {
    #[error("permission denied: {0}")]
    PermissionDenied(String),
    #[error("invalid token")]
    InvalidToken,
    #[error("io error: {0}")]
    Io(String),
    #[error("db error: {0}")]
    Db(String),
    #[error("integrity error: {0}")]
    Integrity(String),
    #[error("git error: {0}")]
    Git(String),
}

impl From<std::io::Error> for AnalyzerError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value.to_string())
    }
}

impl From<rusqlite::Error> for AnalyzerError {
    fn from(value: rusqlite::Error) -> Self {
        Self::Db(value.to_string())
    }
}
