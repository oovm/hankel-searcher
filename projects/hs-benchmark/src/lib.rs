#![deny(missing_debug_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]

mod ferguson_tables;
mod limits;
mod metrics;
mod search;

pub use crate::{
    ferguson_tables::{
        delta_table, gamma_table, zeta2_table, zeta3_table, FergusonReference,
    },
    limits::{
        DELTA_REGRESSION_MAX_N, GAMMA_REGRESSION_MAX_N, ZETA5_DEFAULT_MAX_N, ZETA_REGRESSION_MAX_N,
    },
    metrics::{convergence_gap, energy_rate_curve, ConvergenceReport},
    search::{
        zeta5_best_gap, zeta5_convergence_ladder, zeta5_convergence_ladder_default, PRIMARY_TARGET,
    },
};
