use crate::errors::AnalyzerError;
use crate::plugins::BeadPlugin;
use crate::types::{AnalysisInput, AnalysisMetric};

const REDACTED: &str = "[REDACTED]";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SanitizerPolicyPack {
    General,
    Security,
    Privacy,
    Payments,
}

impl SanitizerPolicyPack {
    fn extra_keys(self) -> &'static [&'static str] {
        match self {
            SanitizerPolicyPack::General => &[],
            SanitizerPolicyPack::Security => &["client_secret", "private_key", "bearer"],
            SanitizerPolicyPack::Privacy => &["ssn", "dob", "birth_date"],
            SanitizerPolicyPack::Payments => &["card_number", "pan", "cvv"],
        }
    }
}

fn redact_with_patterns(
    input: &str,
    pack: SanitizerPolicyPack,
    apply_general_cleanup: bool,
) -> String {
    let mut out = input.to_string();

    for key in [
        "x-api-key",
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

    for key in pack.extra_keys() {
        out = redact_key_value(&out, key);
    }

    if apply_general_cleanup {
        out = redact_emails(&out);
        out = redact_phone_like(&out);
    }

    out
}

fn redact_key_value(text: &str, key: &str) -> String {
    let lower = text.to_lowercase();
    let key_lower = key.to_lowercase();
    let mut out = String::new();
    let mut cursor = 0usize;
    let key_len = key_lower.len();

    while cursor < lower.len() {
        let remaining = &lower[cursor..];
        let Some(match_offset) = remaining.find(&key_lower) else {
            out.push_str(&text[cursor..]);
            break;
        };

        let key_start = cursor + match_offset;
        let key_end = key_start + key_len;
        if key_end > lower.len() {
            out.push_str(&text[cursor..]);
            break;
        }

        if !lower.is_char_boundary(key_start) || !lower.is_char_boundary(key_end) {
            out.push_str(&text[cursor..cursor + 1]);
            cursor += 1;
            continue;
        }

        if !is_token_boundary_before(lower.as_bytes(), key_start)
            || !is_token_boundary_after(lower.as_bytes(), key_end)
        {
            out.push_str(&text[cursor..key_end]);
            cursor = key_end;
            continue;
        }

        let mut j = key_end;
        let mut separator_found = false;
        while j < lower.len() {
            if !lower.is_char_boundary(j) {
                j = next_char_boundary(text, j);
                continue;
            }
            let ch = text[j..].chars().next().unwrap_or_default();
            if ch == '=' || ch == ':' {
                separator_found = true;
                break;
            }
            if ch.is_ascii_alphanumeric() || ch == '_' {
                break;
            }
            j += ch.len_utf8();
        }

        if !separator_found {
            out.push_str(&text[cursor..key_end]);
            cursor = key_end;
            continue;
        }

        out.push_str(&text[cursor..key_start]);

        out.push_str(&text[key_start..=j]);
        j += 1;
        while j < text.len() {
            if !lower.is_char_boundary(j) {
                j = next_char_boundary(text, j);
                continue;
            }
            let next = text.as_bytes()[j];
            if next == b' ' || next == b'\t' {
                out.push(next as char);
                j += 1;
                continue;
            }
            break;
        }

        out.push_str(REDACTED);
        while j < text.len() {
            if !lower.is_char_boundary(j) {
                j = next_char_boundary(text, j);
                continue;
            }
            let next = text.as_bytes()[j];
            if matches!(next, b' ' | b'\t' | b'\r' | b'\n' | b';' | b',') {
                break;
            }
            j += 1;
        }
        cursor = j;
        continue;
    }

    out
}

fn next_char_boundary(text: &str, cursor: usize) -> usize {
    if cursor >= text.len() {
        return cursor;
    }
    text.char_indices()
        .find_map(|(idx, _)| (idx > cursor).then_some(idx))
        .unwrap_or(text.len())
}

fn is_token_boundary_before(bytes: &[u8], key_start: usize) -> bool {
    if key_start == 0 {
        return true;
    }
    !is_token_byte(bytes[key_start - 1])
}

fn is_token_boundary_after(bytes: &[u8], key_end: usize) -> bool {
    if key_end >= bytes.len() {
        return true;
    }
    !is_token_byte(bytes[key_end])
}

fn is_token_byte(byte: u8) -> bool {
    (byte as char).is_ascii_alphanumeric() || byte == b'_'
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
    redact_with_patterns(input, SanitizerPolicyPack::General, true)
}

pub fn scrub_text_with_pack(input: &str, pack: SanitizerPolicyPack) -> String {
    redact_with_patterns(input, pack, false)
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
    use super::{scrub_text, scrub_text_with_pack, SanitizerPolicyPack};

    #[test]
    fn redacts_token_and_email() {
        let raw = "token=abc123 owner=alice@example.com";
        let scrubbed = scrub_text(raw);
        assert!(scrubbed.contains("token=[REDACTED]"));
        assert!(!scrubbed.contains("alice@example.com"));
    }

    #[test]
    fn handles_unicode_key_and_value_boundaries() {
        let raw = "x-api-key=abc🙂 secret=top-secret payload=abc";
        let scrubbed = scrub_text(raw);
        assert!(scrubbed.contains("x-api-key=[REDACTED]"));
        assert!(scrubbed.contains("secret=[REDACTED]"));
        assert!(!scrubbed.contains("top-secret"));
    }

    #[test]
    fn applies_security_policy_pack_to_additional_credentials() {
        let raw = "client_secret=sk_live bearer=eyJhbGciOiJIUzI1NiJ9 private_key=-----BEGIN";
        let scrubbed = scrub_text_with_pack(raw, SanitizerPolicyPack::Security);
        assert!(scrubbed.contains("client_secret=[REDACTED]"));
        assert!(scrubbed.contains("bearer=[REDACTED]"));
        assert!(scrubbed.contains("private_key=[REDACTED]"));
    }

    #[test]
    fn applies_privacy_and_payments_packs_without_affecting_general_scrub() {
        let privacy = scrub_text_with_pack(
            "ssn=123-45-6789 dob=1990-01-01",
            SanitizerPolicyPack::Privacy,
        );
        let payments = scrub_text_with_pack(
            "card_number=4111111111111111 cvv=123",
            SanitizerPolicyPack::Payments,
        );
        let general = scrub_text_with_pack("token=abc123", SanitizerPolicyPack::General);

        assert!(privacy.contains("ssn=[REDACTED]"));
        assert!(privacy.contains("dob=[REDACTED]"));
        assert!(payments.contains("card_number=[REDACTED]"));
        assert!(payments.contains("cvv=[REDACTED]"));
        assert!(general.contains("token=[REDACTED]"));
    }
}
