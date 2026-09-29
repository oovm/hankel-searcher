//! Offline grid search over `(N, q)` at fixed `K=k_per_n·n` for polynomial Hankel energy.

use crate::{
    polynomial_hankel_params, zeta_polynomial_delta, zeta_polynomial_entries_with_params, zeta_polynomial_energy_report,
    ZetaPolynomialEnergyReport,
};
use serde::Serialize;

/// One successful sweep row with full `Δ_K` energy at `ζ(s)`.
#[derive(Debug, Clone, Serialize)]
pub struct PolynomialHankelSweepRow {
    /// Zeta order `s`.
    pub order: u32,
    /// Construction index `n`.
    pub n: usize,
    /// `K = k_per_n * n`.
    pub k: usize,
    /// `N = capital_n_per_n * n`.
    pub capital_n: usize,
    /// Vanishing exponent `q` on `D_N^q`.
    pub q: usize,
    /// `h = K - N`.
    pub h: usize,
    /// `log P_K(ζ(s))` for the primitive polynomial.
    pub log_primitive_at_zeta: f64,
    /// `log P_K(ζ(s)) / n^2` (paper comparison scale).
    pub log_primitive_per_n2: f64,
    /// `log Δ_K(ζ(s))`.
    pub log_delta_at_zeta: f64,
    /// `log S_K`.
    pub log_s_k: f64,
    /// Maximum bit length of primitive coefficients.
    pub max_primitive_coeff_bits: usize,
}

/// Configuration for a polynomial Hankel parameter grid.
#[derive(Debug, Clone, Copy)]
pub struct PolynomialHankelSweepConfig {
    /// Construction index `n`.
    pub n: usize,
    /// `K = k_per_n * n`.
    pub k_per_n: usize,
    /// Inclusive range `capital_n_per_n ∈ [capital_n_min, capital_n_max]`.
    pub capital_n_min: usize,
    pub capital_n_max: usize,
    /// Vanishing exponents `q` to try.
    pub q_values: &'static [usize],
    /// Zeta orders `s` to evaluate.
    pub orders: &'static [u32],
}

impl Default for PolynomialHankelSweepConfig {
    fn default() -> Self {
        Self {
            n: 1,
            k_per_n: 40,
            capital_n_min: 1,
            capital_n_max: 8,
            q_values: &[4, 6, 8],
            orders: &[2, 3, 5],
        }
    }
}

/// Run the full `Δ_K` grid and return successful rows (skips invalid scaling).
pub fn sweep_polynomial_hankel(config: &PolynomialHankelSweepConfig) -> Vec<PolynomialHankelSweepRow> {
    let mut rows = Vec::new();
    for &order in config.orders {
        for capital_n_per_n in config.capital_n_min..=config.capital_n_max {
            for &q in config.q_values {
                if let Some(row) = sweep_single(config, order, capital_n_per_n, q) {
                    rows.push(row);
                }
            }
        }
    }
    rows
}

fn sweep_single(
    config: &PolynomialHankelSweepConfig,
    order: u32,
    capital_n_per_n: usize,
    q: usize,
) -> Option<PolynomialHankelSweepRow> {
    let params = polynomial_hankel_params(config.n, config.k_per_n, capital_n_per_n, q).ok()?;
    let entries = zeta_polynomial_entries_with_params(order, params).ok()?;
    let delta = zeta_polynomial_delta(&entries).ok()?;
    let energy = zeta_polynomial_energy_report(order, &params, &delta).ok()?;
    Some(row_from_energy(order, &params, &energy))
}

fn row_from_energy(order: u32, params: &crate::Zeta5PaperParams, energy: &ZetaPolynomialEnergyReport) -> PolynomialHankelSweepRow {
    let n2 = (params.n * params.n) as f64;
    PolynomialHankelSweepRow {
        order,
        n: params.n,
        k: params.k,
        capital_n: params.capital_n,
        q: params.q,
        h: params.h,
        log_primitive_at_zeta: energy.log_primitive_at_zeta,
        log_delta_at_zeta: energy.log_delta_at_zeta,
        log_s_k: energy.log_s_k,
        log_primitive_per_n2: energy.log_primitive_at_zeta / n2,
        max_primitive_coeff_bits: energy.max_primitive_coeff_bits,
    }
}

/// Best row per `ζ(s)` order by smallest `log P_K(ζ(s)) / n^2` (most negative wins).
pub fn best_sweep_rows_by_order(rows: &[PolynomialHankelSweepRow]) -> Vec<PolynomialHankelSweepRow> {
    let mut best: Vec<PolynomialHankelSweepRow> = Vec::new();
    for row in rows {
        if let Some(existing) = best.iter().find(|candidate| candidate.order == row.order) {
            if row.log_primitive_per_n2 < existing.log_primitive_per_n2 {
                let order = row.order;
                best.retain(|candidate| candidate.order != order);
                best.push(row.clone());
            }
        } else {
            best.push(row.clone());
        }
    }
    best.sort_by_key(|row| row.order);
    best
}
