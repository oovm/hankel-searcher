use hs_problems::{zeta5_bigdecimal, zeta5_log_s_k, zeta5_paper_params};
use hs_checkpoint::zeta_series_bounds;
use num_traits::ToPrimitive;

#[test]
fn log_s_k_matches_hankel_py_for_n1() {
    let params = zeta5_paper_params(1);
    let log_s = zeta5_log_s_k(&params);
    assert!((log_s - (-204.319)).abs() < 0.01);
}

#[test]
fn zeta5_bigdecimal_lies_in_rigorous_series_enclosure() {
    let terms = 5_000usize;
    let (lower, upper) = zeta_series_bounds(5, terms).unwrap();
    let value = zeta5_bigdecimal(terms).unwrap();
    let value = value.to_f64().expect("zeta5 fits in f64 head");
    let lower = lower.to_f64().expect("lower fits in f64");
    let upper = upper.to_f64().expect("upper fits in f64");
    assert!(value >= lower && value <= upper);
}
