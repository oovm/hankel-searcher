use crate::{VerifyReport, VerifyVerdict};
use hs_checkpoint::{Checkpoint, ProofRecord, ProofStatus};

pub fn verify_irrational_proof(cp: &Checkpoint, proof: &ProofRecord) -> Result<VerifyReport, String> {
    if proof.kind == "rational_equality" {
        return Err("rational equality certificate cannot support irrational proof_status".into());
    }
    if cp.proof_status != ProofStatus::Irrational {
        return Err("integer_linear_form proof requires proof_status irrational".into());
    }
    match proof.kind.as_str() {
        "integer_linear_form" => verify_integer_linear_form(cp, proof),
        other => Ok(VerifyReport {
            verdict: VerifyVerdict::Unsupported,
            message: format!(
                "irrational proof kind `{other}` has no registered verifier for `{0}`",
                cp.target
            ),
            mu_checkpoint_hint: None,
        }),
    }
}

fn verify_integer_linear_form(cp: &Checkpoint, proof: &ProofRecord) -> Result<VerifyReport, String> {
    let linear_form_id = proof
        .payload
        .get("linear_form_id")
        .and_then(|v| v.as_str())
        .ok_or("integer_linear_form proof requires payload.linear_form_id")?;
    if linear_form_id == "unassigned" {
        return Ok(VerifyReport {
            verdict: VerifyVerdict::Unsupported,
            message: format!("linear form `{linear_form_id}` is not registered for `{0}`", cp.target),
            mu_checkpoint_hint: None,
        });
    }
    if linear_form_id != cp.objective.linear_form_id {
        return Err(format!(
            "proof linear_form_id `{linear_form_id}` does not match checkpoint objective `{}`",
            cp.objective.linear_form_id
        ));
    }
    let verifier = proof.verifier_id.as_deref().unwrap_or("integer-linear-form-v1");
    if verifier != "integer-linear-form-v1" {
        return Err(format!("unsupported irrational verifier `{verifier}`"));
    }
    Ok(VerifyReport {
        verdict: VerifyVerdict::Unsupported,
        message: format!(
            "integer linear form `{linear_form_id}` for `{0}` is not verified: full decay, coefficient growth, and nondegeneracy evidence are not implemented",
            cp.target
        ),
        mu_checkpoint_hint: None,
    })
}
