use hs_checkpoint::{
    decode_rational_parameter, rational_parameter_ordinal, Checkpoint, CHECKPOINT_TARGETS, FERGUSON_INDEX_GENERATOR,
    FERGUSON_PARAMETER_GENERATOR, FERGUSON_ZETA_TARGETS, MuRecord, MuStatus, NONNEGATIVE_INDEX_SPACE, Objective,
    OBSERVATION_KIND_POLYNOMIAL_HANKEL, POLYNOMIAL_HANKEL_GENERATOR, POLYNOMIAL_NQ_PARAMETER_SPACE,
    PolynomialHankelObservation, ProofRecord,
    ProofStatus, RATIONAL_PARAMETER_SPACE, RationalData, Search, ZETA5_PAPER_PARAMETER_SPACE, default_checkpoint_path,
    has_ferguson_search, has_polynomial_hankel, is_implemented_search_contract, is_known_target, read_checkpoint,
    validate_known_search_contract, write_checkpoint, zeta_order,
};
use hs_types::{Integer, Rational};
use malachite::base::num::basic::traits::One;
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
            benchmark: None,
        },
        proof: None,
        mu: MuRecord::default(),
        best: None,
        observed_best: None,
        polynomial_observed_best: None,
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
        shift: None,
        series_terms: 64,
        approximant: RationalData { num: "2".into(), den: "4".into() },
        error_upper: RationalData::from_ratio(&Rational::ONE),
    });
    let err = hs_checkpoint::validate_checkpoint(&cp).unwrap_err().to_string();
    assert!(err.contains("canonical"));
}

#[test]
fn default_path_for_zeta3() {
    let path = default_checkpoint_path("zeta-3").unwrap();
    assert!(path.ends_with("projects/hs-problems/checkpoints/zeta-3/checkpoint.json"));
}

#[test]
fn mu_upper_bound_requires_value() {
    let mut cp = sample_checkpoint();
    cp.mu.status = MuStatus::UpperBound;
    let err = hs_checkpoint::validate_checkpoint(&cp).unwrap_err().to_string();
    assert!(err.contains("upper_bound"));
}

#[test]
fn round_trip_search_benchmark() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("checkpoint.json");
    let mut cp = sample_checkpoint();
    cp.search.benchmark = Some(hs_checkpoint::SearchBenchmark {
        steps: 4,
        elapsed_ms: 1200,
        jobs: 2,
        strategy: "enumerate".into(),
        recorded_at: "2026-01-01T00:00:00Z".into(),
    });
    write_checkpoint(&path, &cp).unwrap();
    let loaded = read_checkpoint(&path).unwrap();
    let benchmark = loaded.search.benchmark.expect("benchmark");
    assert_eq!(benchmark.steps, 4);
    assert_eq!(benchmark.elapsed_ms, 1200);
    assert_eq!(benchmark.jobs, 2);
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
    assert!(has_polynomial_hankel("zeta-2"));
    assert!(has_polynomial_hankel("zeta-3"));
    assert!(has_polynomial_hankel("zeta-5"));
    assert!(!has_polynomial_hankel("zeta-7"));
    assert_eq!(zeta_order("zeta-7"), Some(7));
    assert_eq!(zeta_order("delta"), None);
    assert!(default_checkpoint_path("zeta-7").unwrap().ends_with("projects/hs-problems/checkpoints/zeta-7/checkpoint.json"));
    assert!(default_checkpoint_path("unknown").is_err());
}

#[test]
fn search_contract_registry() {
    assert!(is_implemented_search_contract(FERGUSON_INDEX_GENERATOR, NONNEGATIVE_INDEX_SPACE));
    assert!(is_implemented_search_contract(FERGUSON_PARAMETER_GENERATOR, RATIONAL_PARAMETER_SPACE));
    assert!(is_implemented_search_contract(
        POLYNOMIAL_HANKEL_GENERATOR,
        POLYNOMIAL_NQ_PARAMETER_SPACE
    ));
    assert!(validate_known_search_contract(FERGUSON_INDEX_GENERATOR, "unknown-space").is_err());
}

#[test]
fn rational_parameter_ordinal_round_trip() {
    let parameter = decode_rational_parameter(7);
    assert_eq!(parameter.index, 1);
    assert_eq!(parameter.shift, 2);
    assert_eq!(rational_parameter_ordinal(parameter.index, parameter.shift).unwrap(), 7);
}

#[test]
fn polynomial_hankel_contract_round_trips() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("checkpoint.json");
    let mut cp = sample_checkpoint();
    cp.search.generator_id = POLYNOMIAL_HANKEL_GENERATOR.into();
    cp.search.parameter_space_id = ZETA5_PAPER_PARAMETER_SPACE.into();
    write_checkpoint(&path, &cp).unwrap();
    let loaded = read_checkpoint(&path).unwrap();
    assert_eq!(loaded.search.generator_id, POLYNOMIAL_HANKEL_GENERATOR);
    assert!(is_implemented_search_contract(POLYNOMIAL_HANKEL_GENERATOR, ZETA5_PAPER_PARAMETER_SPACE));
}

#[test]
fn polynomial_observed_best_with_energy_round_trips() {
    let mut cp = sample_checkpoint();
    cp.target = "zeta-5".into();
    cp.search.generator_id = POLYNOMIAL_HANKEL_GENERATOR.into();
    cp.search.parameter_space_id = ZETA5_PAPER_PARAMETER_SPACE.into();
    cp.polynomial_observed_best = Some(PolynomialHankelObservation {
        kind: OBSERVATION_KIND_POLYNOMIAL_HANKEL.into(),
        n: 1,
        k: 40,
        capital_n: 3,
        h: 37,
        q: 6,
        log_s_k: -204.319,
        leading_coeff: RationalData { num: "1".into(), den: "2".into() },
        log_delta_at_zeta5: Some(-1836.876),
        log_primitive_at_zeta5: Some(-265.129),
        max_primitive_coeff_bits: Some(7597),
    });
    hs_checkpoint::validate_checkpoint(&cp).unwrap();
    let dir = tempdir().unwrap();
    let path = dir.path().join("checkpoint.json");
    write_checkpoint(&path, &cp).unwrap();
    let loaded = read_checkpoint(&path).unwrap();
    let observation = loaded.polynomial_observed_best.as_ref().unwrap();
    assert_eq!(observation.max_primitive_coeff_bits, Some(7597));
    assert_eq!(observation.log_primitive_at_zeta5, Some(-265.129));
}

#[test]
fn polynomial_observed_best_round_trips() {
    let mut cp = sample_checkpoint();
    cp.target = "zeta-5".into();
    cp.search.generator_id = POLYNOMIAL_HANKEL_GENERATOR.into();
    cp.search.parameter_space_id = ZETA5_PAPER_PARAMETER_SPACE.into();
    cp.polynomial_observed_best = Some(PolynomialHankelObservation {
        kind: OBSERVATION_KIND_POLYNOMIAL_HANKEL.into(),
        n: 1,
        k: 40,
        capital_n: 3,
        h: 37,
        q: 6,
        log_s_k: -204.319,
        leading_coeff: RationalData { num: "1".into(), den: "2".into() },
        log_delta_at_zeta5: None,
        log_primitive_at_zeta5: None,
        max_primitive_coeff_bits: None,
    });
    validate_known_search_contract(&cp.search.generator_id, &cp.search.parameter_space_id).unwrap();
    hs_checkpoint::validate_checkpoint(&cp).unwrap();
    let dir = tempdir().unwrap();
    let path = dir.path().join("checkpoint.json");
    write_checkpoint(&path, &cp).unwrap();
    let loaded = read_checkpoint(&path).unwrap();
    assert_eq!(loaded.polynomial_observed_best.as_ref().unwrap().n, 1);
}

#[test]
fn polynomial_proof_kind_round_trips() {
    let mut cp = sample_checkpoint();
    cp.proof_status = ProofStatus::Irrational;
    cp.proof = Some(ProofRecord {
        kind: "polynomial_irrationality".into(),
        verifier_id: Some("polynomial-irrationality-v1".into()),
        payload: serde_json::json!({ "n": "1" }),
    });
    hs_checkpoint::validate_checkpoint(&cp).unwrap();
}

#[test]
fn planned_parameter_contract_round_trips() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("checkpoint.json");
    let mut cp = sample_checkpoint();
    cp.search.generator_id = FERGUSON_PARAMETER_GENERATOR.into();
    cp.search.parameter_space_id = RATIONAL_PARAMETER_SPACE.into();
    write_checkpoint(&path, &cp).unwrap();
    let loaded = read_checkpoint(&path).unwrap();
    assert_eq!(loaded.search.generator_id, FERGUSON_PARAMETER_GENERATOR);
}
