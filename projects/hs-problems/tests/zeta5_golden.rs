use hs_problems::{zeta5_polynomial_golden_fast, zeta5_polynomial_golden_full};

#[test]
fn polynomial_golden_fast_n1() {
    zeta5_polynomial_golden_fast().expect("fast golden");
}

/// Offline gate: exact `Δ_K` + energy logs vs `mo271/Zeta5` `hankel.py` at `n=1`.
///
/// `cargo test -p hs-problems polynomial_golden_full --release --features offline-golden -- --ignored`
#[test]
#[ignore = "exact Δ_K at n=1 takes tens of minutes in release, not for CI"]
fn polynomial_golden_full_n1() {
    zeta5_polynomial_golden_full().expect("full golden");
}
