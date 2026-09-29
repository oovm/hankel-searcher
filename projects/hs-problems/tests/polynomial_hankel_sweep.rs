use hs_problems::{best_sweep_rows_by_order, PolynomialHankelSweepRow};

fn sample_row(order: u32, capital_n: usize, q: usize, log_primitive_per_n2: f64) -> PolynomialHankelSweepRow {
    PolynomialHankelSweepRow {
        order,
        n: 1,
        k: 40,
        capital_n,
        q,
        h: 40 - capital_n,
        log_primitive_at_zeta: log_primitive_per_n2,
        log_primitive_per_n2,
        log_delta_at_zeta: 0.0,
        log_s_k: 0.0,
        max_primitive_coeff_bits: 0,
    }
}

#[test]
fn best_sweep_rows_picks_most_negative_log_primitive_per_n2() {
    let rows = vec![
        sample_row(2, 3, 6, -100.0),
        sample_row(2, 2, 6, -200.0),
        sample_row(3, 3, 6, -50.0),
        sample_row(3, 1, 4, -150.0),
    ];
    let best = best_sweep_rows_by_order(&rows);
    assert_eq!(best.len(), 2);
    assert_eq!(best[0].order, 2);
    assert_eq!(best[0].capital_n, 2);
    assert_eq!(best[1].order, 3);
    assert_eq!(best[1].capital_n, 1);
    assert!((best[0].log_primitive_per_n2 + 200.0).abs() < 1e-9);
}
