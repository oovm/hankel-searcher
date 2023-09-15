use thiserror::Error;

/// Errors from `hs-problems` certificate export.
#[derive(Debug, Error)]
pub enum HsProblemsError {
    /// Hankel/Ferguson construction failed upstream.
    #[error(transparent)]
    Hankel(#[from] hs_types::HankelError),
    /// Lean artifact export failed.
    #[error(transparent)]
    Lean(#[from] hs_lean_helper::LeanHelperError),
}
