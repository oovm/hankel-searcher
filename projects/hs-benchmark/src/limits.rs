//! Explicit CI budgets for exact rational Hankel regression.
//!
//! γ moments contain `(i+1)^{n+1}` denominators and become impractical past
//! [`GAMMA_REGRESSION_MAX_N`]. Deep Ferguson table rows remain in
//! `ferguson_tables.rs` for manual `cargo test -- --ignored` runs.

/// Default Ferguson index cap for `ζ(5)` exploratory search (exact rational Hankel).
pub const ZETA5_DEFAULT_MAX_N: usize = 15;

/// γ moments carry `(i+1)^{n+1}` denominators; exact Hankel blows up past this index.
pub const GAMMA_REGRESSION_MAX_N: usize = 12;

/// δ / ζ Ferguson table rows checked in default `cargo test`.
pub const ZETA_REGRESSION_MAX_N: usize = 15;

/// δ table rows checked in default `cargo test` (tail gap checks are manual).
pub const DELTA_REGRESSION_MAX_N: usize = 15;
