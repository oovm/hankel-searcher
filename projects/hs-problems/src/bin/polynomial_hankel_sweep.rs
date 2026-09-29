//! Offline sweep of `(N, q)` at fixed `K=40n` for polynomial Hankel `log P_K(ζ(s))`.
//!
//! ```bash
//! cargo run --release -p hs-problems --features offline-golden --bin polynomial-hankel-sweep
//! cargo run --release -p hs-problems --features offline-golden --bin polynomial-hankel-sweep -- --orders 2,3 --n-max-n 6
//! ```

use hs_problems::{
    best_sweep_rows_by_order, init_progress_tracing, sweep_polynomial_hankel, PolynomialHankelSweepConfig,
};
use std::env;

fn main() {
    init_progress_tracing();
    let config = parse_config();
    println!(
        "polynomial Hankel sweep at n={}, K={}n, N={}..{}n, q={:?}, orders={:?}",
        config.n,
        config.k_per_n,
        config.capital_n_min,
        config.capital_n_max,
        config.q_values,
        config.orders,
    );
    println!("{:<4} {:<3} {:<4} {:<6} {:>12} {:>12}", "s", "N", "q", "h", "logP", "logP/n^2");

    let rows = sweep_polynomial_hankel(&config);
    for row in &rows {
        let capital_n_per_n = row.capital_n / row.n;
        println!(
            "{:<4} {:<3} {:<4} {:<6} {:>12.3} {:>12.3}",
            row.order,
            capital_n_per_n,
            row.q,
            row.h,
            row.log_primitive_at_zeta,
            row.log_primitive_per_n2,
        );
    }

    println!("\nbest per order (min logP/n^2):");
    let best = best_sweep_rows_by_order(&rows);
    for row in &best {
        let capital_n_per_n = row.capital_n / row.n;
        println!(
            "s={} best: N={}n q={} h={} logP/n^2={:.3} max_bits={}",
            row.order,
            capital_n_per_n,
            row.q,
            row.h,
            row.log_primitive_per_n2,
            row.max_primitive_coeff_bits,
        );
    }

    if let Ok(path) = env::var("POLY_HANKEL_SWEEP_JSON") {
        let json = serde_json::to_string_pretty(&best).expect("serialize best rows");
        std::fs::write(&path, json).expect("write sweep json");
        println!("wrote best rows to {path}");
    }
}

fn parse_config() -> PolynomialHankelSweepConfig {
    let mut config = PolynomialHankelSweepConfig::default();
    let args: Vec<String> = env::args().skip(1).collect();
    let mut index = 0usize;
    while index < args.len() {
        match args[index].as_str() {
            "--orders" => {
                index += 1;
                config.orders = parse_orders(args.get(index).expect("--orders value"));
            }
            "--n-per-n-max" => {
                index += 1;
                config.capital_n_max = args[index].parse().expect("n-per-n-max");
            }
            "--n-per-n-min" => {
                index += 1;
                config.capital_n_min = args[index].parse().expect("n-per-n-min");
            }
            "--q" => {
                index += 1;
                config.q_values = parse_q_values(args.get(index).expect("--q value"));
            }
            "--n" => {
                index += 1;
                config.n = args[index].parse().expect("--n");
            }
            flag => {
                eprintln!("unknown flag: {flag}");
                std::process::exit(2);
            }
        }
        index += 1;
    }
    config
}

fn parse_orders(raw: &str) -> &'static [u32] {
    match raw {
        "2" => &[2],
        "3" => &[3],
        "5" => &[5],
        "2,3" => &[2, 3],
        "2,3,5" => &[2, 3, 5],
        other => {
            eprintln!("unsupported --orders `{other}` (use 2, 3, 5, or comma-separated)");
            std::process::exit(2);
        }
    }
}

fn parse_q_values(raw: &str) -> &'static [usize] {
    match raw {
        "4" => &[4],
        "6" => &[6],
        "8" => &[8],
        "4,6" => &[4, 6],
        "4,6,8" => &[4, 6, 8],
        other => {
            eprintln!("unsupported --q `{other}`");
            std::process::exit(2);
        }
    }
}
