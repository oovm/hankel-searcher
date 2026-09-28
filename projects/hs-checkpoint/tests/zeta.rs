use hs_checkpoint::zeta_series_bounds;
use hs_types::{Integer, Rational};
use malachite::base::num::arithmetic::traits::Pow;
use malachite::base::num::basic::traits::{One, Zero};

#[test]
fn zeta3_tail_matches_legacy_formula() {
    let terms = 128usize;
    let (lower, upper) = zeta_series_bounds(3, terms).unwrap();
    let legacy_tail = Rational::from_integers(Integer::ONE, Integer::from(2) * Integer::from(terms).pow(2u64));
    assert_eq!(upper - lower, legacy_tail);
}

#[test]
fn zeta2_tail_matches_integral_bound() {
    let terms = 64usize;
    let (lower, upper) = zeta_series_bounds(2, terms).unwrap();
    assert_eq!(upper - lower, Rational::from_integers(Integer::ONE, Integer::from(terms)));
}
