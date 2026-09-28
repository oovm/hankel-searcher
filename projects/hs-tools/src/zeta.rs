use hs_checkpoint::{Checkpoint, Observation};
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

pub fn observe(
    order: u32,
    n: usize,
    terms: usize,
    interval: &(Ratio<BigInt>, Ratio<BigInt>),
) -> Result<Observation, String> {
    let count = n.checked_add(2).and_then(|x| x.checked_mul(2)).ok_or("index overflow")?;
    let (p, q) = ferguson_pair(order, n, count)?;
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
) -> Result<SearchReport, String> {
    let order = hs_checkpoint::zeta_order(&cp.target).ok_or_else(|| format!("unsupported target `{}`", cp.target))?;
    if steps == 0 {
        return Err("steps must be positive".into());
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
            enumerate_indices(start, initial, &budget, |n| observe(order, n, terms, &interval))?
        }
        SearchStrategy::Local => {
            let center = cp.observed_best.as_ref().map(|o| o.n).unwrap_or(start);
            local_indices(center, start, initial, &budget, |n| observe(order, n, terms, &interval))?
        }
        SearchStrategy::Sample => {
            sample_indices(&cp.search.seed, start, initial, &budget, |n| observe(order, n, terms, &interval))?
        }
    };
    cp.observed_best = report.best.clone();
    cp.search.generator_id = "ferguson-index-v1".into();
    cp.search.parameter_space_id = "nonnegative-index-v1".into();
    cp.search.next_candidate = report.end_index.to_string();
    cp.search.series_terms = Some(terms);
    cp.status = "draft".into();
    cp.updated_at = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
    Ok(report)
}

pub fn check_observation(cp: &Checkpoint) -> Result<(), String> {
    let order = hs_checkpoint::zeta_order(&cp.target).ok_or_else(|| format!("unsupported target `{}`", cp.target))?;
    let observation = cp.observed_best.as_ref().ok_or("checkpoint has no observation")?;
    if observation.kind != "finite_approximation_error_upper" {
        return Err("unsupported observation kind".into());
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
        let better = match &best {
            Some(old) => candidate.error_upper.ratio()? < old.error_upper.ratio()?,
            None => true,
        };
        if better {
            best = Some(candidate);
        }
    }
    let actual = best.ok_or("no candidates were checked")?;
    if actual.n != observation.n {
        return Err("saved best index is not minimal".into());
    }
    if actual.approximant.num != observation.approximant.num
        || actual.approximant.den != observation.approximant.den
        || actual.error_upper.num != observation.error_upper.num
        || actual.error_upper.den != observation.error_upper.den
    {
        return Err("saved observation does not match recomputation".into());
    }
    Ok(())
}
