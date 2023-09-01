#![deny(missing_debug_implementations)]

#![warn(missing_docs, rustdoc::missing_crate_level_docs)]

#![doc = include_str!("../readme.md")]



mod errors;

mod export;

mod ferguson;



pub use crate::{

    errors::{Zeta5Error, Zeta5Result},

    export::{export_lean_certificates, render_zeta5_certificates, zeta5_certificate_table},

    ferguson::{ferguson_approximant, zeta5_moments, Zeta5Approximant},

};



pub use hs_lean_helper::{FergusonCertificate, ScaledIntegers};


