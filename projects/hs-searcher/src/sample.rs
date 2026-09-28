use crate::budget::SearchBudget;
use crate::parallel::run_parallel_ordered;
use crate::report::{SearchReport, run_ordered};
use crate::rng::shuffle_indices;
use hs_checkpoint::Observation;

/// Evaluate a deterministic pseudo-random permutation of the forward window.
pub fn sample_indices<E>(
    seed: &str,
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
    let indices: Vec<usize> = (start_index..start_index + budget.max_steps).collect();
    let order = shuffle_indices(seed, indices);
    if jobs == 1 {
        return run_ordered(start_index, initial_best, &order, evaluate);
    }
    run_parallel_ordered(start_index, initial_best, &order, jobs, evaluate)
}