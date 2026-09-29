//! Whole-polynomial Hankel construction for `ζ(2)` (`μ_{2,X}` / design doc `06`).

use crate::polynomial_hankel::{
    ZetaPolynomialEntries, polynomial_hankel_log_s_k, zeta_polynomial_delta, zeta_polynomial_delta_degree,
    zeta_polynomial_delta_leading_coeff, zeta_polynomial_entries, zeta_polynomial_entries_with_params,
    zeta_polynomial_hankel_a, zeta_polynomial_hankel_b,
};
use crate::zeta5_polynomial::Zeta5PaperParams;
use hs_types::Rational;

const ORDER: u32 = 2;

/// Moment sequences for the `ζ(2)` polynomial Hankel line.
pub type Zeta2Entries = ZetaPolynomialEntries;

/// Build paper-scaled entries for construction index `n`.
pub fn zeta2_entries(n: usize) -> Result<Zeta2Entries, String> {
    zeta_polynomial_entries(ORDER, n)
}

/// Build entries at explicit `(K,N,q,h)` scaling.
pub fn zeta2_entries_with_params(params: Zeta5PaperParams) -> Result<Zeta2Entries, String> {
    zeta_polynomial_entries_with_params(ORDER, params)
}

/// `log S_K` normalization for the paper scaling.
pub fn zeta2_log_s_k(params: &Zeta5PaperParams) -> f64 {
    polynomial_hankel_log_s_k(params)
}

/// Hankel matrix `(a_{i+j})`.
pub fn zeta2_hankel_a(entries: &Zeta2Entries) -> Vec<Vec<Rational>> {
    zeta_polynomial_hankel_a(entries)
}

/// Hankel matrix `(b_{i+j})`.
pub fn zeta2_hankel_b(entries: &Zeta2Entries) -> Vec<Vec<Rational>> {
    zeta_polynomial_hankel_b(entries)
}

/// Determinant polynomial `Δ_K`.
pub fn zeta2_delta_polynomial(entries: &Zeta2Entries) -> Result<Vec<Rational>, String> {
    zeta_polynomial_delta(entries)
}

/// Expected degree of `Δ_K`.
pub fn zeta2_delta_degree(entries: &Zeta2Entries) -> usize {
    zeta_polynomial_delta_degree(entries)
}

/// Leading coefficient from the pole product formula.
pub fn zeta2_delta_leading_coeff(entries: &Zeta2Entries) -> Rational {
    zeta_polynomial_delta_leading_coeff(entries)
}
