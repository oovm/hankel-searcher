use crate::error::CheckpointError;
use crate::proof::is_known_proof_kind;
use crate::search::{validate_known_search_contract, RATIONAL_PARAMETER_MAX_SHIFT};
use crate::targets::{is_known_target, is_project_checkpoint_target};
use crate::rational::ratio_from_data;
use num_bigint::BigInt;
use num_rational::Ratio;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

pub const SCHEMA_VERSION: u32 = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProofStatus {
    Unknown,
    Rational,
    Irrational,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MuStatus {
    Unavailable,
    Exact,
    UpperBound,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RationalData {
    pub num: String,
    pub den: String,
}

impl RationalData {
    pub fn from_ratio(value: &Ratio<BigInt>) -> Self {
        Self { num: value.numer().to_string(), den: value.denom().to_string() }
    }

    pub fn ratio(&self) -> Result<Ratio<BigInt>, String> {
        ratio_from_data(self)
    }
}

/// Observation kind for Ferguson finite-index bounds.
pub const OBSERVATION_KIND_FERGUSON: &str = "finite_approximation_error_upper";

/// Observation kind for Zeta5 whole-polynomial Hankel energy logs.
pub const OBSERVATION_KIND_POLYNOMIAL_HANKEL: &str = "polynomial_hankel_energy";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Observation {
    pub kind: String,
    pub n: usize,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shift: Option<usize>,
    pub series_terms: usize,
    pub approximant: RationalData,
    pub error_upper: RationalData,
}

/// Best polynomial Hankel observation for `polynomial-hankel-v1` (separate from Ferguson `observed_best`).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PolynomialHankelObservation {
    pub kind: String,
    /// Construction index `n` with `K=40n`, `N=3n`, `h=37n`.
    pub n: usize,
    pub k: usize,
    pub capital_n: usize,
    pub h: usize,
    /// `log S_K` paper normalization at this scaling.
    pub log_s_k: f64,
    /// Leading coefficient from paper (2.9), exact rational.
    pub leading_coeff: RationalData,
    /// Present only after an exact `Δ_K` run (`--full-delta`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub log_delta_at_zeta5: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub log_primitive_at_zeta5: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_primitive_coeff_bits: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Objective {
    pub linear_form_id: String,
    pub normalization_id: String,
    pub bound_kind: String,
    pub direction: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SearchBenchmark {
    pub steps: usize,
    pub elapsed_ms: u64,
    pub jobs: usize,
    pub strategy: String,
    pub recorded_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Search {
    pub generator_id: String,
    pub parameter_space_id: String,
    pub seed: String,
    pub next_candidate: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub series_terms: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub benchmark: Option<SearchBenchmark>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MuRecord {
    pub status: MuStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exact: Option<RationalData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upper_bound: Option<RationalData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verifier_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

impl Default for MuRecord {
    fn default() -> Self {
        Self { status: MuStatus::Unavailable, exact: None, upper_bound: None, verifier_id: None, note: None }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProofRecord {
    pub kind: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verifier_id: Option<String>,
    #[serde(default)]
    pub payload: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Checkpoint {
    pub schema_version: u32,
    pub status: String,
    pub target: String,
    pub proof_status: ProofStatus,
    pub objective: Objective,
    pub search: Search,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proof: Option<ProofRecord>,
    #[serde(default)]
    pub mu: MuRecord,
    pub best: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub observed_best: Option<Observation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub polynomial_observed_best: Option<PolynomialHankelObservation>,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
struct CheckpointV1 {
    #[allow(dead_code)]
    schema_version: u32,
    status: String,
    target: String,
    objective: Objective,
    search: Search,
    best: Option<serde_json::Value>,
    observed_best: Option<Observation>,
    updated_at: String,
}

pub fn default_checkpoint_path(target: &str) -> Result<PathBuf, CheckpointError> {
    if !is_project_checkpoint_target(target) {
        return Err(CheckpointError::UnsupportedTarget(target.to_string()));
    }
    Ok(PathBuf::from(format!("projects/hs-problems/checkpoints/{target}/checkpoint.json")))
}

pub fn read_checkpoint(path: &Path) -> Result<Checkpoint, CheckpointError> {
    let data =
        fs::read_to_string(path).map_err(|e| CheckpointError::Io { path: path.to_path_buf(), message: e.to_string() })?;
    let value: serde_json::Value = serde_json::from_str(&data).map_err(|e| CheckpointError::Invalid(e.to_string()))?;
    let version = value
        .get("schema_version")
        .and_then(|v| v.as_u64())
        .ok_or_else(|| CheckpointError::Invalid("missing schema_version".into()))?;
    let checkpoint = match version {
        1 => {
            let legacy: CheckpointV1 = serde_json::from_value(value).map_err(|e| CheckpointError::Invalid(e.to_string()))?;
            migrate_v1(legacy)
        }
        2 => serde_json::from_value(value).map_err(|e| CheckpointError::Invalid(e.to_string()))?,
        other => return Err(CheckpointError::Invalid(format!("unsupported schema_version {other}"))),
    };
    validate_checkpoint(&checkpoint)?;
    Ok(checkpoint)
}

fn migrate_v1(legacy: CheckpointV1) -> Checkpoint {
    Checkpoint {
        schema_version: SCHEMA_VERSION,
        status: legacy.status,
        target: legacy.target,
        proof_status: ProofStatus::Unknown,
        objective: legacy.objective,
        search: legacy.search,
        proof: None,
        mu: MuRecord::default(),
        best: legacy.best,
        observed_best: legacy.observed_best,
        polynomial_observed_best: None,
        updated_at: legacy.updated_at,
    }
}

pub fn validate_checkpoint(cp: &Checkpoint) -> Result<(), CheckpointError> {
    if cp.schema_version != SCHEMA_VERSION {
        return Err(CheckpointError::Invalid(format!("expected schema_version {SCHEMA_VERSION}, got {}", cp.schema_version)));
    }
    if !is_known_target(&cp.target) {
        return Err(CheckpointError::Invalid(format!("unsupported checkpoint target `{}`", cp.target)));
    }
    validate_known_search_contract(&cp.search.generator_id, &cp.search.parameter_space_id)?;
    if let Some(benchmark) = &cp.search.benchmark {
        if benchmark.steps == 0 {
            return Err(CheckpointError::Invalid("search benchmark steps must be positive".into()));
        }
        if benchmark.jobs == 0 {
            return Err(CheckpointError::Invalid("search benchmark jobs must be positive".into()));
        }
        if benchmark.strategy.is_empty() {
            return Err(CheckpointError::Invalid("search benchmark strategy must not be empty".into()));
        }
    }
    match cp.proof_status {
        ProofStatus::Unknown => {}
        ProofStatus::Rational | ProofStatus::Irrational => {
            if cp.proof.is_none() {
                return Err(CheckpointError::Invalid("proof_status requires proof record".into()));
            }
        }
    }
    if let Some(proof) = &cp.proof {
        if !is_known_proof_kind(&proof.kind) {
            return Err(CheckpointError::Invalid(format!("unknown proof kind `{}`", proof.kind)));
        }
    }
    if let Some(observation) = &cp.polynomial_observed_best {
        validate_polynomial_hankel_observation(observation)?;
    }
    if let Some(observation) = &cp.observed_best {
        if observation.kind != OBSERVATION_KIND_FERGUSON {
            return Err(CheckpointError::Invalid("unsupported observation kind".into()));
        }
        if let Some(shift) = observation.shift {
            if shift > RATIONAL_PARAMETER_MAX_SHIFT {
                return Err(CheckpointError::Invalid(format!(
                    "observation shift `{shift}` exceeds rational-parameter max `{RATIONAL_PARAMETER_MAX_SHIFT}`"
                )));
            }
        }
        observation.approximant.ratio().map_err(CheckpointError::Invalid)?;
        observation.error_upper.ratio().map_err(CheckpointError::Invalid)?;
    }
    if cp.best.is_some() {
        return Err(CheckpointError::Invalid("certified best records are not validated by this release".into()));
    }
    validate_mu_record(&cp.mu)?;
    Ok(())
}

fn validate_polynomial_hankel_observation(observation: &PolynomialHankelObservation) -> Result<(), CheckpointError> {
    if observation.kind != OBSERVATION_KIND_POLYNOMIAL_HANKEL {
        return Err(CheckpointError::Invalid("unsupported polynomial observation kind".into()));
    }
    if observation.n == 0 {
        return Err(CheckpointError::Invalid("polynomial observation n must be positive".into()));
    }
    if observation.k != 40 * observation.n
        || observation.capital_n != 3 * observation.n
        || observation.h != 37 * observation.n
    {
        return Err(CheckpointError::Invalid("polynomial observation scaling must match K=40n N=3n h=37n".into()));
    }
    observation.leading_coeff.ratio().map_err(CheckpointError::Invalid)?;
    let has_delta = observation.log_delta_at_zeta5.is_some();
    let has_primitive = observation.log_primitive_at_zeta5.is_some();
    if has_delta != has_primitive {
        return Err(CheckpointError::Invalid(
            "polynomial energy logs require both log_delta_at_zeta5 and log_primitive_at_zeta5".into(),
        ));
    }
    if has_primitive && observation.max_primitive_coeff_bits.is_none() {
        return Err(CheckpointError::Invalid(
            "polynomial energy observation requires max_primitive_coeff_bits".into(),
        ));
    }
    Ok(())
}

fn validate_mu_record(mu: &MuRecord) -> Result<(), CheckpointError> {
    match mu.status {
        MuStatus::Unavailable => {
            if mu.exact.is_some() || mu.upper_bound.is_some() {
                return Err(CheckpointError::Invalid("mu unavailable must not carry exact or upper_bound".into()));
            }
        }
        MuStatus::Exact => {
            let exact = mu.exact.as_ref().ok_or_else(|| CheckpointError::Invalid("mu exact requires exact field".into()))?;
            exact.ratio().map_err(CheckpointError::Invalid)?;
            if mu.upper_bound.is_some() {
                return Err(CheckpointError::Invalid("mu exact must not carry upper_bound".into()));
            }
        }
        MuStatus::UpperBound => {
            let upper = mu
                .upper_bound
                .as_ref()
                .ok_or_else(|| CheckpointError::Invalid("mu upper_bound requires upper_bound field".into()))?;
            upper.ratio().map_err(CheckpointError::Invalid)?;
            if mu.exact.is_some() {
                return Err(CheckpointError::Invalid("mu upper_bound must not carry exact".into()));
            }
        }
    }
    Ok(())
}

pub fn write_checkpoint(path: &Path, cp: &Checkpoint) -> Result<(), CheckpointError> {
    validate_checkpoint(cp)?;
    let parent = path.parent().ok_or_else(|| CheckpointError::Invalid("checkpoint has no parent".into()))?;
    let name =
        path.file_name().ok_or_else(|| CheckpointError::Invalid("checkpoint has no file name".into()))?.to_string_lossy();
    let temporary = parent.join(format!(".{name}.{}.tmp", std::process::id()));
    let mut content = serde_json::to_vec_pretty(cp).map_err(|e| CheckpointError::Invalid(e.to_string()))?;
    content.push(b'\n');
    fs::write(&temporary, content).map_err(|e| CheckpointError::Io { path: temporary.clone(), message: e.to_string() })?;
    fs::rename(&temporary, path)
        .map_err(|e| CheckpointError::Io { path: path.to_path_buf(), message: format!("checkpoint replace failed: {e}") })?;
    Ok(())
}
