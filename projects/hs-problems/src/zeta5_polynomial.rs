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
    /// Polynomial determinant degree `h = K - N`.
    pub h: usize,
    /// Vanishing exponent `q` on `D_N(t)^q`.
    pub q: usize,
}

/// Return `K=40n`, `N=3n`, `h=37n`, `q=6`.
pub fn zeta5_paper_params(n: usize) -> Zeta5PaperParams {
    polynomial_hankel_params(n, 40, 3, 6).expect("default paper scaling")
}

/// Build `(K,N,h,q)` from per-`n` multiples `K=k_per_n·n`, `N=capital_n_per_n·n`.
pub fn polynomial_hankel_params(
    n: usize,
    k_per_n: usize,
    capital_n_per_n: usize,
    q: usize,
) -> Result<Zeta5PaperParams, String> {
    if n == 0 {
        return Err("construction index `n` must be positive".into());
    }
    if q == 0 {
        return Err("vanishing exponent `q` must be positive".into());
    }
    if capital_n_per_n >= k_per_n {
        return Err(format!(
            "need capital_n_per_n < k_per_n, got N={capital_n_per_n}n and K={k_per_n}n"
        ));
    }
    let k = k_per_n * n;
    let capital_n = capital_n_per_n * n;
    let h = k - capital_n;
    if h == 0 {
        return Err("h = K - N must be positive".into());
    }
    Ok(Zeta5PaperParams {
        n,
        k,
        capital_n,
        h,
        q,
    })
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
