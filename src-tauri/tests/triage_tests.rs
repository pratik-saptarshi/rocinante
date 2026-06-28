use repo_analyzer_core::triage::{build_triage_report, TriageFinding, TriageInput};

#[test]
fn triage_report_uses_the_expected_sections_and_report_only_banner() {
    let report = build_triage_report(TriageInput {
        report_only: true,
        findings: vec![
            TriageFinding {
                title: "panic in release flow".to_string(),
                score: 0.91,
                details: "high confidence".to_string(),
            },
            TriageFinding {
                title: "slow path".to_string(),
                score: 0.50,
                details: "watch".to_string(),
            },
            TriageFinding {
                title: "noise".to_string(),
                score: 0.10,
                details: "ignore".to_string(),
            },
        ],
        state_updates: vec![
            "blocked merge queue".to_string(),
            "waiting on review".to_string(),
        ],
    });

    assert!(report.report_only);
    assert!(report.body.starts_with("Report-only mode"));
    assert!(report.body.contains("## High-Priority"));
    assert!(report.body.contains("## Watch"));
    assert!(report.body.contains("## Noise"));
    assert!(report.body.contains("## State Updates"));
    assert_eq!(report.body.matches("## ").count(), 4);
}

#[test]
fn triage_report_keeps_only_the_declared_sections_and_sorts_highest_priority_first() {
    let report = build_triage_report(TriageInput {
        report_only: false,
        findings: vec![
            TriageFinding {
                title: "watch-first".to_string(),
                score: 0.40,
                details: "watch".to_string(),
            },
            TriageFinding {
                title: "high-first".to_string(),
                score: 0.95,
                details: "high".to_string(),
            },
            TriageFinding {
                title: "noise-first".to_string(),
                score: 0.05,
                details: "noise".to_string(),
            },
        ],
        state_updates: vec![],
    });

    assert!(!report.report_only);
    assert!(report.body.contains("high-first"));
    assert!(report.body.contains("watch-first"));
    assert!(report.body.contains("noise-first"));
    assert!(report.high_priority.len() == 1);
    assert_eq!(report.high_priority[0].title, "high-first");
    assert_eq!(report.watch[0].title, "watch-first");
    assert_eq!(report.noise[0].title, "noise-first");
    assert!(report.body.contains("No architectural invention"));
}
