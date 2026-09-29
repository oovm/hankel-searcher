//! Offline polynomial Hankel energy probe for `ζ(2)` at `n=1`.
//!
//! ```bash
//! cargo run --release -p hs-problems --features offline-golden --bin zeta2-hankel-golden
//! ```

use hs_problems::{init_progress_tracing, zeta2_polynomial_golden_full};

fn main() {
    init_progress_tracing();
    match zeta2_polynomial_golden_full() {
        Ok(energy) => {
            println!("zeta2 polynomial Hankel golden n=1 (full): ok");
            println!("  max |coeff P_K| bits = {}", energy.max_primitive_coeff_bits);
            println!("  log S_K = {:.3}", energy.log_s_k);
            println!("  log Delta_K(zeta2) = {:.3}", energy.log_delta_at_zeta);
            println!("  log P_K(zeta2) / n^2 = {:.3}", energy.log_primitive_at_zeta);
        }
        Err(error) => {
            eprintln!("zeta2-hankel-golden: {error}");
            std::process::exit(1);
        }
    }
}
