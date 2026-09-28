use crate::{VerifyReport, VerifyVerdict};
use hs_checkpoint::{
    Checkpoint, ProofRecord, VERIFIER_POLYNOMIAL_IRRATIONALITY_V1, ZETA5_PAPER_PARAMETER_SPACE, has_polynomial_hankel,
};
use serde_json::Value;

/// Expected paper scaling for `ζ(5)` polynomial Hankel certificates.
pub fn expected_zeta5_paper_dimensions(n: usize) -> (usize, usize, usize) {
    (40 * n, 3 * n, 37 * n)
}

pub fn verify_polynomial_irrationality(cp: &Checkpoint, proof: &ProofRecord) -> Result<VerifyReport, String> {
    if !has_polynomial_hankel(&cp.target) {
        return Err(format!(
            "polynomial_irrationality proof is not registered for checkpoint target `{}`",
            cp.target
        ));
    }
    let verifier = proof.verifier_id.as_deref().unwrap_or(VERIFIER_POLYNOMIAL_IRRATIONALITY_V1);
    if verifier != VERIFIER_POLYNOMIAL_IRRATIONALITY_V1 {
        return Err(format!("unsupported polynomial irrational verifier `{verifier}`"));
    }
    let parameter_space = proof
        .payload
        .get("parameter_space_id")
        .and_then(|v| v.as_str())
        .unwrap_or(ZETA5_PAPER_PARAMETER_SPACE);
    if parameter_space != ZETA5_PAPER_PARAMETER_SPACE {
        return Err(format!(
            "unsupported polynomial parameter space `{parameter_space}` for `{0}`",
            cp.target
        ));
    }
    let n = payload_usize(&proof.payload, "n")?;
    if n == 0 {
        return Err("polynomial_irrationality payload `n` must be positive".into());
    }
    let (k, capital_n, h) = expected_zeta5_paper_dimensions(n);
    validate_optional_usize(&proof.payload, "K", k)?;
    validate_optional_usize(&proof.payload, "N", capital_n)?;
    validate_optional_usize(&proof.payload, "h", h)?;
    Ok(VerifyReport {
        verdict: VerifyVerdict::Unsupported,
        message: format!(
            "polynomial irrationality for `{0}` at n={n} (K={k}, N={capital_n}, h={h}) is not verified: decay, positivity, and normalization evidence are not implemented",
            cp.target
        ),
        mu_checkpoint_hint: None,
    })
}

fn validate_optional_usize(payload: &Value, key: &str, expected: usize) -> Result<(), String> {
    if let Some(raw) = payload.get(key).and_then(|v| v.as_str()) {
        let actual = raw.parse::<usize>().map_err(|e| e.to_string())?;
        if actual != expected {
            return Err(format!("proof payload `{key}` must be {expected}, got {actual}"));
        }
    }
    Ok(())
}

fn payload_usize(payload: &Value, key: &str) -> Result<usize, String> {
    let raw = payload
        .get(key)
        .and_then(|v| v.as_str())
        .ok_or_else(|| format!("proof payload missing `{key}`"))?;
    raw.parse::<usize>().map_err(|e| e.to_string())
}
