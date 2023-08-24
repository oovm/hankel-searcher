use num_rational::Ratio;
use num_bigint::BigInt;
use zeta_2::{export_lean_certificates, ferguson_approximant};

#[test]
fn zeta2_n0_matches_ferguson_table() {
    let approx = ferguson_approximant(0, 8).unwrap();
    assert_eq!(approx.p / approx.q, Ratio::new(BigInt::from(4), BigInt::from(3)));
}

#[test]
fn zeta2_n1_matches_ferguson_table() {
    let approx = ferguson_approximant(1, 10).unwrap();
    assert_eq!(
        approx.p / approx.q,
        Ratio::new(BigInt::from(135), BigInt::from(89))
    );
}

#[test]
fn export_lean_certificates_writes_file() {
    let path = export_lean_certificates(4).unwrap();
    assert!(path.exists());
    let text = std::fs::read_to_string(path).unwrap();
    assert!(text.contains("def certificates"));
    assert!(text.contains("135"));
}
