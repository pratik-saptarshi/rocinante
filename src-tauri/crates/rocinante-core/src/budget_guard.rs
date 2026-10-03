#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BudgetGuard;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BudgetDecision {
    Continue,
    ReportOnly,
    Stop(StopReason),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StopReason {
    BudgetExceeded,
    Paused,
}

impl BudgetGuard {
    pub const REPORT_ONLY_THRESHOLD: u8 = 80;
    pub const HARD_STOP_THRESHOLD: u8 = 100;

    pub fn evaluate(&self, budget_percent: u8, pause_requested: bool) -> BudgetDecision {
        if pause_requested {
            return BudgetDecision::Stop(StopReason::Paused);
        }

        if budget_percent >= Self::HARD_STOP_THRESHOLD {
            return BudgetDecision::Stop(StopReason::BudgetExceeded);
        }

        if budget_percent >= Self::REPORT_ONLY_THRESHOLD {
            return BudgetDecision::ReportOnly;
        }

        BudgetDecision::Continue
    }
}
