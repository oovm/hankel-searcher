use hs_types::{Integer, Rational, bareiss_det, bareiss_det_rational, f64_det, hankel_matrix};
use malachite::base::num::basic::traits::{One, Zero};

fn q(n: i64, d: i64) -> Rational {
    Rational::from_integers(Integer::from(n), Integer::from(d))
}

#[test]
fn bareiss_rational_matches_small_matrix() {
    let matrix = vec![vec![q(1, 2), q(1, 3)], vec![q(1, 4), q(2, 5)]];
    assert_eq!(bareiss_det_rational(&matrix), bareiss_det(&matrix));
}

#[test]
fn bareiss_matches_small_integer_matrix() {
    let matrix = vec![
        vec![Integer::from(1), Integer::from(2)],
        vec![Integer::from(3), Integer::from(4)],
    ];
    let det = bareiss_det(&matrix);
    assert_eq!(det, Integer::from(-2));
}

#[test]
fn f64_det_matches_known_two_by_two() {
    let matrix = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
    assert!((f64_det(&matrix) + 2.0).abs() < 1e-12);
}

#[test]
fn hankel_det_of_moment_tail() {
    let moments = vec![
        Rational::ZERO,
        Rational::ONE,
        Rational::from(Integer::from(2)),
        Rational::from(Integer::from(5)),
        Rational::from(Integer::from(16)),
    ];
    let matrix = hankel_matrix(&moments, 0, 2);
    let det = bareiss_det(&matrix);
    assert_ne!(det, Rational::ZERO);
}
