/// Checkpoint targets with collaborative JSON under `projects/hs-problems/checkpoints/<id>/`.
pub const CHECKPOINT_TARGETS: &[&str] = &["zeta-2", "zeta-3", "zeta-5", "zeta-7"];

/// Verify-only conformance fixtures without project checkpoint JSON.
pub const VERIFY_FIXTURE_TARGETS: &[&str] = &["fixture-half"];

/// Subset with registered Ferguson finite-index exports in `hs-problems`.
pub const FERGUSON_ZETA_TARGETS: &[&str] = &["zeta-2", "zeta-3", "zeta-5", "zeta-7"];

/// Subset with a registered whole-polynomial Hankel construction.
pub const POLYNOMIAL_HANKEL_TARGETS: &[&str] = &["zeta-2", "zeta-3", "zeta-5"];

pub fn is_known_target(target: &str) -> bool {
    CHECKPOINT_TARGETS.contains(&target) || VERIFY_FIXTURE_TARGETS.contains(&target)
}

pub fn is_project_checkpoint_target(target: &str) -> bool {
    CHECKPOINT_TARGETS.contains(&target)
}

pub fn has_ferguson_search(target: &str) -> bool {
    FERGUSON_ZETA_TARGETS.contains(&target)
}

pub fn has_polynomial_hankel(target: &str) -> bool {
    POLYNOMIAL_HANKEL_TARGETS.contains(&target)
}

pub fn zeta_order(target: &str) -> Option<u32> {
    match target {
        "zeta-2" => Some(2),
        "zeta-3" => Some(3),
        "zeta-5" => Some(5),
        "zeta-7" => Some(7),
        _ => None,
    }
}
