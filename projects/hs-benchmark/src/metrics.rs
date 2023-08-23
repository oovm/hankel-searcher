use hs_types::{estimate_energy_rate, ferguson_pair_at, RationalMomentSequence};

/// Convergence diagnostics for one constant.
#[derive(Debug, Clone)]
pub struct ConvergenceReport {
    /// Approximant index.
    pub n: usize,
    /// `P_n / Q_n`.
    pub approximant: f64,
    /// `target - approximant`.
    pub gap: f64,
    /// Estimated `-log|H_n| / n^2` when available.
    pub energy_rate: Option<f64>,
}

/// Build a convergence report for one index.
pub fn convergence_gap(
    moments: &RationalMomentSequence,
    target: f64,
    n: usize,
    float_moments: &[f64],
) -> Result<ConvergenceReport, hs_types::HankelError> {
    use hs_types::approximant_to_f64;
    let pair = ferguson_pair_at(moments, n)?;
    let approximant = approximant_to_f64(&pair)?;
    let energy_rate = hs_types::log_abs_hankel_det(float_moments, 0, n + 1);
    let energy = if energy_rate.is_finite() {
        Some(-energy_rate / (n as f64).powi(2))
    } else {
        None
    };
    Ok(ConvergenceReport {
        n,
        approximant,
        gap: target - approximant,
        energy_rate: energy,
    })
}

/// Energy-rate curve for exploratory searches.
pub fn energy_rate_curve(float_moments: &[f64], max_n: usize) -> Vec<(usize, f64)> {
    estimate_energy_rate(float_moments, max_n)
}
