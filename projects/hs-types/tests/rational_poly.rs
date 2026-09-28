use hs_types::{
    Integer, Rational, bareiss_det, det_linear_pencil, rational_charpoly, rational_mat_inv, rational_mat_mul,
    scale_polynomial,
};
use malachite::base::num::basic::traits::{One, Zero};

fn q(n: i64, d: i64) -> Rational {
    Rational::from_integers(Integer::from(n), Integer::from(d))
}

fn scale_matrix(matrix: &[Vec<Rational>], factor: &Rational) -> Vec<Vec<Rational>> {
    matrix
        .iter()
        .map(|row| row.iter().map(|entry| entry.clone() * factor).collect())
        .collect()
}

fn delta_via_charpoly(a: &[Vec<Rational>], b: &[Vec<Rational>]) -> Vec<Rational> {
    let det_b = bareiss_det(b);
    let b_inv = rational_mat_inv(b).expect("invertible");
    let neg_a = scale_matrix(a, &-Rational::ONE);
    let c = rational_mat_mul(&b_inv, &neg_a);
    scale_polynomial(&rational_charpoly(&c), &det_b)
}

#[test]
fn det_linear_pencil_matches_charpoly_route_for_three_by_three() {
    let a = vec![
        vec![q(1, 2), q(1, 3), q(1, 5)],
        vec![q(1, 3), q(2, 5), q(3, 7)],
        vec![q(1, 5), q(3, 7), q(4, 9)],
    ];
    let b = vec![
        vec![q(2, 1), q(1, 2), Rational::ZERO],
        vec![q(1, 2), q(3, 2), q(1, 4)],
        vec![Rational::ZERO, q(1, 4), q(5, 3)],
    ];
    let pencil = det_linear_pencil(&a, &b).expect("pencil det");
    let charpoly = delta_via_charpoly(&a, &b);
    assert_eq!(pencil, charpoly);
}
