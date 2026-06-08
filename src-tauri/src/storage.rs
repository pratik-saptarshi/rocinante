use crate::errors::AnalyzerError;
use crate::plugins::sanitizer::scrub_text;
use crate::types::{
    AdminQuery, CommitIngestionEvent, CommitterScore, PrCandidate, PrRanking, ScoringWeights,
    TelemetryPoint,
};
use duckdb::{params, Connection};
use serde::{Deserialize, Serialize};
use sled::Db;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::mpsc::{sync_channel, Receiver, RecvTimeoutError, SyncSender};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
#[cfg(unix)]
use std::{io::Write, os::unix::net::UnixStream};

type QueuedIngestionEvent = (CommitIngestionEvent, Instant);

fn update_max_u64(metric: &AtomicU64, value: u64) {
    let mut observed = metric.load(Ordering::Acquire);
    while value > observed {
        match metric.compare_exchange(observed, value, Ordering::AcqRel, Ordering::Acquire) {
            Ok(_) => break,
            Err(current) => observed = current,
        }
    }
}

fn update_max_usize(metric: &AtomicUsize, value: usize) {
    let mut observed = metric.load(Ordering::Acquire);
    while value > observed {
        match metric.compare_exchange(observed, value, Ordering::AcqRel, Ordering::Acquire) {
            Ok(_) => break,
            Err(current) => observed = current,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct AsyncIngestionMetrics {
    pub queue_depth: usize,
    pub max_queue_depth: usize,
    pub promotion_count: usize,
    pub max_queue_lag_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum IngestionEngine {
    BadgerDb,
    Sled,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AnalyticsEngine {
    DuckDb,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum StorageRoute {
    Ingestion,
    Analytics,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum StorageOperation {
    IngestWrite,
    AnalyticsQuery,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AnalyticsQueryMode {
    ReadOnly,
    Mutable,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsSnapshot {
    pub path: String,
    pub snapshot_id: u64,
    pub immutable: bool,
}

impl AnalyticsSnapshot {
    pub fn new(path: &str, snapshot_id: u64) -> Self {
        Self {
            path: path.to_string(),
            snapshot_id,
            immutable: true,
        }
    }

    pub fn enforce_mode(&self, mode: AnalyticsQueryMode) -> Result<(), AnalyzerError> {
        match mode {
            AnalyticsQueryMode::ReadOnly => Ok(()),
            AnalyticsQueryMode::Mutable => Err(AnalyzerError::Db(
                "Analytics queries must execute against read-only snapshots".to_string(),
            )),
        }
    }
}

impl StorageRoute {
    pub fn enforce(&self, op: StorageOperation) -> Result<(), AnalyzerError> {
        match (self, op) {
            (StorageRoute::Ingestion, StorageOperation::IngestWrite) => Ok(()),
            (StorageRoute::Analytics, StorageOperation::AnalyticsQuery) => Ok(()),
            (StorageRoute::Analytics, StorageOperation::IngestWrite) => Err(AnalyzerError::Db(
                "Ingestion writes must route exclusively to BadgerDB".to_string(),
            )),
            (StorageRoute::Ingestion, StorageOperation::AnalyticsQuery) => Err(AnalyzerError::Db(
                "Analytics queries must route exclusively to DuckDB".to_string(),
            )),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageProfile {
    pub ingestion_engine: IngestionEngine,
    pub analytics_engine: String,
}

impl StorageProfile {
    pub fn strict_badger_duckdb() -> Self {
        Self {
            ingestion_engine: IngestionEngine::BadgerDb,
            analytics_engine: "duckdb".to_string(),
        }
    }

    pub fn validate(&self) -> Result<(), AnalyzerError> {
        if self.ingestion_engine != IngestionEngine::BadgerDb {
            return Err(AnalyzerError::Db(
                "Storage boundary violation: ingestion engine must be BadgerDB".to_string(),
            ));
        }
        if self.analytics_engine.to_lowercase() != "duckdb" {
            return Err(AnalyzerError::Db(
                "Storage boundary violation: analytics engine must be DuckDB".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionPolicy {
    pub raw_ttl_secs: i64,
}

impl RetentionPolicy {
    pub fn is_raw_event_expired(&self, event_ts: i64, now_ts: i64) -> bool {
        now_ts - event_ts > self.raw_ttl_secs
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum IngestionBackendKind {
    BadgerSidecar,
    SledTransitional,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngestionBackendConfig {
    pub kind: IngestionBackendKind,
    pub strict_badger_required: bool,
    pub endpoint: Option<String>,
}

impl IngestionBackendConfig {
    pub fn validate(&self) -> Result<(), AnalyzerError> {
        if self.strict_badger_required && self.kind != IngestionBackendKind::BadgerSidecar {
            return Err(AnalyzerError::Db(
                "Badger sidecar backend is required in strict mode".to_string(),
            ));
        }
        if self.kind == IngestionBackendKind::BadgerSidecar {
            let endpoint = self.endpoint.as_deref().unwrap_or_default().trim();
            if endpoint.is_empty() {
                return Err(AnalyzerError::Db(
                    "Badger sidecar endpoint is required".to_string(),
                ));
            }
        }
        Ok(())
    }
}

#[derive(Clone)]
pub struct DualLayerStore {
    kv: Arc<Db>,
    columnar_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifecycleStats {
    pub promoted_events: usize,
    pub pruned_events: usize,
}

pub struct AsyncIngestionEngine {
    tx: SyncSender<QueuedIngestionEvent>,
    queue_depth: Arc<AtomicUsize>,
    max_queue_depth: Arc<AtomicUsize>,
    promotion_count: Arc<AtomicUsize>,
    max_queue_lag_ms: Arc<AtomicU64>,
}

impl AsyncIngestionEngine {
    pub fn start(
        kv_path: &str,
        columnar_path: &str,
        buffer_size: usize,
    ) -> Result<Self, AnalyzerError> {
        Self::start_with_interval(kv_path, columnar_path, buffer_size, 1000)
    }

    pub fn start_with_interval(
        kv_path: &str,
        columnar_path: &str,
        buffer_size: usize,
        promotion_interval_ms: u64,
    ) -> Result<Self, AnalyzerError> {
        let store = DualLayerStore::open(kv_path, columnar_path)?;
        let (tx, rx): (
            SyncSender<QueuedIngestionEvent>,
            Receiver<QueuedIngestionEvent>,
        ) = sync_channel(buffer_size.max(1));
        let queue_depth = Arc::new(AtomicUsize::new(0));
        let max_queue_depth = Arc::new(AtomicUsize::new(0));
        let promotion_count = Arc::new(AtomicUsize::new(0));
        let max_queue_lag_ms = Arc::new(AtomicU64::new(0));
        let queue_depth_bg = Arc::clone(&queue_depth);
        let _max_queue_depth_bg = Arc::clone(&max_queue_depth);
        let promotion_count_bg = Arc::clone(&promotion_count);
        let max_queue_lag_bg = Arc::clone(&max_queue_lag_ms);
        let promotion_interval = Duration::from_millis(promotion_interval_ms.max(1));
        let mut last_promotion = Instant::now();

        thread::spawn(move || loop {
            match rx.recv_timeout(promotion_interval) {
                Ok((evt, queued_at)) => {
                    queue_depth_bg.fetch_sub(1, Ordering::AcqRel);
                    let _ = store.ingest_commit_event(&evt);
                    update_max_u64(
                        &max_queue_lag_bg,
                        queued_at.elapsed().as_millis().min(u64::MAX as u128) as u64,
                    );

                    if last_promotion.elapsed() >= promotion_interval {
                        let _ = store.promote_to_columnar();
                        promotion_count_bg.fetch_add(1, Ordering::AcqRel);
                        last_promotion = Instant::now();
                    }
                }
                Err(RecvTimeoutError::Timeout) => {
                    if last_promotion.elapsed() >= promotion_interval {
                        let _ = store.promote_to_columnar();
                        promotion_count_bg.fetch_add(1, Ordering::AcqRel);
                        last_promotion = Instant::now();
                    }
                }
                Err(RecvTimeoutError::Disconnected) => break,
            }
        });

        Ok(Self {
            tx,
            queue_depth,
            max_queue_depth,
            promotion_count,
            max_queue_lag_ms,
        })
    }

    pub fn enqueue(&self, evt: CommitIngestionEvent) -> Result<(), AnalyzerError> {
        self.queue_depth.fetch_add(1, Ordering::AcqRel);
        let queued_depth = self.queue_depth.load(Ordering::Acquire);
        update_max_usize(&self.max_queue_depth, queued_depth);

        self.tx.try_send((evt, Instant::now())).map_err(|e| {
            self.queue_depth.fetch_sub(1, Ordering::AcqRel);
            AnalyzerError::Io(format!("buffer enqueue failed: {e}"))
        })?;

        Ok(())
    }

    pub fn metrics(&self) -> AsyncIngestionMetrics {
        AsyncIngestionMetrics {
            queue_depth: self.queue_depth.load(Ordering::Acquire),
            max_queue_depth: self.max_queue_depth.load(Ordering::Acquire),
            promotion_count: self.promotion_count.load(Ordering::Acquire),
            max_queue_lag_ms: self.max_queue_lag_ms.load(Ordering::Acquire),
        }
    }

    pub fn queue_depth(&self) -> usize {
        self.queue_depth.load(Ordering::Acquire)
    }

    pub fn promotion_count(&self) -> usize {
        self.promotion_count.load(Ordering::Acquire)
    }

    pub fn max_queue_depth(&self) -> usize {
        self.max_queue_depth.load(Ordering::Acquire)
    }

    pub fn max_queue_lag_ms(&self) -> u64 {
        self.max_queue_lag_ms.load(Ordering::Acquire)
    }
}

impl DualLayerStore {
    pub fn open(kv_path: &str, columnar_path: &str) -> Result<Self, AnalyzerError> {
        let kv = sled::open(kv_path).map_err(|e| AnalyzerError::Db(e.to_string()))?;
        let this = Self {
            kv: Arc::new(kv),
            columnar_path: columnar_path.to_string(),
        };
        this.init_columnar()?;
        Ok(this)
    }

    fn event_prefix(timestamp: i64, commit_id: &str) -> String {
        let shard = shard_suffix(commit_id);
        format!("evt:{timestamp}:{shard}:{commit_id}")
    }

    fn parse_event_timestamp(key: &str) -> Option<i64> {
        let mut parts = key.split(':');
        if parts.next()? != "evt" {
            return None;
        }
        parts.next()?.parse::<i64>().ok()
    }

    fn init_columnar(&self) -> Result<(), AnalyzerError> {
        let conn =
            Connection::open(&self.columnar_path).map_err(|e| AnalyzerError::Db(e.to_string()))?;
        conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS telemetry_history (
              ts BIGINT,
              repo_name TEXT,
              release TEXT,
              committer TEXT,
              plugin TEXT,
              metric_key TEXT,
              metric_value DOUBLE,
              details TEXT
            );
            CREATE TABLE IF NOT EXISTS repo_baseline (
              repo_name TEXT PRIMARY KEY,
              baseline_complexity DOUBLE
            );
            CREATE TABLE IF NOT EXISTS pr_history (
              ts BIGINT,
              pr_id TEXT,
              repo_name TEXT,
              author TEXT,
              release TEXT,
              file_risk DOUBLE,
              author_velocity DOUBLE,
              approval_fidelity DOUBLE,
              rank_score DOUBLE
            );
            ",
        )
        .map_err(|e| AnalyzerError::Db(e.to_string()))?;
        Ok(())
    }

    pub fn ingest_commit_event(&self, event: &CommitIngestionEvent) -> Result<(), AnalyzerError> {
        let ts = now_ts();
        let mut clean = event.clone();
        clean.repo_name = scrub_text(&clean.repo_name);
        clean.release = scrub_text(&clean.release);
        clean.committer = scrub_text(&clean.committer);
        for p in &mut clean.telemetry {
            p.plugin = scrub_text(&p.plugin);
            p.metric_key = scrub_text(&p.metric_key);
            p.details = scrub_text(&p.details);
        }

        let key = Self::event_prefix(ts, &clean.commit_id);
        let bytes = serde_json::to_vec(&clean).map_err(|e| AnalyzerError::Db(e.to_string()))?;
        self.kv
            .insert(key.as_bytes(), bytes)
            .map_err(|e| AnalyzerError::Db(e.to_string()))?;
        self.kv
            .flush()
            .map_err(|e| AnalyzerError::Db(e.to_string()))?;
        Ok(())
    }

    pub fn ingest_commit_event_with_backend(
        &self,
        event: &CommitIngestionEvent,
        backend: &IngestionBackendConfig,
    ) -> Result<(), AnalyzerError> {
        backend.validate()?;
        match backend.kind {
            IngestionBackendKind::SledTransitional => self.ingest_commit_event(event),
            IngestionBackendKind::BadgerSidecar => {
                let endpoint = backend.endpoint.clone().unwrap_or_default();
                if endpoint.starts_with("inproc://") {
                    self.ingest_commit_event(event)
                } else if endpoint.starts_with("unix://") {
                    #[cfg(unix)]
                    {
                        let socket_path = endpoint.trim_start_matches("unix://");
                        let mut stream = UnixStream::connect(socket_path).map_err(|e| {
                            AnalyzerError::Io(format!("badger sidecar transport failed: {e}"))
                        })?;
                        let payload = serde_json::to_vec(event)
                            .map_err(|e| AnalyzerError::Db(e.to_string()))?;
                        stream.write_all(&payload).map_err(|e| {
                            AnalyzerError::Io(format!("badger sidecar transport failed: {e}"))
                        })?;
                        Ok(())
                    }
                    #[cfg(not(unix))]
                    {
                        Err(AnalyzerError::Io(
                            "badger sidecar transport failed: unix transport unavailable"
                                .to_string(),
                        ))
                    }
                } else {
                    Err(AnalyzerError::Io(
                        "badger sidecar transport failed: unsupported endpoint scheme".to_string(),
                    ))
                }
            }
        }
    }

    pub fn prune_raw_events(
        &self,
        policy: &RetentionPolicy,
        now_ts: i64,
    ) -> Result<usize, AnalyzerError> {
        let mut pruned = 0usize;

        for row in self.kv.scan_prefix("evt:") {
            let (k, _) = row.map_err(|e| AnalyzerError::Db(e.to_string()))?;
            let key = String::from_utf8_lossy(&k).to_string();

            if let Some(event_ts) = Self::parse_event_timestamp(&key) {
                if policy.is_raw_event_expired(event_ts, now_ts) {
                    self.kv
                        .remove(&k)
                        .map_err(|e| AnalyzerError::Db(e.to_string()))?;
                    pruned += 1;
                }
            }
        }

        self.kv
            .flush()
            .map_err(|e| AnalyzerError::Db(e.to_string()))?;
        Ok(pruned)
    }

    pub fn prune_raw_events_now(&self, policy: &RetentionPolicy) -> Result<usize, AnalyzerError> {
        self.prune_raw_events(policy, now_ts())
    }

    pub fn promote_to_columnar_with_retention(
        &self,
        policy: &RetentionPolicy,
        now_ts: i64,
    ) -> Result<LifecycleStats, AnalyzerError> {
        let pruned = self.prune_raw_events(policy, now_ts)?;
        let mut stats = self.promote_to_columnar()?;
        stats.pruned_events = pruned;
        Ok(stats)
    }

    pub fn promote_to_columnar(&self) -> Result<LifecycleStats, AnalyzerError> {
        let conn =
            Connection::open(&self.columnar_path).map_err(|e| AnalyzerError::Db(e.to_string()))?;
        let mut promoted = 0usize;

        for row in self.kv.scan_prefix("evt:") {
            let (k, v) = row.map_err(|e| AnalyzerError::Db(e.to_string()))?;
            let key = String::from_utf8_lossy(&k).to_string();
            let ts = Self::parse_event_timestamp(&key).unwrap_or_else(now_ts);
            let event: CommitIngestionEvent =
                serde_json::from_slice(&v).map_err(|e| AnalyzerError::Db(e.to_string()))?;

            for point in &event.telemetry {
                conn.execute(
                    "INSERT INTO telemetry_history
                    (ts, repo_name, release, committer, plugin, metric_key, metric_value, details)
                    VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                    params![
                        ts,
                        event.repo_name,
                        event.release,
                        event.committer,
                        point.plugin,
                        point.metric_key,
                        point.metric_value,
                        point.details
                    ],
                )
                .map_err(|e| AnalyzerError::Db(e.to_string()))?;
            }

            // Seed baseline complexity for fair delta-based scoring.
            let baseline_exists: i64 = conn
                .query_row(
                    "SELECT COUNT(*) FROM repo_baseline WHERE repo_name = ?1",
                    params![event.repo_name],
                    |r| r.get(0),
                )
                .unwrap_or(0);
            if baseline_exists == 0 {
                let initial_complexity = event
                    .telemetry
                    .iter()
                    .find(|t| t.metric_key == "estimated_cyclomatic_complexity")
                    .map(|t| t.metric_value)
                    .unwrap_or(0.0);
                conn.execute(
                    "INSERT INTO repo_baseline (repo_name, baseline_complexity) VALUES (?1, ?2)",
                    params![event.repo_name, initial_complexity],
                )
                .map_err(|e| AnalyzerError::Db(e.to_string()))?;
            }

            self.kv
                .remove(k)
                .map_err(|e| AnalyzerError::Db(e.to_string()))?;
            promoted += 1;
        }

        self.kv
            .flush()
            .map_err(|e| AnalyzerError::Db(e.to_string()))?;
        Ok(LifecycleStats {
            promoted_events: promoted,
            pruned_events: 0,
        })
    }

    pub fn aggregate_by_query(
        &self,
        query: &AdminQuery,
    ) -> Result<Vec<TelemetryPoint>, AnalyzerError> {
        let conn =
            Connection::open(&self.columnar_path).map_err(|e| AnalyzerError::Db(e.to_string()))?;
        let name = scrub_text(&query.name.clone().unwrap_or_default());
        let release = scrub_text(&query.release.clone().unwrap_or_default());
        let mut stmt = conn
            .prepare(
                "SELECT plugin, metric_key, AVG(metric_value) AS avg_value, MIN(details) AS details
                 FROM telemetry_history
                 WHERE repo_name LIKE '%' || ?1 || '%'
                   AND release LIKE '%' || ?2 || '%'
                 GROUP BY plugin, metric_key
                 ORDER BY plugin, metric_key",
            )
            .map_err(|e| AnalyzerError::Db(e.to_string()))?;

        let mut rows = stmt
            .query(params![name, release])
            .map_err(|e| AnalyzerError::Db(e.to_string()))?;
        let mut out = Vec::new();
        while let Some(row) = rows.next().map_err(|e| AnalyzerError::Db(e.to_string()))? {
            out.push(TelemetryPoint {
                plugin: row.get(0).map_err(|e| AnalyzerError::Db(e.to_string()))?,
                metric_key: row.get(1).map_err(|e| AnalyzerError::Db(e.to_string()))?,
                metric_value: row.get(2).map_err(|e| AnalyzerError::Db(e.to_string()))?,
                details: row.get(3).map_err(|e| AnalyzerError::Db(e.to_string()))?,
            });
        }
        Ok(out)
    }
    pub fn compute_committer_scores(
        &self,
        query: &AdminQuery,
        weights: &ScoringWeights,
    ) -> Result<Vec<CommitterScore>, AnalyzerError> {
        let conn =
            Connection::open(&self.columnar_path).map_err(|e| AnalyzerError::Db(e.to_string()))?;
        let name = scrub_text(&query.name.clone().unwrap_or_default());
        let release = scrub_text(&query.release.clone().unwrap_or_default());

        let sql = "
            SELECT
              h.committer,
              h.repo_name,
              AVG(CASE WHEN h.metric_key = 'estimated_cyclomatic_complexity' THEN h.metric_value END) AS complexity,
              AVG(CASE WHEN h.metric_key = 'coverage_delta' THEN h.metric_value END) AS coverage_delta,
              AVG(CASE WHEN h.metric_key = 'churn_efficiency' THEN h.metric_value END) AS churn_efficiency,
              AVG(CASE WHEN h.metric_key = 'pipeline_success' THEN h.metric_value END) AS pipeline_success,
              b.baseline_complexity
            FROM telemetry_history h
            LEFT JOIN repo_baseline b ON b.repo_name = h.repo_name
            WHERE h.repo_name LIKE '%' || ?1 || '%'
              AND h.release LIKE '%' || ?2 || '%'
            GROUP BY h.committer, h.repo_name, b.baseline_complexity
            ORDER BY h.committer
        ";

        let mut stmt = conn
            .prepare(sql)
            .map_err(|e| AnalyzerError::Db(e.to_string()))?;
        let mut rows = stmt
            .query(params![name, release])
            .map_err(|e| AnalyzerError::Db(e.to_string()))?;

        let mut out = Vec::new();
        while let Some(row) = rows.next().map_err(|e| AnalyzerError::Db(e.to_string()))? {
            let committer: String = row.get(0).map_err(|e| AnalyzerError::Db(e.to_string()))?;
            let complexity: Option<f64> = row.get(2).ok();
            let coverage_delta: Option<f64> = row.get(3).ok();
            let churn_efficiency: Option<f64> = row.get(4).ok();
            let pipeline_success: Option<f64> = row.get(5).ok();
            let baseline: Option<f64> = row.get(6).ok();

            // Deterministic baseline normalization: delta complexity vs initial state.
            let delta_c = complexity.unwrap_or(0.0) - baseline.unwrap_or(0.0);
            let cplx_component =
                (1.0 / (1.0 + delta_c.max(0.0))) * (weights.complexity_weight * 100.0);
            let cov_component = coverage_delta
                .map(|v| (v.max(-20.0) + 20.0) / 40.0 * (weights.coverage_weight * 100.0))
                .unwrap_or(weights.coverage_weight * 50.0);
            let churn_component = churn_efficiency
                .map(|v| v.clamp(0.0, 1.0) * (weights.churn_weight * 100.0))
                .unwrap_or(weights.churn_weight * 50.0);
            let pipeline_component = pipeline_success
                .map(|v| v.clamp(0.0, 1.0) * (weights.pipeline_weight * 100.0))
                .unwrap_or(weights.pipeline_weight * 50.0);
            let score = cplx_component + cov_component + churn_component + pipeline_component;

            out.push(CommitterScore {
                committer,
                score,
                complexity_component: cplx_component,
                coverage_component: cov_component,
                churn_component,
                pipeline_component,
            });
        }

        out.sort_by(|a, b| b.score.total_cmp(&a.score));
        Ok(out)
    }

    pub fn rank_open_prs(
        &self,
        prs: &[PrCandidate],
        weights: &ScoringWeights,
    ) -> Result<Vec<PrRanking>, AnalyzerError> {
        let mut out = Vec::new();
        for pr in prs {
            let risk = pr.file_risk.clamp(0.0, 1.0);
            let velocity = pr.author_velocity.clamp(0.0, 1.0);
            let approval = pr.approval_fidelity.clamp(0.0, 1.0);

            let score = (risk * weights.pr_file_risk_weight)
                + ((1.0 - velocity) * weights.pr_velocity_weight)
                + ((1.0 - approval) * weights.pr_approval_weight);
            out.push(PrRanking {
                pr_id: pr.pr_id.clone(),
                repo_name: pr.repo_name.clone(),
                author: pr.author.clone(),
                rank_score: score,
                rationale: format!(
                    "risk={:.2} velocity={:.2} approval={:.2} weights={}",
                    risk, velocity, approval, weights.version
                ),
            });
        }
        out.sort_by(|a, b| b.rank_score.total_cmp(&a.rank_score));
        Ok(out)
    }
}

fn shard_suffix(commit_id: &str) -> String {
    let mut checksum: u16 = 0;
    for byte in commit_id.as_bytes() {
        checksum = checksum.wrapping_add(*byte as u16);
    }
    format!("{:02x}", checksum % 16)
}

fn now_ts() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}
