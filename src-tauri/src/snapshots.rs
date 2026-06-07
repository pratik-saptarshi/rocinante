use crate::errors::AnalyzerError;
use crate::storage::AnalyticsSnapshot;
use std::path::{Path, PathBuf};

pub struct AnalyticsSnapshotManager {
    root: PathBuf,
}

impl AnalyticsSnapshotManager {
    pub fn new(root: &Path) -> Self {
        Self {
            root: root.to_path_buf(),
        }
    }

    pub fn materialize(
        &self,
        source_path: &str,
        snapshot_id: u64,
    ) -> Result<AnalyticsSnapshot, AnalyzerError> {
        let target = self.root.join(format!("snapshot-{snapshot_id}.duckdb"));
        std::fs::copy(source_path, &target).map_err(|e| AnalyzerError::Io(e.to_string()))?;
        Ok(AnalyticsSnapshot::new(
            target.to_str().unwrap_or_default(),
            snapshot_id,
        ))
    }
}
