//! Polynomial Hankel diagnostics for `ζ(2)` at construction index `n=1`.
//!
//! **CI / `cargo test`:** only [`zeta2_polynomial_golden_fast`].
//!
//! **Offline (not tests):** [`zeta2_polynomial_golden_full`] via
//! `cargo run --release -p hs-problems --features offline-golden --bin zeta2-hankel-golden`
//! or `hs check zeta-2 --polynomial-golden --full-delta`.

use crate::{
    polynomial_hankel_golden::{zeta2_golden_n1, ZetaPolynomialGoldenN1},
    zeta2_delta_polynomial, zeta2_entries, zeta2_log_s_k, zeta_polynomial_energy_report, ZetaPolynomialEnergyReport,
};
use hs_types::is_zero;

/// Expected outputs for `n=1`, `K=40`, `N=2`, `q=4`, `h=38` (offline N-q sweep best).
pub type Zeta2GoldenN1 = ZetaPolynomialGoldenN1;

/// Fast golden gate: optimal scaling, `log S_K`, and nonzero leading-coefficient formula.
pub fn zeta2_polynomial_golden_fast() -> Result<(), String> {
    let entries = zeta2_entries(1)?;
    let params = &entries.params;
    if params.k != 40 || params.capital_n != 2 || params.q != 4 || params.h != 38 {
        return Err(format!(
            "optimal scaling mismatch: K={} N={} q={} h={}",
            params.k,
            params.capital_n,
            params.q,
            params.h
        ));
    }
    let e_len = 2 * params.h - 1;
    if entries.a.len() != e_len || entries.b.len() != e_len {
        return Err("moment sequence length mismatch".into());
    }
    zeta2_golden_n1().assert_log_s_k(zeta2_log_s_k(params))?;
    let lead = crate::zeta2_delta_leading_coeff(&entries);
    if is_zero(&lead) {
        return Err("leading coefficient formula is zero".into());
    }
    Ok(())
}

/// Full offline golden: exact `Δ_K` and energy logs at `ζ(2)`.
pub fn zeta2_polynomial_golden_full() -> Result<ZetaPolynomialEnergyReport, String> {
    zeta2_polynomial_golden_fast()?;
    let entries = zeta2_entries(1)?;
    let delta = zeta2_delta_polynomial(&entries)?;
    let degree = entries.params.h;
    if delta.len() != degree + 1 {
        return Err(format!("unexpected Δ_K length {}", delta.len()));
    }
    if is_zero(&delta[degree]) {
        return Err("Δ_K leading coefficient is zero".into());
    }
    let energy = zeta_polynomial_energy_report(2, &entries.params, &delta)?;
    eprintln!(
        "energy: max_bits={} logS={:.3} logD={:.3} logF={:.3} logcontF={:.3} logP={:.3}",
        energy.max_primitive_coeff_bits,
        energy.log_s_k,
        energy.log_delta_at_zeta,
        energy.log_f_k,
        energy.log_content_f_k,
        energy.log_primitive_at_zeta,
    );
    zeta2_golden_n1().assert_energy(&energy)?;
    Ok(energy)
}
