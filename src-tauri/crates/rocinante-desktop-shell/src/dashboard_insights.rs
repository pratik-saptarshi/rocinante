use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InsightCommit {
    pub id: String,
    pub files: f64,
    pub changed_lines: f64,
    pub dependency_changes: f64,
    pub test_touch: bool,
    pub failed_automations: f64,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InsightStage {
    pub name: String,
    pub queue_depth: f64,
    pub throughput: f64,
    pub avg_latency_ms: f64,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct InsightSignal {
    pub id: String,
    pub area: String,
    pub title: String,
    pub impact: f64,
    pub effort: f64,
    pub confidence: f64,
}

#[derive(Debug, Clone, Default, PartialEq, Deserialize)]
pub struct InsightPayload {
    #[serde(default)]
    pub commits: Vec<InsightCommit>,
    #[serde(default)]
    pub stages: Vec<InsightStage>,
    #[serde(default)]
    pub signals: Vec<InsightSignal>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct InsightLimits {
    pub risks: Option<usize>,
    pub opportunities: Option<usize>,
    pub severity_threshold: Option<f64>,
    pub latency_p95_ms: Option<f64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InsightSource {
    Sample,
    Payload,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommitRiskCard {
    pub id: String,
    pub score: u8,
    pub level: RiskLevel,
    pub reasons: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RiskLevel {
    High,
    Medium,
    Good,
}

impl RiskLevel {
    pub fn label(self) -> &'static str {
        match self {
            Self::High => "high",
            Self::Medium => "medium",
            Self::Good => "good",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BottleneckCard {
    pub name: String,
    pub status: BottleneckStatus,
    pub impact: f64,
    pub rationale: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BottleneckStatus {
    Critical,
    High,
    Good,
}

impl BottleneckStatus {
    pub fn label(self) -> &'static str {
        match self {
            Self::Critical => "critical",
            Self::High => "high",
            Self::Good => "good",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpportunityCard {
    pub id: String,
    pub title: String,
    pub priority_score: u8,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DashboardInsights {
    pub source: InsightSource,
    pub commit_risk_cards: Vec<CommitRiskCard>,
    pub bottlenecks: Vec<BottleneckCard>,
    pub opportunities: Vec<OpportunityCard>,
    pub stages: Vec<InsightStage>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InsightInputError {
    InvalidJson(String),
    InvalidPayload(String),
}

impl std::fmt::Display for InsightInputError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidJson(error) => write!(formatter, "invalid JSON payload: {error}"),
            Self::InvalidPayload(error) => write!(formatter, "invalid telemetry payload: {error}"),
        }
    }
}

impl std::error::Error for InsightInputError {}

#[derive(Debug, Clone, PartialEq)]
pub struct AppliedInsights {
    pub insights: DashboardInsights,
    pub error: Option<String>,
}

impl Default for AppliedInsights {
    fn default() -> Self {
        Self {
            insights: build_dashboard_insights(None, InsightLimits::default()),
            error: None,
        }
    }
}

impl AppliedInsights {
    pub fn apply_json(&mut self, input: &str) {
        if input.trim().is_empty() {
            self.reset();
            return;
        }
        match parse_insight_input(input) {
            Ok((payload, limits)) => {
                self.insights = build_dashboard_insights(Some(payload), limits);
                self.insights.source = InsightSource::Payload;
                self.error = None;
            }
            Err(_) => self.error = Some(
                "Invalid JSON payload. Paste a valid telemetry payload to refresh the dashboard."
                    .into(),
            ),
        }
    }

    pub fn reset(&mut self) {
        self.insights = build_dashboard_insights(None, InsightLimits::default());
        self.error = None;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum StakeholderAudience {
    Lead,
    Manager,
    Executive,
    Security,
}

impl StakeholderAudience {
    pub const ALL: [Self; 4] = [Self::Lead, Self::Manager, Self::Executive, Self::Security];

    pub fn label(self) -> &'static str {
        match self {
            Self::Lead => "Team Lead",
            Self::Manager => "Manager",
            Self::Executive => "Executive",
            Self::Security => "Security",
        }
    }

    pub fn tone(self) -> &'static str {
        match self {
            Self::Lead => "Team leads: prioritize blocked PR hotspots and coaching cues.",
            Self::Manager => "Managers: monitor cycle time, reviewer load, and handoff stability.",
            Self::Executive => "Executives: monitor strategic quality and delivery predictability.",
            Self::Security => "Security: prioritize policy drift and dependency risk signals.",
        }
    }

    pub fn guidance(self) -> &'static str {
        match self {
            Self::Lead => "Prioritize review-ready commits before opening the next work cycle.",
            Self::Manager => {
                "Uncover queue pressure to rebalance approval throughput and merge cadence."
            }
            Self::Executive => {
                "Use the opportunity list to stabilize throughput and defect risk over time."
            }
            Self::Security => {
                "Surface release and dependency risks before they enter long-running branches."
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PulseSeverity {
    Good,
    Medium,
    Bad,
}

impl PulseSeverity {
    pub fn label(self) -> &'static str {
        match self {
            Self::Good => "good",
            Self::Medium => "medium",
            Self::Bad => "bad",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PulseAction {
    pub id: String,
    pub message: String,
    pub severity: PulseSeverity,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PulseRoute {
    pub owner: String,
    pub window: String,
    pub actions: Vec<String>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RiskBuckets {
    pub high: usize,
    pub medium: usize,
    pub good: usize,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct BottleneckBuckets {
    pub critical: usize,
    pub high: usize,
    pub medium: usize,
    pub good: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QualityPulse {
    pub overall_score: u8,
    pub security_signal_count: usize,
    pub top_bottleneck_name: String,
    pub top_risk_commit_id: String,
    pub top_opportunity_title: String,
    pub opportunity_count: usize,
    pub risk_buckets: RiskBuckets,
    pub bottleneck_buckets: BottleneckBuckets,
    pub recommendations: std::collections::BTreeMap<StakeholderAudience, Vec<PulseAction>>,
    pub action_routes: std::collections::BTreeMap<StakeholderAudience, PulseRoute>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VisualTone {
    Good,
    Medium,
    Bad,
}

impl VisualTone {
    pub fn label(self) -> &'static str {
        match self {
            Self::Good => "good",
            Self::Medium => "medium",
            Self::Bad => "bad",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExplainabilityTrace {
    pub id: String,
    pub title: String,
    pub summary: String,
    pub detail: String,
    pub tone: VisualTone,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrendLineCard {
    pub id: String,
    pub label: String,
    pub value: String,
    pub tone: VisualTone,
    pub rationale: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrRiskRankingCard {
    pub id: String,
    pub title: String,
    pub score: u8,
    pub tone: VisualTone,
    pub rationale: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DashboardVisuals {
    pub summary: String,
    pub trend_lines: Vec<TrendLineCard>,
    pub pr_risk_rankings: Vec<PrRiskRankingCard>,
}

pub fn build_explainability_traces(pulse: &QualityPulse) -> Vec<ExplainabilityTrace> {
    vec![
        ExplainabilityTrace {
            id: "explain-score".into(),
            title: "Score Decomposition".into(),
            summary: format!("Overall score {}/100", pulse.overall_score),
            detail: format!(
                "Risk {} high / {} medium, bottlenecks {} critical / {} high",
                pulse.risk_buckets.high,
                pulse.risk_buckets.medium,
                pulse.bottleneck_buckets.critical,
                pulse.bottleneck_buckets.high
            ),
            tone: if pulse.overall_score >= 75 {
                VisualTone::Good
            } else if pulse.overall_score >= 45 {
                VisualTone::Medium
            } else {
                VisualTone::Bad
            },
        },
        ExplainabilityTrace {
            id: "explain-risk".into(),
            title: "Top Risk Commit".into(),
            summary: pulse.top_risk_commit_id.clone(),
            detail: if pulse.risk_buckets.high > 0 {
                "High-risk commit drives merge caution.".into()
            } else {
                "No high-risk commit currently dominates the pulse.".into()
            },
            tone: if pulse.risk_buckets.high > 0 {
                VisualTone::Bad
            } else {
                VisualTone::Good
            },
        },
        ExplainabilityTrace {
            id: "explain-bottleneck".into(),
            title: "Top Bottleneck".into(),
            summary: pulse.top_bottleneck_name.clone(),
            detail: if pulse.bottleneck_buckets.critical > 0 {
                "Critical stage limits delivery confidence.".into()
            } else {
                "No critical stage is currently suppressing flow.".into()
            },
            tone: if pulse.bottleneck_buckets.critical > 0 {
                VisualTone::Bad
            } else {
                VisualTone::Medium
            },
        },
        ExplainabilityTrace {
            id: "explain-opportunity".into(),
            title: "Opportunity Lift".into(),
            summary: if pulse.opportunity_count > 0 {
                pulse.top_opportunity_title.clone()
            } else {
                "No opportunity signal".into()
            },
            detail: if pulse.opportunity_count > 0 {
                format!(
                    "{} opportunity signal(s) are boosting the score.",
                    pulse.opportunity_count
                )
            } else {
                "No opportunity signals are contributing to the current score.".into()
            },
            tone: if pulse.opportunity_count > 0 {
                VisualTone::Good
            } else {
                VisualTone::Medium
            },
        },
    ]
}

pub fn build_dashboard_visuals(insights: &DashboardInsights) -> DashboardVisuals {
    let mut sorted_risks = insights.commit_risk_cards.clone();
    sorted_risks.sort_by_key(|risk| std::cmp::Reverse(risk.score));
    let mut sorted_bottlenecks = insights.bottlenecks.clone();
    sorted_bottlenecks.sort_by(|left, right| right.impact.total_cmp(&left.impact));
    let mut sorted_opportunities = insights.opportunities.clone();
    sorted_opportunities.sort_by_key(|opportunity| std::cmp::Reverse(opportunity.priority_score));

    let top_risk = sorted_risks.first();
    let critical_risks = sorted_risks
        .iter()
        .filter(|risk| risk.level == RiskLevel::High)
        .count();
    let pressure_stages = insights
        .bottlenecks
        .iter()
        .filter(|stage| {
            matches!(
                stage.status,
                BottleneckStatus::Critical | BottleneckStatus::High
            )
        })
        .count();
    let top_stage = sorted_bottlenecks.first();
    let top_opportunity = sorted_opportunities.first();

    let trend_lines = vec![
        TrendLineCard {
            id: "risk-trajectory".into(),
            label: "PR Risk Trajectory".into(),
            value: top_risk.map_or_else(
                || "No high-risk commits".into(),
                |_| format!("{critical_risks} high-risk commits"),
            ),
            tone: top_risk.map_or(VisualTone::Good, |risk| tone_from_risk_score(risk.score)),
            rationale: top_risk.map_or_else(
                || "The current sample window has no elevated commit risks.".into(),
                |risk| {
                    format!(
                        "Top risk {} is driven by {}.",
                        risk.id,
                        risk.reasons
                            .iter()
                            .take(2)
                            .cloned()
                            .collect::<Vec<_>>()
                            .join(", ")
                            .if_empty("sample data")
                    )
                },
            ),
        },
        TrendLineCard {
            id: "bottleneck-pressure".into(),
            label: "Bottleneck Pressure".into(),
            value: format!("{pressure_stages} pressured stages"),
            tone: top_stage.map_or(VisualTone::Good, |stage| match stage.status {
                BottleneckStatus::Critical => VisualTone::Bad,
                BottleneckStatus::High => VisualTone::Medium,
                BottleneckStatus::Good => VisualTone::Good,
            }),
            rationale: top_stage.map_or_else(
                || "The current sample window has no pressured stages.".into(),
                |stage| {
                    format!(
                        "Highest-pressure stage {} needs {} attention.",
                        stage.name,
                        if stage.status == BottleneckStatus::Critical {
                            "immediate"
                        } else {
                            "near-term"
                        }
                    )
                },
            ),
        },
        TrendLineCard {
            id: "opportunity-velocity".into(),
            label: "Opportunity Velocity".into(),
            value: format!("{} actionable opportunities", insights.opportunities.len()),
            tone: top_opportunity.map_or(VisualTone::Good, |opportunity| {
                tone_from_opportunity_score(opportunity.priority_score)
            }),
            rationale: top_opportunity.map_or_else(
                || "No opportunities surfaced in the current payload.".into(),
                |opportunity| {
                    format!(
                        "Lead opportunity {} should unblock the next cycle.",
                        opportunity.title
                    )
                },
            ),
        },
    ];
    DashboardVisuals {
        summary: top_risk.map_or_else(
            || "No risk signals available".into(),
            |_| {
                format!("{critical_risks} high-risk commits and {pressure_stages} pressured stages")
            },
        ),
        trend_lines,
        pr_risk_rankings: sorted_risks
            .iter()
            .take(3)
            .map(|risk| PrRiskRankingCard {
                id: risk.id.clone(),
                title: format!("{} score {}", risk.id, risk.score),
                score: risk.score,
                tone: match risk.level {
                    RiskLevel::High => VisualTone::Bad,
                    RiskLevel::Medium => VisualTone::Medium,
                    RiskLevel::Good => VisualTone::Good,
                },
                rationale: risk.reasons.join(", ").if_empty("No named risk factors"),
            })
            .collect(),
    }
}

fn tone_from_risk_score(score: u8) -> VisualTone {
    if score >= 80 {
        VisualTone::Bad
    } else if score >= 50 {
        VisualTone::Medium
    } else {
        VisualTone::Good
    }
}

fn tone_from_opportunity_score(score: u8) -> VisualTone {
    if score >= 70 {
        VisualTone::Good
    } else if score >= 45 {
        VisualTone::Medium
    } else {
        VisualTone::Bad
    }
}

trait IfEmpty {
    fn if_empty(self, fallback: &str) -> String;
}

impl IfEmpty for String {
    fn if_empty(self, fallback: &str) -> String {
        if self.is_empty() {
            fallback.into()
        } else {
            self
        }
    }
}

pub fn build_quality_pulse(insights: &DashboardInsights) -> QualityPulse {
    let mut risk_buckets = RiskBuckets::default();
    for risk in &insights.commit_risk_cards {
        match risk.level {
            RiskLevel::High => risk_buckets.high += 1,
            RiskLevel::Medium => risk_buckets.medium += 1,
            RiskLevel::Good => risk_buckets.good += 1,
        }
    }
    let mut bottleneck_buckets = BottleneckBuckets::default();
    for bottleneck in &insights.bottlenecks {
        match bottleneck.status {
            BottleneckStatus::Critical => bottleneck_buckets.critical += 1,
            BottleneckStatus::High => bottleneck_buckets.high += 1,
            BottleneckStatus::Good => bottleneck_buckets.good += 1,
        }
    }

    let mut ranked_risks = insights.commit_risk_cards.clone();
    ranked_risks.sort_by(|left, right| {
        right
            .score
            .cmp(&left.score)
            .then_with(|| left.id.cmp(&right.id))
    });
    let mut ranked_opportunities = insights.opportunities.clone();
    ranked_opportunities.sort_by(|left, right| {
        right
            .priority_score
            .cmp(&left.priority_score)
            .then_with(|| left.id.cmp(&right.id))
    });
    let top_bottleneck =
        insights
            .bottlenecks
            .iter()
            .fold(None::<&BottleneckCard>, |best, current| match best {
                Some(best) if best.impact >= current.impact => Some(best),
                _ => Some(current),
            });
    let critical_stage = insights.bottlenecks.iter().find(|bottleneck| {
        matches!(
            bottleneck.status,
            BottleneckStatus::Critical | BottleneckStatus::High
        )
    });
    let security_signals = ranked_risks
        .iter()
        .filter(|risk| {
            risk.reasons
                .iter()
                .any(|reason| reason == "Dependency risk" || reason == "Automation failures")
        })
        .collect::<Vec<_>>();
    let recommendations = build_pulse_recommendations(
        &ranked_risks,
        &ranked_opportunities,
        critical_stage
            .map(|stage| stage.name.as_str())
            .unwrap_or("review"),
        &security_signals,
    );
    let overall_score = 100_usize
        .saturating_sub(risk_buckets.high * 10)
        .saturating_sub(bottleneck_buckets.critical * 15)
        .saturating_sub(bottleneck_buckets.high * 5)
        .max(45) as u8;

    QualityPulse {
        overall_score,
        security_signal_count: security_signals.len(),
        top_bottleneck_name: top_bottleneck
            .map(|bottleneck| bottleneck.name.clone())
            .unwrap_or_else(|| "review".into()),
        top_risk_commit_id: ranked_risks
            .first()
            .map(|risk| risk.id.clone())
            .unwrap_or_default(),
        top_opportunity_title: ranked_opportunities
            .first()
            .map(|opportunity| opportunity.title.clone())
            .unwrap_or_else(|| "trim flaky tests".into()),
        opportunity_count: insights.opportunities.len(),
        risk_buckets,
        bottleneck_buckets,
        recommendations,
        action_routes: build_pulse_routes(),
    }
}

fn normalize_pulse_severity(level: Option<RiskLevel>) -> PulseSeverity {
    match level {
        Some(RiskLevel::High) => PulseSeverity::Bad,
        Some(RiskLevel::Medium) => PulseSeverity::Medium,
        Some(RiskLevel::Good) => PulseSeverity::Good,
        None => PulseSeverity::Medium,
    }
}

fn build_pulse_recommendations(
    risks: &[CommitRiskCard],
    opportunities: &[OpportunityCard],
    critical_stage_name: &str,
    security_signals: &[&CommitRiskCard],
) -> std::collections::BTreeMap<StakeholderAudience, Vec<PulseAction>> {
    let lead = vec![
        PulseAction {
            id: "lead-1".into(),
            message: format!(
                "Focus first on high-risk commit {} before expanding the next cycle.",
                risks
                    .first()
                    .map(|risk| risk.id.as_str())
                    .unwrap_or("sample")
            ),
            severity: normalize_pulse_severity(risks.first().map(|risk| risk.level)),
        },
        PulseAction {
            id: "lead-2".into(),
            message: format!(
                "Focus first on high-risk commit {} after automation stabilizes.",
                risks
                    .get(1)
                    .map(|risk| risk.id.as_str())
                    .unwrap_or("sample")
            ),
            severity: normalize_pulse_severity(risks.get(1).map(|risk| risk.level)),
        },
    ];
    let manager = vec![
        PulseAction {
            id: "manager-1".into(),
            message: format!(
                "Critical stage(s): {critical_stage_name} need additional reviewer capacity."
            ),
            severity: PulseSeverity::Bad,
        },
        PulseAction {
            id: "manager-2".into(),
            message: format!(
                "Critical stage(s): {critical_stage_name} should stay unblocked until the queue drops."
            ),
            severity: PulseSeverity::Bad,
        },
    ];
    let executive = vec![
        PulseAction {
            id: "executive-1".into(),
            message: format!(
                "Top opportunity: {}",
                opportunities
                    .first()
                    .map(|opportunity| opportunity.title.as_str())
                    .unwrap_or("trim flaky tests")
            ),
            severity: PulseSeverity::Good,
        },
        PulseAction {
            id: "executive-2".into(),
            message: format!(
                "Top opportunity: {}",
                opportunities
                    .get(1)
                    .map(|opportunity| opportunity.title.as_str())
                    .unwrap_or("reduce dependency churn")
            ),
            severity: PulseSeverity::Good,
        },
    ];
    let security = if security_signals.is_empty() {
        vec![PulseAction {
            id: "security-1".into(),
            message: "Security-sensitive signals from the sample window should be reviewed before release.".into(),
            severity: PulseSeverity::Good,
        }]
    } else {
        security_signals
            .iter()
            .take(2)
            .enumerate()
            .map(|(index, risk)| PulseAction {
                id: format!("security-{}", index + 1),
                message: format!(
                    "Security-sensitive signals from {} should be reviewed before release.",
                    risk.id
                ),
                severity: normalize_pulse_severity(Some(risk.level)),
            })
            .collect()
    };
    std::collections::BTreeMap::from([
        (StakeholderAudience::Lead, lead),
        (StakeholderAudience::Manager, manager),
        (StakeholderAudience::Executive, executive),
        (StakeholderAudience::Security, security),
    ])
}

fn build_pulse_routes() -> std::collections::BTreeMap<StakeholderAudience, PulseRoute> {
    use StakeholderAudience::{Executive, Lead, Manager, Security};
    std::collections::BTreeMap::from([
        (
            Lead,
            PulseRoute {
                owner: "Lead Reviewer".into(),
                window: "Sprint now".into(),
                actions: vec![
                    "Focus first on high-risk commit A-124".into(),
                    "Pair with the author to clear the hot path".into(),
                ],
            },
        ),
        (
            Manager,
            PulseRoute {
                owner: "Engineering Manager".into(),
                window: "This week".into(),
                actions: vec![
                    "Critical stage(s): review queue should be rebalanced".into(),
                    "Clear review backlog before the next merge window".into(),
                ],
            },
        ),
        (
            Executive,
            PulseRoute {
                owner: "Delivery Leadership".into(),
                window: "This month".into(),
                actions: vec![
                    "Top opportunity: trim flaky tests".into(),
                    "Top opportunity: reduce dependency churn".into(),
                ],
            },
        ),
        (
            Security,
            PulseRoute {
                owner: "Security Operations".into(),
                window: "Before release".into(),
                actions: vec![
                    "Security-sensitive signals from risky commits need review".into(),
                    "Block release until policy drift is resolved".into(),
                ],
            },
        ),
    ])
}

fn parse_insight_input(input: &str) -> Result<(InsightPayload, InsightLimits), InsightInputError> {
    let value: serde_json::Value = serde_json::from_str(input)
        .map_err(|error| InsightInputError::InvalidJson(error.to_string()))?;
    let root = value.as_object();
    let payload_value = match root.and_then(|root| root.get("payload")) {
        Some(value) if value.is_array() => serde_json::Value::Object(serde_json::Map::new()),
        Some(value) if value.is_object() => value.clone(),
        _ => root
            .map(|root| serde_json::Value::Object(root.clone()))
            .unwrap_or_else(|| serde_json::Value::Object(serde_json::Map::new())),
    };
    let raw_payload: RawInsightPayload = serde_json::from_value(payload_value)
        .map_err(|error| InsightInputError::InvalidPayload(error.to_string()))?;
    let payload = InsightPayload {
        commits: raw_payload.commits.unwrap_or_default(),
        stages: raw_payload.stages.unwrap_or_default(),
        signals: raw_payload.signals.unwrap_or_default(),
    };

    let limits_value = match root.and_then(|root| root.get("limits")) {
        Some(value) if value.is_array() => serde_json::Value::Object(serde_json::Map::new()),
        Some(value) if value.is_object() => value.clone(),
        _ => root
            .map(|root| serde_json::Value::Object(root.clone()))
            .unwrap_or_else(|| serde_json::Value::Object(serde_json::Map::new())),
    };
    let limits = limits_from_value(&limits_value);
    Ok((payload, limits))
}

#[derive(Deserialize, Default)]
struct RawInsightPayload {
    commits: Option<Vec<InsightCommit>>,
    stages: Option<Vec<InsightStage>>,
    signals: Option<Vec<InsightSignal>>,
}

fn limits_from_value(value: &serde_json::Value) -> InsightLimits {
    let object = value.as_object();
    InsightLimits {
        risks: positive_limit(
            object
                .and_then(|object| object.get("risks"))
                .and_then(serde_json::Value::as_f64),
        ),
        opportunities: positive_limit(
            object
                .and_then(|object| object.get("opportunities"))
                .and_then(serde_json::Value::as_f64),
        ),
        severity_threshold: object
            .and_then(|object| object.get("severityThreshold"))
            .and_then(serde_json::Value::as_f64)
            .filter(|value| value.is_finite()),
        latency_p95_ms: positive_number(
            object
                .and_then(|object| object.get("latencyP95Ms"))
                .and_then(serde_json::Value::as_f64),
        ),
    }
}

fn positive_limit(value: Option<f64>) -> Option<usize> {
    value
        .filter(|value| value.is_finite())
        .map(|value| value.floor().max(1.0) as usize)
}

fn positive_number(value: Option<f64>) -> Option<f64> {
    value
        .filter(|value| value.is_finite())
        .map(|value| value.floor().max(1.0))
}

pub fn build_dashboard_insights(
    payload: Option<InsightPayload>,
    limits: InsightLimits,
) -> DashboardInsights {
    let source = if payload.is_some() {
        InsightSource::Payload
    } else {
        InsightSource::Sample
    };
    let mut payload = payload.unwrap_or_else(sample_payload);
    if payload.commits.is_empty() {
        payload.commits = sample_payload().commits;
    }
    if payload.stages.is_empty() {
        payload.stages = sample_payload().stages;
    }
    if payload.signals.is_empty() {
        payload.signals = sample_payload().signals;
    }
    let mut commit_risk_cards = payload.commits.iter().map(score_commit).collect::<Vec<_>>();
    commit_risk_cards.sort_by(|left, right| {
        right
            .score
            .cmp(&left.score)
            .then_with(|| left.id.cmp(&right.id))
    });
    let latency_ceiling = limits.latency_p95_ms.unwrap_or(1_000.0);
    let bottlenecks = payload
        .stages
        .iter()
        .map(|stage| stage_to_bottleneck(stage, latency_ceiling))
        .collect();
    let mut opportunities = payload
        .signals
        .iter()
        .map(signal_to_opportunity)
        .collect::<Vec<_>>();
    opportunities.sort_by(|left, right| {
        right
            .priority_score
            .cmp(&left.priority_score)
            .then_with(|| left.id.cmp(&right.id))
    });
    if let Some(limit) = limits.risks {
        commit_risk_cards.truncate(limit);
    }
    if let Some(limit) = limits.opportunities {
        opportunities.truncate(limit);
    }

    DashboardInsights {
        source,
        commit_risk_cards,
        bottlenecks,
        opportunities,
        stages: payload.stages,
    }
}

fn score_commit(commit: &InsightCommit) -> CommitRiskCard {
    let raw_score = commit.changed_lines / 8.0
        + commit.files * 2.0
        + commit.dependency_changes * 8.0
        + commit.failed_automations * 20.0
        + if commit.test_touch { 0.0 } else { 10.0 };
    let score = clamp_score(raw_score);
    let level = if score >= 80 {
        RiskLevel::High
    } else if score >= 50 {
        RiskLevel::Medium
    } else {
        RiskLevel::Good
    };
    let mut reasons = Vec::new();
    if commit.dependency_changes > 0.0 {
        reasons.push("Dependency risk".to_string());
    }
    if commit.failed_automations > 0.0 {
        reasons.push("Automation failures".to_string());
    }
    if commit.files >= 12.0 {
        reasons.push("Large diff surface".to_string());
    }
    if !commit.test_touch {
        reasons.push("Missing test coverage".to_string());
    }
    CommitRiskCard {
        id: commit.id.clone(),
        score,
        level,
        reasons,
    }
}

fn stage_to_bottleneck(stage: &InsightStage, latency_ceiling: f64) -> BottleneckCard {
    if stage.queue_depth >= 10.0 || stage.avg_latency_ms >= latency_ceiling * 3.0 {
        BottleneckCard {
            name: stage.name.clone(),
            status: BottleneckStatus::Critical,
            impact: stage.queue_depth + 5.0,
            rationale: format!(
                "Critical stage(s): {} is exceeding the tolerated queue window.",
                stage.name
            ),
        }
    } else if stage.queue_depth >= 4.0 || stage.avg_latency_ms >= latency_ceiling {
        BottleneckCard {
            name: stage.name.clone(),
            status: BottleneckStatus::High,
            impact: stage.queue_depth + 2.0,
            rationale: format!(
                "Critical stage(s): {} is approaching the queue pressure ceiling.",
                stage.name
            ),
        }
    } else {
        BottleneckCard {
            name: stage.name.clone(),
            status: BottleneckStatus::Good,
            impact: 1.0_f64.max(stage.queue_depth),
            rationale: format!("{} stays within the healthy execution window.", stage.name),
        }
    }
}

fn signal_to_opportunity(signal: &InsightSignal) -> OpportunityCard {
    OpportunityCard {
        id: signal.id.clone(),
        title: signal.title.clone(),
        priority_score: clamp_score(
            signal.impact * 16.0 + signal.confidence * 10.0 - signal.effort * 3.0,
        ),
    }
}

fn clamp_score(value: f64) -> u8 {
    value.round().clamp(0.0, 100.0) as u8
}

fn sample_payload() -> InsightPayload {
    InsightPayload {
        commits: vec![
            InsightCommit {
                id: "A-124".into(),
                files: 16.0,
                changed_lines: 280.0,
                dependency_changes: 1.0,
                test_touch: false,
                failed_automations: 1.0,
            },
            InsightCommit {
                id: "B-245".into(),
                files: 9.0,
                changed_lines: 110.0,
                dependency_changes: 0.0,
                test_touch: true,
                failed_automations: 0.0,
            },
            InsightCommit {
                id: "C-381".into(),
                files: 4.0,
                changed_lines: 28.0,
                dependency_changes: 0.0,
                test_touch: true,
                failed_automations: 0.0,
            },
        ],
        stages: vec![
            InsightStage {
                name: "review".into(),
                queue_depth: 10.0,
                throughput: 9.0,
                avg_latency_ms: 1_100.0,
            },
            InsightStage {
                name: "build".into(),
                queue_depth: 5.0,
                throughput: 12.0,
                avg_latency_ms: 850.0,
            },
            InsightStage {
                name: "release".into(),
                queue_depth: 4.0,
                throughput: 18.0,
                avg_latency_ms: 420.0,
            },
        ],
        signals: vec![
            InsightSignal {
                id: "op-1".into(),
                area: "tests".into(),
                title: "Trim flaky tests".into(),
                impact: 5.0,
                effort: 2.0,
                confidence: 0.9,
            },
            InsightSignal {
                id: "op-2".into(),
                area: "deps".into(),
                title: "Reduce dependency churn".into(),
                impact: 4.0,
                effort: 3.0,
                confidence: 0.8,
            },
            InsightSignal {
                id: "op-3".into(),
                area: "ui".into(),
                title: "Split dashboard rendering concerns".into(),
                impact: 3.0,
                effort: 2.0,
                confidence: 0.75,
            },
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::{
        build_dashboard_insights, build_dashboard_visuals, build_explainability_traces,
        build_quality_pulse, AppliedInsights, BottleneckStatus, CommitRiskCard, InsightLimits,
        InsightSource, OpportunityCard, PulseSeverity, RiskLevel, StakeholderAudience, VisualTone,
    };

    #[test]
    fn default_insights_match_the_companion_sample_seed() {
        let insights = build_dashboard_insights(None, InsightLimits::default());

        assert_eq!(insights.source, InsightSource::Sample);
        assert_eq!(insights.commit_risk_cards.len(), 3);
        assert_eq!(insights.commit_risk_cards[0].id, "A-124");
        assert_eq!(insights.commit_risk_cards[0].score, 100);
        assert_eq!(insights.commit_risk_cards[0].level, RiskLevel::High);
        assert_eq!(insights.bottlenecks.len(), 3);
        assert_eq!(insights.bottlenecks[0].status, BottleneckStatus::Critical);
        assert_eq!(insights.opportunities.len(), 3);
    }

    #[test]
    fn payload_envelopes_rank_then_limit_custom_insights() {
        let mut applied = AppliedInsights::default();
        applied.apply_json(
            r#"{
                "payload": {
                    "commits": [
                        {"id":"low-risk","files":1,"changedLines":4,"dependencyChanges":0,"testTouch":true,"failedAutomations":0},
                        {"id":"high-risk","files":24,"changedLines":900,"dependencyChanges":2,"testTouch":false,"failedAutomations":1}
                    ],
                    "stages": [{"name":"build","queueDepth":8,"throughput":8,"avgLatencyMs":1500}],
                    "signals": [
                        {"id":"low-op","area":"ops","title":"Low opportunity","impact":1,"effort":5,"confidence":0.2},
                        {"id":"high-op","area":"infra","title":"High opportunity","impact":5,"effort":1,"confidence":0.95}
                    ]
                },
                "limits": {"risks":1,"opportunities":1,"severityThreshold":4,"latencyP95Ms":700}
            }"#,
        );

        assert_eq!(applied.error, None);
        assert_eq!(applied.insights.source, InsightSource::Payload);
        assert_eq!(applied.insights.commit_risk_cards.len(), 1);
        assert_eq!(applied.insights.commit_risk_cards[0].id, "high-risk");
        assert_eq!(applied.insights.opportunities.len(), 1);
        assert_eq!(applied.insights.opportunities[0].id, "high-op");
        assert_eq!(applied.insights.bottlenecks.len(), 1);
        assert_eq!(
            applied.insights.bottlenecks[0].status,
            BottleneckStatus::High
        );
    }

    #[test]
    fn limit_parsing_ignores_bad_values_and_clamps_finite_values_to_one() {
        let mut applied = AppliedInsights::default();
        applied.apply_json(
            r#"{"risks":0,"opportunities":2.9,"severityThreshold":"bad","latencyP95Ms":null}"#,
        );

        assert_eq!(applied.error, None);
        assert_eq!(applied.insights.commit_risk_cards.len(), 1);
        assert_eq!(applied.insights.opportunities.len(), 2);
        assert_eq!(applied.insights.bottlenecks.len(), 3);
    }

    #[test]
    fn array_envelopes_follow_the_companion_object_fallback() {
        let mut applied = AppliedInsights::default();
        applied.apply_json(
            r#"{"payload":[],"limits":[],"commits":[{"id":"ignored","files":1,"changedLines":1,"dependencyChanges":0,"testTouch":true,"failedAutomations":0}],"risks":1}"#,
        );

        assert_eq!(applied.error, None);
        assert_eq!(applied.insights.commit_risk_cards[0].id, "A-124");
        assert_eq!(applied.insights.commit_risk_cards.len(), 3);
    }

    #[test]
    fn invalid_json_keeps_the_last_valid_insights_and_reset_restores_samples() {
        let mut applied = AppliedInsights::default();
        applied.apply_json(r#"{"payload":{"commits":[]}}"#);
        let last_valid = applied.insights.clone();
        applied.apply_json("{invalid");

        assert!(applied.error.is_some());
        assert_eq!(applied.insights, last_valid);
        applied.reset();
        assert_eq!(applied.error, None);
        assert_eq!(applied.insights.source, InsightSource::Sample);
    }

    #[test]
    fn quality_pulse_matches_default_audience_buckets_and_routes() {
        let pulse = build_quality_pulse(&build_dashboard_insights(None, InsightLimits::default()));

        assert_eq!(pulse.risk_buckets.high, 1);
        assert_eq!(pulse.bottleneck_buckets.critical, 1);
        assert_eq!(pulse.bottleneck_buckets.high, 2);
        assert!(pulse.security_signal_count >= 1);
        assert_eq!(pulse.top_bottleneck_name, "review");
        assert_eq!(pulse.recommendations[&StakeholderAudience::Lead].len(), 2);
        assert!(pulse.recommendations[&StakeholderAudience::Lead][0]
            .message
            .contains("Focus first on high-risk commit"));
        assert!(pulse.recommendations[&StakeholderAudience::Manager][0]
            .message
            .contains("Critical stage(s): review"));
        assert!(pulse.recommendations[&StakeholderAudience::Executive][0]
            .message
            .contains("Top opportunity:"));
        assert!(pulse.recommendations[&StakeholderAudience::Security][0]
            .message
            .contains("Security-sensitive signals from"));
        assert_eq!(
            pulse.action_routes[&StakeholderAudience::Lead].owner,
            "Lead Reviewer"
        );
        assert_eq!(
            pulse.action_routes[&StakeholderAudience::Manager].window,
            "This week"
        );
    }

    #[test]
    fn audience_highlights_match_the_companion_copy() {
        assert_eq!(
            StakeholderAudience::Lead.tone(),
            "Team leads: prioritize blocked PR hotspots and coaching cues."
        );
        assert_eq!(
            StakeholderAudience::Lead.guidance(),
            "Prioritize review-ready commits before opening the next work cycle."
        );
        assert_eq!(
            StakeholderAudience::Manager.tone(),
            "Managers: monitor cycle time, reviewer load, and handoff stability."
        );
        assert_eq!(
            StakeholderAudience::Executive.guidance(),
            "Use the opportunity list to stabilize throughput and defect risk over time."
        );
        assert_eq!(
            StakeholderAudience::Security.guidance(),
            "Surface release and dependency risks before they enter long-running branches."
        );
    }

    #[test]
    fn quality_pulse_ranks_custom_risks_and_opportunities_before_routing() {
        let mut insights = build_dashboard_insights(None, InsightLimits::default());
        insights.commit_risk_cards = vec![
            CommitRiskCard {
                id: "low-risk".into(),
                score: 12,
                level: RiskLevel::Good,
                reasons: vec![],
            },
            CommitRiskCard {
                id: "high-risk".into(),
                score: 92,
                level: RiskLevel::High,
                reasons: vec!["Automation failures".into()],
            },
        ];
        insights.bottlenecks.clear();
        insights.opportunities = vec![
            OpportunityCard {
                id: "low-opportunity".into(),
                title: "Low opportunity".into(),
                priority_score: 8,
            },
            OpportunityCard {
                id: "high-opportunity".into(),
                title: "High opportunity".into(),
                priority_score: 88,
            },
        ];
        let pulse = build_quality_pulse(&insights);

        assert_eq!(pulse.top_risk_commit_id, "high-risk");
        assert_eq!(pulse.top_opportunity_title, "High opportunity");
        assert!(pulse.recommendations[&StakeholderAudience::Lead][0]
            .message
            .contains("high-risk"));
        assert!(pulse.recommendations[&StakeholderAudience::Executive][0]
            .message
            .contains("High opportunity"));
    }

    #[test]
    fn quality_pulse_uses_fallback_copy_for_empty_signals() {
        let mut insights = build_dashboard_insights(None, InsightLimits::default());
        insights.commit_risk_cards.clear();
        insights.bottlenecks.clear();
        insights.opportunities.clear();
        let pulse = build_quality_pulse(&insights);

        assert_eq!(pulse.top_risk_commit_id, "");
        assert_eq!(pulse.top_opportunity_title, "trim flaky tests");
        assert_eq!(pulse.top_bottleneck_name, "review");
        assert_eq!(pulse.security_signal_count, 0);
        assert_eq!(
            pulse.recommendations[&StakeholderAudience::Lead][0].severity,
            PulseSeverity::Medium
        );
        assert!(pulse.recommendations[&StakeholderAudience::Executive][0]
            .message
            .contains("trim flaky tests"));
        assert_eq!(
            pulse.recommendations[&StakeholderAudience::Security][0].severity,
            PulseSeverity::Good
        );
    }

    #[test]
    fn explainability_traces_preserve_sample_score_decomposition() {
        let pulse = build_quality_pulse(&build_dashboard_insights(None, InsightLimits::default()));
        let traces = build_explainability_traces(&pulse);

        assert_eq!(traces.len(), 4);
        assert_eq!(traces[0].title, "Score Decomposition");
        assert_eq!(traces[0].summary, "Overall score 65/100");
        assert_eq!(
            traces[0].detail,
            "Risk 1 high / 0 medium, bottlenecks 1 critical / 2 high"
        );
        assert_eq!(traces[0].tone, VisualTone::Medium);
        assert_eq!(traces[1].summary, "A-124");
        assert_eq!(traces[1].tone, VisualTone::Bad);
        assert_eq!(traces[2].summary, "review");
        assert_eq!(traces[2].tone, VisualTone::Bad);
        assert_eq!(traces[3].summary, "Trim flaky tests");
        assert!(traces[3].detail.contains("3 opportunity signal(s)"));
    }

    #[test]
    fn visuals_rank_trend_lines_and_risks_and_handle_empty_data() {
        let mut insights = build_dashboard_insights(None, InsightLimits::default());
        let visuals = build_dashboard_visuals(&insights);
        assert_eq!(
            visuals.summary,
            "1 high-risk commits and 3 pressured stages"
        );
        assert_eq!(visuals.trend_lines.len(), 3);
        assert_eq!(visuals.trend_lines[0].value, "1 high-risk commits");
        assert!(visuals.trend_lines[0].rationale.contains("A-124"));
        assert_eq!(visuals.trend_lines[1].value, "3 pressured stages");
        assert_eq!(visuals.pr_risk_rankings[0].id, "A-124");
        assert_eq!(visuals.pr_risk_rankings[0].title, "A-124 score 100");
        assert_eq!(visuals.pr_risk_rankings[0].tone, VisualTone::Bad);

        insights.commit_risk_cards.clear();
        insights.bottlenecks.clear();
        insights.opportunities.clear();
        let empty_visuals = build_dashboard_visuals(&insights);
        assert_eq!(empty_visuals.summary, "No risk signals available");
        assert_eq!(empty_visuals.trend_lines[0].value, "No high-risk commits");
        assert_eq!(empty_visuals.trend_lines[0].tone, VisualTone::Good);
        assert_eq!(empty_visuals.trend_lines[1].value, "0 pressured stages");
        assert_eq!(
            empty_visuals.trend_lines[2].value,
            "0 actionable opportunities"
        );
    }
}
