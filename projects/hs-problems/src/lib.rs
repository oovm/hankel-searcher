#![deny(missing_debug_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("readme.md")]

mod error;
mod export;
/// Ferguson approximants and moments for `ζ(2)`.
pub mod zeta2;
/// Ferguson approximants and moments for `ζ(3)`.
pub mod zeta3;
/// Ferguson approximants and moments for `ζ(5)`.
pub mod zeta5;

pub use crate::error::HsProblemsError;
pub use crate::export::{
    export_catalan_certificates, export_delta_certificates, export_gamma_certificates, export_zeta2_certificates,
    export_zeta3_certificates, export_zeta5_certificates,
};
