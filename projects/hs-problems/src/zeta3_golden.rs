//! Polynomial Hankel diagnostics for `ζ(3)` at construction index `n=1`.
//!
//! **CI / `cargo test`:** only [`zeta3_polynomial_golden_fast`].
//!
//! **Offline (not tests):** [`zeta3_polynomial_golden_full`] via
//! `cargo run --release -p hs-problems --features offline-golden --bin zeta3-hankel-golden`
//! or `hs check zeta-3 --polynomial-golden --full-delta`.

use crate::{
    polynomial_hankel_golden::{zeta3_golden_n1, ZetaPolynomialGoldenN1},
    zeta3_delta_polynomial, zeta3_entries, zeta3_log_s_k, zeta_polynomial_energy_report, ZetaPolynomialEnergyReport,
};
use hs_types::is_zero;

/// Expected outputs for `n=1`, `K=40`, `N=3`, `h=37`.
pub type Zeta3GoldenN1 = ZetaPolynomialGoldenN1;

/// Fast golden gate: paper scaling, `log S_K`, and nonzero leading-coefficient formula.
pub fn zeta3_polynomial_golden_fast() -> Result<(), String> {
    let entries = zeta3_entries(1)?;
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
    zeta3_golden_n1().assert_log_s_k(zeta3_log_s_k(params))?;
    let lead = crate::zeta3_delta_leading_coeff(&entries);
    if is_zero(&lead) {
        return Err("leading coefficient formula is zero".into());
    }
    Ok(())
}

/// Full offline golden: exact `Δ_K` and energy logs at `ζ(3)`.
pub fn zeta3_polynomial_golden_full() -> Result<ZetaPolynomialEnergyReport, String> {
    zeta3_polynomial_golden_fast()?;
    let entries = zeta3_entries(1)?;
    let delta = zeta3_delta_polynomial(&entries)?;
    let degree = entries.params.h;
    if delta.len() != degree + 1 {
        return Err(format!("unexpected Δ_K length {}", delta.len()));
    }
    if is_zero(&delta[degree]) {
        return Err("Δ_K leading coefficient is zero".into());
    }
    let energy = zeta_polynomial_energy_report(3, &entries.params, &delta)?;
    eprintln!(
        "energy: max_bits={} logS={:.3} logD={:.3} logF={:.3} logcontF={:.3} logP={:.3}",
        energy.max_primitive_coeff_bits,
        energy.log_s_k,
        energy.log_delta_at_zeta,
        energy.log_f_k,
        energy.log_content_f_k,
        energy.log_primitive_at_zeta,
    );
    zeta3_golden_n1().assert_energy(&energy)?;
    Ok(energy)
}
