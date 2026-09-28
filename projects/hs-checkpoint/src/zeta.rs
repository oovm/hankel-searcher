use crate::RationalData;
use num_bigint::BigInt;
use num_rational::Ratio;
use num_traits::Zero;

/// Rigorous partial-sum lower bound and integral tail upper bound for `ζ(order)`.
///
/// For `order >= 2` and `terms = M`:
/// `lower = Σ_{j=1}^M 1/j^order`
/// `upper = lower + 1/((order-1) M^{order-1})`.
pub fn zeta_series_bounds(order: u32, terms: usize) -> Result<(Ratio<BigInt>, Ratio<BigInt>), String> {
    if order < 2 {
        return Err("zeta series bounds require order >= 2".into());
    }
    if terms == 0 {
        return Err("series terms must be positive".into());
    }
    let mut lower = Ratio::from_integer(BigInt::zero());
    for index in 1..=terms {
        let index = BigInt::from(index);
        lower += Ratio::new(BigInt::from(1), index.pow(order));
    }
    let m = BigInt::from(terms);
    let tail_den = BigInt::from(order - 1) * m.pow(order - 1);
    let upper = &lower + Ratio::new(BigInt::from(1), tail_den);
    Ok((lower, upper))
}

/// Return whether `value` lies inside the rigorous `ζ(order)` enclosure at `terms`.
pub fn zeta_value_in_enclosure(order: u32, terms: usize, value: &Ratio<BigInt>) -> Result<bool, String> {
    let (lower, upper) = zeta_series_bounds(order, terms)?;
    Ok(value >= &lower && value <= &upper)
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
