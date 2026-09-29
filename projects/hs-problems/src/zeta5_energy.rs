//! Energy diagnostics for the Zeta5 polynomial Hankel line, aligned with `hankel.py#run`.

use crate::polynomial_hankel::polynomial_hankel_log_s_k;
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

/// Paper energy logs at `ζ(s)` for polynomial Hankel diagnostics.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ZetaPolynomialEnergyReport {
    /// `log S_K` normalization factor from the paper.
    pub log_s_k: f64,
    /// `log Δ_K(ζ(s))`.
    pub log_delta_at_zeta: f64,
    /// `log F_K(ζ(s)) = log Δ_K + log S_K`.
    pub log_f_k: f64,
    /// `log content(F_K)`.
    pub log_content_f_k: f64,
    /// `log P_K(ζ(s))` for the primitive integer polynomial.
    pub log_primitive_at_zeta: f64,
    /// Maximum bit length of primitive coefficients.
    pub max_primitive_coeff_bits: usize,
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
    polynomial_hankel_log_s_k(params)
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

/// Evaluate energy logs for an already computed `Δ_K` at `ζ(s)`.
pub fn zeta_polynomial_energy_report(
    order: u32,
    params: &Zeta5PaperParams,
    delta: &[Rational],
) -> Result<ZetaPolynomialEnergyReport, String> {
    if order < 2 {
        return Err("zeta order must be >= 2".into());
    }
    tracing::info!(phase = "energy_report", order, "start");
    let primitive = zeta5_delta_primitive(delta);
    let max_bits = max_coeff_bits(&primitive.coefficients);
    let prec_bits = energy_eval_precision_bits(max_bits, params.k);
    let work_prec = prec_bits + 64;
    tracing::info!(
        phase = "energy_report",
        order,
        max_coeff_bits = max_bits,
        prec_bits,
        work_prec,
        "primitive content cleared"
    );
    tracing::info!(phase = "zeta_borwein", order, prec_bits, "computing zeta(s)");
    let zeta_value = zeta_at_precision(order, prec_bits)?;
    tracing::info!(phase = "energy_report", order, "zeta(s) ready, evaluating Delta_K");
    let log_s_k = polynomial_hankel_log_s_k(params);
    let delta_value = evaluate_rational_poly_float(delta, &zeta_value, work_prec)?;
    tracing::info!(phase = "energy_report", order, "Delta_K evaluated, scaling to P_K");
    let content_scale = Float::from_rational_prec(
        Rational::from_integers(
            Integer::from(primitive.content_denominator.clone()),
            Integer::from(primitive.content_numerator.clone()),
        ),
        work_prec,
    )
    .0;
    let primitive_value = delta_value.clone() * content_scale;
    let log_delta_at_zeta = log_positive_float("Delta_K", &delta_value)?;
    let log_primitive_at_zeta = log_positive_float("P_K", &primitive_value)?;
    let log_content = log_natural_ratio(&primitive.content_numerator, &primitive.content_denominator)?;
    Ok(ZetaPolynomialEnergyReport {
        log_s_k,
        log_delta_at_zeta,
        log_f_k: log_delta_at_zeta + log_s_k,
        log_content_f_k: log_content + log_s_k,
        log_primitive_at_zeta,
        max_primitive_coeff_bits: max_bits,
    })
}

/// Evaluate energy logs for an already computed `Δ_K` at `ζ(5)`.
pub fn zeta5_energy_report(params: &Zeta5PaperParams, delta: &[Rational]) -> Result<Zeta5EnergyReport, String> {
    let report = zeta_polynomial_energy_report(5, params, delta)?;
    Ok(Zeta5EnergyReport {
        log_s_k: report.log_s_k,
        log_delta_at_zeta5: report.log_delta_at_zeta,
        log_f_k: report.log_f_k,
        log_content_f_k: report.log_content_f_k,
        log_primitive_at_zeta5: report.log_primitive_at_zeta,
        max_primitive_coeff_bits: report.max_primitive_coeff_bits,
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

fn evaluate_rational_poly_float(coefficients: &[Rational], point: &Float, work_prec: u64) -> Result<Float, String> {
    if coefficients.is_empty() {
        return Ok(float_at_unsigned(work_prec, 0));
    }
    let mut sum = float_at_unsigned(work_prec, 0);
    let mut power = Float::one_prec(work_prec);
    for coeff in coefficients {
        sum += Float::from_rational_prec(coeff.clone(), work_prec).0 * power.clone();
        power *= point;
    }
    Ok(sum)
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
    let required_terms = zeta_series_required_terms_saturated(order, prec_bits);
    if required_terms <= ZETA_DIRECT_SUM_TERM_CAP && prec_bits < 4096 {
        tracing::info!(
            phase = "zeta5",
            route = "direct_series",
            order,
            prec_bits,
            terms = required_terms,
        );
        zeta_series_float(order, required_terms, prec_bits)
    } else {
        tracing::info!(
            phase = "zeta5",
            route = "borwein",
            order,
            prec_bits,
            required_terms,
            cap = ZETA_DIRECT_SUM_TERM_CAP,
        );
        borwein_zeta_float(order, prec_bits)
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

fn zeta_series_required_terms_saturated(order: u32, prec_bits: u64) -> usize {
    let exponent = (order - 1) as f64;
    let value = 2f64.powf((prec_bits as f64 + 1.0) / exponent);
    if value.is_finite() && value <= ZETA_DIRECT_SUM_TERM_CAP as f64 {
        value.ceil() as usize
    } else {
        ZETA_DIRECT_SUM_TERM_CAP + 1
    }
}

/// Borwein ζ(s) for integer `s >= 2`, matching MPFR / Flint `arb_zeta` on the real axis.
fn borwein_zeta_float(order: u32, prec_bits: u64) -> Result<Float, String> {
    if order < 2 {
        return Err("zeta order must be >= 2".into());
    }
    let prec = prec_bits + 64;
    let n = borwein_zeta_degree(prec_bits);
    tracing::info!(phase = "zeta5_borwein", order, prec_bits, prec, borwein_n = n, "start");
    let d = borwein_d_table(n, prec);
    let dn = d[n].clone();
    let mut sum = float_at_unsigned(prec, 0);
    let step = (n / 20).max(1);
    for k in 0..n {
        let coeff = if k % 2 == 0 {
            d[k].clone() - dn.clone()
        } else {
            -(d[k].clone() - dn.clone())
        };
        let base = float_at_unsigned(prec, (k + 1) as u64);
        sum += coeff / base.pow(order as i64);
        trace_step("zeta5_borwein", k + 1, n, step);
    }
    let two = float_at_unsigned(prec, 2);
    let multiplier = float_at_unsigned(prec, 1) - float_at_unsigned(prec, 2) / two.pow(order as i64);
    tracing::info!(phase = "zeta5_borwein", order, prec, "complete");
    Ok(-sum / (dn * multiplier))
}

/// `n ≈ 1.3 d` decimal digits (Gourdon–Sebah), with guard bits for `ζ(5)` energy logs.
fn borwein_zeta_degree(prec_bits: u64) -> usize {
    let decimal_digits = prec_bits as f64 * std::f64::consts::LOG10_2;
    ((1.3 * decimal_digits + 16.0).ceil() as usize).clamp(10, 100_000)
}

fn borwein_d_table(n: usize, prec: u64) -> Vec<Float> {
    let nf = float_at_unsigned(prec, n as u64);
    let mut inner = float_at_unsigned(prec, 0);
    let mut d = Vec::with_capacity(n + 1);
    for i in 0..=n {
        inner += borwein_d_term(n, i, prec);
        d.push(nf.clone() * inner.clone());
    }
    d
}

fn borwein_d_term(n: usize, i: usize, prec: u64) -> Float {
    if i == 0 {
        return float_at_unsigned(prec, 1) / float_at_unsigned(prec, n as u64);
    }
    let mut ratio = Float::one_prec(prec);
    for m in 0..(2 * i - 1) {
        ratio *= float_at_unsigned(prec, (n - i + 1 + m) as u64);
    }
    let mut factorial = Float::one_prec(prec);
    for m in 1..=(2 * i) {
        factorial *= float_at_unsigned(prec, m as u64);
    }
    ratio * float_at_unsigned(prec, 4).pow(i as u64) / factorial
}
