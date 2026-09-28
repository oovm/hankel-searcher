use hs_checkpoint::{
    Checkpoint, CHECKPOINT_TARGETS, FERGUSON_ZETA_TARGETS, MuRecord, MuStatus, Objective, ProofStatus, RationalData,
    Search, default_checkpoint_path, has_ferguson_search, is_known_target, read_checkpoint, write_checkpoint, zeta_order,
};
use num_bigint::BigInt;
use num_rational::Ratio;
use std::fs;
use tempfile::tempdir;

fn sample_checkpoint() -> Checkpoint {
    Checkpoint {
        schema_version: hs_checkpoint::SCHEMA_VERSION,
        status: "draft".into(),
        target: "zeta-3".into(),
        proof_status: ProofStatus::Unknown,
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
            next_candidate: "2".into(),
            series_terms: Some(64),
        },
        proof: None,
        mu: MuRecord::default(),
        best: None,
        observed_best: None,
        updated_at: "2026-01-01T00:00:00Z".into(),
    }
}

#[test]
fn round_trip_v2_checkpoint() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("checkpoint.json");
    let cp = sample_checkpoint();
    write_checkpoint(&path, &cp).unwrap();
    let loaded = read_checkpoint(&path).unwrap();
    assert_eq!(loaded.schema_version, 2);
    assert_eq!(loaded.proof_status, ProofStatus::Unknown);
}

#[test]
fn migrates_schema_v1_on_read() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("checkpoint.json");
    let v1 = r#"{
  "schema_version": 1,
  "status": "draft",
  "target": "zeta-3",
  "objective": {
    "linear_form_id": "unassigned",
    "normalization_id": "unassigned",
    "bound_kind": "certified_margin_lower_bound",
    "direction": "maximize"
  },
  "search": {
    "generator_id": "ferguson-index-v1",
    "parameter_space_id": "nonnegative-index-v1",
    "seed": "0",
    "next_candidate": "1",
    "series_terms": 64
  },
  "best": null,
  "updated_at": "2026-01-01T00:00:00Z"
}"#;
    fs::write(&path, v1).unwrap();
    let loaded = read_checkpoint(&path).unwrap();
    assert_eq!(loaded.proof_status, ProofStatus::Unknown);
    assert_eq!(loaded.mu.status, MuStatus::Unavailable);
}

#[test]
fn rejects_noncanonical_rational() {
    let mut cp = sample_checkpoint();
    cp.observed_best = Some(hs_checkpoint::Observation {
        kind: "finite_approximation_error_upper".into(),
        n: 0,
        series_terms: 64,
        approximant: RationalData { num: "2".into(), den: "4".into() },
        error_upper: RationalData::from_ratio(&Ratio::from_integer(BigInt::from(1))),
    });
    let err = hs_checkpoint::validate_checkpoint(&cp).unwrap_err().to_string();
    assert!(err.contains("canonical"));
}

#[test]
fn default_path_for_zeta3() {
    let path = default_checkpoint_path("zeta-3").unwrap();
    assert!(path.ends_with("projects/targets/zeta-3/checkpoint.json"));
}

#[test]
fn checkpoint_targets_registry() {
    assert_eq!(CHECKPOINT_TARGETS, &["zeta-2", "zeta-3", "zeta-5", "zeta-7"]);
    assert_eq!(FERGUSON_ZETA_TARGETS, &["zeta-2", "zeta-3", "zeta-5", "zeta-7"]);
    assert!(is_known_target("zeta-2"));
    assert!(is_known_target("zeta-7"));
    assert!(!is_known_target("zeta-4"));
    assert!(has_ferguson_search("zeta-3"));
    assert!(has_ferguson_search("zeta-7"));
    assert_eq!(zeta_order("zeta-7"), Some(7));
    assert_eq!(zeta_order("delta"), None);
    assert!(default_checkpoint_path("zeta-7").unwrap().ends_with("projects/targets/zeta-7/checkpoint.json"));
    assert!(default_checkpoint_path("unknown").is_err());
}
