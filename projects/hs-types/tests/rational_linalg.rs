use hs_types::{Integer, Rational, rational_charpoly, rational_mat_inv, rational_mat_mul};
use malachite::base::num::basic::traits::{One, Zero};

fn q(n: i64, d: i64) -> Rational {
    Rational::from_integers(Integer::from(n), Integer::from(d))
}

#[test]
fn rational_mat_inv_round_trips() {
    let matrix = vec![vec![q(4, 1), q(7, 1)], vec![q(2, 1), q(6, 1)]];
    let inv = rational_mat_inv(&matrix).expect("matrix must invert");
    let product = rational_mat_mul(&matrix, &inv);
    for row in 0..2 {
        for col in 0..2 {
            let expected = if row == col { Rational::ONE } else { Rational::ZERO };
            assert_eq!(product[row][col], expected);
        }
    }
}

#[test]
fn rational_charpoly_matches_two_by_two() {
    let matrix = vec![vec![q(1, 1), q(2, 1)], vec![q(3, 1), q(4, 1)]];
    let poly = rational_charpoly(&matrix);
    assert_eq!(poly.len(), 3);
    assert_eq!(poly[0], q(-2, 1));
    assert_eq!(poly[1], q(-5, 1));
    assert_eq!(poly[2], Rational::ONE);
}
