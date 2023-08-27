#![deny(missing_debug_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]

mod errors;
mod export;
mod ferguson;

pub use crate::{
    errors::{Zeta3Error, Zeta3Result},
    export::{export_lean_certificates, render_zeta3_certificates, zeta3_certificate_table},
    ferguson::{ferguson_approximant, zeta3_moments, Zeta3Approximant},
};

pub use hs_lean_helper::{FergusonCertificate, ScaledIntegers};
