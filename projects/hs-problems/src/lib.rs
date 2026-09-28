#![deny(missing_debug_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("readme.md")]

mod error;
mod export;
mod ferguson;
/// Whole-polynomial Hankel construction for `ζ(5)`.
pub mod zeta5_polynomial;
/// Gram/Hankel determinant pipeline for `ζ(5)`.
pub mod zeta5_hankel;
/// Energy logs at `ζ(5)` for polynomial Hankel diagnostics.
pub mod zeta5_energy;
/// Golden expectations for `hankel.py` regression at `n=1`.
pub mod zeta5_golden;
/// Ferguson approximants and moments for `ζ(2)`.
pub mod zeta2;
/// Ferguson approximants and moments for `ζ(3)`.
pub mod zeta3;
/// Ferguson approximants and moments for `ζ(5)`.
pub mod zeta5;
/// Ferguson approximants and moments for `ζ(7)`.
pub mod zeta7;

pub use crate::error::HsProblemsError;
pub use crate::ferguson::{ShiftedApproximant, zeta_ferguson_shifted};
pub use crate::zeta5_polynomial::{Zeta5PaperParams, d_polynomial, d_polynomial_degree, evaluate_polynomial, zeta5_paper_params};
pub use crate::zeta5_hankel::{
    Zeta5Entries, zeta5_delta_degree, zeta5_delta_leading_coeff, zeta5_delta_polynomial, zeta5_entries,
    zeta5_hankel_a, zeta5_hankel_b,
};
pub use crate::zeta5_energy::{
    Zeta5DeltaPrimitive, Zeta5EnergyReport, zeta5_delta_primitive, zeta5_energy_report, zeta5_log_s_k,
};
pub use crate::zeta5_golden::{Zeta5GoldenN1, zeta5_polynomial_golden_check};
pub use crate::export::{
    export_catalan_certificates, export_delta_certificates, export_gamma_certificates, export_zeta2_certificates,
    export_zeta3_certificates, export_zeta5_certificates, export_zeta7_certificates,
};
