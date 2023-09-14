use hs_lean_helper::{
    certificates_from_range, lean_prove_certificates_path, write_certificates, CertificateExport,
    FergusonCertificate,
};
use hs_moments::zeta_moments;
use std::path::PathBuf;

use crate::Zeta3Result;

/// Write `lean-prove/problems/Zeta3/Certificates.lean`.
pub fn export_lean_certificates(max_n: usize) -> Zeta3Result<PathBuf> {
    // Ferguson 在 index `n` 处需要 `2*(n+2)` 个正矩
    let moment_count = 2 * (max_n + 2);
    let moments = zeta_moments(3, moment_count);
    let rows = certificates_from_range(&moments, 0, max_n)?;
    let config = CertificateExport::zeta3_default();
    let path = lean_prove_certificates_path("Zeta3");
    write_certificates(&path, &config, &rows)?;
    Ok(path)
}

/// Build a `ζ(3)` certificate table without writing files.
pub fn zeta3_certificate_table(max_n: usize) -> Zeta3Result<Vec<FergusonCertificate>> {
    let moment_count = 2 * (max_n + 2);
    let moments = zeta_moments(3, moment_count);
    Ok(certificates_from_range(&moments, 0, max_n)?)
}

/// Preview rendered Lean source.
pub fn render_zeta3_certificates(max_n: usize) -> Zeta3Result<String> {
    let rows = zeta3_certificate_table(max_n)?;
    Ok(hs_lean_helper::render_certificates(
        &CertificateExport::zeta3_default(),
        &rows,
    ))
}
