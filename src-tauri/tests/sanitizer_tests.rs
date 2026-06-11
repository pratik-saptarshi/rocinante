use repo_analyzer_core::plugins::sanitizer::scrub_text;

#[test]
fn scrubs_pii_and_secret_values() {
    let raw = "password:letmein email=bob@corp.local phone=3125550101";
    let scrubbed = scrub_text(raw);
    assert!(scrubbed.contains("password:[REDACTED]") || scrubbed.contains("password: [REDACTED]"));
    assert!(!scrubbed.contains("bob@corp.local"));
    assert!(!scrubbed.contains("3125550101"));
}

#[test]
fn scrubs_secret_values_with_emoji_separator_noise() {
    let raw = "password🙂=letmein token🔥:abc123";
    let scrubbed = scrub_text(raw);
    assert!(scrubbed.contains("password🙂=[REDACTED]"));
    assert!(scrubbed.contains("token🔥:[REDACTED]"));
    assert!(!scrubbed.contains("letmein"));
    assert!(!scrubbed.contains("abc123"));
}
