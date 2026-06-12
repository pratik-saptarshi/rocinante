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
fn scrubs_domain_specific_policy_pack_values() {
    let raw = "ssn=123-45-6789 client_secret=sk_live_private card_number=4111111111111111";
    let scrubbed = scrub_text_with_pack(raw, SanitizerPolicyPack::Privacy);
    let scrubbed = scrub_text_with_pack(&scrubbed, SanitizerPolicyPack::Security);
    let scrubbed = scrub_text_with_pack(&scrubbed, SanitizerPolicyPack::Payments);

    assert!(scrubbed.contains("ssn=[REDACTED]"));
    assert!(scrubbed.contains("client_secret=[REDACTED]"));
    assert!(scrubbed.contains("card_number=[REDACTED]"));
    assert!(!scrubbed.contains("123-45-6789"));
    assert!(!scrubbed.contains("sk_live_private"));
    assert!(!scrubbed.contains("4111111111111111"));
}
