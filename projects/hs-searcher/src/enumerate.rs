use crate::budget::SearchBudget;
use crate::parallel::run_parallel_ordered;
use crate::report::{SearchReport, run_ordered};
use hs_checkpoint::Observation;

/// Evaluate consecutive indices `[start_index, ..)` until the budget is exhausted.
pub fn enumerate_indices<E>(
    start_index: usize,
    initial_best: Option<Observation>,
    budget: &SearchBudget,
    jobs: usize,
    evaluate: E,
) -> Result<SearchReport, String>
where
    E: Fn(usize) -> Result<Observation, String> + Send + Sync,
{
    if budget.max_steps == 0 {
        return Err("max_steps must be positive".into());
    }
    if jobs == 0 {
        return Err("jobs must be positive".into());
    }
    let mut order = Vec::with_capacity(budget.max_steps);
    let mut index = start_index;
    let mut completed = 0usize;
    while budget.allows_more(completed) {
        order.push(index);
        index += 1;
        completed += 1;
    }
    if jobs == 1 {
        return run_ordered(start_index, initial_best, &order, evaluate);
    }
    run_parallel_ordered(start_index, initial_best, &order, jobs, evaluate)
}

/// Backward-compatible alias for the first search strategy.
pub type EnumerateReport = SearchReport;
