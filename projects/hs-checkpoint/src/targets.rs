/// Stable checkpoint target identifiers with Ferguson finite-index support.
pub const FERGUSON_ZETA_TARGETS: &[&str] = &["zeta-2", "zeta-3", "zeta-5"];

pub fn is_known_target(target: &str) -> bool {
    FERGUSON_ZETA_TARGETS.contains(&target)
}

pub fn zeta_order(target: &str) -> Option<u32> {
    match target {
        "zeta-2" => Some(2),
        "zeta-3" => Some(3),
        "zeta-5" => Some(5),
        _ => None,
    }
}
