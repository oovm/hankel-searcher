use hs_moments::zeta_moments;
use hs_types::{Rational, RationalMomentSequence, ferguson_pair_at};

/// One Ferguson convergent for `ζ(3)`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Zeta3Approximant {
    /// Index `n`.
    pub index: usize,
    /// Numerator `P_n`.
    pub p: Rational,
    /// Denominator `Q_n`.
    pub q: Rational,
}

/// Build the first `count` Ferguson moments for `ζ(3)`.
pub fn zeta3_moments(count: usize) -> RationalMomentSequence {
    zeta_moments(3, count)
}

/// Compute `(P_n, Q_n)` for `ζ(3)` at index `n`.
pub fn ferguson_approximant(n: usize, moment_count: usize) -> Result<Zeta3Approximant, hs_types::HankelError> {
    let moments = zeta3_moments(moment_count);
    let pair = ferguson_pair_at(&moments, n)?;
    Ok(Zeta3Approximant { index: n, p: pair.p, q: pair.q })
}

/// Compute `(P_n, Q_n)` for `ζ(3)` at index `n` with moment shift `shift`.
pub fn ferguson_approximant_shifted(
    n: usize,
    shift: usize,
    moment_count: usize,
) -> Result<Zeta3Approximant, hs_types::HankelError> {
    let moments = zeta3_moments(moment_count);
    let pair = hs_types::ferguson_pair_at_shifted(&moments, shift, n)?;
    Ok(Zeta3Approximant { index: n, p: pair.p, q: pair.q })
}
