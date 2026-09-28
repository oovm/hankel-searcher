use hs_problems::{export_zeta3_certificates, zeta3};
use num_bigint::BigInt;
use num_rational::Ratio;

#[test]
fn zeta3_n0_matches_ferguson_table() {
    let approx = zeta3::ferguson_approximant(0, 8).unwrap();
    assert_eq!(approx.p / approx.q, Ratio::new(BigInt::from(8), BigInt::from(7)));
}

#[test]
fn zeta3_n1_matches_ferguson_table() {
    let approx = zeta3::ferguson_approximant(1, 10).unwrap();
    assert_eq!(approx.p / approx.q, Ratio::new(BigInt::from(4887), BigInt::from(4105)));
}

#[test]
fn zeta3_shifted_n0_shift1_differs_from_unshifted() {
    let base = zeta3::ferguson_approximant(0, 12).unwrap();
    let shifted = zeta3::ferguson_approximant_shifted(0, 1, 12).unwrap();
    assert_ne!(base.p / base.q, shifted.p / shifted.q);
}

#[test]
fn zeta_ferguson_shifted_matches_module_wrapper() {
    let direct = hs_problems::zeta_ferguson_shifted(3, 1, 2, 16).unwrap();
    let wrapped = zeta3::ferguson_approximant_shifted(1, 2, 16).unwrap();
    assert_eq!(direct.p, wrapped.p);
    assert_eq!(direct.q, wrapped.q);
}

#[test]
fn export_zeta3_certificates_writes_file() {
    let path = export_zeta3_certificates(4).unwrap();
    assert!(path.exists());
    let text = std::fs::read_to_string(path).unwrap();
    assert!(text.contains("namespace Problems.Zeta3"));
    assert!(text.contains("4887"));
}

#[test]
fn export_zeta7_certificates_writes_file() {
    let path = hs_problems::export_zeta7_certificates(2).unwrap();
    assert!(path.exists());
    let text = std::fs::read_to_string(path).unwrap();
    assert!(text.contains("namespace Problems.Zeta7"));
}
