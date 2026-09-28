use hs_lean_helper::{CertificateExport, FergusonCertificate, certificates_from_range, render_certificates};
use hs_types::{Approximant, Integer, Rational, RationalMomentSequence};
use malachite::base::num::basic::traits::One;

#[test]
fn scaled_integers_clear_denominators() {
    let p = Rational::from(Integer::from(4));
    let q = Rational::from(Integer::from(3));
    let cert = FergusonCertificate::from_approximant(&Approximant {
        p,
        q,
        index: 0,
    });
    assert_eq!(cert.scaled.int_p, Integer::from(4));
    assert_eq!(cert.scaled.int_q, Integer::from(3));
    assert_eq!(cert.scaled.scale, Integer::ONE);
}

fn sample_moments() -> RationalMomentSequence {
    RationalMomentSequence::from_positive_moments(vec![
        Rational::ONE,
        Rational::from_integers(Integer::from(3), Integer::from(4)),
        Rational::from_integers(Integer::from(11), Integer::from(18)),
        Rational::from_integers(Integer::from(25), Integer::from(36)),
    ])
}

#[test]
fn render_contains_namespace_and_rows() {
    let moments = sample_moments();
    let rows = certificates_from_range(&moments, 0, 0).unwrap();
    let rendered = render_certificates(&CertificateExport::zeta2_default(), &rows);
    assert!(rendered.contains("namespace Problems.Zeta2"));
    assert!(rendered.contains("import LeanProve.Certificate"));
    assert!(rendered.contains("import Problems.Zeta2.Ferguson"));
    assert!(rendered.contains("List LeanProve.FergusonCertificate"));
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
