use hs_checkpoint::zeta_series_bounds;
use num_bigint::BigInt;
use num_rational::Ratio;

#[test]
fn zeta3_tail_matches_legacy_formula() {
    let terms = 256usize;
    let (lower, upper) = zeta_series_bounds(3, terms).unwrap();
    let legacy_tail = Ratio::new(BigInt::from(1), BigInt::from(2) * BigInt::from(terms).pow(2));
    assert_eq!(upper - lower, legacy_tail);
}

#[test]
fn zeta2_tail_matches_integral_bound() {
    let terms = 128usize;
    let (lower, upper) = zeta_series_bounds(2, terms).unwrap();
    assert_eq!(upper - lower, Ratio::new(BigInt::from(1), BigInt::from(terms)));
}
