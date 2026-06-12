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

    pub fn auth_header_name(&self) -> &'static str {
        "Authorization"
    }

    pub fn auth_header_value(&self) -> String {
        format!("Bearer {}", self.token)
    }
}

fn normalize_host(host: &str) -> String {
    host.trim_end_matches('/').to_string()
}
