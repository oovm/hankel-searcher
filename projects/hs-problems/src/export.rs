use hs_lean_helper::{
    certificates_from_range, lean_prove_certificates_path, write_certificates, CertificateExport,
};
use hs_moments::{catalan_beta_moments, delta_moments, gamma_moments};
use std::path::PathBuf;

use crate::HsProblemsError;

fn export_moments(
    problem: &str,
    config: CertificateExport,
    moments: &hs_types::RationalMomentSequence,
    max_n: usize,
) -> Result<PathBuf, HsProblemsError> {
    let rows = certificates_from_range(moments, 0, max_n)?;
    let path = lean_prove_certificates_path(problem);
    write_certificates(&path, &config, &rows)?;
    Ok(path)
}

/// Write `lean-prove/problems/Delta/Certificates.lean`.
pub fn export_delta_certificates(max_n: usize) -> Result<PathBuf, HsProblemsError> {
    let moment_count = 2 * (max_n + 2);
    let moments = delta_moments(moment_count);
    export_moments("Delta", CertificateExport::delta_default(), &moments, max_n)
}

/// Write `lean-prove/problems/Gamma/Certificates.lean`.
pub fn export_gamma_certificates(max_n: usize) -> Result<PathBuf, HsProblemsError> {
    let moment_count = 2 * (max_n + 2);
    let moments = gamma_moments(moment_count);
    export_moments("Gamma", CertificateExport::gamma_default(), &moments, max_n)
}

/// Write `lean-prove/problems/Catalan/Certificates.lean`.
pub fn export_catalan_certificates(max_n: usize) -> Result<PathBuf, HsProblemsError> {
    let moment_count = 2 * (max_n + 2);
    let moments = catalan_beta_moments(moment_count);
    export_moments("Catalan", CertificateExport::catalan_default(), &moments, max_n)
}
