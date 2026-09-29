use hs_problems::{zeta3_delta_degree, zeta3_delta_leading_coeff, zeta3_delta_polynomial, zeta3_entries, zeta3_hankel_a, zeta3_log_s_k};
use hs_types::is_zero;

#[test]
fn entries_length_matches_optimal_scaling() {
    let entries = zeta3_entries(1).expect("zeta3 entries");
    assert_eq!(entries.order, 3);
    assert_eq!(entries.params.k, 40);
    assert_eq!(entries.params.capital_n, 3);
    assert_eq!(entries.params.q, 4);
    assert_eq!(entries.params.h, 37);
    assert_eq!(entries.a.len(), 2 * entries.params.h - 1);
    assert_eq!(entries.b.len(), entries.a.len());
}

#[test]
fn hankel_matrix_has_optimal_dimensions_for_n1() {
    let entries = zeta3_entries(1).expect("zeta3 entries");
    let h = entries.params.h;
    let a = zeta3_hankel_a(&entries);
    assert_eq!(a.len(), h);
    assert_eq!(a[0].len(), h);
}

#[test]
fn leading_coeff_formula_is_nonzero_for_n1() {
    let entries = zeta3_entries(1).expect("zeta3 entries");
    assert_eq!(zeta3_delta_degree(&entries), entries.params.h);
    assert!(!is_zero(&zeta3_delta_leading_coeff(&entries)));
}

#[test]
fn log_s_k_matches_optimal_scaling_at_n1() {
    let entries = zeta3_entries(1).expect("zeta3 entries");
    let log_s = zeta3_log_s_k(&entries.params);
    assert!((log_s - 60.861).abs() < 0.05, "log S_K should match optimal N=3n q=4 scaling");
}

#[test]
fn delta_polynomial_has_expected_degree_for_n1() {
    let entries = zeta3_entries(1).expect("zeta3 entries");
    let delta = zeta3_delta_polynomial(&entries).expect("delta");
    let degree = entries.params.h;
    assert_eq!(delta.len(), degree + 1);
    assert!(!is_zero(&delta[degree]), "Δ_K leading coefficient must be nonzero");
}
