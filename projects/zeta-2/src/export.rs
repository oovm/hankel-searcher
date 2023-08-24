use hs_lean_helper::{
    certificates_from_range, write_certificates, CertificateExport, FergusonCertificate,
};
use hs_moments::zeta_moments;
use std::path::{Path, PathBuf};

use crate::Zeta2Result;

/// Write `lean/Zeta2/Certificates.lean` for the bundled Lean project.
pub fn export_lean_certificates(max_n: usize) -> Zeta2Result<PathBuf> {
    let moment_count = 2 * (max_n + 2);
    let moments = zeta_moments(2, moment_count);
    let rows = certificates_from_range(&moments, 0, max_n)?;
    let config = CertificateExport::zeta2_default();
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("lean/Zeta2/Certificates.lean");
    write_certificates(&path, &config, &rows)?;
    Ok(path)
}

/// Build a `ζ(2)` certificate table without writing files.
pub fn zeta2_certificate_table(max_n: usize) -> Zeta2Result<Vec<FergusonCertificate>> {
    let moment_count = 2 * (max_n + 2);
    let moments = zeta_moments(2, moment_count);
    Ok(certificates_from_range(&moments, 0, max_n)?)
}

/// Preview rendered Lean source.
pub fn render_zeta2_certificates(max_n: usize) -> Zeta2Result<String> {
    let rows = zeta2_certificate_table(max_n)?;
    Ok(hs_lean_helper::render_certificates(
        &CertificateExport::zeta2_default(),
        &rows,
    ))
}
