use hs_moments::zeta_moments;
use hs_types::{ferguson_pair_at, RationalMomentSequence};
use num_rational::Ratio;
use num_bigint::BigInt;

/// One Ferguson convergent for `ζ(2)`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Zeta2Approximant {
    /// Index `n`.
    pub index: usize,
    /// Numerator `P_n`.
    pub p: Ratio<BigInt>,
    /// Denominator `Q_n`.
    pub q: Ratio<BigInt>,
}

/// Build the first `count` Ferguson moments for `ζ(2)`.
pub fn zeta2_moments(count: usize) -> RationalMomentSequence {
    zeta_moments(2, count)
}

/// Compute `(P_n, Q_n)` for `ζ(2)` at index `n`.
pub fn ferguson_approximant(n: usize, moment_count: usize) -> Result<Zeta2Approximant, hs_types::HankelError> {
    let moments = zeta2_moments(moment_count);
    let pair = ferguson_pair_at(&moments, n)?;
    Ok(Zeta2Approximant {
        index: n,
        p: pair.p,
        q: pair.q,
    })
}
