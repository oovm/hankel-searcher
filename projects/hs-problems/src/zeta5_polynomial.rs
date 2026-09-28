//! Whole-polynomial Hankel construction for `ζ(5)` following the Zeta5 paper scaling.
//!
//! Reference: `mo271/Zeta5` (`scripts/hankel.py`), with `K=40n`, `N=3n`, `h=37n`.

use num_bigint::BigInt;
use num_rational::Ratio;
use num_traits::{One, Zero};

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
pub fn d_polynomial(m: usize) -> Vec<Ratio<BigInt>> {
    let mut poly = vec![Ratio::one()];
    for j in 1..=m {
        let j2 = Ratio::from_integer(BigInt::from(j * j));
        poly = multiply_by_linear(poly, j2);
    }
    poly
}

fn multiply_by_linear(poly: Vec<Ratio<BigInt>>, constant: Ratio<BigInt>) -> Vec<Ratio<BigInt>> {
    let mut out = vec![Ratio::zero(); poly.len() + 1];
    for (index, coeff) in poly.iter().enumerate() {
        out[index] += &constant * coeff;
        out[index + 1] += coeff.clone();
    }
    out
}

/// Evaluate a polynomial given by ascending coefficients.
pub fn evaluate_polynomial(poly: &[Ratio<BigInt>], point: &Ratio<BigInt>) -> Ratio<BigInt> {
    let mut acc = Ratio::zero();
    let mut power = Ratio::one();
    for coeff in poly {
        acc += coeff * &power;
        power *= point;
    }
    acc
}

/// Degree of `D_m`; equals `m`.
pub fn d_polynomial_degree(m: usize) -> usize {
    m
}
