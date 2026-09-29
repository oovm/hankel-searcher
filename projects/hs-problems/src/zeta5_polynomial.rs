//! Whole-polynomial Hankel construction for `ζ(5)` following the Zeta5 paper scaling.
//!
//! Reference: `mo271/Zeta5` (`scripts/hankel.py`), with `K=40n`, `N=3n`, `h=37n`.

use hs_types::{Integer, Rational};
use malachite::base::num::basic::traits::{One, Zero};

/// Paper scaling for index `n`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Zeta5PaperParams {
    /// Construction index `n`.
    pub n: usize,
    /// Hankel size parameter `K`.
    pub k: usize,
    /// Vanishing-order parameter `N`.
    pub capital_n: usize,
    /// Polynomial determinant degree `h`.
    pub h: usize,
}

/// Return `K=40n`, `N=3n`, `h=37n`.
pub fn zeta5_paper_params(n: usize) -> Zeta5PaperParams {
    Zeta5PaperParams {
        n,
        k: 40 * n,
        capital_n: 3 * n,
        h: 37 * n,
    }
}

/// Coefficients in ascending order for `D_m(t) = ∏_{j=1}^m (t + j^2)`.
pub fn d_polynomial(m: usize) -> Vec<Rational> {
    let mut poly = vec![Rational::ONE];
    for j in 1..=m {
        let j2 = Rational::from(Integer::from(j * j));
        poly = multiply_by_linear(poly, j2);
    }
    poly
}

fn multiply_by_linear(poly: Vec<Rational>, constant: Rational) -> Vec<Rational> {
    let mut out = vec![Rational::ZERO; poly.len() + 1];
    for (index, coeff) in poly.iter().enumerate() {
        out[index] += constant.clone() * coeff;
        out[index + 1] += coeff.clone();
    }
    out
}

/// Evaluate a polynomial given by ascending coefficients.
pub fn evaluate_polynomial(poly: &[Rational], point: &Rational) -> Rational {
    let mut acc = Rational::ZERO;
    let mut power = Rational::ONE;
    for coeff in poly {
        acc += coeff.clone() * power.clone();
        power *= point.clone();
    }
    acc
}

/// Degree of `D_m`; equals `m`.
pub fn d_polynomial_degree(m: usize) -> usize {
    m
}
