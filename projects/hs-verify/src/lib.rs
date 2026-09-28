mod rational;

use hs_checkpoint::{Checkpoint, ProofStatus};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerifyVerdict {
    Verified,
    Unsupported,
}

#[derive(Debug, Clone)]
pub struct VerifyReport {
    pub verdict: VerifyVerdict,
    pub message: String,
}

impl fmt::Display for VerifyReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

/// Verify `proof` against `proof_status`. Returns `Unsupported` when no verifier is registered for the kind.
pub fn verify_checkpoint(cp: &Checkpoint) -> Result<VerifyReport, String> {
    match cp.proof_status {
        ProofStatus::Unknown => {
            if cp.proof.is_some() {
                return Err("proof record present but proof_status is unknown".into());
            }
            Ok(VerifyReport {
                verdict: VerifyVerdict::Unsupported,
                message: format!("no proof registered for `{0}`", cp.target),
            })
        }
        ProofStatus::Rational => {
            let proof = cp.proof.as_ref().ok_or("proof_status rational requires proof record")?;
            rational::verify_rational_proof(cp, proof)
        }
        ProofStatus::Irrational => {
            let proof = cp.proof.as_ref().ok_or("proof_status irrational requires proof record")?;
            if proof.kind == "rational_equality" {
                return Err("rational equality certificate cannot support irrational proof_status".into());
            }
            Ok(VerifyReport {
                verdict: VerifyVerdict::Unsupported,
                message: format!(
                    "irrational proof kind `{0}` has no registered verifier for `{1}`",
                    proof.kind,
                    cp.target
                ),
            })
        }
    }
}
