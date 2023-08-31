use hs_benchmark::{
    zeta5_best_gap, zeta5_convergence_ladder_default, ZETA5_DEFAULT_MAX_N, PRIMARY_TARGET,
};

#[test]
fn primary_target_is_zeta5() {
    assert_eq!(PRIMARY_TARGET, "zeta-5");
}

#[test]
fn zeta5_ladder_reports_positive_gaps() {
    let ladder = zeta5_convergence_ladder_default().expect("ladder");
    assert_eq!(ladder.len(), ZETA5_DEFAULT_MAX_N + 1);
    for row in &ladder {
        assert!(row.gap > 0.0, "n={}", row.n);
        assert!(row.approximant > 0.0);
    }
}

#[test]
fn zeta5_best_gap_within_default_budget() {
    let best = zeta5_best_gap(ZETA5_DEFAULT_MAX_N)
        .expect("best gap")
        .expect("some row");
    assert_eq!(best.n, ZETA5_DEFAULT_MAX_N);
    assert!(best.gap < 1e-6);
}
