use hs_moments::zeta_moments;
use hs_types::{RationalMomentSequence, ferguson_pair_at};
use num_bigint::BigInt;
use num_rational::Ratio;

/// One Ferguson convergent for `ζ(5)`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Zeta5Approximant {
    /// Index `n`.
    pub index: usize,
    /// Numerator `P_n`.
    pub p: Ratio<BigInt>,
    /// Denominator `Q_n`.
    pub q: Ratio<BigInt>,
}

/// Build the first `count` Ferguson moments for `ζ(5)`.
pub fn zeta5_moments(count: usize) -> RationalMomentSequence {
    zeta_moments(5, count)
}

/// Compute `(P_n, Q_n)` for `ζ(5)` at index `n`.
pub fn ferguson_approximant(n: usize, moment_count: usize) -> Result<Zeta5Approximant, hs_types::HankelError> {
    let moments = zeta5_moments(moment_count);
    let pair = ferguson_pair_at(&moments, n)?;
    Ok(Zeta5Approximant { index: n, p: pair.p, q: pair.q })
}
