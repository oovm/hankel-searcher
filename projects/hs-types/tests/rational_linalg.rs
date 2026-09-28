use hs_types::{rational_charpoly, rational_mat_inv, rational_mat_mul};
use num_bigint::BigInt;
use num_rational::Ratio;
use num_traits::{One, Zero};

#[test]
fn rational_mat_inv_round_trips() {
    let matrix = vec![
        vec![Ratio::from_integer(BigInt::from(4)), Ratio::from_integer(BigInt::from(7))],
        vec![Ratio::from_integer(BigInt::from(2)), Ratio::from_integer(BigInt::from(6))],
    ];
    let inv = rational_mat_inv(&matrix).expect("matrix must invert");
    let product = rational_mat_mul(&matrix, &inv);
    for row in 0..2 {
        for col in 0..2 {
            let expected = if row == col { Ratio::one() } else { Ratio::zero() };
            assert_eq!(product[row][col], expected);
        }
    }
}

#[test]
fn rational_charpoly_matches_two_by_two() {
    let matrix = vec![
        vec![Ratio::from_integer(BigInt::from(1)), Ratio::from_integer(BigInt::from(2))],
        vec![Ratio::from_integer(BigInt::from(3)), Ratio::from_integer(BigInt::from(4))],
    ];
    let poly = rational_charpoly(&matrix);
    assert_eq!(poly.len(), 3);
    assert_eq!(poly[0], Ratio::from_integer(BigInt::from(-2)));
    assert_eq!(poly[1], Ratio::from_integer(BigInt::from(-5)));
    assert_eq!(poly[2], Ratio::one());
}
