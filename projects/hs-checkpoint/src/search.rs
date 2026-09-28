use crate::error::CheckpointError;

/// Ferguson search over nonnegative candidate indices.
pub const FERGUSON_INDEX_GENERATOR: &str = "ferguson-index-v1";
/// Index-only parameter space used by current `hs improve`.
pub const NONNEGATIVE_INDEX_SPACE: &str = "nonnegative-index-v1";

/// Planned generator for rational parameter families beyond pure index enumeration.
pub const FERGUSON_PARAMETER_GENERATOR: &str = "ferguson-parameter-v1";
/// Planned parameter space for rational parameter vectors.
pub const RATIONAL_PARAMETER_SPACE: &str = "rational-parameter-v1";

/// Registered checkpoint search contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SearchContract {
    pub generator_id: &'static str,
    pub parameter_space_id: &'static str,
    pub label: &'static str,
    pub implemented: bool,
}

/// Known generator and parameter-space pairs.
pub const SEARCH_CONTRACTS: &[SearchContract] = &[
    SearchContract {
        generator_id: FERGUSON_INDEX_GENERATOR,
        parameter_space_id: NONNEGATIVE_INDEX_SPACE,
        label: "Ferguson nonnegative index",
        implemented: true,
    },
    SearchContract {
        generator_id: FERGUSON_PARAMETER_GENERATOR,
        parameter_space_id: RATIONAL_PARAMETER_SPACE,
        label: "Ferguson rational parameter family",
        implemented: false,
    },
];

pub fn is_unassigned_search_contract(generator_id: &str, parameter_space_id: &str) -> bool {
    generator_id == "unassigned" && parameter_space_id == "unassigned"
}

pub fn find_search_contract(generator_id: &str, parameter_space_id: &str) -> Option<&'static SearchContract> {
    SEARCH_CONTRACTS
        .iter()
        .find(|contract| contract.generator_id == generator_id && contract.parameter_space_id == parameter_space_id)
}

pub fn is_implemented_search_contract(generator_id: &str, parameter_space_id: &str) -> bool {
    if is_unassigned_search_contract(generator_id, parameter_space_id) {
        return true;
    }
    find_search_contract(generator_id, parameter_space_id).is_some_and(|contract| contract.implemented)
}

pub fn validate_known_search_contract(generator_id: &str, parameter_space_id: &str) -> Result<(), CheckpointError> {
    if is_unassigned_search_contract(generator_id, parameter_space_id) {
        return Ok(());
    }
    if generator_id == "unassigned" || parameter_space_id == "unassigned" {
        return Err(CheckpointError::Invalid(
            "generator_id and parameter_space_id must both be unassigned or both name a registered contract".into(),
        ));
    }
    if find_search_contract(generator_id, parameter_space_id).is_some() {
        return Ok(());
    }
    Err(CheckpointError::Invalid(format!(
        "unknown search contract `{generator_id}` + `{parameter_space_id}`"
    )))
}

pub fn improve_error_for_search_contract(generator_id: &str, parameter_space_id: &str) -> Option<String> {
    if is_implemented_search_contract(generator_id, parameter_space_id) {
        return None;
    }
    if let Some(contract) = find_search_contract(generator_id, parameter_space_id) {
        return Some(format!(
            "search contract `{0}` + `{1}` ({2}) is not implemented",
            contract.generator_id,
            contract.parameter_space_id,
            contract.label
        ));
    }
    Some(format!(
        "unknown search contract `{generator_id}` + `{parameter_space_id}`"
    ))
}
