use crate::RationalData;
use hs_types::{Integer, Rational};
use malachite::base::num::arithmetic::traits::Pow;
use malachite::base::num::basic::traits::{One, Zero};

/// Rigorous partial-sum lower bound and integral tail upper bound for `ζ(order)`.
///
/// For `order >= 2` and `terms = M`:
/// `lower = Σ_{j=1}^M 1/j^order`
/// `upper = lower + 1/((order-1) M^{order-1})`.
pub fn zeta_series_bounds(order: u32, terms: usize) -> Result<(Rational, Rational), String> {
    if order < 2 {
        return Err("zeta series bounds require order >= 2".into());
    }
    if terms == 0 {
        return Err("series terms must be positive".into());
    }
    let mut lower = Rational::ZERO;
    for index in 1..=terms {
        let index = Integer::from(index);
        lower += Rational::from_integers(Integer::ONE, index.clone().pow(u64::from(order)));
    }
    let m = Integer::from(terms);
    let tail_den = Integer::from(order - 1) * m.pow(u64::from(order - 1));
    let upper = lower.clone() + Rational::from_integers(Integer::ONE, tail_den);
    Ok((lower, upper))
}

/// Return whether `value` lies inside the rigorous `ζ(order)` enclosure at `terms`.
pub fn zeta_value_in_enclosure(order: u32, terms: usize, value: &Rational) -> Result<bool, String> {
    let (lower, upper) = zeta_series_bounds(order, terms)?;
    Ok(*value >= lower && *value <= upper)
}

/// Canonical rational value for conformance fixtures.
pub fn fixture_exact_rational(target: &str) -> Option<RationalData> {
    match target {
        "fixture-half" => Some(RationalData { num: "1".into(), den: "2".into() }),
        _ => None,
    }
}

/// Whether `target` admits an `exact-rational-v1` certificate today.
pub fn target_supports_rational_certificate(target: &str) -> bool {
    fixture_exact_rational(target).is_some()
}
