#![deny(missing_debug_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("readme.md")]

mod determinant;
mod error;
mod ferguson;
mod rational_linalg;
mod sequence;

pub use crate::{
    determinant::{
        bareiss_det, estimate_energy_rate, f64_det, hankel_det_f64, hankel_det_rational, hankel_matrix,
        log_abs_hankel_det,
    },
    error::HankelError,
    ferguson::{
        approximant_to_f64, ferguson_pair, ferguson_pair_at, ferguson_pair_at_shifted, Approximant, FergusonPair,
    },
    rational_linalg::{rational_charpoly, rational_mat_inv, rational_mat_mul, scale_polynomial},
    sequence::{MomentSequence, RationalMomentSequence, ShiftedMomentSequence},
};
