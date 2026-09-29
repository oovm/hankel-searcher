//! Gram/Hankel determinant pipeline for `ζ(5)` aligned with `mo271/Zeta5` `scripts/hankel.py`.

use crate::polynomial_hankel::{
    zeta_polynomial_delta, zeta_polynomial_delta_degree, zeta_polynomial_delta_leading_coeff,
    zeta_polynomial_entries, zeta_polynomial_hankel_a, zeta_polynomial_hankel_b,
};
use crate::zeta5_polynomial::Zeta5PaperParams;
use hs_types::Rational;

const ORDER: u32 = 5;

/// Moment sequences `(a_e, b_e)` for `e = 0..2h-2` from the Zeta5 paper construction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Zeta5Entries {
    /// Paper scaling `K`, `N`, `h` for this construction index.
    pub params: Zeta5PaperParams,
    /// Moment sequence `a_e` for `e = 0..2h-2`.
    pub a: Vec<Rational>,
    /// Moment sequence `b_e` for `e = 0..2h-2`.
    pub b: Vec<Rational>,
}

/// Build `(K, N, h, a, b)` matching `hankel.py#entries`.
pub fn zeta5_entries(n: usize) -> Zeta5Entries {
    let entries = zeta_polynomial_entries(ORDER, n).expect("zeta5 entries");
    Zeta5Entries {
        params: entries.params,
        a: entries.a,
        b: entries.b,
    }
}

/// Hankel matrix `(a_{i+j})` of size `h × h`.
pub fn zeta5_hankel_a(entries: &Zeta5Entries) -> Vec<Vec<Rational>> {
    zeta_polynomial_hankel_a(&to_polynomial_entries(entries))
}

/// Hankel matrix `(b_{i+j})` of size `h × h`.
pub fn zeta5_hankel_b(entries: &Zeta5Entries) -> Vec<Vec<Rational>> {
    zeta_polynomial_hankel_b(&to_polynomial_entries(entries))
}

/// Determinant polynomial `Δ_K` with ascending coefficients, matching `hankel.py#det_poly`.
pub fn zeta5_delta_polynomial(entries: &Zeta5Entries) -> Vec<Rational> {
    zeta_polynomial_delta(&to_polynomial_entries(entries)).expect("Δ_K pencil determinant")
}

/// Expected degree of `Δ_K`.
pub fn zeta5_delta_degree(entries: &Zeta5Entries) -> usize {
    zeta_polynomial_delta_degree(&to_polynomial_entries(entries))
}

/// Leading coefficient check from paper (2.9).
pub fn zeta5_delta_leading_coeff(entries: &Zeta5Entries) -> Rational {
    zeta_polynomial_delta_leading_coeff(&to_polynomial_entries(entries))
}

fn to_polynomial_entries(entries: &Zeta5Entries) -> crate::polynomial_hankel::ZetaPolynomialEntries {
    crate::polynomial_hankel::ZetaPolynomialEntries {
        order: ORDER,
        params: entries.params,
        a: entries.a.clone(),
        b: entries.b.clone(),
    }
}

#[cfg(all(test, feature = "offline-golden"))]
mod offline_det_tests {
    use super::*;
    use hs_types::{Integer, Rational, bareiss_det, bareiss_det_rational};

    #[test]
    fn bareiss_rational_matches_generic_on_zeta5_a() {
        let entries = zeta5_entries(1);
        let a = zeta5_hankel_a(&entries);
        assert_eq!(bareiss_det(&a), bareiss_det_rational(&a));
    }

    #[test]
    fn delta_polynomial_matches_pencil_samples() {
        let entries = zeta5_entries(1);
        let a = zeta5_hankel_a(&entries);
        let b = zeta5_hankel_b(&entries);
        let delta = zeta5_delta_polynomial(&entries);
        for point in 0..=entries.params.h {
            let x = Rational::from(Integer::from(point));
            let mut power = Rational::ONE;
            let mut value = Rational::ZERO;
            for coeff in &delta {
                value += coeff.clone() * power.clone();
                power *= x.clone();
            }
            let matrix = a
                .iter()
                .zip(&b)
                .map(|(row_a, row_b)| {
                    row_a
                        .iter()
                        .zip(row_b)
                        .map(|(left, right)| left.clone() + x.clone() * right.clone())
                        .collect()
                })
                .collect::<Vec<_>>();
            let expected = bareiss_det_rational(&matrix);
            assert_eq!(value, expected, "mismatch at x={point}");
        }
    }
}

#[cfg(test)]
mod poly_tests {
    use super::*;
    use crate::polynomial_hankel::{poly_div, poly_mul_monomial, poly_pow};
    use crate::zeta5_polynomial::{d_polynomial, zeta5_paper_params};
    use hs_types::Integer;
    use malachite::base::num::basic::traits::One;

    #[test]
    fn pe_division_exact_at_e22_for_n1() {
        let params = zeta5_paper_params(1);
        let dn = d_polynomial(params.capital_n);
        let dk = d_polynomial(params.k);
        let w = poly_pow(&dn, params.q);
        let shifted = poly_mul_monomial(&w, 22);
        assert_eq!(poly_div(&shifted, &dk), vec![Rational::from(Integer::ONE)]);
    }
}
