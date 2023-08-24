use thiserror::Error;

/// Errors raised while exporting Lean artifacts.
#[derive(Debug, Error)]
pub enum LeanHelperError {
    /// Hankel/Ferguson construction failed upstream.
    #[error(transparent)]
    Hankel(#[from] hs_types::HankelError),
    /// File IO failed while writing Lean output.
    #[error("failed to write Lean artifact: {0}")]
    Io(#[from] std::io::Error),
}
