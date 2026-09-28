use crate::{VerifyReport, VerifyVerdict};
use hs_checkpoint::{Checkpoint, ProofRecord, ProofStatus, RationalData};
use num_bigint::BigInt;
use num_rational::Ratio;
use num_traits::Zero;
use serde_json::Value;

pub fn verify_rational_proof(cp: &Checkpoint, proof: &ProofRecord) -> Result<VerifyReport, String> {
    if proof.kind != "rational_equality" {
        return Ok(VerifyReport {
            verdict: VerifyVerdict::Unsupported,
            message: format!("proof kind `{0}` has no registered verifier", proof.kind),
        });
    }
    if cp.proof_status != ProofStatus::Rational {
        return Err("rational_equality proof requires proof_status rational".into());
    }
    let numerator = payload_str(&proof.payload, "numerator")?;
    let denominator = payload_str(&proof.payload, "denominator")?;
    let num = numerator.parse::<BigInt>().map_err(|e| e.to_string())?;
    let den = denominator.parse::<BigInt>().map_err(|e| e.to_string())?;
    if den <= BigInt::zero() {
        return Err("rational denominator must be positive".into());
    }
    let ratio = Ratio::new(num, den);
    let canonical = RationalData::from_ratio(&ratio);
    if canonical.num != numerator || canonical.den != denominator {
        return Err("rational certificate must use canonical numerator and denominator".into());
    }
    let verifier = proof.verifier_id.as_deref().unwrap_or("exact-rational-v1");
    if verifier != "exact-rational-v1" {
        return Err(format!("unsupported rational verifier `{verifier}`"));
    }
    Ok(VerifyReport {
        verdict: VerifyVerdict::Verified,
        message: format!(
            "verified rational equality for `{0}` as {1}/{2} via `{3}`",
            cp.target,
            canonical.num,
            canonical.den,
            verifier
        ),
    })
}

fn payload_str(payload: &Value, key: &str) -> Result<String, String> {
    payload
        .get(key)
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("proof payload missing `{key}`"))
}
