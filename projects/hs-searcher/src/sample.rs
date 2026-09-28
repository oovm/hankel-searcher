use crate::budget::SearchBudget;
use crate::report::{SearchReport, run_ordered};
use crate::rng::shuffle_indices;
use hs_checkpoint::Observation;

/// Evaluate a deterministic pseudo-random permutation of the forward window.
pub fn sample_indices<E>(
    seed: &str,
    start_index: usize,
    initial_best: Option<Observation>,
    budget: &SearchBudget,
    evaluate: E,
) -> Result<SearchReport, String>
where
    E: FnMut(usize) -> Result<Observation, String>,
{
    if budget.max_steps == 0 {
        return Err("max_steps must be positive".into());
    }
    let indices: Vec<usize> = (start_index..start_index + budget.max_steps).collect();
    let order = shuffle_indices(seed, indices);
    run_ordered(start_index, initial_best, &order, evaluate)
}
