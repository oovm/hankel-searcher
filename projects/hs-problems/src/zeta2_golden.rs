//! Polynomial Hankel diagnostics for `ζ(2)` at construction index `n=1`.

use crate::{
    zeta2_delta_polynomial, zeta2_entries, zeta2_log_s_k, zeta_polynomial_energy_report, ZetaPolynomialEnergyReport,
};
use hs_types::is_zero;

/// Fast golden gate: paper scaling, `log S_K`, and nonzero leading-coefficient formula.
pub fn zeta2_polynomial_golden_fast() -> Result<(), String> {
    let entries = zeta2_entries(1)?;
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
    let log_s = zeta2_log_s_k(params);
    if (log_s + 204.319).abs() > 0.05 {
        return Err(format!("log S_K: got {log_s:.6}, expected about -204.319"));
    }
    let lead = crate::zeta2_delta_leading_coeff(&entries);
    if is_zero(&lead) {
        return Err("leading coefficient formula is zero".into());
    }
    Ok(())
}

/// Full offline probe: exact `Δ_K`, energy logs at `ζ(2)` (no embedded golden constants yet).
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
        "zeta2 energy n=1: max_bits={} logS={:.3} logD={:.3} logF={:.3} logcontF={:.3} logP={:.3}",
        energy.max_primitive_coeff_bits,
        energy.log_s_k,
        energy.log_delta_at_zeta,
        energy.log_f_k,
        energy.log_content_f_k,
        energy.log_primitive_at_zeta,
    );
    Ok(energy)
}
