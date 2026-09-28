use crate::RationalData;
use hs_types::{Integer, Natural, Rational, is_zero, rational_from_str};
use malachite::base::num::basic::traits::Zero;

pub fn ratio_from_data(data: &RationalData) -> Result<Rational, String> {
    let ratio = rational_from_str(&data.num, &data.den)?;
    let canonical = RationalData::from_ratio(&ratio);
    if canonical.num != data.num || canonical.den != data.den {
        return Err("rational must be canonical".into());
    }
    Ok(ratio)
}

/// Best-effort `f64` head for diagnostics (not used in proof paths).
pub fn rational_to_f64(value: &Rational) -> Option<f64> {
    use malachite::Float;
    use malachite::base::num::conversion::traits::RoundingFrom;
    use malachite::base::rounding_modes::RoundingMode::Nearest;
    if is_zero(value) {
        return Some(0.0);
    }
    let (head, _) = Float::from_rational_prec_ref(value, 128);
    if head.is_nan() {
        return None;
    }
    Some(f64::rounding_from(&head, Nearest).0)
}

/// Parse a base-10 natural string and reject zero.
pub fn natural_from_str(value: &str) -> Result<Natural, String> {
    let parsed = value
        .parse::<Natural>()
        .map_err(|_| format!("invalid natural `{value}`"))?;
    if parsed == Natural::ZERO {
        return Err("natural must be positive".into());
    }
    Ok(parsed)
}

/// Parse a base-10 integer string.
pub fn integer_from_str(value: &str) -> Result<Integer, String> {
    value
        .parse::<Integer>()
        .map_err(|_| format!("invalid integer `{value}`"))
}
