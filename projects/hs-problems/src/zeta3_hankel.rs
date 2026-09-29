//! Whole-polynomial Hankel construction for `ζ(3)` (`μ_{3,X}` / design doc `06`).

use crate::polynomial_hankel::{
    ZetaPolynomialEntries, polynomial_hankel_log_s_k, zeta_polynomial_delta, zeta_polynomial_delta_degree,
    zeta_polynomial_delta_leading_coeff, zeta_polynomial_entries_with_params, zeta_polynomial_hankel_a,
    zeta_polynomial_hankel_b,
};
use crate::zeta5_polynomial::{Zeta5PaperParams, polynomial_hankel_params};
use hs_types::Rational;

const ORDER: u32 = 3;

/// `K=40n` with `N=3n` from offline N-q sweep at `n=1` (2026-09-29).
pub const ZETA3_OPTIMAL_K_PER_N: usize = 40;
/// Vanishing-order multiple `N=3n` (same as paper, sweep best at `n=1`).
pub const ZETA3_OPTIMAL_CAPITAL_N_PER_N: usize = 3;
/// Vanishing exponent `q=4` on `D_N^q` (sweep best for `ζ(3)` at `n=1`, not paper `q=6`).
pub const ZETA3_OPTIMAL_Q: usize = 4;

/// Moment sequences for the `ζ(3)` polynomial Hankel line.
pub type Zeta3Entries = ZetaPolynomialEntries;

/// Build entries at the offline N-q sweep optimum for construction index `n`.
pub fn zeta3_entries(n: usize) -> Result<Zeta3Entries, String> {
    zeta3_entries_with_params(zeta3_optimal_params(n)?)
}

/// `K=40n`, `N=3n`, `q=4`, `h=37n` at construction index `n`.
pub fn zeta3_optimal_params(n: usize) -> Result<Zeta5PaperParams, String> {
    polynomial_hankel_params(n, ZETA3_OPTIMAL_K_PER_N, ZETA3_OPTIMAL_CAPITAL_N_PER_N, ZETA3_OPTIMAL_Q)
}

/// Build entries at explicit `(K,N,q,h)` scaling.
pub fn zeta3_entries_with_params(params: Zeta5PaperParams) -> Result<Zeta3Entries, String> {
    zeta_polynomial_entries_with_params(ORDER, params)
}

/// `log S_K` normalization for the paper scaling.
pub fn zeta3_log_s_k(params: &Zeta5PaperParams) -> f64 {
    polynomial_hankel_log_s_k(params)
}

/// Hankel matrix `(a_{i+j})`.
pub fn zeta3_hankel_a(entries: &Zeta3Entries) -> Vec<Vec<Rational>> {
    zeta_polynomial_hankel_a(entries)
}

/// Hankel matrix `(b_{i+j})`.
pub fn zeta3_hankel_b(entries: &Zeta3Entries) -> Vec<Vec<Rational>> {
    zeta_polynomial_hankel_b(entries)
}

/// Determinant polynomial `Δ_K`.
pub fn zeta3_delta_polynomial(entries: &Zeta3Entries) -> Result<Vec<Rational>, String> {
    zeta_polynomial_delta(entries)
}

/// Expected degree of `Δ_K`.
pub fn zeta3_delta_degree(entries: &Zeta3Entries) -> usize {
    zeta_polynomial_delta_degree(entries)
}

/// Leading coefficient from the pole product formula.
pub fn zeta3_delta_leading_coeff(entries: &Zeta3Entries) -> Rational {
    zeta_polynomial_delta_leading_coeff(entries)
}
