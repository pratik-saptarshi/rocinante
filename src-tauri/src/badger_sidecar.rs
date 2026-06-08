use crate::types::CommitIngestionEvent;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BadgerSidecarRequest {
    pub key: String,
    pub event: CommitIngestionEvent,
}

impl BadgerSidecarRequest {
    pub fn from_event(event: &CommitIngestionEvent, shard_count: u16) -> Self {
        let shard_count = shard_count.max(1);
        let shard = stable_shard(&event.commit_id, shard_count);
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        Self {
            key: format!("evt:shard:{shard:04}:{ts}:{}", event.commit_id),
            event: event.clone(),
        }
    }
}

fn stable_shard(value: &str, shard_count: u16) -> u16 {
    value
        .bytes()
        .fold(0u16, |acc, b| acc.wrapping_add(b as u16))
        % shard_count
}
