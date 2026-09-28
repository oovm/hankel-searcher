use crate::{VerifyReport, VerifyVerdict};
use hs_checkpoint::{
    fixture_exact_rational, target_supports_rational_certificate, zeta_order, zeta_value_in_enclosure, Checkpoint,
    ProofRecord, ProofStatus, RationalData,
};
use num_bigint::BigInt;
use num_rational::Ratio;
use num_traits::Zero;
use serde_json::Value;

const ZETA_ENCLOSURE_TERMS: usize = 2048;

pub fn verify_rational_proof(cp: &Checkpoint, proof: &ProofRecord) -> Result<VerifyReport, String> {
    if proof.kind != "rational_equality" {
        return Ok(VerifyReport {
            verdict: VerifyVerdict::Unsupported,
            message: format!("proof kind `{0}` has no registered verifier", proof.kind),
            mu_checkpoint_hint: None,
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
    confirm_target_rational_identity(&cp.target, &ratio)?;
    Ok(VerifyReport {
        verdict: VerifyVerdict::Verified,
        message: format!(
            "verified rational equality for `{0}` as {1}/{2} via `{3}`",
            cp.target,
            canonical.num,
            canonical.den,
            verifier
        ),
        mu_checkpoint_hint: None,
    })
}

fn confirm_target_rational_identity(target: &str, value: &Ratio<BigInt>) -> Result<(), String> {
    if let Some(fixture) = fixture_exact_rational(target) {
        let expected = fixture.ratio()?;
        if *value != expected {
            return Err(format!(
                "certificate {}/{} does not match fixture target `{target}`",
                value.numer(),
                value.denom()
            ));
        }
        return Ok(());
    }
    if let Some(order) = zeta_order(target) {
        if zeta_value_in_enclosure(order, ZETA_ENCLOSURE_TERMS, value)? {
            return Err(format!(
                "certificate lies inside a finite `ζ({order})` partial-sum enclosure but exact equality is not registered for `{target}`"
            ));
        }
        return Err(format!(
            "certificate {}/{} is outside the rigorous `ζ({order})` enclosure at {ZETA_ENCLOSURE_TERMS} terms",
            value.numer(),
            value.denom()
        ));
    }
    if target_supports_rational_certificate(target) {
        return Err(format!("target `{target}` has no exact rational identity registered"));
    }
    Err(format!("unsupported rational certificate target `{target}`"))
}

fn payload_str(payload: &Value, key: &str) -> Result<String, String> {
    payload
        .get(key)
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("proof payload missing `{key}`"))
}
