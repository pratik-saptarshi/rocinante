use crate::errors::AnalyzerError;
use crate::plugins::BeadPlugin;
use crate::types::{AnalysisInput, AnalysisMetric};

const REDACTED: &str = "[REDACTED]";

fn redact_with_patterns(input: &str) -> String {
    let mut out = input.to_string();

    // Common key=value secret patterns.
    for key in [
        "api_key",
        "apikey",
        "access_token",
        "refresh_token",
        "password",
        "secret",
        "authorization",
        "auth",
        "token",
    ] {
        out = redact_key_value(&out, key);
    }

    // Basic PII patterns.
    out = redact_emails(&out);
    out = redact_phone_like(&out);

    out
}

fn redact_key_value(text: &str, key: &str) -> String {
    let lower = text.to_lowercase();
    let key_lower = key.to_lowercase();
    let mut out = String::new();
    let mut i = 0usize;

    while i < text.len() {
        if let Some(pos) = lower[i..].find(&key_lower) {
            let start = i + pos;
            out.push_str(&text[i..start]);

            let mut j = start + key_lower.len();
            while j < text.len() {
                let ch = text.as_bytes()[j] as char;
                if ch == ' ' || ch == '\t' {
                    j += 1;
                    continue;
                }
                break;
            }

            if j < text.len() {
                let sep = text.as_bytes()[j] as char;
                if sep == '=' || sep == ':' {
                    out.push_str(&text[start..=j]);
                    j += 1;
                    while j < text.len() {
                        let ch = text.as_bytes()[j] as char;
                        if ch == ' ' || ch == '\t' {
                            out.push(ch);
                            j += 1;
                            continue;
                        }
                        break;
                    }

                    out.push_str(REDACTED);
                    while j < text.len() {
                        let ch = text.as_bytes()[j] as char;
                        if ch == '\n'
                            || ch == '\r'
                            || ch == ';'
                            || ch == ','
                            || ch == ' '
                            || ch == '\t'
                        {
                            break;
                        }
                        j += 1;
                    }
                    i = j;
                    continue;
                }
            }

            out.push_str(&text[start..start + key_lower.len()]);
            i = start + key_lower.len();
        } else {
            out.push_str(&text[i..]);
            break;
        }
    }

    out
}

fn redact_emails(text: &str) -> String {
    text.split_whitespace()
        .map(|token| {
            if token.contains('@') && token.contains('.') {
                REDACTED.to_string()
            } else {
                token.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn redact_phone_like(text: &str) -> String {
    text.split_whitespace()
        .map(|token| {
            let digits = token.chars().filter(|c| c.is_ascii_digit()).count();
            if digits >= 10 {
                REDACTED.to_string()
            } else {
                token.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn scrub_text(input: &str) -> String {
    redact_with_patterns(input)
}

pub fn scrub_metric(metric: &mut AnalysisMetric) {
    metric.plugin = scrub_text(&metric.plugin);
    metric.key = scrub_text(&metric.key);
    metric.details = scrub_text(&metric.details);
}

pub fn scrub_record_strings(repo_name: &str, release: &str) -> (String, String) {
    (scrub_text(repo_name), scrub_text(release))
}

pub struct MandatorySanitizerPlugin;

impl BeadPlugin for MandatorySanitizerPlugin {
    fn name(&self) -> &'static str {
        "mandatory_sanitizer"
    }

    fn run(&self, input: &AnalysisInput) -> Result<Vec<AnalysisMetric>, AnalyzerError> {
        let mut findings = 0.0;
        for target in [&input.repo.name, &input.repo.path] {
            let scrubbed = scrub_text(target);
            if scrubbed != *target {
                findings += 1.0;
            }
        }
        Ok(vec![AnalysisMetric {
            plugin: self.name().to_string(),
            key: "preprocess_redaction_findings".to_string(),
            value: findings,
            details: "Mandatory pre-processing sanitizer executed".to_string(),
        }])
    }
}

#[cfg(test)]
mod tests {
    use super::scrub_text;

    #[test]
    fn redacts_token_and_email() {
        let raw = "token=abc123 owner=alice@example.com";
        let scrubbed = scrub_text(raw);
        assert!(scrubbed.contains("token=[REDACTED]"));
        assert!(!scrubbed.contains("alice@example.com"));
    }
}
