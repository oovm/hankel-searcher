//! Energy diagnostics for the Zeta5 polynomial Hankel line, aligned with `hankel.py#run`.

use crate::zeta5_polynomial::Zeta5PaperParams;
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_integer::Integer;
use num_rational::Ratio;
use num_traits::{One, Signed, ToPrimitive, Zero};
use std::str::FromStr;

type Rational = Ratio<BigInt>;

/// Primitive integer coefficients of `Δ_K` after clearing content.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Zeta5DeltaPrimitive {
    /// `gcd` of all coefficient numerators in `Δ_K`.
    pub content_numerator: BigInt,
    /// `lcm` of all coefficient denominators in `Δ_K`.
    pub content_denominator: BigInt,
    /// Integer coefficients of the primitive polynomial `P_K`.
    pub coefficients: Vec<BigInt>,
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
    let mut content_numerator = BigInt::zero();
    let mut content_denominator = BigInt::one();
    for coeff in delta {
        content_numerator = content_numerator.gcd(coeff.numer());
        content_denominator = lcm_bigint(&content_denominator, coeff.denom());
    }
    let coefficients = delta
        .iter()
        .map(|coeff| {
            let scaled = coeff.numer().clone() * (&content_denominator / coeff.denom());
            scaled / &content_numerator
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
    let primitive = zeta5_delta_primitive(delta);
    let zeta5 = zeta5_f64(12_000);
    let log_s_k = zeta5_log_s_k(params);
    let delta_value = evaluate_rational_poly_bigdecimal(delta, zeta5)?;
    let primitive_value = evaluate_integer_poly_bigdecimal(&primitive.coefficients, zeta5)?;
    let log_delta_at_zeta5 = log_positive_bigdecimal(&delta_value)?;
    let log_primitive_at_zeta5 = log_positive_bigdecimal(&primitive_value)?;
    let log_content = log_bigint_ratio(&primitive.content_numerator, &primitive.content_denominator)?;
    Ok(Zeta5EnergyReport {
        log_s_k,
        log_delta_at_zeta5,
        log_f_k: log_delta_at_zeta5 + log_s_k,
        log_content_f_k: log_content + log_s_k,
        log_primitive_at_zeta5,
        max_primitive_coeff_bits: max_coeff_bits(&primitive.coefficients),
    })
}

fn lcm_bigint(left: &BigInt, right: &BigInt) -> BigInt {
    if left.is_zero() || right.is_zero() {
        return BigInt::zero();
    }
    let gcd = left.gcd(right);
    (left / &gcd) * right
}

fn max_coeff_bits(coefficients: &[BigInt]) -> usize {
    coefficients
        .iter()
        .map(|coeff| coeff.magnitude().bits() as usize)
        .max()
        .unwrap_or(0)
}

fn zeta5_f64(terms: usize) -> f64 {
    let mut sum = 0.0;
    for index in 1..=terms {
        sum += 1.0 / (index as f64).powi(5);
    }
    sum + 1.0 / (4.0 * (terms as f64).powi(4))
}

fn evaluate_rational_poly_bigdecimal(coefficients: &[Rational], point: f64) -> Result<BigDecimal, String> {
    let point = BigDecimal::from_str(&point.to_string()).map_err(|error| error.to_string())?;
    let mut sum = BigDecimal::zero();
    let mut power = BigDecimal::one();
    for coeff in coefficients {
        let rational =
            BigDecimal::from_str(&format!("{}/{}", coeff.numer(), coeff.denom())).map_err(|error| error.to_string())?;
        sum += rational * &power;
        power *= &point;
    }
    Ok(sum)
}

fn evaluate_integer_poly_bigdecimal(coefficients: &[BigInt], point: f64) -> Result<BigDecimal, String> {
    let point = BigDecimal::from_str(&point.to_string()).map_err(|error| error.to_string())?;
    let mut sum = BigDecimal::zero();
    let mut power = BigDecimal::one();
    for coeff in coefficients {
        let integer = bigint_to_bigdecimal(coeff)?;
        sum += integer * &power;
        power *= &point;
    }
    Ok(sum)
}

fn bigint_to_bigdecimal(value: &BigInt) -> Result<BigDecimal, String> {
    BigDecimal::from_str(&value.to_string()).map_err(|error| error.to_string())
}

fn log_bigint_ratio(numerator: &BigInt, denominator: &BigInt) -> Result<f64, String> {
    let log_num = log_positive_bigint(numerator)?;
    let log_den = log_positive_bigint(denominator)?;
    Ok(log_num - log_den)
}

fn log_positive_bigdecimal(value: &BigDecimal) -> Result<f64, String> {
    if *value <= BigDecimal::zero() {
        return Err("log of non-positive BigDecimal".into());
    }
    let (mantissa, scale) = value.clone().into_bigint_and_scale();
    let log_mantissa = log_positive_bigint(&mantissa)?;
    Ok(log_mantissa - scale as f64 * std::f64::consts::LN_10)
}

fn log_positive_bigint(value: &BigInt) -> Result<f64, String> {
    if value.is_zero() {
        return Err("log of zero".into());
    }
    let magnitude = value.abs();
    let bits = magnitude.bits() as u32;
    if bits <= 53 {
        return magnitude
            .to_f64()
            .ok_or_else(|| "bigint to f64 failed".to_string())
            .map(|value| value.ln());
    }
    let shift = bits - 53;
    let top = (magnitude >> shift)
        .to_f64()
        .ok_or_else(|| "bigint top bits to f64 failed".to_string())?;
    Ok((bits as f64 - 1.0) * std::f64::consts::LN_2 + top.ln())
}
