use hs_checkpoint::{
    Checkpoint, POLYNOMIAL_HANKEL_GENERATOR, SearchBenchmark, ZETA5_PAPER_PARAMETER_SPACE,
};
use hs_problems::{
    zeta5_delta_degree, zeta5_delta_leading_coeff, zeta5_delta_polynomial, zeta5_entries, zeta5_paper_params,
};
use num_traits::Zero;
use std::time::{Duration, Instant};

pub struct PolynomialImproveReport {
    pub start_n: usize,
    pub end_n: usize,
    pub completed_steps: usize,
    pub full_delta: bool,
}

pub fn improve_polynomial(
    cp: &mut Checkpoint,
    steps: usize,
    time_budget: Option<Duration>,
    jobs: usize,
    full_delta: bool,
) -> Result<PolynomialImproveReport, String> {
    if steps == 0 {
        return Err("steps must be positive".into());
    }
    if jobs == 0 {
        return Err("jobs must be positive".into());
    }
    if jobs > 1 {
        return Err("polynomial Hankel improve does not support --jobs > 1 yet".into());
    }
    if cp.target != "zeta-5" {
        return Err(format!(
            "polynomial Hankel improve is only registered for `zeta-5`, not `{}`",
            cp.target
        ));
    }

    let mut start_n = cp.search.next_candidate.parse::<usize>().map_err(|e| e.to_string())?;
    if cp.search.generator_id != POLYNOMIAL_HANKEL_GENERATOR {
        println!(
            "note: checkpoint cursor reset to n=1 for polynomial Hankel (was Ferguson `{}`)",
            cp.search.next_candidate
        );
        start_n = 1;
        cp.search.next_candidate = "1".into();
    }
    if start_n == 0 {
        return Err("polynomial construction index `n` must be positive".into());
    }

    let deadline = time_budget.map(|budget| Instant::now() + budget);
    let mut current = start_n;
    let mut completed = 0usize;

    println!("target: {}", cp.target);
    println!("level: polynomial Hankel ({ZETA5_PAPER_PARAMETER_SPACE})");
    if full_delta {
        println!("mode: full Δ_K (exact rational, very expensive at large h)");
    } else {
        println!("mode: entries + leading-coeff check only (pass --full-delta for Δ_K)");
    }

    while completed < steps {
        if let Some(deadline) = deadline {
            if Instant::now() >= deadline {
                println!("time budget exhausted after {completed} construction index(es)");
                break;
            }
        }

        let params = zeta5_paper_params(current);
        let entries_started = Instant::now();
        let entries = zeta5_entries(current);
        let entries_ms = entries_started.elapsed().as_millis();

        let lead = zeta5_delta_leading_coeff(&entries);
        let mut delta_ms = None;
        let mut leading_match = None;
        if full_delta {
            let delta_started = Instant::now();
            let delta = zeta5_delta_polynomial(&entries);
            delta_ms = Some(delta_started.elapsed().as_millis());
            let degree = zeta5_delta_degree(&entries);
            if delta.len() != degree + 1 {
                return Err(format!(
                    "Δ_K coefficient count mismatch at n={current}: expected {}, got {}",
                    degree + 1,
                    delta.len()
                ));
            }
            if delta[degree].is_zero() {
                return Err(format!("Δ_K leading coefficient vanished at n={current}"));
            }
            leading_match = Some(delta[degree] == lead);
        }

        println!(
            "n={current} K={} N={} h={} entries_ms={entries_ms}",
            params.k, params.capital_n, params.h
        );
        if let Some(ms) = delta_ms {
            println!("  delta_ms={ms}");
        }
        println!("  leading_coeff (2.9): {}/{}", lead.numer(), lead.denom());
        if let Some(matches) = leading_match {
            println!("  leading_coeff matches Δ_K: {matches}");
        }

        completed += 1;
        current += 1;
    }

    cp.search.generator_id = POLYNOMIAL_HANKEL_GENERATOR.into();
    cp.search.parameter_space_id = ZETA5_PAPER_PARAMETER_SPACE.into();
    cp.search.next_candidate = current.to_string();
    cp.status = "draft".into();
    cp.updated_at = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();

    Ok(PolynomialImproveReport {
        start_n,
        end_n: current,
        completed_steps: completed,
        full_delta,
    })
}

pub fn record_benchmark(cp: &mut Checkpoint, report: &PolynomialImproveReport, elapsed_ms: u64, jobs: usize) {
    cp.search.benchmark = Some(SearchBenchmark {
        steps: report.completed_steps,
        elapsed_ms,
        jobs,
        strategy: if report.full_delta {
            "polynomial-hankel-full-delta".into()
        } else {
            "polynomial-hankel-entries".into()
        },
        recorded_at: chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string(),
    });
}
