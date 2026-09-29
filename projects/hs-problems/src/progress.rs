//! Progress tracing for long-running offline jobs (`zeta5-hankel-golden`, full energy logs).

use tracing_subscriber::EnvFilter;

/// Install a stderr `tracing` subscriber for offline progress.
///
/// Default filter: `hs_problems=info,hs_types=info`. Override with `RUST_LOG`.
pub fn init_progress_tracing() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("hs_problems=info,hs_types=info"));
    let _ = tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(true)
        .with_writer(std::io::stderr)
        .try_init();
}

/// Emit `info` when `current` hits `1`, every `step` steps, or `total`.
pub fn trace_step(phase: &'static str, current: usize, total: usize, step: usize) {
    if total == 0 {
        return;
    }
    let step = step.max(1);
    if current == 1 || current == total || current % step == 0 {
        let pct = current.saturating_mul(100) / total;
        tracing::info!(phase, current, total, pct, "progress");
    }
}
