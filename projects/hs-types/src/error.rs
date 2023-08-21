use thiserror::Error;

/// Errors raised while building or evaluating Hankel data.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum HankelError {
    /// The supplied moment sequence is too short for the requested index.
    #[error("moment sequence too short: need at least {need} terms, got {got}")]
    SequenceTooShort {
        /// Required number of moments.
        need: usize,
        /// Available number of moments.
        got: usize,
    },
    /// A Hankel minor has zero determinant, so the Ferguson quotient is undefined.
    #[error("singular Hankel minor at index {index}")]
    SingularMinor {
        /// Index of the undefined approximant.
        index: usize,
    },
    /// The convergent denominator vanished.
    #[error("zero denominator in approximant at index {index}")]
    ZeroDenominator {
        /// Index of the undefined approximant.
        index: usize,
    },
}
