use hs_checkpoint::{
    Checkpoint, OBSERVATION_KIND_POLYNOMIAL_HANKEL, POLYNOMIAL_HANKEL_GENERATOR, POLYNOMIAL_NQ_PARAMETER_SPACE,
    PolynomialHankelObservation, RationalData, SearchBenchmark, ZETA5_PAPER_PARAMETER_SPACE, zeta_order,
};
use hs_problems::{
    best_sweep_rows_by_order, polynomial_hankel_log_s_k, polynomial_hankel_params, sweep_polynomial_hankel,
    PolynomialHankelSweepConfig, Zeta5PaperParams, ZetaPolynomialEnergyReport, zeta2_delta_degree,
    zeta2_delta_leading_coeff, zeta2_delta_polynomial, zeta2_entries_with_params, zeta3_delta_degree,
    zeta3_delta_leading_coeff, zeta3_delta_polynomial, zeta3_entries_with_params, zeta5_delta_degree,
    zeta5_delta_leading_coeff, zeta5_delta_polynomial, zeta5_entries, zeta5_log_s_k, zeta5_paper_params,
    zeta_polynomial_energy_report,
};
use hs_types::{Rational, is_zero};
use std::time::{Duration, Instant};

pub struct PolynomialImproveReport {
    pub start_n: usize,
    pub end_n: usize,
    pub completed_steps: usize,
    pub full_delta: bool,
    pub polynomial_observation_updated: bool,
}

struct PolynomialStep {
    params: Zeta5PaperParams,
    leading_coeff: Rational,
    log_s_k: f64,
    energy: Option<ZetaPolynomialEnergyReport>,
    entries_ms: u128,
    delta_ms: Option<u128>,
    leading_match: Option<bool>,
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
    if !hs_checkpoint::has_polynomial_hankel(&cp.target) {
        return Err(format!(
            "polynomial Hankel improve is not registered for `{}`",
            cp.target
        ));
    }

    let mut start_n = cp.search.next_candidate.parse::<usize>().map_err(|e| e.to_string())?;
    if cp.search.generator_id != POLYNOMIAL_HANKEL_GENERATOR
        || cp.search.parameter_space_id != ZETA5_PAPER_PARAMETER_SPACE
    {
        println!(
            "note: checkpoint cursor reset to n=1 for polynomial Hankel (was `{}` + `{}`)",
            cp.search.generator_id, cp.search.parameter_space_id
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

        let step = run_polynomial_step(&cp.target, current, full_delta)?;
        if record_polynomial_observation(cp, &step.params, &step.leading_coeff, step.log_s_k, step.energy.as_ref()) {
            polynomial_observation_updated = true;
        }
        print_polynomial_step(&cp.target, current, &step);
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

/// Grid search over `(N,q)` at fixed `n` with full `Δ_K` energy (offline, expensive).
pub fn improve_polynomial_nq_sweep(
    cp: &mut Checkpoint,
    config: PolynomialHankelSweepConfig,
) -> Result<PolynomialImproveReport, String> {
    if !hs_checkpoint::has_polynomial_hankel(&cp.target) {
        return Err(format!(
            "polynomial Hankel improve is not registered for `{}`",
            cp.target
        ));
    }
    let order = zeta_order(&cp.target).ok_or_else(|| format!("unsupported polynomial target `{}`", cp.target))?;
    let mut sweep_config = config;
    sweep_config.orders = match order {
        2 => &[2][..],
        3 => &[3][..],
        5 => &[5][..],
        other => return Err(format!("polynomial N-q sweep is not registered for zeta order `{other}`")),
    };

    println!("target: {}", cp.target);
    println!("level: polynomial Hankel ({POLYNOMIAL_NQ_PARAMETER_SPACE})");
    println!(
        "mode: full Δ_K grid at n={}, K={}n, N={}..{}n, q={:?}",
        sweep_config.n,
        sweep_config.k_per_n,
        sweep_config.capital_n_min,
        sweep_config.capital_n_max,
        sweep_config.q_values,
    );
    println!("{:<4} {:<3} {:>12} {:>12}", "N", "q", "logP", "logP/n^2");

    let rows = sweep_polynomial_hankel(&sweep_config);
    for row in &rows {
        if row.order != order {
            continue;
        }
        let capital_n_per_n = row.capital_n / row.n;
        println!(
            "{:<4} {:<3} {:>12.3} {:>12.3}",
            capital_n_per_n,
            row.q,
            row.log_primitive_at_zeta,
            row.log_primitive_per_n2,
        );
    }

    let best = best_sweep_rows_by_order(&rows)
        .into_iter()
        .find(|row| row.order == order)
        .ok_or("no successful (N,q) grid point")?;
    let capital_n_per_n = best.capital_n / best.n;
    let params = polynomial_hankel_params(best.n, sweep_config.k_per_n, capital_n_per_n, best.q)?;
    let step = run_polynomial_step_with_params(&cp.target, params, true)?;
    let polynomial_observation_updated =
        record_polynomial_observation(cp, &step.params, &step.leading_coeff, step.log_s_k, step.energy.as_ref());

    println!("\nbest (min logP/n^2): N={capital_n_per_n}n q={} h={}", best.q, best.h);
    print_polynomial_step(&cp.target, best.n, &step);

    cp.search.generator_id = POLYNOMIAL_HANKEL_GENERATOR.into();
    cp.search.parameter_space_id = POLYNOMIAL_NQ_PARAMETER_SPACE.into();
    cp.search.next_candidate = "1".into();
    cp.status = "draft".into();
    cp.updated_at = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();

    Ok(PolynomialImproveReport {
        start_n: best.n,
        end_n: best.n + 1,
        completed_steps: rows.iter().filter(|row| row.order == order).count(),
        full_delta: true,
        polynomial_observation_updated,
    })
}

fn run_polynomial_step(target: &str, n: usize, full_delta: bool) -> Result<PolynomialStep, String> {
    run_polynomial_step_with_params(target, zeta5_paper_params(n), full_delta)
}

fn run_polynomial_step_with_params(
    target: &str,
    params: Zeta5PaperParams,
    full_delta: bool,
) -> Result<PolynomialStep, String> {
    let order = zeta_order(target).ok_or_else(|| format!("unsupported polynomial target `{target}`"))?;
    let entries_started = Instant::now();
    let (leading_coeff, log_s_k, delta_degree, delta_poly) = match target {
        "zeta-2" => {
            let entries = zeta2_entries_with_params(params)?;
            (
                zeta2_delta_leading_coeff(&entries),
                polynomial_hankel_log_s_k(&params),
                zeta2_delta_degree(&entries),
                if full_delta {
                    Some(zeta2_delta_polynomial(&entries)?)
                } else {
                    None
                },
            )
        }
        "zeta-3" => {
            let entries = zeta3_entries_with_params(params)?;
            (
                zeta3_delta_leading_coeff(&entries),
                polynomial_hankel_log_s_k(&params),
                zeta3_delta_degree(&entries),
                if full_delta {
                    Some(zeta3_delta_polynomial(&entries)?)
                } else {
                    None
                },
            )
        }
        "zeta-5" => {
            let entries = zeta5_entries(params.n);
            if entries.params != params {
                return Err("zeta-5 custom (N,q) scaling is not wired yet".into());
            }
            (
                zeta5_delta_leading_coeff(&entries),
                zeta5_log_s_k(&params),
                zeta5_delta_degree(&entries),
                if full_delta {
                    Some(zeta5_delta_polynomial(&entries))
                } else {
                    None
                },
            )
        }
        other => return Err(format!("polynomial Hankel improve is not registered for `{other}`")),
    };
    let entries_ms = entries_started.elapsed().as_millis();

    let mut delta_ms = None;
    let mut leading_match = None;
    let mut energy = None;
    if let Some(delta) = delta_poly {
        let delta_started = Instant::now();
        if delta.len() != delta_degree + 1 {
            return Err(format!(
                "Δ_K coefficient count mismatch at n={}: expected {}, got {}",
                params.n,
                delta_degree + 1,
                delta.len()
            ));
        }
        if is_zero(&delta[delta_degree]) {
            return Err(format!("Δ_K leading coefficient vanished at n={}", params.n));
        }
        leading_match = Some(delta[delta_degree] == leading_coeff);
        let report = zeta_polynomial_energy_report(order, &params, &delta)?;
        energy = Some(report);
        delta_ms = Some(delta_started.elapsed().as_millis());
    }

    Ok(PolynomialStep {
        params,
        leading_coeff,
        log_s_k,
        energy,
        entries_ms,
        delta_ms,
        leading_match,
    })
}

fn print_polynomial_step(target: &str, n: usize, step: &PolynomialStep) {
    println!(
        "n={n} K={} N={} q={} h={} entries_ms={}",
        step.params.k,
        step.params.capital_n,
        step.params.q,
        step.params.h,
        step.entries_ms
    );
    if let Some(ms) = step.delta_ms {
        println!("  delta_ms={ms}");
    }
    println!("  log S_K: {:.3}", step.log_s_k);
    println!(
        "  leading_coeff (2.9): {}/{}",
        step.leading_coeff.to_numerator(),
        step.leading_coeff.to_denominator()
    );
    if let Some(matches) = step.leading_match {
        println!("  leading_coeff matches Δ_K: {matches}");
    }
    if let Some(energy) = &step.energy {
        print_energy_report(target, &step.params, energy);
    }
}

fn print_energy_report(target: &str, params: &Zeta5PaperParams, energy: &ZetaPolynomialEnergyReport) {
    let k = params.k;
    let n = params.n;
    let k2 = (k * k) as f64;
    let n2 = (n * n) as f64;
    println!("  log Delta_K({target}): {:.3}", energy.log_delta_at_zeta);
    println!("  log F_K({target}): {:.3}    /K^2 = {:.5}", energy.log_f_k, energy.log_f_k / k2);
    println!(
        "  log content(F_K): {:.3}    -log content /K^2 = {:.5}",
        energy.log_content_f_k,
        -energy.log_content_f_k / k2
    );
    println!(
        "  log P_K({target}) primitive: {:.3}    /K^2 = {:.5}    /n^2 = {:.3}",
        energy.log_primitive_at_zeta,
        energy.log_primitive_at_zeta / k2,
        energy.log_primitive_at_zeta / n2
    );
    println!(
        "  max |coeff P_K| bits = {}, log H(P_K)/K^2 = {:.4}",
        energy.max_primitive_coeff_bits,
        energy.max_primitive_coeff_bits as f64 * std::f64::consts::LN_2 / k2
    );
}

fn record_polynomial_observation(
    cp: &mut Checkpoint,
    params: &Zeta5PaperParams,
    leading_coeff: &Rational,
    log_s_k: f64,
    energy: Option<&ZetaPolynomialEnergyReport>,
) -> bool {
    let candidate = PolynomialHankelObservation {
        kind: OBSERVATION_KIND_POLYNOMIAL_HANKEL.into(),
        n: params.n,
        k: params.k,
        capital_n: params.capital_n,
        h: params.h,
        q: params.q,
        log_s_k,
        leading_coeff: RationalData::from_ratio(leading_coeff),
        log_delta_at_zeta5: energy.map(|report| report.log_delta_at_zeta),
        log_primitive_at_zeta5: energy.map(|report| report.log_primitive_at_zeta),
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
    let strategy = if report.full_delta {
        if cp.search.parameter_space_id == POLYNOMIAL_NQ_PARAMETER_SPACE {
            "polynomial-hankel-nq-sweep".into()
        } else {
            "polynomial-hankel-full-delta".into()
        }
    } else {
        "polynomial-hankel-entries".into()
    };
    cp.search.benchmark = Some(SearchBenchmark {
        steps: report.completed_steps,
        elapsed_ms,
        jobs,
        strategy,
        recorded_at: chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string(),
    });
}
