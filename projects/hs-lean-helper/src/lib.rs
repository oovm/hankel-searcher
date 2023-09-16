#![deny(missing_debug_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("readme.md")]

mod certificate;
mod error;
mod export;
mod rational;
mod render;

pub use crate::{
    certificate::{certificates_from_range, FergusonCertificate, ScaledIntegers},
    error::LeanHelperError,
    export::{
        CertificateExport, export_range, lean_prove_certificates_path, write_certificates,
    },
    rational::{lcm_bigint, lcm_ratio_denoms, ratio_to_lean_ints},
    render::render_certificates,
};
