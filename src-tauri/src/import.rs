use crate::types::CommitIngestionEvent;
use std::collections::BTreeSet;

#[derive(Debug, Clone)]
pub struct BulkImportPlan {
    pub unique_events: Vec<CommitIngestionEvent>,
    pub duplicates: usize,
}

impl BulkImportPlan {
    pub fn from_events(events: Vec<CommitIngestionEvent>) -> Self {
        let mut seen = BTreeSet::new();
        let mut unique_events = Vec::new();
        let mut duplicates = 0;

        for event in events {
            if seen.insert(event.commit_id.clone()) {
                unique_events.push(event);
            } else {
                duplicates += 1;
            }
        }

        Self {
            unique_events,
            duplicates,
        }
    }
}
