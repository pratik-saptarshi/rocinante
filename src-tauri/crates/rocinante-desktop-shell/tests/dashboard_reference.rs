#![cfg(feature = "native-ui")]

use rocinante_desktop_shell::dashboard_reference::{
    Finding, FindingStatus, PerformanceDataMode, SeoScope, ACCESSIBILITY_FINDINGS,
    ACCESSIBILITY_SCORE, DRUPAL_RECOMMENDATION, PERFORMANCE_FINDINGS, PERFORMANCE_SCORE,
    SECURITY_FINDINGS, SEO_FINDINGS,
};

#[test]
fn companion_audit_panels_keep_their_sample_metrics_and_findings() {
    assert_eq!(ACCESSIBILITY_SCORE, 85);
    assert_eq!(
        ACCESSIBILITY_FINDINGS,
        [
            Finding {
                id: "alt-text",
                text: "Missing image alt text (7 instances)",
                status: FindingStatus::Bad,
            },
            Finding {
                id: "contrast",
                text: "Low color contrast on buttons",
                status: FindingStatus::Medium,
            },
            Finding {
                id: "keyboard",
                text: "Keyboard navigation traps found",
                status: FindingStatus::Medium,
            },
        ]
    );
    assert_eq!(
        SEO_FINDINGS.map(|finding| finding.text),
        [
            "Improve H1 and title tag consistency",
            "Add FAQ schema",
            "Optimize for long-tail, question-based queries",
        ]
    );
    assert_eq!(
        SECURITY_FINDINGS.map(|finding| finding.status),
        [FindingStatus::Bad, FindingStatus::Good, FindingStatus::Good]
    );
    assert_eq!(
        PERFORMANCE_FINDINGS.map(|finding| finding.id),
        ["lcp", "cls", "tbt"]
    );
    assert_eq!(PERFORMANCE_SCORE, 65);
    assert_eq!(
        DRUPAL_RECOMMENDATION,
        "Install SecKit module for enhanced CSP."
    );
}

#[test]
fn companion_audit_selectors_preserve_the_react_initial_state_and_labels() {
    assert_eq!(SeoScope::default().label(), "Current Page");
    assert_eq!(SeoScope::SiteWide.label(), "Site-Wide");
    assert_eq!(PerformanceDataMode::default().label(), "Field Data");
    assert_eq!(PerformanceDataMode::default().toggled().label(), "Lab Data");
}

#[test]
fn dashboard_renders_the_reference_panels_in_the_native_companion_view() {
    let source = include_str!("../src/lib.rs");
    let panels = source
        .find("fn show_companion_reference_panels")
        .expect("native reference panel renderer exists");
    let companion_view = source
        .find("show_companion_reference_panels(")
        .expect("Dashboard invokes reference panel renderer");
    assert!(companion_view < panels);
    for label in [
        "Showing static sample audit data; these panels are not repository scan results.",
        "Site audits and reference metrics",
        "WCAG 2.1/2.2 AA Accessibility Audit",
        "Run Full Audit",
        "SEO, GEO & AEO Performance",
        "Security & Drupal Review",
        "Page Performance Metrics",
        "Field Data",
        "Lab Data",
    ] {
        assert!(
            source.contains(label),
            "missing native panel label: {label}"
        );
    }
}

#[test]
fn native_insight_panel_keeps_audience_specific_focus_sections() {
    let source = include_str!("../src/lib.rs");
    for label in [
        "Team Lead Focus",
        "Top Commit Risks",
        "Manager Focus",
        "Bottleneck Radar",
        "Executive Focus",
        "Top Improvement Opportunities",
        "Security Focus",
        "Security-Weighted Commit Signals",
    ] {
        assert!(
            source.contains(label),
            "missing audience focus label: {label}"
        );
    }
    assert!(source.contains("audience.tone()"));
    assert!(source.contains("audience.guidance()"));
}
