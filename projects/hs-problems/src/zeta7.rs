use hs_moments::zeta_moments;
use hs_types::{Rational, RationalMomentSequence, ferguson_pair_at};

/// One Ferguson convergent for `ζ(7)`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Zeta7Approximant {
    /// Index `n`.
    pub index: usize,
    /// Numerator `P_n`.
    pub p: Rational,
    /// Denominator `Q_n`.
    pub q: Rational,
}

/// Build the first `count` Ferguson moments for `ζ(7)`.
pub fn zeta7_moments(count: usize) -> RationalMomentSequence {
    zeta_moments(7, count)
}

/// Compute `(P_n, Q_n)` for `ζ(7)` at index `n`.
pub fn ferguson_approximant(n: usize, moment_count: usize) -> Result<Zeta7Approximant, hs_types::HankelError> {
    let moments = zeta7_moments(moment_count);
    let pair = ferguson_pair_at(&moments, n)?;
    Ok(Zeta7Approximant { index: n, p: pair.p, q: pair.q })
}
