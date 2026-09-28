mod checkpoint;
mod error;
mod proof;
mod rational;
mod search;
mod targets;
mod zeta;

pub use checkpoint::{
    Checkpoint, MuRecord, MuStatus, Objective, Observation, PolynomialHankelObservation, ProofRecord, ProofStatus,
    RationalData, SCHEMA_VERSION, Search, SearchBenchmark, OBSERVATION_KIND_FERGUSON,
    OBSERVATION_KIND_POLYNOMIAL_HANKEL, default_checkpoint_path, read_checkpoint, validate_checkpoint, write_checkpoint,
};
pub use error::CheckpointError;
pub use rational::ratio_from_data;
pub use proof::{
    is_known_proof_kind, PROOF_KIND_INTEGER_LINEAR_FORM, PROOF_KIND_POLYNOMIAL_IRRATIONALITY,
    PROOF_KIND_RATIONAL_EQUALITY, VERIFIER_POLYNOMIAL_IRRATIONALITY_V1,
};
pub use search::{
    decode_rational_parameter, rational_parameter_ordinal, FERGUSON_INDEX_GENERATOR, FERGUSON_PARAMETER_GENERATOR,
    NONNEGATIVE_INDEX_SPACE, POLYNOMIAL_HANKEL_GENERATOR, RATIONAL_PARAMETER_MAX_SHIFT, RATIONAL_PARAMETER_SPACE,
    RationalParameter, SEARCH_CONTRACTS, SearchContract, ZETA5_PAPER_PARAMETER_SPACE, find_search_contract,
    improve_error_for_search_contract, is_implemented_search_contract, is_unassigned_search_contract,
    validate_known_search_contract,
};
pub use targets::{
    CHECKPOINT_TARGETS, FERGUSON_ZETA_TARGETS, POLYNOMIAL_HANKEL_TARGETS, VERIFY_FIXTURE_TARGETS,
    has_ferguson_search, has_polynomial_hankel, is_known_target, is_project_checkpoint_target, zeta_order,
};
pub use zeta::{
    fixture_exact_rational, target_supports_rational_certificate, zeta_series_bounds, zeta_value_in_enclosure,
};
