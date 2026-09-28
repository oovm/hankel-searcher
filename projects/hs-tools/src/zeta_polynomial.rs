use hs_checkpoint::{
    Checkpoint, OBSERVATION_KIND_POLYNOMIAL_HANKEL, POLYNOMIAL_HANKEL_GENERATOR, PolynomialHankelObservation,
    RationalData, SearchBenchmark, ZETA5_PAPER_PARAMETER_SPACE,
};
use hs_problems::{
    zeta5_delta_degree, zeta5_delta_leading_coeff, zeta5_delta_polynomial, zeta5_energy_report, zeta5_entries,
    zeta5_log_s_k, zeta5_paper_params,
};
use num_traits::Zero;
use std::time::{Duration, Instant};

pub struct PolynomialImproveReport {
    pub start_n: usize,
    pub end_n: usize,
    pub completed_steps: usize,
    pub full_delta: bool,
    pub polynomial_observation_updated: bool,
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
        cp.observed_best = None;
    }
    if start_n == 0 {
        return Err("polynomial construction index `n` must be positive".into());
    }

    let deadline = time_budget.map(|budget| Instant::now() + budget);
    let mut current = start_n;
    let mut completed = 0usize;
    let mut polynomial_observation_updated = false;

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
        let log_s = zeta5_log_s_k(&params);
        let mut delta_ms = None;
        let mut leading_match = None;
        let mut energy_report = None;
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
            let energy = zeta5_energy_report(&params, &delta)?;
            print_energy_report(&params, &energy);
            energy_report = Some(energy);
        }
        if record_polynomial_observation(cp, &params, &lead, log_s, energy_report.as_ref()) {
            polynomial_observation_updated = true;
        }

        println!(
            "n={current} K={} N={} h={} entries_ms={entries_ms}",
            params.k, params.capital_n, params.h
        );
        if let Some(ms) = delta_ms {
            println!("  delta_ms={ms}");
        }
        println!("  log S_K: {log_s:.3}");
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
        polynomial_observation_updated,
    })
}

fn print_energy_report(params: &hs_problems::Zeta5PaperParams, energy: &hs_problems::Zeta5EnergyReport) {
    let k = params.k;
    let n = params.n;
    let k2 = (k * k) as f64;
    let n2 = (n * n) as f64;
    println!("  log Delta_K(zeta5): {:.3}", energy.log_delta_at_zeta5);
    println!("  log F_K(zeta5): {:.3}    /K^2 = {:.5}", energy.log_f_k, energy.log_f_k / k2);
    println!(
        "  log content(F_K): {:.3}    -log content /K^2 = {:.5}",
        energy.log_content_f_k,
        -energy.log_content_f_k / k2
    );
    println!(
        "  log P_K(zeta5) primitive: {:.3}    /K^2 = {:.5}    /n^2 = {:.3}",
        energy.log_primitive_at_zeta5,
        energy.log_primitive_at_zeta5 / k2,
        energy.log_primitive_at_zeta5 / n2
    );
    println!(
        "  max |coeff P_K| bits = {}, log H(P_K)/K^2 = {:.4}",
        energy.max_primitive_coeff_bits,
        energy.max_primitive_coeff_bits as f64 * std::f64::consts::LN_2 / k2
    );
}

fn record_polynomial_observation(
    cp: &mut Checkpoint,
    params: &hs_problems::Zeta5PaperParams,
    leading_coeff: &num_rational::Ratio<num_bigint::BigInt>,
    log_s_k: f64,
    energy: Option<&hs_problems::Zeta5EnergyReport>,
) -> bool {
    let candidate = PolynomialHankelObservation {
        kind: OBSERVATION_KIND_POLYNOMIAL_HANKEL.into(),
        n: params.n,
        k: params.k,
        capital_n: params.capital_n,
        h: params.h,
        log_s_k,
        leading_coeff: RationalData::from_ratio(leading_coeff),
        log_delta_at_zeta5: energy.map(|report| report.log_delta_at_zeta5),
        log_primitive_at_zeta5: energy.map(|report| report.log_primitive_at_zeta5),
        max_primitive_coeff_bits: energy.map(|report| report.max_primitive_coeff_bits),
    };
    let replace = match &cp.polynomial_observed_best {
        None => true,
        Some(best) => match (candidate.log_primitive_at_zeta5, best.log_primitive_at_zeta5) {
            (Some(new_log), Some(old_log)) => new_log < old_log,
            (Some(_), None) => true,
            (None, None) => params.n > best.n,
            (None, Some(_)) => false,
        },
    };
    if replace {
        cp.polynomial_observed_best = Some(candidate);
    }
    replace
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
