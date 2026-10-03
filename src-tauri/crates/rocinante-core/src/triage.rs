use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TriageFinding {
    pub title: String,
    pub score: f64,
    pub details: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TriageInput {
    pub report_only: bool,
    pub findings: Vec<TriageFinding>,
    pub state_updates: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TriageReport {
    pub report_only: bool,
    pub high_priority: Vec<TriageFinding>,
    pub watch: Vec<TriageFinding>,
    pub noise: Vec<TriageFinding>,
    pub state_updates: Vec<String>,
    pub body: String,
}

pub fn build_triage_report(input: TriageInput) -> TriageReport {
    let mut high_priority = Vec::new();
    let mut watch = Vec::new();
    let mut noise = Vec::new();

    for finding in input.findings {
        if finding.score >= 0.75 {
            high_priority.push(finding);
        } else if finding.score >= 0.35 {
            watch.push(finding);
        } else {
            noise.push(finding);
        }
    }

    high_priority.sort_by(|a, b| b.score.total_cmp(&a.score));
    watch.sort_by(|a, b| b.score.total_cmp(&a.score));
    noise.sort_by(|a, b| b.score.total_cmp(&a.score));

    let mut body = String::new();
    if input.report_only {
        body.push_str("Report-only mode\n");
    }
    body.push_str("## High-Priority\n");
    body.push_str(&format_section(&high_priority));
    body.push_str("## Watch\n");
    body.push_str(&format_section(&watch));
    body.push_str("## Noise\n");
    body.push_str(&format_section(&noise));
    body.push_str("## State Updates\n");
    if input.state_updates.is_empty() {
        body.push_str("No architectural invention\n");
    } else {
        for update in &input.state_updates {
            body.push_str("- ");
            body.push_str(update);
            body.push('\n');
        }
        body.push_str("No architectural invention\n");
    }

    TriageReport {
        report_only: input.report_only,
        high_priority,
        watch,
        noise,
        state_updates: input.state_updates,
        body,
    }
}

fn format_section(findings: &[TriageFinding]) -> String {
    if findings.is_empty() {
        return "None\n".to_string();
    }

    let mut out = String::new();
    for finding in findings {
        out.push_str("- ");
        out.push_str(&finding.title);
        out.push_str(" | ");
        out.push_str(&finding.details);
        out.push('\n');
    }
    out
}
