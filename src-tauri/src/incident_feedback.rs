use crate::risk_contract::{PrRiskDecision, PrRiskEvaluation};
use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RiskFeedbackEntry {
    pub revision: u64,
    pub kind: String,
    pub actor: String,
    pub subject: String,
    pub note: String,
    pub risk_delta: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AdjustedRiskAssessment {
    pub cache_revision: u64,
    pub risk_score: f64,
    pub decision: PrRiskDecision,
    pub audit_log: Vec<RiskFeedbackEntry>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RiskFeedbackStore {
    revision: u64,
    audit_log: Vec<RiskFeedbackEntry>,
    feedback_path: Option<PathBuf>,
}

impl RiskFeedbackStore {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, std::io::Error> {
        let feedback_path = path.as_ref().to_path_buf();
        let mut audit_log = Vec::new();
        if feedback_path.exists() {
            let raw = fs::read_to_string(&feedback_path)?;
            for line in raw.lines() {
                if line.trim().is_empty() {
                    continue;
                }
                let entry = serde_json::from_str::<RiskFeedbackEntry>(line)
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
                audit_log.push(entry);
            }
        }

        let revision = audit_log.last().map(|entry| entry.revision).unwrap_or(0);
        Ok(Self {
            revision,
            audit_log,
            feedback_path: Some(feedback_path),
        })
    }

    pub fn current_revision(&self) -> u64 {
        self.revision
    }

    pub fn audit_log(&self) -> &[RiskFeedbackEntry] {
        &self.audit_log
    }

    pub fn is_cache_valid(&self, cached_revision: u64) -> bool {
        cached_revision == self.revision
    }

    pub fn record_incident(
        &mut self,
        actor: &str,
        subject: &str,
        note: &str,
        risk_delta: f64,
    ) -> RiskFeedbackEntry {
        self.record_feedback("incident", actor, subject, note, risk_delta)
    }

    pub fn record_annotation(
        &mut self,
        actor: &str,
        subject: &str,
        note: &str,
        risk_delta: f64,
    ) -> RiskFeedbackEntry {
        self.record_feedback("annotation", actor, subject, note, risk_delta)
    }

    pub fn apply_to(&self, evaluation: &PrRiskEvaluation) -> AdjustedRiskAssessment {
        let risk_delta: f64 = self
            .audit_log
            .iter()
            .map(|entry| entry.risk_delta.max(0.0))
            .sum();
        let risk_score = (evaluation.risk_score + risk_delta).clamp(0.0, 1.0);
        let decision = if risk_score >= 0.60 {
            PrRiskDecision::Block
        } else if risk_score >= 0.35 {
            PrRiskDecision::Review
        } else {
            evaluation.decision.clone()
        };

        AdjustedRiskAssessment {
            cache_revision: self.revision,
            risk_score,
            decision,
            audit_log: self.audit_log.clone(),
        }
    }

    fn record_feedback(
        &mut self,
        kind: &str,
        actor: &str,
        subject: &str,
        note: &str,
        risk_delta: f64,
    ) -> RiskFeedbackEntry {
        self.revision += 1;
        let entry = RiskFeedbackEntry {
            revision: self.revision,
            kind: kind.to_string(),
            actor: actor.to_string(),
            subject: subject.to_string(),
            note: note.to_string(),
            risk_delta,
        };
        self.audit_log.push(entry.clone());
        if let Some(path) = &self.feedback_path {
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(path)
                .unwrap_or_else(|e| panic!("failed to open risk feedback log: {e}"));
            let raw = serde_json::to_string(&entry)
                .unwrap_or_else(|e| panic!("failed to serialize risk feedback entry: {e}"));
            file.write_all(raw.as_bytes())
                .unwrap_or_else(|e| panic!("failed to write risk feedback entry: {e}"));
            file.write_all(b"\n")
                .unwrap_or_else(|e| panic!("failed to terminate risk feedback entry: {e}"));
        }
        entry
    }
}
