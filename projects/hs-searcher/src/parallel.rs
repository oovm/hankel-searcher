use crate::report::{SearchReport, observation_better, observation_improved};
use hs_checkpoint::Observation;
use std::collections::BTreeMap;
use std::sync::Mutex;

/// Evaluate `order` with up to `jobs` worker threads, then merge in index order.
pub(crate) fn run_parallel_ordered<E>(
    start_index: usize,
    initial_best: Option<Observation>,
    order: &[usize],
    jobs: usize,
    evaluate: E,
) -> Result<SearchReport, String>
where
    E: Fn(usize) -> Result<Observation, String> + Send + Sync,
{
    if order.is_empty() {
        return Err("search order must not be empty".into());
    }
    if jobs == 0 {
        return Err("jobs must be positive".into());
    }
    let workers = jobs.min(order.len());
    let next_task = Mutex::new(0usize);
    let computed = Mutex::new(BTreeMap::<usize, Observation>::new());
    let failure = Mutex::new(None::<String>);

    std::thread::scope(|scope| {
        for _ in 0..workers {
            scope.spawn(|| {
                loop {
                    let task = {
                        let mut cursor = next_task.lock().expect("task cursor lock");
                        if *cursor >= order.len() {
                            break;
                        }
                        let task = *cursor;
                        *cursor += 1;
                        task
                    };
                    let index = order[task];
                    match evaluate(index) {
                        Ok(observation) => {
                            computed.lock().expect("computed lock").insert(index, observation);
                        }
                        Err(message) => {
                            let mut slot = failure.lock().expect("failure lock");
                            if slot.is_none() {
                                *slot = Some(message);
                            }
                            break;
                        }
                    }
                }
            });
        }
    });

    if let Some(message) = failure.lock().expect("failure lock").clone() {
        return Err(message);
    }
    let computed = computed.lock().expect("computed lock");
    let mut best = initial_best;
    let before_best = best.clone();
    let mut improvements = 0usize;
    for &index in order {
        let observation = computed
            .get(&index)
            .cloned()
            .ok_or_else(|| format!("parallel search missing observation for n={index}"))?;
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
