//! Energy diagnostics for the Zeta5 polynomial Hankel line, aligned with `hankel.py#run`.

use crate::progress::trace_step;
use crate::zeta5_polynomial::Zeta5PaperParams;
use hs_types::{Integer, Natural, Rational, is_zero};
use malachite::Float;
use malachite::base::num::arithmetic::traits::{Abs, Gcd, Lcm, Ln, Pow};
use malachite::base::num::basic::traits::{One, Zero};
use malachite::base::num::conversion::traits::RoundingFrom;
use malachite::base::num::logic::traits::SignificantBits;
use malachite::base::rounding_modes::RoundingMode::Nearest;

/// Primitive integer coefficients of `Δ_K` after clearing content.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Zeta5DeltaPrimitive {
    /// `gcd` of all coefficient numerators in `Δ_K`.
    pub content_numerator: Natural,
    /// `lcm` of all coefficient denominators in `Δ_K`.
    pub content_denominator: Natural,
    /// Integer coefficients of the primitive polynomial `P_K`.
    pub coefficients: Vec<Integer>,
}

/// Paper energy logs at `ζ(5)`, matching `hankel.py` output fields.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Zeta5EnergyReport {
    /// `log S_K` normalization factor from the paper.
    pub log_s_k: f64,
    /// `log Δ_K(ζ(5))`.
    pub log_delta_at_zeta5: f64,
    /// `log F_K(ζ(5)) = log Δ_K + log S_K`.
    pub log_f_k: f64,
    /// `log content(F_K)`.
    pub log_content_f_k: f64,
    /// `log P_K(ζ(5))` for the primitive integer polynomial.
    pub log_primitive_at_zeta5: f64,
    /// Maximum bit length of primitive coefficients.
    pub max_primitive_coeff_bits: usize,
}

/// Bit precision for `Float` evaluation, matching `hankel.py` `ctx.prec = maxbits + K^2 + 4000`.
pub fn energy_eval_precision_bits(max_coeff_bits: usize, k: usize) -> u64 {
    (max_coeff_bits + k * k + 4000) as u64
}

/// `log S_K` from the paper normalization (uses `libm::lgamma`, no `Δ_K` needed).
pub fn zeta5_log_s_k(params: &Zeta5PaperParams) -> f64 {
    let h = params.h as f64;
    let logfact = |m: usize| libm::lgamma(m as f64 + 1.0);
    let mut tail = 0.0;
    for index in 1..params.h {
        tail += logfact(2 * index);
    }
    2.0 * h * logfact(params.k)
        + (h - 1.0) * 4.0f64.ln()
        - 12.0 * h * logfact(params.capital_n)
        - 2.0 * tail
}

/// Clear `gcd(numerators)/lcm(denominators)` from ascending `Δ_K` coefficients.
pub fn zeta5_delta_primitive(delta: &[Rational]) -> Zeta5DeltaPrimitive {
    let mut content_numerator = Natural::ZERO;
    let mut content_denominator = Natural::ONE;
    for coeff in delta {
        if is_zero(coeff) {
            continue;
        }
        content_numerator = Natural::gcd(content_numerator, coeff.to_numerator().clone());
        content_denominator = Natural::lcm(content_denominator, coeff.to_denominator());
    }
    let coefficients = delta
        .iter()
        .map(|coeff| {
            let scaled = coeff.to_numerator().clone() * (&content_denominator / coeff.to_denominator());
            Integer::from(scaled / content_numerator.clone())
        })
        .collect();
    Zeta5DeltaPrimitive {
        content_numerator,
        content_denominator,
        coefficients,
    }
}

/// Evaluate energy logs for an already computed `Δ_K`.
pub fn zeta5_energy_report(params: &Zeta5PaperParams, delta: &[Rational]) -> Result<Zeta5EnergyReport, String> {
    tracing::info!(phase = "energy_report", "start");
    let primitive = zeta5_delta_primitive(delta);
    let max_bits = max_coeff_bits(&primitive.coefficients);
    let prec_bits = energy_eval_precision_bits(max_bits, params.k);
    let eval_prec = prec_bits + 512;
    tracing::info!(
        phase = "energy_report",
        max_coeff_bits = max_bits,
        prec_bits,
        eval_prec,
        "primitive content cleared"
    );
    tracing::info!(phase = "zeta5", order = 5, prec_bits, "computing zeta(5)");
    let zeta5 = zeta_at_precision(5, prec_bits)?;
    tracing::info!(phase = "energy_report", "zeta(5) ready, evaluating Delta_K");
    let log_s_k = zeta5_log_s_k(params);
    let delta_value = evaluate_rational_poly_float(delta, &zeta5, eval_prec)?;
    tracing::info!(phase = "energy_report", "Delta_K evaluated, evaluating P_K");
    let primitive_value = evaluate_integer_poly_float(&primitive.coefficients, &zeta5, eval_prec)?;
    let log_delta_at_zeta5 = log_positive_float("Delta_K", &delta_value)?;
    let log_primitive_at_zeta5 = log_positive_float("P_K", &primitive_value)?;
    let log_content = log_natural_ratio(&primitive.content_numerator, &primitive.content_denominator)?;
    Ok(Zeta5EnergyReport {
        log_s_k,
        log_delta_at_zeta5,
        log_f_k: log_delta_at_zeta5 + log_s_k,
        log_content_f_k: log_content + log_s_k,
        log_primitive_at_zeta5,
        max_primitive_coeff_bits: max_bits,
    })
}

fn max_coeff_bits(coefficients: &[Integer]) -> usize {
    coefficients
        .iter()
        .map(|coeff| coeff.abs().significant_bits() as usize)
        .max()
        .unwrap_or(0)
}

fn log_natural_ratio(numerator: &Natural, denominator: &Natural) -> Result<f64, String> {
    let log_num = log_positive_natural(numerator)?;
    let log_den = log_positive_natural(denominator)?;
    Ok(log_num - log_den)
}

fn log_positive_natural(value: &Natural) -> Result<f64, String> {
    if *value == Natural::ZERO {
        return Err("log of zero".into());
    }
    let bits = value.significant_bits() as u32;
    if bits <= 53 {
        let (converted, _) = f64::rounding_from(value, Nearest);
        return Ok(converted.ln());
    }
    let shift = bits - 53;
    let (top, _) = f64::rounding_from(&(value >> shift), Nearest);
    Ok((bits as f64 - 1.0) * std::f64::consts::LN_2 + top.ln())
}

fn log_positive_float(label: &str, value: &Float) -> Result<f64, String> {
    if *value <= Float::ZERO {
        return Err(format!("{label}: log of non-positive Float ({value})"));
    }
    let ln = value.ln();
    let (converted, _) = f64::rounding_from(&ln, Nearest);
    Ok(converted)
}

fn evaluate_rational_poly_float(coefficients: &[Rational], point: &Float, eval_prec: u64) -> Result<Float, String> {
    let prec = eval_prec + 32;
    if coefficients.is_empty() {
        return Ok(float_at_unsigned(prec, 0));
    }
    let mut acc = Float::from_rational_prec(coefficients[coefficients.len() - 1].clone(), prec).0;
    for coeff in coefficients[..coefficients.len() - 1].iter().rev() {
        acc = acc * point + Float::from_rational_prec(coeff.clone(), prec).0;
    }
    Ok(acc)
}

fn evaluate_integer_poly_float(coefficients: &[Integer], point: &Float, eval_prec: u64) -> Result<Float, String> {
    let prec = eval_prec + 32;
    if coefficients.is_empty() {
        return Ok(float_at_unsigned(prec, 0));
    }
    let mut acc = Float::from_integer_prec(coefficients[coefficients.len() - 1].clone(), prec).0;
    for coeff in coefficients[..coefficients.len() - 1].iter().rev() {
        acc = acc * point + Float::from_integer_prec(coeff.clone(), prec).0;
    }
    Ok(acc)
}

fn float_at_unsigned(prec: u64, value: u64) -> Float {
    Float::from_unsigned_prec(value, prec).0
}

const ZETA_DIRECT_SUM_TERM_CAP: usize = 200_000;

/// `ζ(s)` for integer `s >= 2` at `prec_bits` working precision.
pub fn zeta_integer_float(order: u32, prec_bits: u64) -> Result<Float, String> {
    zeta_at_precision(order, prec_bits)
}

fn zeta_at_precision(order: u32, prec_bits: u64) -> Result<Float, String> {
    let terms = zeta_series_terms_for_precision(order, prec_bits);
    if terms <= ZETA_DIRECT_SUM_TERM_CAP {
        tracing::info!(
            phase = "zeta5",
            route = "direct_series",
            order,
            prec_bits,
            terms,
        );
        zeta_series_float(order, terms, prec_bits)
    } else {
        tracing::info!(
            phase = "zeta5",
            route = "euler_maclaurin",
            order,
            prec_bits,
            requested_terms = terms,
        );
        zeta_euler_maclaurin_float(order, prec_bits)
    }
}

/// Partial sum plus half the integral tail bound (rigorous midpoint of `zeta_series_bounds`).
pub(crate) fn zeta_series_float(order: u32, terms: usize, prec_bits: u64) -> Result<Float, String> {
    if order < 2 {
        return Err("zeta order must be >= 2".into());
    }
    if terms == 0 {
        return Err("series terms must be positive".into());
    }
    let prec = prec_bits + 64;
    tracing::info!(phase = "zeta5_direct", order, terms, prec, "partial sum start");
    let mut sum = float_at_unsigned(prec, 0);
    let progress_step = (terms / 20).max(1_000);
    for index in 1..=terms {
        sum += float_at_unsigned(prec, index as u64).pow(-(order as i64));
        trace_step("zeta5_direct", index, terms, progress_step);
    }
    let tail_denominator = float_at_unsigned(prec, u64::from(order - 1))
        * float_at_unsigned(prec, terms as u64).pow((order as i64) - 1);
    let tail = float_at_unsigned(prec, 1) / tail_denominator;
    Ok(sum + tail / float_at_unsigned(prec, 2))
}

fn zeta_series_terms_for_precision(order: u32, prec_bits: u64) -> usize {
    let exponent = (order - 1) as f64;
    let min_terms = 2f64.powf((prec_bits as f64 + 1.0) / exponent).ceil() as usize;
    min_terms.clamp(10, ZETA_DIRECT_SUM_TERM_CAP)
}

/// Euler–Maclaurin tail for `ζ(order)` when the direct partial sum needs too many terms.
fn zeta_euler_maclaurin_float(order: u32, prec_bits: u64) -> Result<Float, String> {
    if order < 2 {
        return Err("zeta order must be >= 2".into());
    }
    let prec = prec_bits + 64;
    let (n, k_max) = euler_maclaurin_zeta_params(prec_bits, order);
    tracing::info!(phase = "zeta5_em", order, prec, n, k_max, bernoulli_max = 2 * k_max, "Euler-Maclaurin start");
    let bernoulli = bernoulli_float_table(2 * k_max, prec);
    tracing::info!(phase = "zeta5_em", "Bernoulli table ready, direct head sum");
    let mut sum = float_at_unsigned(prec, 0);
    for index in 1..=n {
        sum += float_at_unsigned(prec, index as u64).pow(-(order as i64));
    }
    let nf = float_at_unsigned(prec, n as u64);
    sum += nf.clone().pow(1 - order as i64) / float_at_unsigned(prec, u64::from(order - 1));
    sum += nf.clone().pow(-(order as i64)) / float_at_unsigned(prec, 2);
    let em_step = (k_max / 20).max(1);
    for k in 1..=k_max {
        let b = bernoulli[2 * k].clone();
        let factorial = factorial_float(2 * k, prec)?;
        let rising = rising_factorial_float(order, k, prec)?;
        let exponent = 1 - order as i64 - 2 * k as i64;
        sum += b / factorial * rising * nf.clone().pow(exponent);
        trace_step("zeta5_em", k, k_max, em_step);
    }
    tracing::info!(phase = "zeta5_em", order, prec, "Euler-Maclaurin complete");
    Ok(sum)
}

fn euler_maclaurin_zeta_params(prec_bits: u64, order: u32) -> (usize, usize) {
    let prec = (prec_bits + 64) as usize;
    let order = order as usize;
    let n = ((prec as f64).sqrt().round() as usize).clamp(64, 65_536);
    let log2_n = (n as f64).log2();
    let k_max = (((prec as f64) / log2_n - (order - 1) as f64) / 2.0).ceil() as usize;
    (n, k_max.clamp(32, prec / 4))
}

fn bernoulli_float_table(max_index: usize, prec: u64) -> Vec<Float> {
    let mut bernoulli = vec![float_at_unsigned(prec, 0); max_index + 1];
    bernoulli[0] = Float::one_prec(prec);
    let step = (max_index / 20).max(1);
    for m in 1..=max_index {
        let mut acc = float_at_unsigned(prec, 0);
        for k in 0..m {
            acc += binomial_float(m + 1, k, prec) * bernoulli[k].clone();
        }
        bernoulli[m] = -(acc / float_at_unsigned(prec, m as u64 + 1));
        trace_step("bernoulli_table", m, max_index, step);
    }
    bernoulli
}

fn binomial_float(n: usize, k: usize, prec: u64) -> Float {
    if k > n {
        return float_at_unsigned(prec, 0);
    }
    if k == 0 {
        return Float::one_prec(prec);
    }
    let k = k.min(n - k);
    let mut acc = Float::one_prec(prec);
    for index in 0..k {
        acc *= float_at_unsigned(prec, (n - index) as u64);
        acc /= float_at_unsigned(prec, (index + 1) as u64);
    }
    acc
}

fn factorial_float(n: usize, prec: u64) -> Result<Float, String> {
    let mut acc = Float::one_prec(prec);
    for index in 2..=n {
        acc *= float_at_unsigned(prec, index as u64);
    }
    Ok(acc)
}

fn rising_factorial_float(s: u32, k: usize, prec: u64) -> Result<Float, String> {
    let mut acc = Float::one_prec(prec);
    for j in 0..(2 * k - 1) {
        acc *= float_at_unsigned(prec, u64::from(s + j as u32));
    }
    Ok(acc)
}
