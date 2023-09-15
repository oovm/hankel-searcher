use hs_problems::{export_catalan_certificates, export_delta_certificates, export_gamma_certificates};

#[test]
fn export_delta_certificates_writes_file() {
    let path = export_delta_certificates(4).unwrap();
    assert!(path.exists());
    let text = std::fs::read_to_string(path).unwrap();
    assert!(text.contains("namespace Delta"));
    assert!(text.contains("def certificates"));
}

#[test]
fn export_gamma_certificates_writes_file() {
    let path = export_gamma_certificates(4).unwrap();
    assert!(path.exists());
    let text = std::fs::read_to_string(path).unwrap();
    assert!(text.contains("namespace Gamma"));
}

#[test]
fn export_catalan_certificates_writes_file() {
    let path = export_catalan_certificates(4).unwrap();
    assert!(path.exists());
    let text = std::fs::read_to_string(path).unwrap();
    assert!(text.contains("namespace Catalan"));
}
