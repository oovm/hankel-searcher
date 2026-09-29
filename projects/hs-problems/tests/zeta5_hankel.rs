use hs_problems::zeta5_hankel::{
    zeta5_delta_degree, zeta5_delta_leading_coeff, zeta5_entries, zeta5_hankel_a, zeta5_hankel_b,
};
use hs_types::is_zero;

#[test]
fn entries_length_matches_paper_scaling() {
    let entries = zeta5_entries(1);
    assert_eq!(entries.params.k, 40);
    assert_eq!(entries.params.capital_n, 3);
    assert_eq!(entries.params.h, 37);
    assert_eq!(entries.a.len(), 2 * entries.params.h - 1);
    assert_eq!(entries.b.len(), entries.a.len());
}

#[test]
fn hankel_matrices_have_paper_dimensions_for_n1() {
    let entries = zeta5_entries(1);
    let h = entries.params.h;
    let a = zeta5_hankel_a(&entries);
    let b = zeta5_hankel_b(&entries);
    assert_eq!(a.len(), h);
    assert_eq!(a[0].len(), h);
    assert_eq!(b.len(), h);
    assert_eq!(b[0].len(), h);
}

#[test]
fn leading_coeff_formula_is_nonzero_for_n1() {
    let entries = zeta5_entries(1);
    assert_eq!(zeta5_delta_degree(&entries), entries.params.h);
    assert!(!is_zero(&zeta5_delta_leading_coeff(&entries)));
}
