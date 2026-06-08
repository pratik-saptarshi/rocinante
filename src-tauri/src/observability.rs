#[derive(Debug, Default, Clone)]
pub struct JobMetrics {
    pub queue_depth: usize,
    pub promotion_lag_ms: u64,
    pub promoted_events: usize,
}

impl JobMetrics {
    pub fn record_enqueue(&mut self, depth: usize) {
        self.queue_depth = depth;
    }

    pub fn record_promotion_lag_ms(&mut self, lag_ms: u64) {
        self.promotion_lag_ms = lag_ms;
    }

    pub fn record_promoted_events(&mut self, count: usize) {
        self.promoted_events += count;
    }
}
