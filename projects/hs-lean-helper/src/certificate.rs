use crate::rational::lcm_ratio_denoms;
use hs_types::{Approximant, RationalMomentSequence, ferguson_pair_at};
use num_bigint::BigInt;
use num_rational::Ratio;

/// Integer-scaled Ferguson data used in Lean linear-form arguments.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScaledIntegers {
    /// `lcm(denom P, denom Q)`.
    pub scale: BigInt,
    /// `scale * P`.
    pub int_p: BigInt,
    /// `scale * Q`.
    pub int_q: BigInt,
}

/// One Ferguson row exported to Lean.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FergusonCertificate {
    /// Ferguson index `n`.
    pub n: usize,
    /// Numerator `P_n`.
    pub p: Ratio<BigInt>,
    /// Denominator `Q_n`.
    pub q: Ratio<BigInt>,
    /// Integer clearing denominators for Lean.
    pub scaled: ScaledIntegers,
}

impl FergusonCertificate {
    /// Build a certificate from a Ferguson approximant.
    pub fn from_approximant(approx: &Approximant<Ratio<BigInt>>) -> Self {
        // 用 P/Q 分母的最小公倍数清分母，供 Lean 整数线性型使用
        let scale = lcm_ratio_denoms(&approx.p, &approx.q);
        let int_p = (&approx.p * Ratio::from_integer(scale.clone())).numer().clone();
        let int_q = (&approx.q * Ratio::from_integer(scale.clone())).numer().clone();
        Self {
            n: approx.index,
            p: approx.p.clone(),
            q: approx.q.clone(),
            scaled: ScaledIntegers {
                scale,
                int_p,
                int_q,
            },
        }
    }
}

/// Build certificates for every `n` in `start..=end`.
pub fn certificates_from_range(
    moments: &RationalMomentSequence,
    start: usize,
    end: usize,
) -> Result<Vec<FergusonCertificate>, hs_types::HankelError> {
    // 与 `ferguson_pair_at` 对 `n=end` 的矩长度要求一致
    let need = 2 * (end + 2);
    if moments.len() < need {
        return Err(hs_types::HankelError::SequenceTooShort {
            need,
            got: moments.len(),
        });
    }
    let mut rows = Vec::with_capacity(end.saturating_sub(start) + 1);
    for n in start..=end {
        let approx = ferguson_pair_at(moments, n)?;
        rows.push(FergusonCertificate::from_approximant(&approx));
    }
    Ok(rows)
}
