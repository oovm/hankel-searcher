use hs_problems::zeta3_polynomial_golden_fast;

#[test]
fn polynomial_golden_fast_n1() {
    zeta3_polynomial_golden_fast().expect("fast golden");
}
