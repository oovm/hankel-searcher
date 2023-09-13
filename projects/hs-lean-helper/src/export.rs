use crate::certificate::{FergusonCertificate, certificates_from_range};
use crate::error::LeanHelperError;
use crate::render::render_certificates;
use hs_types::RationalMomentSequence;
use std::path::{Path, PathBuf};

/// Lean module export configuration.
#[derive(Debug, Clone)]
pub struct CertificateExport {
    /// Human-readable generator label.
    pub generator: String,
    /// Shell command to regenerate this file.
    pub regenerate_cmd: String,
    /// Top-level Lean namespace.
    pub namespace: String,
    /// Shared certificate structure module.
    pub certificate_module: String,
    /// Problem-local Ferguson marker module.
    pub ferguson_module: String,
    /// Names opened inside the namespace.
    pub open_namespaces: String,
    /// Certificate structure type (usually `LeanProve.FergusonCertificate`).
    pub certificate_type: String,
    /// List definition name.
    pub list_name: String,
}

impl CertificateExport {
    fn problem_defaults(namespace: &str, regenerate_cmd: &str) -> Self {
        Self {
            generator: "export-lean-certificates".into(),
            regenerate_cmd: regenerate_cmd.into(),
            namespace: namespace.into(),
            certificate_module: "LeanProve.Certificate".into(),
            ferguson_module: format!("{}.Ferguson", namespace),
            open_namespaces: "Hankel Ferguson".into(),
            certificate_type: "LeanProve.FergusonCertificate".into(),
            list_name: "certificates".into(),
        }
    }

    /// Default export settings for the `zeta-2` problem.
    pub fn zeta2_default() -> Self {
        Self::problem_defaults(
            "Zeta2",
            "cargo run --release -p zeta-2 --bin export-zeta2-certificates",
        )
    }

    /// Default export settings for the `zeta-3` problem.
    pub fn zeta3_default() -> Self {
        Self::problem_defaults(
            "Zeta3",
            "cargo run --release -p zeta-3 --bin export-zeta3-certificates",
        )
    }

    /// Default export settings for the `zeta-5` problem.
    pub fn zeta5_default() -> Self {
        Self::problem_defaults(
            "Zeta5",
            "cargo run --release -p zeta-5 --bin export-zeta5-certificates",
        )
    }
}

/// Path to `lean-prove/problems/<Problem>/Certificates.lean` from this crate layout.
pub fn lean_prove_certificates_path(problem: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../lean-prove/problems")
        .join(problem)
        .join("Certificates.lean")
}

/// Write rendered Lean certificate source to `path`.
pub fn write_certificates(
    path: &Path,
    config: &CertificateExport,
    rows: &[FergusonCertificate],
) -> Result<PathBuf, LeanHelperError> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let rendered = render_certificates(config, rows);
    std::fs::write(path, rendered)?;
    Ok(path.to_path_buf())
}

/// Build and write certificates for `moments[start..=end]`.
pub fn export_range(
    path: &Path,
    config: &CertificateExport,
    moments: &RationalMomentSequence,
    start: usize,
    end: usize,
) -> Result<PathBuf, LeanHelperError> {
    let rows = certificates_from_range(moments, start, end)?;
    write_certificates(path, config, &rows)
}
