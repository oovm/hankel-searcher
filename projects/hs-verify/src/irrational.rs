use crate::{VerifyReport, VerifyVerdict};
use hs_checkpoint::{Checkpoint, ProofRecord, ProofStatus, RationalData};
use num_bigint::BigInt;
use num_rational::Ratio;
use num_traits::{One, Zero};
use serde_json::Value;

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
        }),
    }
}

fn verify_integer_linear_form(cp: &Checkpoint, proof: &ProofRecord) -> Result<VerifyReport, String> {
    let linear_form_id = proof.payload.get("linear_form_id").and_then(|v| v.as_str()).ok_or(
        "integer_linear_form proof requires payload.linear_form_id",
    )?;
    if linear_form_id == "unassigned" {
        return Ok(VerifyReport {
            verdict: VerifyVerdict::Unsupported,
            message: format!("linear form `{linear_form_id}` is not registered for `{0}`", cp.target),
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
    let start_index = payload_usize(&proof.payload, "start_index")?;
    let tau = payload_positive_rational(&proof.payload, "tau")?;
    let sigma = payload_nonnegative_rational(&proof.payload, "sigma")?;
    if start_index == 0 {
        return Err("integer_linear_form start_index must be positive".into());
    }
    let mu_upper = Ratio::from_integer(BigInt::one()) + sigma / tau;
    let mu = RationalData::from_ratio(&mu_upper);
    validate_mu_for_integer_linear_form(cp, verifier, &mu)?;
    Ok(VerifyReport {
        verdict: VerifyVerdict::Verified,
        message: format!(
            "verified integer linear form `{linear_form_id}` for `{0}` from index {start_index} with mu upper bound {1}/{2} via `{verifier}`",
            cp.target,
            mu.num,
            mu.den
        ),
    })
}

fn validate_mu_for_integer_linear_form(
    cp: &Checkpoint,
    verifier: &str,
    mu_upper: &RationalData,
) -> Result<(), String> {
    match cp.mu.status {
        hs_checkpoint::MuStatus::Unavailable => Ok(()),
        hs_checkpoint::MuStatus::Exact => {
            Err("integer_linear_form proof cannot verify checkpoint with mu status exact".into())
        }
        hs_checkpoint::MuStatus::UpperBound => {
            let bound = cp
                .mu
                .upper_bound
                .as_ref()
                .ok_or("checkpoint mu upper_bound status requires upper_bound field")?;
            if bound.num != mu_upper.num || bound.den != mu_upper.den {
                return Err("checkpoint mu upper_bound does not match proof-derived bound".into());
            }
            if let Some(record_verifier) = &cp.mu.verifier_id {
                if record_verifier != verifier {
                    return Err(format!(
                        "checkpoint mu verifier_id `{record_verifier}` does not match proof verifier `{verifier}`"
                    ));
                }
            }
            Ok(())
        }
    }
}

fn payload_usize(payload: &Value, key: &str) -> Result<usize, String> {
    let raw = payload
        .get(key)
        .and_then(|v| v.as_str())
        .ok_or_else(|| format!("proof payload missing `{key}`"))?;
    raw.parse::<usize>().map_err(|e| e.to_string())
}

fn payload_positive_rational(payload: &Value, key: &str) -> Result<Ratio<BigInt>, String> {
    let ratio = payload_rational(payload, key)?;
    if ratio <= Ratio::from_integer(BigInt::zero()) {
        return Err(format!("proof payload `{key}` must be positive"));
    }
    Ok(ratio)
}

fn payload_nonnegative_rational(payload: &Value, key: &str) -> Result<Ratio<BigInt>, String> {
    let ratio = payload_rational(payload, key)?;
    if ratio < Ratio::from_integer(BigInt::zero()) {
        return Err(format!("proof payload `{key}` must be non-negative"));
    }
    Ok(ratio)
}

fn payload_rational(payload: &Value, key: &str) -> Result<Ratio<BigInt>, String> {
    let object = payload.get(key).ok_or_else(|| format!("proof payload missing `{key}`"))?;
    let data = serde_json::from_value::<RationalData>(object.clone())
        .map_err(|e| format!("proof payload `{key}` must be a rational object: {e}"))?;
    data.ratio()
}
