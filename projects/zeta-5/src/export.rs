use hs_lean_helper::{
    certificates_from_range, lean_prove_certificates_path, render_certificates,
    write_certificates, CertificateExport, FergusonCertificate,
};
use hs_moments::zeta_moments;
use std::path::PathBuf;

use crate::Zeta5Result;

/// Write `lean-prove/Zeta5/Certificates.lean`.
pub fn export_lean_certificates(max_n: usize) -> Zeta5Result<PathBuf> {
    let moment_count = 2 * (max_n + 2);
    let moments = zeta_moments(5, moment_count);
    let rows = certificates_from_range(&moments, 0, max_n)?;
    let config = CertificateExport::zeta5_default();
    let path = lean_prove_certificates_path("Zeta5");
    write_certificates(&path, &config, &rows)?;
    Ok(path)
}

/// Build a `ζ(5)` certificate table without writing files.
pub fn zeta5_certificate_table(max_n: usize) -> Zeta5Result<Vec<FergusonCertificate>> {
    let moment_count = 2 * (max_n + 2);
    let moments = zeta_moments(5, moment_count);
    Ok(certificates_from_range(&moments, 0, max_n)?)
}

/// Preview rendered Lean source.
pub fn render_zeta5_certificates(max_n: usize) -> Zeta5Result<String> {
    let rows = zeta5_certificate_table(max_n)?;
    Ok(render_certificates(&CertificateExport::zeta5_default(), &rows))
}
