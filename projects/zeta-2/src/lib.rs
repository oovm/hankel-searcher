#![deny(missing_debug_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("readme.md")]

mod errors;
mod export;
mod ferguson;

pub use crate::{
    errors::{Zeta2Error, Zeta2Result},
    export::{export_lean_certificates, render_zeta2_certificates, zeta2_certificate_table},
    ferguson::{ferguson_approximant, zeta2_moments, Zeta2Approximant},
};

pub use hs_lean_helper::{FergusonCertificate, ScaledIntegers};
