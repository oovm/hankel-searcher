use thiserror::Error;



/// Errors from the `zeta-5` crate.

#[derive(Debug, Error)]

pub enum Zeta5Error {

    /// Hankel/Ferguson construction failed.

    #[error(transparent)]

    Hankel(#[from] hs_types::HankelError),

    /// Lean export failed.

    #[error(transparent)]

    Lean(#[from] hs_lean_helper::LeanHelperError),

}



/// Convenience result alias.

pub type Zeta5Result<T> = Result<T, Zeta5Error>;


