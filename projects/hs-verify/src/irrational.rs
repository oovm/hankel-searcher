use crate::{VerifyReport, VerifyVerdict};
use hs_checkpoint::{Checkpoint, ProofRecord};

pub fn verify_irrational_proof(cp: &Checkpoint, proof: &ProofRecord) -> Result<VerifyReport, String> {
    if proof.kind == "rational_equality" {
        return Err("rational equality certificate cannot support irrational proof_status".into());
    }
    match proof.kind.as_str() {
        "integer_linear_form" => verify_integer_linear_form(cp, proof),
        other => Ok(VerifyReport {
            verdict: VerifyVerdict::Unsupported,
            message: format!(
                "irrational proof kind `{other}` has no registered verifier for `{0}`",
                cp.target
            ),
        }),
    }
}

fn verify_integer_linear_form(cp: &Checkpoint, proof: &ProofRecord) -> Result<VerifyReport, String> {
    let _ = proof.payload.get("linear_form_id").and_then(|v| v.as_str()).ok_or(
        "integer_linear_form proof requires payload.linear_form_id",
    )?;
    let verifier = proof.verifier_id.as_deref().unwrap_or("integer-linear-form-v1");
    if verifier != "integer-linear-form-v1" {
        return Err(format!("unsupported irrational verifier `{verifier}`"));
    }
    Ok(VerifyReport {
        verdict: VerifyVerdict::Unsupported,
        message: format!(
            "integer linear form verifier `{verifier}` is not implemented for `{0}`",
            cp.target
        ),
    })
}
