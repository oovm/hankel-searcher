use hs_checkpoint::{
    decode_rational_parameter, rational_parameter_ordinal, Checkpoint, FERGUSON_INDEX_GENERATOR,
    FERGUSON_PARAMETER_GENERATOR, NONNEGATIVE_INDEX_SPACE, Observation, RATIONAL_PARAMETER_SPACE,
};
use hs_searcher::{SearchBudget, SearchReport, SearchStrategy, enumerate_indices, local_indices, sample_indices};
use num_bigint::BigInt;
use num_rational::Ratio;
use num_traits::{Signed, Zero};
use std::time::Duration;

pub fn zeta_interval(order: u32, terms: usize) -> Result<(Ratio<BigInt>, Ratio<BigInt>), String> {
    if terms == 0 {
        return Err("series_terms must be positive".into());
    }
    let mut lower = Ratio::from_integer(BigInt::zero());
    for index in 1..=terms {
        let index = BigInt::from(index);
        lower += Ratio::new(BigInt::from(1), index.pow(order));
    }
    let m = BigInt::from(terms);
    let upper = &lower + Ratio::new(BigInt::from(1), BigInt::from(2) * m.pow(2));
    Ok((lower, upper))
}

fn ferguson_pair(order: u32, n: usize, count: usize) -> Result<(Ratio<BigInt>, Ratio<BigInt>), String> {
    match order {
        2 => {
            let pair = hs_problems::zeta2::ferguson_approximant(n, count).map_err(|e| e.to_string())?;
            Ok((pair.p, pair.q))
        }
        3 => {
            let pair = hs_problems::zeta3::ferguson_approximant(n, count).map_err(|e| e.to_string())?;
            Ok((pair.p, pair.q))
        }
        5 => {
            let pair = hs_problems::zeta5::ferguson_approximant(n, count).map_err(|e| e.to_string())?;
            Ok((pair.p, pair.q))
        }
        7 => {
            let pair = hs_problems::zeta7::ferguson_approximant(n, count).map_err(|e| e.to_string())?;
            Ok((pair.p, pair.q))
        }
        other => Err(format!("unsupported zeta order `{other}`")),
    }
}

fn ferguson_pair_shifted(
    order: u32,
    n: usize,
    shift: usize,
    count: usize,
) -> Result<(Ratio<BigInt>, Ratio<BigInt>), String> {
    let pair = hs_problems::zeta_ferguson_shifted(order, n, shift, count).map_err(|e| e.to_string())?;
    Ok((pair.p, pair.q))
}

fn is_parameter_contract(cp: &Checkpoint) -> bool {
    cp.search.generator_id == FERGUSON_PARAMETER_GENERATOR
        && cp.search.parameter_space_id == RATIONAL_PARAMETER_SPACE
}

pub fn observe(
    order: u32,
    n: usize,
    terms: usize,
    interval: &(Ratio<BigInt>, Ratio<BigInt>),
) -> Result<Observation, String> {
    observe_with_shift(order, n, None, terms, interval)
}

pub fn observe_with_shift(
    order: u32,
    n: usize,
    shift: Option<usize>,
    terms: usize,
    interval: &(Ratio<BigInt>, Ratio<BigInt>),
) -> Result<Observation, String> {
    let count = n.checked_add(2).and_then(|x| x.checked_mul(2)).ok_or("index overflow")?;
    let shift_value = shift.unwrap_or(0);
    let moment_count = if shift.is_some() {
        count.checked_add(shift_value).ok_or("moment count overflow")?
    } else {
        count
    };
    let (p, q) = if shift.is_some() {
        ferguson_pair_shifted(order, n, shift_value, moment_count)?
    } else {
        ferguson_pair(order, n, count)?
    };
    if q.is_zero() {
        return Err("zero Ferguson denominator".into());
    }
    let approx = p / q;
    let low_error = (&interval.0 - &approx).abs();
    let high_error = (&interval.1 - &approx).abs();
    let bound = low_error.max(high_error);
    Ok(Observation {
        kind: "finite_approximation_error_upper".into(),
        n,
        shift,
        series_terms: terms,
        approximant: hs_checkpoint::RationalData::from_ratio(&approx),
        error_upper: hs_checkpoint::RationalData::from_ratio(&bound),
    })
}

pub fn improve(
    cp: &mut Checkpoint,
    steps: usize,
    terms: usize,
    time_budget: Option<Duration>,
    strategy: SearchStrategy,
    jobs: usize,
) -> Result<SearchReport, String> {
    if is_parameter_contract(cp) {
        improve_parameters(cp, steps, terms, time_budget, strategy, jobs)
    } else {
        improve_index(cp, steps, terms, time_budget, strategy, jobs)
    }
}

fn improve_index(
    cp: &mut Checkpoint,
    steps: usize,
    terms: usize,
    time_budget: Option<Duration>,
    strategy: SearchStrategy,
    jobs: usize,
) -> Result<SearchReport, String> {
    let order = hs_checkpoint::zeta_order(&cp.target).ok_or_else(|| format!("unsupported target `{}`", cp.target))?;
    if steps == 0 {
        return Err("steps must be positive".into());
    }
    if jobs == 0 {
        return Err("jobs must be positive".into());
    }
    if let Some(saved_terms) = cp.search.series_terms {
        if saved_terms != terms {
            return Err(format!("series_terms must match checkpoint value {saved_terms}"));
        }
    }
    let start = cp.search.next_candidate.parse::<usize>().map_err(|e| e.to_string())?;
    let budget = match time_budget {
        Some(limit) => SearchBudget::from_duration(steps, limit),
        None => SearchBudget::new(steps),
    };
    let interval = zeta_interval(order, terms)?;
    let initial = cp.observed_best.clone();
    let report = match strategy {
        SearchStrategy::Enumerate => {
            enumerate_indices(start, initial, &budget, jobs, |n| observe(order, n, terms, &interval))?
        }
        SearchStrategy::Local => {
            let center = cp.observed_best.as_ref().map(|o| o.n).unwrap_or(start);
            local_indices(center, start, initial, &budget, jobs, |n| observe(order, n, terms, &interval))?
        }
        SearchStrategy::Sample => {
            sample_indices(&cp.search.seed, start, initial, &budget, jobs, |n| observe(order, n, terms, &interval))?
        }
    };
    cp.observed_best = report.best.clone();
    cp.search.generator_id = FERGUSON_INDEX_GENERATOR.into();
    cp.search.parameter_space_id = NONNEGATIVE_INDEX_SPACE.into();
    cp.search.next_candidate = report.end_index.to_string();
    cp.search.series_terms = Some(terms);
    cp.status = "draft".into();
    cp.updated_at = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
    Ok(report)
}

fn improve_parameters(
    cp: &mut Checkpoint,
    steps: usize,
    terms: usize,
    time_budget: Option<Duration>,
    strategy: SearchStrategy,
    jobs: usize,
) -> Result<SearchReport, String> {
    let order = hs_checkpoint::zeta_order(&cp.target).ok_or_else(|| format!("unsupported target `{}`", cp.target))?;
    if steps == 0 {
        return Err("steps must be positive".into());
    }
    if jobs == 0 {
        return Err("jobs must be positive".into());
    }
    if let Some(saved_terms) = cp.search.series_terms {
        if saved_terms != terms {
            return Err(format!("series_terms must match checkpoint value {saved_terms}"));
        }
    }
    let start = cp.search.next_candidate.parse::<usize>().map_err(|e| e.to_string())?;
    let budget = match time_budget {
        Some(limit) => SearchBudget::from_duration(steps, limit),
        None => SearchBudget::new(steps),
    };
    let interval = zeta_interval(order, terms)?;
    let initial = cp.observed_best.clone();
    let report = match strategy {
        SearchStrategy::Enumerate => enumerate_indices(start, initial, &budget, jobs, |ordinal| {
            let parameter = decode_rational_parameter(ordinal);
            observe_with_shift(order, parameter.index, Some(parameter.shift), terms, &interval)
        })?,
        SearchStrategy::Local => {
            let center = cp
                .observed_best
                .as_ref()
                .map(|o| rational_parameter_ordinal(o.n, o.shift.unwrap_or(0)))
                .transpose()?
                .unwrap_or(start);
            local_indices(center, start, initial, &budget, jobs, |ordinal| {
                let parameter = decode_rational_parameter(ordinal);
                observe_with_shift(order, parameter.index, Some(parameter.shift), terms, &interval)
            })?
        }
        SearchStrategy::Sample => sample_indices(&cp.search.seed, start, initial, &budget, jobs, |ordinal| {
            let parameter = decode_rational_parameter(ordinal);
            observe_with_shift(order, parameter.index, Some(parameter.shift), terms, &interval)
        })?,
    };
    cp.observed_best = report.best.clone();
    cp.search.generator_id = FERGUSON_PARAMETER_GENERATOR.into();
    cp.search.parameter_space_id = RATIONAL_PARAMETER_SPACE.into();
    cp.search.next_candidate = report.end_index.to_string();
    cp.search.series_terms = Some(terms);
    cp.status = "draft".into();
    cp.updated_at = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
    Ok(report)
}

pub fn check_observation(cp: &Checkpoint) -> Result<(), String> {
    if is_parameter_contract(cp) {
        check_parameter_observation(cp)
    } else {
        check_index_observation(cp)
    }
}

fn check_index_observation(cp: &Checkpoint) -> Result<(), String> {
    let order = hs_checkpoint::zeta_order(&cp.target).ok_or_else(|| format!("unsupported target `{}`", cp.target))?;
    let observation = cp.observed_best.as_ref().ok_or("checkpoint has no observation")?;
    if observation.kind != "finite_approximation_error_upper" {
        return Err("unsupported observation kind".into());
    }
    if observation.shift.is_some() {
        return Err("index search checkpoint must not store observation shift".into());
    }
    let terms = cp.search.series_terms.unwrap_or(observation.series_terms);
    if terms != observation.series_terms {
        return Err("observation and search precision mismatch".into());
    }
    let next = cp.search.next_candidate.parse::<usize>().map_err(|e| e.to_string())?;
    if next == 0 {
        return Err("observation exists without searched candidates".into());
    }
    let interval = zeta_interval(order, terms)?;
    let mut best: Option<Observation> = None;
    for n in 0..next {
        let candidate = observe(order, n, terms, &interval)?;
        if observation_is_better(&candidate, best.as_ref())? {
            best = Some(candidate);
        }
    }
    compare_saved_observation(observation, best.as_ref())
}

fn check_parameter_observation(cp: &Checkpoint) -> Result<(), String> {
    let order = hs_checkpoint::zeta_order(&cp.target).ok_or_else(|| format!("unsupported target `{}`", cp.target))?;
    let observation = cp.observed_best.as_ref().ok_or("checkpoint has no observation")?;
    if observation.kind != "finite_approximation_error_upper" {
        return Err("unsupported observation kind".into());
    }
    let shift = observation.shift.ok_or("parameter search observation requires shift")?;
    let terms = cp.search.series_terms.unwrap_or(observation.series_terms);
    if terms != observation.series_terms {
        return Err("observation and search precision mismatch".into());
    }
    let next = cp.search.next_candidate.parse::<usize>().map_err(|e| e.to_string())?;
    if next == 0 {
        return Err("observation exists without searched candidates".into());
    }
    let interval = zeta_interval(order, terms)?;
    let mut best: Option<Observation> = None;
    for ordinal in 0..next {
        let parameter = decode_rational_parameter(ordinal);
        let candidate = observe_with_shift(order, parameter.index, Some(parameter.shift), terms, &interval)?;
        if observation_is_better(&candidate, best.as_ref())? {
            best = Some(candidate);
        }
    }
    compare_saved_observation(observation, best.as_ref())?;
    if best.as_ref().expect("validated").shift != Some(shift) {
        return Err("saved observation shift does not match recomputation".into());
    }
    Ok(())
}

fn observation_is_better(candidate: &Observation, best: Option<&Observation>) -> Result<bool, String> {
    Ok(match best {
        Some(old) => candidate.error_upper.ratio()? < old.error_upper.ratio()?,
        None => true,
    })
}

fn compare_saved_observation(saved: &Observation, actual: Option<&Observation>) -> Result<(), String> {
    let actual = actual.ok_or("no candidates were checked")?;
    if actual.n != saved.n {
        return Err("saved best index is not minimal".into());
    }
    if actual.approximant.num != saved.approximant.num
        || actual.approximant.den != saved.approximant.den
        || actual.error_upper.num != saved.error_upper.num
        || actual.error_upper.den != saved.error_upper.den
    {
        return Err("saved observation does not match recomputation".into());
    }
    Ok(())
}

pub fn format_parameter_cursor(start: usize, end_exclusive: usize) -> String {
    if start >= end_exclusive {
        return format!("ordinal={start}..{end_exclusive}");
    }
    let first = decode_rational_parameter(start);
    let last = decode_rational_parameter(end_exclusive - 1);
    format!(
        "ordinal={start}..{end_exclusive} -> (n={},s={})..(n={},s={})",
        first.index, first.shift, last.index, last.shift
    )
}

pub fn uses_parameter_contract(cp: &Checkpoint) -> bool {
    is_parameter_contract(cp)
}
