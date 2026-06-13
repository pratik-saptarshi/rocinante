use repo_analyzer_core::git_providers::{GitProviderKind, GitProviderSpec};

#[test]
fn builds_provider_specific_api_bases() {
    let ghe = GitProviderSpec {
        kind: GitProviderKind::GitHubEnterprise,
        host: "https://ghe.example.com/".to_string(),
        namespace: "acme".to_string(),
        repository: "rocinante".to_string(),
        token: "ghe-token".to_string(),
    };
    let gitlab = GitProviderSpec {
        kind: GitProviderKind::GitLab,
        host: "https://gitlab.example.com".to_string(),
        namespace: "acme".to_string(),
        repository: "rocinante".to_string(),
        token: "gl-token".to_string(),
    };
    let bitbucket = GitProviderSpec {
        kind: GitProviderKind::BitbucketServer,
        host: "https://bitbucket.example.com/".to_string(),
        namespace: "PROJ".to_string(),
        repository: "rocinante".to_string(),
        token: "bb-token".to_string(),
    };

    assert_eq!(ghe.api_base_url(), "https://ghe.example.com/api/v3");
    assert_eq!(gitlab.api_base_url(), "https://gitlab.example.com/api/v4");
    assert_eq!(
        bitbucket.api_base_url(),
        "https://bitbucket.example.com/rest/api/1.0"
    );
}

#[test]
fn builds_provider_specific_clone_urls_and_auth_headers() {
    let spec = GitProviderSpec {
        kind: GitProviderKind::BitbucketServer,
        host: "https://bitbucket.example.com/".to_string(),
        namespace: "PROJ".to_string(),
        repository: "rocinante".to_string(),
        token: "bb-token".to_string(),
    };

    assert_eq!(
        spec.repo_clone_url(),
        "https://bitbucket.example.com/scm/PROJ/rocinante.git"
    );
    assert_eq!(spec.auth_header_name(), "Authorization");
    assert_eq!(spec.auth_header_value(), "Bearer bb-token");
}

#[test]
fn builds_provider_specific_repo_and_pr_endpoints() {
    let ghe = GitProviderSpec {
        kind: GitProviderKind::GitHubEnterprise,
        host: "https://ghe.example.com/".to_string(),
        namespace: "acme".to_string(),
        repository: "rocinante".to_string(),
        token: "ghe-token".to_string(),
    };
    let gitlab = GitProviderSpec {
        kind: GitProviderKind::GitLab,
        host: "https://gitlab.example.com".to_string(),
        namespace: "acme".to_string(),
        repository: "rocinante".to_string(),
        token: "gl-token".to_string(),
    };
    let bitbucket = GitProviderSpec {
        kind: GitProviderKind::BitbucketServer,
        host: "https://bitbucket.example.com".to_string(),
        namespace: "PROJ".to_string(),
        repository: "rocinante".to_string(),
        token: "bb-token".to_string(),
    };

    assert_eq!(
        ghe.repo_api_url(),
        "https://ghe.example.com/api/v3/repos/acme/rocinante"
    );
    assert_eq!(
        gitlab.repo_api_url(),
        "https://gitlab.example.com/api/v4/projects/acme%2Frocinante"
    );
    assert_eq!(
        bitbucket.repo_api_url(),
        "https://bitbucket.example.com/rest/api/1.0/projects/PROJ/repos/rocinante"
    );

    assert_eq!(
        ghe.pull_request_api_url(),
        "https://ghe.example.com/api/v3/repos/acme/rocinante/pulls"
    );
    assert_eq!(
        gitlab.pull_request_api_url(),
        "https://gitlab.example.com/api/v4/projects/acme%2Frocinante/merge_requests"
    );
    assert_eq!(
        bitbucket.pull_request_api_url(),
        "https://bitbucket.example.com/rest/api/1.0/projects/PROJ/repos/rocinante/pull-requests"
    );
}

#[test]
fn builds_provider_specific_auth_headers() {
    let gitlab = GitProviderSpec {
        kind: GitProviderKind::GitLab,
        host: "https://gitlab.example.com".to_string(),
        namespace: "acme".to_string(),
        repository: "rocinante".to_string(),
        token: "gl-token".to_string(),
    };
    let ghe = GitProviderSpec {
        kind: GitProviderKind::GitHubEnterprise,
        host: "https://ghe.example.com".to_string(),
        namespace: "acme".to_string(),
        repository: "rocinante".to_string(),
        token: "ghe-token".to_string(),
    };

    assert_eq!(gitlab.auth_header_name(), "PRIVATE-TOKEN");
    assert_eq!(gitlab.auth_header_value(), "gl-token");
    assert_eq!(ghe.auth_header_name(), "Authorization");
    assert_eq!(ghe.auth_header_value(), "Bearer ghe-token");
}
