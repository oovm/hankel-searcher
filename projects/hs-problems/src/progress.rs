//! Progress logging for long-running offline jobs (`zeta5-hankel-golden`, full energy logs).

use logger::{Filter, event, set_global_filter};

/// Install the stderr subscriber and filter for offline progress.
///
/// Default filter: `hs_problems=info,hs_types=info`. Override with `RUST_LOG`.
pub fn init_progress_logging() {
    console::install_global_subscriber();
    let filter = Filter::from_env("RUST_LOG")
        .ok()
        .flatten()
        .unwrap_or_else(|| Filter::parse("hs_problems=info,hs_types=info").expect("the default progress filter is valid"));
    set_global_filter(filter);
}

/// Emit `info` when `current` hits `1`, every `step` steps, or `total`.
pub fn trace_step(phase: &'static str, current: usize, total: usize, step: usize) {
    if total == 0 {
        return;
    }
    let step = step.max(1);
    if current == 1 || current == total || current % step == 0 {
        let pct = current.saturating_mul(100) / total;
        event!(Info, module_path!(), message = "progress", phase = phase, current = current, total = total, pct = pct);
    }
}
