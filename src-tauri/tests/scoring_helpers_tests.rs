use repo_analyzer_core::scoring::{normalize_scores, top_prs};
use repo_analyzer_core::types::{CommitterScore, PrRanking};

#[test]
fn normalize_scores_orders_and_scales() {
    let rows = vec![
        CommitterScore { committer: "a".to_string(), score: 10.0, complexity_component: 0.0, coverage_component: 0.0, churn_component: 0.0, pipeline_component: 0.0 },
        CommitterScore { committer: "b".to_string(), score: 20.0, complexity_component: 0.0, coverage_component: 0.0, churn_component: 0.0, pipeline_component: 0.0 },
    ];
    let out = normalize_scores(rows);
    assert_eq!(out[0].committer, "b");
    assert!(out[0].score >= out[1].score);
}

#[test]
fn top_prs_returns_highest_first() {
    let rows = vec![
        PrRanking { pr_id: "1".to_string(), repo_name: "r".to_string(), author: "a".to_string(), rank_score: 0.2, rationale: "x".to_string() },
        PrRanking { pr_id: "2".to_string(), repo_name: "r".to_string(), author: "a".to_string(), rank_score: 0.9, rationale: "x".to_string() },
    ];
    let out = top_prs(rows, 1);
    assert_eq!(out.len(), 1);
    assert_eq!(out[0].pr_id, "2");
}
