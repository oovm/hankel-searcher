use crate::rational::lcm_ratio_denoms;
use hs_types::{Approximant, Integer, Rational, RationalMomentSequence, ferguson_pair_at};

/// Integer-scaled Ferguson data used in Lean linear-form arguments.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScaledIntegers {
    /// `lcm(denom P, denom Q)`.
    pub scale: Integer,
    /// `scale * P`.
    pub int_p: Integer,
    /// `scale * Q`.
    pub int_q: Integer,
}

/// One Ferguson row exported to Lean.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FergusonCertificate {
    /// Ferguson index `n`.
    pub n: usize,
    /// Numerator `P_n`.
    pub p: Rational,
    /// Denominator `Q_n`.
    pub q: Rational,
    /// Integer clearing denominators for Lean.
    pub scaled: ScaledIntegers,
}

impl FergusonCertificate {
    /// Build a certificate from a Ferguson approximant.
    pub fn from_approximant(approx: &Approximant<Rational>) -> Self {
        let scale = lcm_ratio_denoms(&approx.p, &approx.q);
        let int_p = Integer::from((approx.p.clone() * Rational::from(scale.clone())).to_numerator().clone());
        let int_q = Integer::from((approx.q.clone() * Rational::from(scale.clone())).to_numerator().clone());
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
