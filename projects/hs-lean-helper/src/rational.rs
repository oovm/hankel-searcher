use num_bigint::BigInt;
use num_rational::Ratio;
use num_traits::{Signed, Zero};

/// Least common multiple of two integers.
pub fn lcm_bigint(a: &BigInt, b: &BigInt) -> BigInt {
    if a.is_zero() || b.is_zero() {
        return BigInt::zero();
    }
    (a * b).abs() / num_integer::Integer::gcd(a, b)
}

/// Least common multiple of two rational denominators.
pub fn lcm_ratio_denoms(a: &Ratio<BigInt>, b: &Ratio<BigInt>) -> BigInt {
    lcm_bigint(a.denom(), b.denom())
}

/// Render a rational as Lean `(num, den)` integer literals.
pub fn ratio_to_lean_ints(r: &Ratio<BigInt>) -> (BigInt, BigInt) {
    (r.numer().clone(), r.denom().clone())
}
