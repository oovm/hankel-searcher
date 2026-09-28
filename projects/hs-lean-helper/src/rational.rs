use hs_types::{Integer, Natural, Rational};
use malachite::base::num::arithmetic::traits::Lcm;
use malachite::base::num::basic::traits::Zero;

/// Least common multiple of two natural denominators.
pub fn lcm_natural(a: &Natural, b: &Natural) -> Natural {
    if *a == Natural::ZERO || *b == Natural::ZERO {
        return Natural::ZERO;
    }
    Natural::lcm(a.clone(), b.clone())
}

/// Least common multiple of two rational denominators as an integer scale.
pub fn lcm_ratio_denoms(a: &Rational, b: &Rational) -> Integer {
    Integer::from(Natural::lcm(a.to_denominator(), b.to_denominator()))
}

/// Render a rational as Lean `(num, den)` integer literals.
pub fn ratio_to_lean_ints(r: &Rational) -> (Integer, Integer) {
    (
        Integer::from(r.to_numerator().clone()),
        Integer::from(r.to_denominator().clone()),
    )
}
