use hs_checkpoint::Observation;

/// Result of one bounded search pass.
#[derive(Debug, Clone)]
pub struct SearchReport {
    pub start_index: usize,
    pub end_index: usize,
    pub completed_steps: usize,
    pub improvements: usize,
    pub bound_improved: bool,
    pub best: Option<Observation>,
}

pub(crate) fn run_ordered<E>(
    start_index: usize,
    initial_best: Option<Observation>,
    order: &[usize],
    mut evaluate: E,
) -> Result<SearchReport, String>
where
    E: FnMut(usize) -> Result<Observation, String>,
{
    if order.is_empty() {
        return Err("search order must not be empty".into());
    }
    let mut best = initial_best;
    let before_best = best.clone();
    let mut improvements = 0usize;
    for &index in order {
        let observation = evaluate(index)?;
        let is_better = match &best {
            Some(old) => observation_better(&observation, old)?,
            None => true,
        };
        if is_better {
            best = Some(observation);
            improvements += 1;
        }
    }
    let bound_improved = observation_improved(&before_best, &best)?;
    let end_index = order.iter().copied().max().map(|n| n + 1).unwrap_or(start_index);
    Ok(SearchReport {
        start_index,
        end_index,
        completed_steps: order.len(),
        improvements,
        bound_improved,
        best,
    })
}

pub(crate) fn observation_better(new: &Observation, old: &Observation) -> Result<bool, String> {
    Ok(new.error_upper.ratio()? < old.error_upper.ratio()?)
}

pub(crate) fn observation_improved(before: &Option<Observation>, after: &Option<Observation>) -> Result<bool, String> {
    match (before, after) {
        (None, Some(_)) => Ok(true),
        (Some(old), Some(new)) => Ok(new.error_upper.ratio()? < old.error_upper.ratio()?),
        _ => Ok(false),
    }
}
