use hs_lean_helper::{CertificateExport, certificates_from_range, render_certificates, FergusonCertificate};
use hs_types::Approximant;
use num_bigint::BigInt;
use num_rational::Ratio;

#[test]
fn scaled_integers_clear_denominators() {
    let p = Ratio::new(BigInt::from(4), BigInt::from(1));
    let q = Ratio::new(BigInt::from(3), BigInt::from(1));
    let cert = FergusonCertificate::from_approximant(&Approximant {
        p,
        q,
        index: 0,
    });
    assert_eq!(cert.scaled.int_p, BigInt::from(4));
    assert_eq!(cert.scaled.int_q, BigInt::from(3));
    assert_eq!(cert.scaled.scale, BigInt::from(1));
}

fn sample_moments() -> hs_types::RationalMomentSequence {
    hs_types::RationalMomentSequence::from_positive_moments(vec![
        Ratio::from_integer(BigInt::from(1)),
        Ratio::new(BigInt::from(3), BigInt::from(4)),
        Ratio::new(BigInt::from(11), BigInt::from(18)),
        Ratio::new(BigInt::from(25), BigInt::from(36)),
    ])
}

#[test]
fn render_contains_namespace_and_rows() {
    let moments = sample_moments();
    let rows = certificates_from_range(&moments, 0, 0).unwrap();
    let rendered = render_certificates(&CertificateExport::zeta2_default(), &rows);
    assert!(rendered.contains("namespace Zeta2"));
    assert!(rendered.contains("4"));
    assert!(rendered.contains("theorem certificates_nonempty"));
}

#[test]
fn export_range_writes_file() {
    let moments = sample_moments();
    let path = std::env::temp_dir().join("hs-lean-helper-certificates.lean");
    hs_lean_helper::export_range(
        &path,
        &CertificateExport::zeta2_default(),
        &moments,
        0,
        0,
    )
    .unwrap();
    let text = std::fs::read_to_string(&path).unwrap();
    assert!(text.contains("def certificates"));
    let _ = std::fs::remove_file(path);
}
