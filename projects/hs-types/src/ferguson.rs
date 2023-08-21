use crate::{
    determinant::{bareiss_det, hankel_matrix},
    error::HankelError,
    sequence::MomentSequence,
};
use num_rational::Ratio;
use num_traits::{One, ToPrimitive, Zero};

/// A single Ferguson convergent `P_n / Q_n`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Approximant<T> {
    /// Numerator determinant `P_n`.
    pub p: T,
    /// Denominator determinant `Q_n`.
    pub q: T,
    /// Index `n`.
    pub index: usize,
}

impl<T: Clone + Zero + One + std::ops::Neg<Output = T>> Approximant<T>
where
    T: std::ops::Div<Output = T>,
    T: std::ops::Mul<Output = T>,
    T: std::ops::Sub<Output = T>,
{
    /// Return the rational value when the denominator is non-zero.
    pub fn value(&self) -> Result<T, HankelError> {
        if self.q.is_zero() {
            Err(HankelError::ZeroDenominator { index: self.index })
        } else {
            Ok(self.p.clone() / self.q.clone())
        }
    }
}

/// Full Ferguson data for one moment sequence.
#[derive(Debug, Clone)]
pub struct FergusonPair<T> {
    /// All computed convergents.
    pub approximants: Vec<Approximant<T>>,
}

/// Compute every Ferguson pair up to `max_n`.
pub fn ferguson_pair<T>(moments: &MomentSequence<T>, max_n: usize) -> Result<FergusonPair<T>, HankelError>
where
    T: Clone + Zero + One + std::ops::Neg<Output = T>,
    T: std::ops::Div<Output = T>,
    T: std::ops::Mul<Output = T>,
    T: std::ops::Sub<Output = T>,
{
    let mut approximants = Vec::with_capacity(max_n + 1);
    for n in 0..=max_n {
        approximants.push(ferguson_pair_at(moments, n)?);
    }
    Ok(FergusonPair { approximants })
}

/// Compute the Ferguson pair at a single index `n`.
pub fn ferguson_pair_at<T>(moments: &MomentSequence<T>, n: usize) -> Result<Approximant<T>, HankelError>
where
    T: Clone + Zero + One + std::ops::Neg<Output = T>,
    T: std::ops::Div<Output = T>,
    T: std::ops::Mul<Output = T>,
    T: std::ops::Sub<Output = T>,
{
    let p_size = n + 2;
    let q_size = n + 1;
    let need = 2 * p_size;
    if moments.len() < need {
        return Err(HankelError::SequenceTooShort {
            need,
            got: moments.len(),
        });
    }

    let prefix = moments.as_ferguson_prefix(need - 1);
    let p_matrix = hankel_matrix(&prefix, 0, p_size);
    let q_matrix = hankel_matrix(&prefix, 2, q_size);

    let p_det = bareiss_det(&p_matrix);
    let q_det = bareiss_det(&q_matrix);

    if q_det.is_zero() {
        return Err(HankelError::SingularMinor { index: n });
    }

    Ok(Approximant {
        p: -p_det,
        q: q_det,
        index: n,
    })
}

/// Convert a rational approximant to `f64`.
pub fn approximant_to_f64(approx: &Approximant<Ratio<num_bigint::BigInt>>) -> Result<f64, HankelError> {
    let value = approx.value()?;
    Ok(value
        .to_f64()
        .ok_or(HankelError::SingularMinor { index: approx.index })?)
}
