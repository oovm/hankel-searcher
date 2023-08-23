#![deny(missing_debug_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]

mod ferguson_tables;
mod metrics;

pub use crate::{
    ferguson_tables::{
        delta_table, gamma_table, zeta2_table, zeta3_table, FergusonReference,
    },
    metrics::{convergence_gap, energy_rate_curve, ConvergenceReport},
};
