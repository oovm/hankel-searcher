//! Offline polynomial Hankel energy probe for `ζ(3)` at `n=1`.
//!
//! ```bash
//! cargo run --release -p hs-problems --features offline-golden --bin zeta3-hankel-golden
//! ```

use hs_problems::{init_progress_tracing, zeta3_polynomial_golden_full};

fn main() {
    init_progress_tracing();
    match zeta3_polynomial_golden_full() {
        Ok(energy) => {
            println!("zeta3 polynomial Hankel golden n=1 (full): ok");
            println!("  max |coeff P_K| bits = {}", energy.max_primitive_coeff_bits);
            println!("  log S_K = {:.3}", energy.log_s_k);
            println!("  log Delta_K(zeta3) = {:.3}", energy.log_delta_at_zeta);
            println!("  log P_K(zeta3) / n^2 = {:.3}", energy.log_primitive_at_zeta);
        }
        Err(error) => {
            eprintln!("zeta3-hankel-golden: {error}");
            std::process::exit(1);
        }
    }
}
