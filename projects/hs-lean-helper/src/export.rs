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
    /// `import` target for the generated module.
    pub import_module: String,
    /// Names opened inside the namespace.
    pub open_namespaces: String,
    /// Certificate structure name.
    pub structure_name: String,
    /// List definition name.
    pub list_name: String,
}

impl CertificateExport {
    /// Default export settings for the `zeta-2` Lean project.
    pub fn zeta2_default() -> Self {
        Self {
            generator: "export-lean-certificates".into(),
            regenerate_cmd: "cargo run --release -p zeta-2 --bin export-zeta2-certificates".into(),
            namespace: "Zeta2".into(),
            import_module: "Zeta2.Ferguson".into(),
            open_namespaces: "Hankel Ferguson".into(),
            structure_name: "Certificate".into(),
            list_name: "certificates".into(),
        }
    }

    /// Default export settings for the `zeta-3` Lean project.
    pub fn zeta3_default() -> Self {
        Self {
            generator: "export-lean-certificates".into(),
            regenerate_cmd: "cargo run --release -p zeta-3 --bin export-zeta3-certificates".into(),
            namespace: "Zeta3".into(),
            import_module: "Zeta3.Ferguson".into(),
            open_namespaces: "Hankel Ferguson".into(),
            structure_name: "Certificate".into(),
            list_name: "certificates".into(),
        }
    }
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
