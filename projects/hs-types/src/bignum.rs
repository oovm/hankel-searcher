//! Arbitrary-precision integers and rationals via [Malachite](https://www.malachite.rs/).

pub use malachite::Integer;
pub use malachite::Natural;
pub use malachite::Rational;
pub use malachite::base::num::basic::traits::{One, Zero};

/// Whether `value` is exactly zero (Malachite `Zero` has no `is_zero` helper).
#[inline]
pub fn is_zero<T: Zero + PartialEq>(value: &T) -> bool {
    *value == T::ZERO
}

/// Parse a base-10 integer string.
pub fn integer_from_str(value: &str) -> Result<Integer, String> {
    std::str::FromStr::from_str(value).map_err(|_| format!("invalid integer `{value}`"))
}

/// Parse a reduced rational from decimal numerator and denominator strings.
pub fn rational_from_str(numerator: &str, denominator: &str) -> Result<Rational, String> {
    Ok(Rational::from_integers(
        integer_from_str(numerator)?,
        integer_from_str(denominator)?,
    ))
}
