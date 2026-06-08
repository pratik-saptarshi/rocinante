#[derive(Debug, Clone)]
pub struct RawRetentionDecision {
    ttl_secs: i64,
}

impl RawRetentionDecision {
    pub fn new(ttl_secs: i64) -> Self {
        Self { ttl_secs }
    }

    pub fn should_prune(&self, event_ts: i64, now_ts: i64) -> bool {
        now_ts - event_ts > self.ttl_secs
    }
}
