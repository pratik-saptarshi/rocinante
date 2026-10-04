use rocinante_analysis::{
    auth::issue_test_token,
    git::discover_repositories_path,
    query_repository_metrics, run_scan_with_metrics,
    telemetry::TelemetryStore,
    types::{AnalysisMetric, AnalysisRecord},
};
use std::fs;
use tempfile::NamedTempFile;

#[test]
fn scans_repositories_and_persists_sanitized_metrics_without_a_host_runtime() {
    std::env::set_var(
        "RUNICIPAL_TOKEN_SECRET",
        "test-secret-for-repository-scan-32-bytes",
    );
    let root = tempfile::tempdir().expect("root directory");
    let repository = root.path().join("repo-one");
    fs::create_dir_all(repository.join(".git")).expect("git directory");
    fs::create_dir_all(repository.join("src")).expect("source directory");
    fs::write(repository.join("src/lib.rs"), "pub fn example() {}\n").expect("source file");
    let nested_repository = root.path().join("repo-two");
    fs::create_dir_all(nested_repository.join(".git")).expect("nested git directory");
    fs::create_dir_all(nested_repository.join("src")).expect("nested source directory");
    fs::write(nested_repository.join("src/lib.rs"), "pub fn nested() {}\n")
        .expect("nested source file");
    for group in ["group-a", "group-b"] {
        let duplicate_name_repository = root.path().join(group).join("shared");
        fs::create_dir_all(duplicate_name_repository.join(".git"))
            .expect("duplicate-name git directory");
        fs::create_dir_all(duplicate_name_repository.join("src"))
            .expect("duplicate-name source directory");
        fs::write(
            duplicate_name_repository.join("src/lib.rs"),
            format!("pub fn {group}_shared() {{}}\n"),
        )
        .expect("duplicate-name source file");
    }

    let database = NamedTempFile::new().expect("database file");
    let store = TelemetryStore::open(database.path()).expect("open telemetry database");
    store
        .insert_record(&AnalysisRecord {
            repo_name: "repo-one-shadow".into(),
            release: String::new(),
            metrics: vec![AnalysisMetric {
                plugin: "test".into(),
                key: "similar_repo_metric".into(),
                value: 1.0,
                details: String::new(),
            }],
        })
        .expect("insert unrelated historic metric");
    for repo_name in ["repo-one", "group-a/shared", "shared"] {
        store
            .insert_record(&AnalysisRecord {
                repo_name: repo_name.into(),
                release: "previous-release".into(),
                metrics: vec![AnalysisMetric {
                    plugin: "test".into(),
                    key: format!("legacy_metric_{repo_name}"),
                    value: 1.0,
                    details: String::new(),
                }],
            })
            .expect("insert legacy repository identity");
        store
            .insert_record(&AnalysisRecord {
                repo_name: repo_name.into(),
                release: String::new(),
                metrics: vec![AnalysisMetric {
                    plugin: "test".into(),
                    key: format!("legacy_stale_metric_{repo_name}"),
                    value: 0.0,
                    details: String::new(),
                }],
            })
            .expect("insert same-release legacy snapshot to be replaced");
    }
    drop(store);

    let token = issue_test_token("scan-admin", &["admin"], 300);
    let (result, metrics) = run_scan_with_metrics(&token, root.path(), "", database.path())
        .expect("repository scan and query");

    let stored_metrics = query_repository_metrics(&token, root.path(), "", database.path())
        .expect("query stored repository metrics");
    let historic_metrics =
        query_repository_metrics(&token, root.path(), "previous-release", database.path())
            .expect("query legacy identities for a release with no stable rows");

    assert_eq!(result.records_processed, 4);
    assert!(result.rows_inserted >= 1);
    assert_eq!(result.duplicate_source_keys, 0);
    assert_eq!(
        metrics
            .iter()
            .filter(|metric| metric.repo_name.contains(' '))
            .map(|metric| metric.repo_name.as_str())
            .collect::<std::collections::HashSet<_>>()
            .len(),
        4
    );
    assert!(metrics.iter().any(|metric| metric.release.is_empty()));
    assert!(metrics
        .iter()
        .all(|metric| metric.key != "similar_repo_metric"));
    assert!(!stored_metrics.is_empty());
    assert_eq!(
        stored_metrics
            .iter()
            .filter(|metric| metric.release.is_empty() && metric.repo_name.contains(' '))
            .map(|metric| metric.repo_name.as_str())
            .collect::<std::collections::HashSet<_>>()
            .len(),
        4
    );
    assert!(stored_metrics.iter().any(|metric| {
        metric.release == "previous-release" && metric.key.starts_with("legacy_metric_")
    }));
    assert!(!stored_metrics
        .iter()
        .any(|metric| metric.key.starts_with("legacy_stale_metric_")));
    assert!(historic_metrics.iter().any(|metric| {
        metric.repo_name == "repo-one" && metric.key == "legacy_metric_repo-one"
    }));
    assert!(historic_metrics.iter().any(|metric| {
        metric.repo_name == "group-a/shared" && metric.key == "legacy_metric_group-a/shared"
    }));
    assert_eq!(
        historic_metrics
            .iter()
            .filter(|metric| metric.repo_name == "shared" && metric.key == "legacy_metric_shared")
            .count(),
        1,
        "ambiguous basename telemetry should remain visible once at workspace scope"
    );
    assert!(!stored_metrics
        .iter()
        .any(|metric| metric.key == "legacy_stale_metric_shared"));
    assert!(metrics.iter().all(|metric| stored_metrics.contains(metric)));

    for path in [
        root.path().join("group-a/shared"),
        root.path().join("group-b/shared"),
        root.path().join("repo-one"),
    ] {
        let selected_metrics = query_repository_metrics(&token, &path, "", database.path())
            .expect("query a repository selected by its own directory");
        assert!(!selected_metrics.is_empty());
        assert!(selected_metrics.iter().all(|selected| {
            stored_metrics.iter().any(|scanned| {
                scanned.repo_name == selected.repo_name && scanned.key == selected.key
            })
        }));
    }
}

#[test]
fn scan_rejects_a_directory_without_discovered_repositories() {
    std::env::set_var(
        "RUNICIPAL_TOKEN_SECRET",
        "test-secret-for-repository-scan-32-bytes",
    );
    let root = tempfile::tempdir().expect("root directory");
    let database = NamedTempFile::new().expect("database file");
    let token = issue_test_token("scan-admin", &["admin"], 300);

    let error = run_scan_with_metrics(&token, root.path(), "", database.path())
        .expect_err("empty repository selection must fail");

    assert!(error.to_string().contains("no Git repositories found"));
}

#[test]
fn loading_metrics_rejects_a_directory_without_discovered_repositories() {
    std::env::set_var(
        "RUNICIPAL_TOKEN_SECRET",
        "test-secret-for-repository-scan-32-bytes",
    );
    let root = tempfile::tempdir().expect("root directory");
    let database = root.path().join("must-not-be-created.db");
    let token = issue_test_token("scan-admin", &["admin"], 300);

    let error = query_repository_metrics(&token, root.path(), "", &database)
        .expect_err("empty repository selection must fail");

    assert!(error.to_string().contains("no Git repositories found"));
    assert!(!database.exists());
}

#[test]
fn scan_requires_a_valid_admin_token_before_opening_the_database() {
    let root = tempfile::tempdir().expect("root directory");
    let database = root.path().join("must-not-be-created.db");
    std::env::set_var(
        "RUNICIPAL_TOKEN_SECRET",
        "test-secret-for-repository-scan-32-bytes",
    );
    let result = run_scan_with_metrics("invalid", root.path(), "", &database);
    let query_result = query_repository_metrics("invalid", root.path(), "", &database);

    assert!(result.is_err());
    assert!(query_result.is_err());
    assert!(!database.exists());
}

#[test]
fn repository_discovery_includes_worktrees_and_submodules_with_git_files() {
    let root = tempfile::tempdir().expect("root directory");
    let worktree = root.path().join("worktree");
    fs::create_dir_all(&worktree).expect("worktree");
    fs::write(worktree.join(".git"), "gitdir: ../metadata/worktree\n").expect("git file");

    let repositories = discover_repositories_path(root.path());
    assert_eq!(repositories.len(), 1);
    assert_eq!(repositories[0].path, worktree);
}
