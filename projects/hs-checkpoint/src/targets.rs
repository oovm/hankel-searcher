/// Checkpoint targets with collaborative JSON under `projects/targets/<id>/`.
pub const CHECKPOINT_TARGETS: &[&str] = &["zeta-2", "zeta-3", "zeta-5", "zeta-7"];

/// Subset with registered Ferguson finite-index exports in `hs-problems`.
pub const FERGUSON_ZETA_TARGETS: &[&str] = &["zeta-2", "zeta-3", "zeta-5", "zeta-7"];

pub fn is_known_target(target: &str) -> bool {
    CHECKPOINT_TARGETS.contains(&target)
}

pub fn has_ferguson_search(target: &str) -> bool {
    FERGUSON_ZETA_TARGETS.contains(&target)
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
