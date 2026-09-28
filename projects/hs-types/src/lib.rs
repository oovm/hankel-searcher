#![deny(missing_debug_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("readme.md")]

mod bignum;
mod determinant;
mod error;
mod ferguson;
mod rational_linalg;
mod rational_poly;
mod sequence;

pub use crate::bignum::{Integer, Natural, Rational, integer_from_str, is_zero, rational_from_str};
pub use crate::{
    determinant::{
        bareiss_det, bareiss_det_rational, estimate_energy_rate, f64_det, hankel_det_f64, hankel_det_rational,
        hankel_matrix,
        log_abs_hankel_det,
    },
    error::HankelError,
    ferguson::{
        approximant_to_f64, ferguson_pair, ferguson_pair_at, ferguson_pair_at_shifted, Approximant, FergusonPair,
    },
    rational_linalg::{rational_charpoly, rational_mat_inv, rational_mat_mul, scale_polynomial},
    rational_poly::det_linear_pencil,
    sequence::{MomentSequence, RationalMomentSequence, ShiftedMomentSequence},
};
