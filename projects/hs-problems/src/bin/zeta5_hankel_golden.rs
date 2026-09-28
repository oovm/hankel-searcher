//! Offline **full** golden compare for `n=1` against `mo271/Zeta5` `scripts/hankel.py`.
//!
//! Not for CI. Expect tens of minutes in release:
//! `cargo run --release -p hs-problems --features offline-golden --bin zeta5-hankel-golden`

use hs_problems::zeta5_polynomial_golden_full;

fn main() {
    if let Err(error) = run() {
        eprintln!("zeta5-hankel-golden: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    eprintln!("warning: full Δ_K golden — offline only, not for CI");
    let energy = zeta5_polynomial_golden_full()?;
    println!("zeta5 polynomial Hankel golden n=1 (full): ok");
    println!("  leading_coeff (2.9): ok");
    println!(
        "  max |coeff P_K| bits = {} (expected 7597)",
        energy.max_primitive_coeff_bits
    );
    println!("  log S_K = {:.3}", energy.log_s_k);
    println!("  log Delta_K(zeta5) = {:.3}", energy.log_delta_at_zeta5);
    println!("  log P_K(zeta5) / n^2 = {:.3}", energy.log_primitive_at_zeta5);
    Ok(())
}
