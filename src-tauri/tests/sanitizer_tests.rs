use repo_analyzer_core::plugins::sanitizer::{
    scrub_text, scrub_text_with_pack, SanitizerPolicyPack,
};

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

#[test]
fn scrubs_policy_pack_specific_values() {
    let raw = "ssn=123-45-6789 card_number=masked client_secret=super-secret";

    let privacy = scrub_text_with_pack(raw, SanitizerPolicyPack::Privacy);
    assert!(privacy.contains("ssn=[REDACTED]"));
    assert!(privacy.contains("card_number=masked"));
    assert!(privacy.contains("client_secret=super-secret"));

    let payments = scrub_text_with_pack(raw, SanitizerPolicyPack::Payments);
    assert!(payments.contains("card_number=[REDACTED]"));
    assert!(payments.contains("ssn=123-45-6789"));
    assert!(payments.contains("client_secret=super-secret"));

    let security = scrub_text_with_pack(raw, SanitizerPolicyPack::Security);
    assert!(security.contains("client_secret=[REDACTED]"));
    assert!(security.contains("ssn=123-45-6789"));
    assert!(security.contains("card_number=masked"));
}
