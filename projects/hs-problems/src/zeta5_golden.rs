//! Golden metrics for `mo271/Zeta5` `scripts/hankel.py` at construction index `n=1`.
//!
//! Values captured from `python hankel.py 1` (flint arb), not stored as Python in this repo.
//!
//! **CI / default tests:** only [`zeta5_polynomial_golden_fast`].
//! **Offline:** [`zeta5_polynomial_golden_full`] and `zeta5-hankel-golden` bin (~tens of minutes).

use crate::{
    zeta5_delta_leading_coeff, zeta5_delta_polynomial, zeta5_energy_report, zeta5_entries, zeta5_log_s_k,
    Zeta5EnergyReport,
};
use num_traits::Zero;

/// Expected outputs for `n=1`, `K=40`, `N=3`, `h=37`.
#[derive(Debug, Clone, Copy)]
pub struct Zeta5GoldenN1 {
    /// `max |coeff P_K| bits` from primitive `Δ_K` (full golden only).
    pub max_primitive_coeff_bits: usize,
    /// `log S_K` paper normalization.
    pub log_s_k: f64,
    /// `log Δ_K(ζ(5))` (full golden only).
    pub log_delta_at_zeta5: f64,
    /// `log F_K(ζ(5))` (full golden only).
    pub log_f_k: f64,
    /// `log content(F_K)` (full golden only).
    pub log_content_f_k: f64,
    /// `log P_K(ζ(5))` for the primitive integer polynomial (full golden only).
    pub log_primitive_at_zeta5: f64,
}

impl Default for Zeta5GoldenN1 {
    fn default() -> Self {
        Self {
            max_primitive_coeff_bits: 7597,
            log_s_k: -204.319,
            log_delta_at_zeta5: -1836.876,
            log_f_k: -2041.195,
            log_content_f_k: -1776.066,
            log_primitive_at_zeta5: -265.129,
        }
    }
}

impl Zeta5GoldenN1 {
    const LOG_TOLERANCE: f64 = 0.05;

    /// Compare `log S_K` only (no `Δ_K` required).
    pub fn assert_log_s_k(&self, log_s_k: f64) -> Result<(), String> {
        Self::assert_log("log S_K", log_s_k, self.log_s_k)
    }

    /// Compare an energy report against embedded `hankel.py` golden values.
    pub fn assert_energy(&self, report: &Zeta5EnergyReport) -> Result<(), String> {
        if report.max_primitive_coeff_bits != self.max_primitive_coeff_bits {
            return Err(format!(
                "max |coeff P_K| bits: got {}, expected {}",
                report.max_primitive_coeff_bits, self.max_primitive_coeff_bits
            ));
        }
        self.assert_log_s_k(report.log_s_k)?;
        Self::assert_log("log Delta_K(zeta5)", report.log_delta_at_zeta5, self.log_delta_at_zeta5)?;
        Self::assert_log("log F_K(zeta5)", report.log_f_k, self.log_f_k)?;
        Self::assert_log("log content(F_K)", report.log_content_f_k, self.log_content_f_k)?;
        Self::assert_log("log P_K(zeta5)", report.log_primitive_at_zeta5, self.log_primitive_at_zeta5)?;
        Ok(())
    }

    fn assert_log(label: &str, actual: f64, expected: f64) -> Result<(), String> {
        if (actual - expected).abs() > Self::LOG_TOLERANCE {
            return Err(format!("{label}: got {actual:.6}, expected {expected:.6}"));
        }
        Ok(())
    }
}

/// Fast golden gate: paper scaling, moment lengths, `log S_K`, and (2.9) leading-coeff formula.
///
/// Runs in milliseconds — safe for CI and `cargo test`.
pub fn zeta5_polynomial_golden_fast() -> Result<(), String> {
    let n = 1usize;
    let entries = zeta5_entries(n);
    let params = &entries.params;
    if params.k != 40 || params.capital_n != 3 || params.h != 37 {
        return Err(format!(
            "paper scaling mismatch: K={} N={} h={}",
            params.k,
            params.capital_n,
            params.h
        ));
    }
    let e_len = 2 * params.h - 1;
    if entries.a.len() != e_len || entries.b.len() != e_len {
        return Err("moment sequence length mismatch".into());
    }
    Zeta5GoldenN1::default().assert_log_s_k(zeta5_log_s_k(params))?;
    let lead = zeta5_delta_leading_coeff(&entries);
    if lead.is_zero() {
        return Err("(2.9) leading coefficient formula is zero".into());
    }
    Ok(())
}

/// Full offline golden: exact `Δ_K`, energy logs, and primitive coefficient bit width.
///
/// **Do not** call from CI — exact `37×37` rational charpoly takes tens of minutes in release.
pub fn zeta5_polynomial_golden_full() -> Result<Zeta5EnergyReport, String> {
    zeta5_polynomial_golden_fast()?;
    let entries = zeta5_entries(1);
    let delta = zeta5_delta_polynomial(&entries);
    let lead = zeta5_delta_leading_coeff(&entries);
    let degree = entries.params.h;
    if delta.len() != degree + 1 {
        return Err(format!("unexpected Δ_K length {}", delta.len()));
    }
    if delta[degree] != lead {
        return Err("(2.9) leading coefficient mismatch against Δ_K".into());
    }
    let energy = zeta5_energy_report(&entries.params, &delta)?;
    Zeta5GoldenN1::default().assert_energy(&energy)?;
    Ok(energy)
}
