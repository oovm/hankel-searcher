//! Offline sweep of `(N, q)` at fixed `K=40n` for polynomial Hankel `log P_K(ζ(s))`.
//!
//! ```bash
//! cargo run --release -p hs-problems --features offline-golden --bin polynomial-hankel-sweep
//! ```

use hs_problems::{
    init_progress_tracing, polynomial_hankel_params, zeta_polynomial_delta, zeta_polynomial_entries_with_params,
    zeta_polynomial_energy_report,
};

fn main() {
    init_progress_tracing();
    let n = 1usize;
    let k_per_n = 40usize;
    println!("polynomial Hankel sweep at n={n}, K={k_per_n}n (offline full Δ_K per row)");
    println!("{:<4} {:<3} {:<4} {:<6} {:>12} {:>12}", "s", "N", "q", "h", "logP", "logP/n^2");
    for order in [2u32, 3u32, 5u32] {
        for capital_n_per_n in 1..=8 {
            for q in [4usize, 6, 8] {
                let params = match polynomial_hankel_params(n, k_per_n, capital_n_per_n, q) {
                    Ok(params) => params,
                    Err(_) => continue,
                };
                let entries = match zeta_polynomial_entries_with_params(order, params) {
                    Ok(entries) => entries,
                    Err(error) => {
                        eprintln!("s={order} N={capital_n_per_n}n q={q}: entries failed: {error}");
                        continue;
                    }
                };
                let delta = match zeta_polynomial_delta(&entries) {
                    Ok(delta) => delta,
                    Err(error) => {
                        eprintln!("s={order} N={capital_n_per_n}n q={q}: delta failed: {error}");
                        continue;
                    }
                };
                let energy = match zeta_polynomial_energy_report(order, &params, &delta) {
                    Ok(energy) => energy,
                    Err(error) => {
                        eprintln!("s={order} N={capital_n_per_n}n q={q}: energy failed: {error}");
                        continue;
                    }
                };
                println!(
                    "{:<4} {:<3} {:<4} {:<6} {:>12.3} {:>12.3}",
                    order,
                    capital_n_per_n,
                    q,
                    params.h,
                    energy.log_primitive_at_zeta,
                    energy.log_primitive_at_zeta / (n * n) as f64,
                );
            }
        }
    }
}
