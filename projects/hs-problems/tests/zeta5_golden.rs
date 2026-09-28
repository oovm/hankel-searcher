use hs_problems::zeta5_polynomial_golden_fast;

#[test]
fn polynomial_golden_fast_n1() {
    zeta5_polynomial_golden_fast().expect("fast golden");
}
