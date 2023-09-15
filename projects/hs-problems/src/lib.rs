#![deny(missing_debug_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("readme.md")]

mod error;
mod export;

pub use crate::error::HsProblemsError;
pub use crate::export::{
    export_catalan_certificates, export_delta_certificates, export_gamma_certificates,
};
