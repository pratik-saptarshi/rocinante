use repo_analyzer_core::budget_guard::{BudgetDecision, BudgetGuard, StopReason};

#[test]
fn budget_guard_continues_below_report_only_threshold() {
    let guard = BudgetGuard;

    assert_eq!(guard.evaluate(79, false), BudgetDecision::Continue);
}

#[test]
fn budget_guard_enters_report_only_at_eighty_percent() {
    let guard = BudgetGuard;

    assert_eq!(guard.evaluate(80, false), BudgetDecision::ReportOnly);
    assert_eq!(guard.evaluate(99, false), BudgetDecision::ReportOnly);
}

#[test]
fn budget_guard_hard_stops_at_hundred_percent() {
    let guard = BudgetGuard;

    assert_eq!(
        guard.evaluate(100, false),
        BudgetDecision::Stop(StopReason::BudgetExceeded)
    );
    assert_eq!(
        guard.evaluate(130, false),
        BudgetDecision::Stop(StopReason::BudgetExceeded)
    );
}

#[test]
fn budget_guard_pause_flag_stops_immediately() {
    let guard = BudgetGuard;

    assert_eq!(
        guard.evaluate(0, true),
        BudgetDecision::Stop(StopReason::Paused)
    );
    assert_eq!(
        guard.evaluate(80, true),
        BudgetDecision::Stop(StopReason::Paused)
    );
}
