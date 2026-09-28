use hs_checkpoint::{Checkpoint, Observation};
use num_bigint::BigInt;
use num_rational::Ratio;
use num_traits::{Signed, Zero};

pub fn zeta3_interval(terms: usize) -> Result<(Ratio<BigInt>, Ratio<BigInt>), String> {
    if terms == 0 {
        return Err("series_terms must be positive".into());
    }
    let mut lower = Ratio::from_integer(BigInt::zero());
    for k in 1..=terms {
        let k = BigInt::from(k);
        lower += Ratio::new(BigInt::from(1), k.pow(3));
    }
    let m = BigInt::from(terms);
    let upper = &lower + Ratio::new(BigInt::from(1), BigInt::from(2) * m.pow(2));
    Ok((lower, upper))
}

pub fn observe(n: usize, terms: usize, interval: &(Ratio<BigInt>, Ratio<BigInt>)) -> Result<Observation, String> {
    let count = n.checked_add(2).and_then(|x| x.checked_mul(2)).ok_or("index overflow")?;
    let pair = zeta_3::ferguson_approximant(n, count).map_err(|e| e.to_string())?;
    if pair.q.is_zero() {
        return Err("zero Ferguson denominator".into());
    }
    let approx = pair.p / pair.q;
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

pub fn improve(cp: &mut Checkpoint, steps: usize, terms: usize) -> Result<usize, String> {
    if steps == 0 {
        return Err("steps must be positive".into());
    }
    if let Some(saved_terms) = cp.search.series_terms {
        if saved_terms != terms {
            return Err(format!("series_terms must match checkpoint value {saved_terms}"));
        }
    }
    let next = cp.search.next_candidate.parse::<usize>().map_err(|e| e.to_string())?;
    let end = next.checked_add(steps).ok_or("candidate index overflow")?;
    let interval = zeta3_interval(terms)?;
    let mut improvements = 0;
    for n in next..end {
        let observation = observe(n, terms, &interval)?;
        let is_better = match &cp.observed_best {
            Some(old) => observation.error_upper.ratio()? < old.error_upper.ratio()?,
            None => true,
        };
        if is_better {
            cp.observed_best = Some(observation);
            improvements += 1;
        }
    }
    cp.search.generator_id = "ferguson-index-v1".into();
    cp.search.parameter_space_id = "nonnegative-index-v1".into();
    cp.search.next_candidate = end.to_string();
    cp.search.series_terms = Some(terms);
    cp.status = "draft".into();
    cp.updated_at = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
    Ok(improvements)
}

pub fn check_observation(cp: &Checkpoint) -> Result<(), String> {
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
    let interval = zeta3_interval(terms)?;
    let mut best: Option<Observation> = None;
    for n in 0..next {
        let candidate = observe(n, terms, &interval)?;
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
