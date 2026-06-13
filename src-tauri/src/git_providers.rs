#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GitProviderKind {
    GitHubEnterprise,
    GitLab,
    BitbucketServer,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitProviderSpec {
    pub kind: GitProviderKind,
    pub host: String,
    pub namespace: String,
    pub repository: String,
    pub token: String,
}

impl GitProviderSpec {
    pub fn api_base_url(&self) -> String {
        let host = normalize_host(&self.host);
        match self.kind {
            GitProviderKind::GitHubEnterprise => format!("{host}/api/v3"),
            GitProviderKind::GitLab => format!("{host}/api/v4"),
            GitProviderKind::BitbucketServer => format!("{host}/rest/api/1.0"),
        }
    }

    pub fn repo_clone_url(&self) -> String {
        let host = normalize_host(&self.host);
        match self.kind {
            GitProviderKind::BitbucketServer => {
                format!("{host}/scm/{}/{}.git", self.namespace, self.repository)
            }
            _ => format!("{host}/{}/{}.git", self.namespace, self.repository),
        }
    }

    pub fn repo_api_url(&self) -> String {
        let host = self.api_base_url();
        match self.kind {
            GitProviderKind::GitHubEnterprise => {
                format!("{host}/repos/{}/{}", self.namespace, self.repository)
            }
            GitProviderKind::GitLab => {
                format!("{host}/projects/{}%2F{}", self.namespace, self.repository)
            }
            GitProviderKind::BitbucketServer => format!(
                "{host}/projects/{}/repos/{}",
                self.namespace, self.repository
            ),
        }
    }

    pub fn pull_request_api_url(&self) -> String {
        let repo_api = self.repo_api_url();
        match self.kind {
            GitProviderKind::GitHubEnterprise => format!("{repo_api}/pulls"),
            GitProviderKind::GitLab => format!("{repo_api}/merge_requests"),
            GitProviderKind::BitbucketServer => format!("{repo_api}/pull-requests"),
        }
    }

    pub fn auth_header_name(&self) -> &'static str {
        match self.kind {
            GitProviderKind::GitLab => "PRIVATE-TOKEN",
            _ => "Authorization",
        }
    }

    pub fn auth_header_value(&self) -> String {
        match self.kind {
            GitProviderKind::GitLab => self.token.clone(),
            _ => format!("Bearer {}", self.token),
        }
    }
}

fn normalize_host(host: &str) -> String {
    host.trim_end_matches('/').to_string()
}
