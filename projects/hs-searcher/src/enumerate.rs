use crate::budget::SearchBudget;
use hs_checkpoint::Observation;

/// Result of one enumerate pass over consecutive indices.
#[derive(Debug, Clone)]
pub struct EnumerateReport {
    pub start_index: usize,
    pub end_index: usize,
    pub completed_steps: usize,
    pub improvements: usize,
    pub bound_improved: bool,
    pub best: Option<Observation>,
}

/// Evaluate consecutive indices `[start_index, ..)` until the budget is exhausted.
///
/// The cursor advances to `end_index` only when every index in the attempted batch
/// was evaluated successfully. A partial batch due to budget exhaustion still
/// advances over the indices that were evaluated in that final batch.
pub fn enumerate_indices<E>(
    start_index: usize,
    initial_best: Option<Observation>,
    budget: &SearchBudget,
    mut evaluate: E,
) -> Result<EnumerateReport, String>
where
    E: FnMut(usize) -> Result<Observation, String>,
{
    if budget.max_steps == 0 {
        return Err("max_steps must be positive".into());
    }
    let mut best = initial_best;
    let before_best = best.clone();
    let mut improvements = 0;
    let mut index = start_index;
    let mut completed = 0usize;
    while budget.allows_more(completed) {
        let observation = evaluate(index)?;
        let is_better = match &best {
            Some(old) => observation_better(&observation, old)?,
            None => true,
        };
        if is_better {
            best = Some(observation);
            improvements += 1;
        }
        index += 1;
        completed += 1;
    }
    let bound_improved = observation_improved(&before_best, &best)?;
    Ok(EnumerateReport {
        start_index,
        end_index: index,
        completed_steps: completed,
        improvements,
        bound_improved,
        best,
    })
}

fn observation_better(new: &Observation, old: &Observation) -> Result<bool, String> {
    Ok(new.error_upper.ratio()? < old.error_upper.ratio()?)
}

fn observation_improved(before: &Option<Observation>, after: &Option<Observation>) -> Result<bool, String> {
    match (before, after) {
        (None, Some(_)) => Ok(true),
        (Some(old), Some(new)) => Ok(new.error_upper.ratio()? < old.error_upper.ratio()?),
        _ => Ok(false),
    }
}
