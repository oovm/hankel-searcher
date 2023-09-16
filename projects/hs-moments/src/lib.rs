#![deny(missing_debug_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("readme.md")]

mod beta;
mod catalan;
mod delta;
mod gamma;
mod prevost;
mod rational;
mod zeta;

pub use crate::{
    beta::{beta_moment, beta_moments, beta_moments_f64},
    catalan::{
        catalan_beta_moments, catalan_moment, catalan_moments, catalan_moments_f64,
        catalan_odd_denominator_moment, CATALAN_G,
    },
    delta::{delta_moment, delta_moments, delta_moments_f64, EULER_GOMPERTZ},
    gamma::{gamma_moment, gamma_moments, gamma_moments_f64, EULER_MASCHERONI},
    prevost::{prevost_weight, PrevostWeight, WeightFamily},
    rational::{binomial, factorial, ratio},
    zeta::{zeta_moment, zeta_moments, zeta_moments_f64, ZETA_2, ZETA_3, ZETA_5},
};
