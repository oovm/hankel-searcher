use hs_problems::{zeta2_delta_degree, zeta2_delta_leading_coeff, zeta2_delta_polynomial, zeta2_entries, zeta2_hankel_a, zeta2_log_s_k};
use hs_types::is_zero;

#[test]
fn entries_length_matches_paper_scaling() {
    let entries = zeta2_entries(1).expect("zeta2 entries");
    assert_eq!(entries.order, 2);
    assert_eq!(entries.params.k, 40);
    assert_eq!(entries.params.capital_n, 3);
    assert_eq!(entries.params.h, 37);
    assert_eq!(entries.a.len(), 2 * entries.params.h - 1);
    assert_eq!(entries.b.len(), entries.a.len());
}

#[test]
fn hankel_matrix_has_paper_dimensions_for_n1() {
    let entries = zeta2_entries(1).expect("zeta2 entries");
    let h = entries.params.h;
    let a = zeta2_hankel_a(&entries);
    assert_eq!(a.len(), h);
    assert_eq!(a[0].len(), h);
}

#[test]
fn leading_coeff_formula_is_nonzero_for_n1() {
    let entries = zeta2_entries(1).expect("zeta2 entries");
    assert_eq!(zeta2_delta_degree(&entries), entries.params.h);
    assert!(!is_zero(&zeta2_delta_leading_coeff(&entries)));
}

#[test]
fn log_s_k_matches_zeta5_normalization_at_n1() {
    let entries = zeta2_entries(1).expect("zeta2 entries");
    let log_s = zeta2_log_s_k(&entries.params);
    assert!((log_s + 204.319).abs() < 0.05, "log S_K should match paper scaling");
}

#[test]
fn delta_polynomial_has_expected_degree_for_n1() {
    let entries = zeta2_entries(1).expect("zeta2 entries");
    let delta = zeta2_delta_polynomial(&entries).expect("delta");
    let degree = entries.params.h;
    assert_eq!(delta.len(), degree + 1);
    assert!(!is_zero(&delta[degree]), "Δ_K leading coefficient must be nonzero");
}
