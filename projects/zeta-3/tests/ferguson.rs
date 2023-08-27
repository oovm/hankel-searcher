use num_bigint::BigInt;
use num_rational::Ratio;
use zeta_3::{export_lean_certificates, ferguson_approximant};

#[test]
fn zeta3_n0_matches_ferguson_table() {
    let approx = ferguson_approximant(0, 8).unwrap();
    assert_eq!(approx.p / approx.q, Ratio::new(BigInt::from(8), BigInt::from(7)));
}

#[test]
fn zeta3_n1_matches_ferguson_table() {
    let approx = ferguson_approximant(1, 10).unwrap();
    assert_eq!(
        approx.p / approx.q,
        Ratio::new(BigInt::from(4887), BigInt::from(4105))
    );
}

#[test]
fn export_lean_certificates_writes_file() {
    let path = export_lean_certificates(4).unwrap();
    assert!(path.exists());
    let text = std::fs::read_to_string(path).unwrap();
    assert!(text.contains("namespace Zeta3"));
    assert!(text.contains("4887"));
}
