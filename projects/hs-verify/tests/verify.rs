use hs_checkpoint::{
    Checkpoint, MuRecord, Objective, ProofRecord, ProofStatus, Search, default_checkpoint_path, read_checkpoint, write_checkpoint,
};
use hs_verify::{VerifyVerdict, verify_checkpoint};
use std::fs;
use tempfile::tempdir;

fn sample_checkpoint(proof_status: ProofStatus, proof: Option<ProofRecord>) -> Checkpoint {
    Checkpoint {
        schema_version: hs_checkpoint::SCHEMA_VERSION,
        status: "draft".into(),
        target: "zeta-3".into(),
        proof_status,
        objective: Objective {
            linear_form_id: "unassigned".into(),
            normalization_id: "unassigned".into(),
            bound_kind: "certified_margin_lower_bound".into(),
            direction: "maximize".into(),
        },
        search: Search {
            generator_id: "ferguson-index-v1".into(),
            parameter_space_id: "nonnegative-index-v1".into(),
            seed: "0".into(),
            next_candidate: "1".into(),
            series_terms: Some(64),
            benchmark: None,
        },
        proof,
        mu: MuRecord::default(),
        best: None,
        observed_best: None,
        updated_at: "2026-01-01T00:00:00Z".into(),
    }
}

#[test]
fn unknown_target_is_unsupported() {
    let cp = sample_checkpoint(ProofStatus::Unknown, None);
    let report = verify_checkpoint(&cp).unwrap();
    assert_eq!(report.verdict, VerifyVerdict::Unsupported);
}

#[test]
fn rational_certificate_verifies() {
    let proof = ProofRecord {
        kind: "rational_equality".into(),
        verifier_id: Some("exact-rational-v1".into()),
        payload: serde_json::json!({ "numerator": "1", "denominator": "2" }),
    };
    let cp = sample_checkpoint(ProofStatus::Rational, Some(proof));
    let report = verify_checkpoint(&cp).unwrap();
    assert_eq!(report.verdict, VerifyVerdict::Verified);
}

#[test]
fn noncanonical_rational_certificate_rejects() {
    let proof = ProofRecord {
        kind: "rational_equality".into(),
        verifier_id: Some("exact-rational-v1".into()),
        payload: serde_json::json!({ "numerator": "2", "denominator": "4" }),
    };
    let cp = sample_checkpoint(ProofStatus::Rational, Some(proof));
    let err = verify_checkpoint(&cp).unwrap_err();
    assert!(err.contains("canonical"));
}

#[test]
fn rational_certificate_cannot_support_irrational_status() {
    let proof = ProofRecord {
        kind: "rational_equality".into(),
        verifier_id: Some("exact-rational-v1".into()),
        payload: serde_json::json!({ "numerator": "1", "denominator": "2" }),
    };
    let cp = sample_checkpoint(ProofStatus::Irrational, Some(proof));
    let err = verify_checkpoint(&cp).unwrap_err();
    assert!(err.contains("cannot support irrational"));
}

#[test]
fn integer_linear_form_unassigned_stays_unsupported() {
    let proof = ProofRecord {
        kind: "integer_linear_form".into(),
        verifier_id: Some("integer-linear-form-v1".into()),
        payload: serde_json::json!({ "linear_form_id": "unassigned" }),
    };
    let cp = sample_checkpoint(ProofStatus::Irrational, Some(proof));
    let report = verify_checkpoint(&cp).unwrap();
    assert_eq!(report.verdict, VerifyVerdict::Unsupported);
    assert!(report.message.contains("not registered"));
}

#[test]
fn integer_linear_form_regression_fixture_verifies_mu_upper_bound() {
    let proof = ProofRecord {
        kind: "integer_linear_form".into(),
        verifier_id: Some("integer-linear-form-v1".into()),
        payload: serde_json::json!({
            "linear_form_id": "regression-v1",
            "start_index": "1",
            "tau": { "num": "1", "den": "10" },
            "sigma": { "num": "1", "den": "5" }
        }),
    };
    let mut cp = sample_checkpoint(ProofStatus::Irrational, Some(proof));
    cp.objective.linear_form_id = "regression-v1".into();
    let report = verify_checkpoint(&cp).unwrap();
    assert_eq!(report.verdict, VerifyVerdict::Verified);
    assert!(report.message.contains("mu upper bound 3/1"));
}

#[test]
fn project_zeta3_checkpoint_has_no_proof() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let path = default_checkpoint_path("zeta-3").unwrap();
    let path = root.join(path);
    let cp = read_checkpoint(&path).unwrap();
    let report = verify_checkpoint(&cp).unwrap();
    assert_eq!(report.verdict, VerifyVerdict::Unsupported);
}

#[test]
fn round_trip_rational_proof_checkpoint() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("checkpoint.json");
    let proof = ProofRecord {
        kind: "rational_equality".into(),
        verifier_id: Some("exact-rational-v1".into()),
        payload: serde_json::json!({ "numerator": "22", "denominator": "7" }),
    };
    let cp = sample_checkpoint(ProofStatus::Rational, Some(proof));
    write_checkpoint(&path, &cp).unwrap();
    let loaded = read_checkpoint(&path).unwrap();
    let report = verify_checkpoint(&loaded).unwrap();
    assert_eq!(report.verdict, VerifyVerdict::Verified);
    let text = fs::read_to_string(&path).unwrap();
    assert!(text.contains("\"proof_status\": \"rational\""));
}
