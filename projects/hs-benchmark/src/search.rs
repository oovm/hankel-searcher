use crate::{
    limits::ZETA5_DEFAULT_MAX_N,
    metrics::{convergence_gap, ConvergenceReport},
};
use hs_moments::{zeta_moments, zeta_moments_f64, ZETA_5};
use hs_types::HankelError;

/// Primary irrationality-search target in the main Hankel pipeline.
pub const PRIMARY_TARGET: &str = "zeta-5";

/// Convergence ladder for `ζ(5)` Ferguson approximants up to `max_n`.
pub fn zeta5_convergence_ladder(max_n: usize) -> Result<Vec<ConvergenceReport>, HankelError> {
    let need = 2 * (max_n + 2);
    let moments = zeta_moments(5, need);
    let float_moments = zeta_moments_f64(5, need);
    let mut reports = Vec::with_capacity(max_n + 1);
    for n in 0..=max_n {
        reports.push(convergence_gap(&moments, ZETA_5, n, &float_moments)?);
    }
    Ok(reports)
}

/// Default-bounded ladder for CLI and CI (`ZETA5_DEFAULT_MAX_N`).
pub fn zeta5_convergence_ladder_default() -> Result<Vec<ConvergenceReport>, HankelError> {
    zeta5_convergence_ladder(ZETA5_DEFAULT_MAX_N)
}

/// Best gap in the ladder (smallest positive gap among `0..=max_n`).
pub fn zeta5_best_gap(max_n: usize) -> Result<Option<ConvergenceReport>, HankelError> {
    let ladder = zeta5_convergence_ladder(max_n)?;
    Ok(ladder
        .into_iter()
        .filter(|row| row.gap > 0.0)
        .min_by(|a, b| a.gap.partial_cmp(&b.gap).unwrap_or(std::cmp::Ordering::Equal)))
}
