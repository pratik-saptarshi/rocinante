use crate::errors::AnalyzerError;
use std::collections::BTreeMap;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GitProviderKind {
    GitHubEnterprise,
    GitLabSelfManaged,
    BitbucketServer,
}

#[derive(Debug, Clone)]
pub struct ProviderEndpoint {
    pub kind: GitProviderKind,
    pub url: String,
}

impl ProviderEndpoint {
    pub fn new(kind: GitProviderKind, url: &str) -> Result<Self, AnalyzerError> {
        if url.contains("api.github.com") || url.contains("gitlab.com") || url.contains("bitbucket.org") {
            return Err(AnalyzerError::PermissionDenied(
                "provider endpoint must be on-prem".to_string(),
            ));
        }
        if !url.starts_with("https://") && !url.starts_with("ssh://") {
            return Err(AnalyzerError::PermissionDenied(
                "provider endpoint must use an internal secure scheme".to_string(),
            ));
        }
        Ok(Self {
            kind,
            url: url.to_string(),
        })
    }
}

#[derive(Debug, Default, Clone)]
pub struct DirectoryGroupCache {
    groups_by_user: BTreeMap<String, Vec<String>>,
}

impl DirectoryGroupCache {
    pub fn insert(&mut self, user: &str, groups: Vec<String>) {
        self.groups_by_user.insert(user.to_string(), groups);
    }

    pub fn roles_for(&self, user: &str) -> Vec<String> {
        self.groups_by_user
            .get(user)
            .into_iter()
            .flat_map(|groups| groups.iter())
            .filter_map(|group| match group.as_str() {
                "repo-admins" => Some("admin".to_string()),
                _ => None,
            })
            .collect()
    }
}
