use hs_problems::{zeta5_log_s_k, zeta5_paper_params};

#[test]
fn log_s_k_matches_hankel_py_for_n1() {
    let params = zeta5_paper_params(1);
    let log_s = zeta5_log_s_k(&params);
    assert!((log_s - (-204.319)).abs() < 0.01);
}
