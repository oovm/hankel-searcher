use hs_problems::zeta5_polynomial::{
    d_polynomial, d_polynomial_degree, evaluate_polynomial, zeta5_paper_params,
};
use hs_types::{Integer, Rational, is_zero};
use malachite::base::num::basic::traits::{One, Zero};

#[test]
fn paper_params_scale_linearly() {
    let params = zeta5_paper_params(2);
    assert_eq!(params.k, 80);
    assert_eq!(params.capital_n, 6);
    assert_eq!(params.h, 74);
}

#[test]
fn d_polynomial_matches_product_definition() {
    let poly = d_polynomial(3);
    assert_eq!(d_polynomial_degree(3), 3);
    assert_eq!(poly.len(), 4);
    let root = Rational::from(Integer::from(-9));
    assert!(is_zero(&evaluate_polynomial(&poly, &root)));
    let one = Rational::ONE;
    assert_eq!(evaluate_polynomial(&poly, &one), Rational::from(Integer::from(2 * 5 * 10)));
}
