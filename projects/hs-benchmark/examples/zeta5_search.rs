use hs_benchmark::{energy_rate_curve, zeta5_convergence_ladder, ZETA5_DEFAULT_MAX_N};
use hs_moments::zeta_moments_f64;
use std::env;

fn main() {
    let max_n = env::var("ZETA5_MAX_N")
        .ok()
        .and_then(|raw| raw.parse().ok())
        .unwrap_or(ZETA5_DEFAULT_MAX_N);

    println!("ζ(5) Ferguson search ladder (max_n={})", max_n);
    for row in zeta5_convergence_ladder(max_n).expect("ladder") {
        let energy = row
            .energy_rate
            .map(|e| format!("{:.4}", e))
            .unwrap_or_else(|| "-".into());
        println!(
            "n={:>2}  approx={:.12}  gap={:.3e}  energy={}",
            row.n,
            row.approximant,
            row.gap,
            energy
        );
    }

    let energy_cap = max_n.min(20);
    println!("\nEnergy-rate tail (Hankel log-det heuristic, n<= {})", energy_cap);
    let float_moments = zeta_moments_f64(5, 2 * (energy_cap + 2));
    for (n, rate) in energy_rate_curve(&float_moments, energy_cap) {
        println!("n={:>2}  -log|H|/n^2 = {:.4}", n, rate);
    }
}
