use num_bigint::BigInt;

use num_rational::Ratio;

use zeta_5::{export_lean_certificates, ferguson_approximant};



#[test]

fn zeta5_n0_matches_ferguson_bose_kernel() {

    let approx = ferguson_approximant(0, 8).unwrap();

    assert_eq!(approx.p / approx.q, Ratio::new(BigInt::from(32), BigInt::from(31)));

}



#[test]

fn export_lean_certificates_writes_file() {

    let path = export_lean_certificates(4).unwrap();

    assert!(path.exists());

    let text = std::fs::read_to_string(path).unwrap();

    assert!(text.contains("namespace Zeta5"));

    assert!(text.contains("32"));

}


