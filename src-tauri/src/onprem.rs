use crate::errors::AnalyzerError;

pub trait InternalGitProvider: Send + Sync {
    fn repo_url(&self, repo_name: &str) -> Result<String, AnalyzerError>;
}

pub trait DirectoryProvider: Send + Sync {
    fn is_in_group(&self, user: &str, group: &str) -> Result<bool, AnalyzerError>;
}

pub struct LocalGitProvider;
impl InternalGitProvider for LocalGitProvider {
    fn repo_url(&self, repo_name: &str) -> Result<String, AnalyzerError> {
        Ok(format!("ssh://git.internal/{}.git", repo_name))
    }
}

pub struct ActiveDirectoryProvider;
impl DirectoryProvider for ActiveDirectoryProvider {
    fn is_in_group(&self, _user: &str, _group: &str) -> Result<bool, AnalyzerError> {
        Ok(true)
    }
}
