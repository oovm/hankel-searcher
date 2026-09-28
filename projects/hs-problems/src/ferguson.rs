use crate::HsProblemsError;
use hs_moments::zeta_moments;
use hs_types::{ferguson_pair_at_shifted, RationalMomentSequence};
use num_bigint::BigInt;
use num_rational::Ratio;

/// One shifted Ferguson convergent for `ζ(k)`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShiftedApproximant {
    /// Index `n`.
    pub index: usize,
    /// Moment shift applied before Hankel construction.
    pub shift: usize,
    /// Numerator `P_n`.
    pub p: Ratio<BigInt>,
    /// Denominator `Q_n`.
    pub q: Ratio<BigInt>,
}

fn zeta_moments_for(order: u32, count: usize) -> RationalMomentSequence {
    zeta_moments(order, count)
}

/// Compute `(P_n, Q_n)` for `ζ(order)` at index `n` with moment shift `shift`.
pub fn zeta_ferguson_shifted(
    order: u32,
    n: usize,
    shift: usize,
    moment_count: usize,
) -> Result<ShiftedApproximant, HsProblemsError> {
    let moments = zeta_moments_for(order, moment_count);
    let pair = ferguson_pair_at_shifted(&moments, shift, n).map_err(HsProblemsError::from)?;
    Ok(ShiftedApproximant {
        index: n,
        shift,
        p: pair.p,
        q: pair.q,
    })
}
