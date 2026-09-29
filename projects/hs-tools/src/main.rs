use clap::{Parser, Subcommand};
use hs_checkpoint::{CheckpointError, ProofStatus, SearchBenchmark, rational_to_f64, read_checkpoint, write_checkpoint};
use hs_verify::{VerifyVerdict, verify_checkpoint};
use std::path::PathBuf;
use std::time::Duration;

use hs_searcher::SearchStrategy;

mod duration;
mod repo;
mod zeta;
mod zeta_polynomial;

#[derive(Parser)]
#[command(name = "hs", about = "Hankel search tools")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// List compiled targets and supported capabilities.
    Targets,
    /// Print a read-only checkpoint summary.
    Status {
        target: String,
        #[arg(long)]
        checkpoint: Option<PathBuf>,
    },
    /// Continue finite-index approximation search and save an improved observation.
    Improve {
        target: String,
        #[arg(long)]
        checkpoint: Option<PathBuf>,
        #[arg(long, default_value_t = 1)]
        steps: usize,
        #[arg(long, default_value_t = 256)]
        series_terms: usize,
        #[arg(long)]
        time: Option<String>,
        #[arg(long, default_value_t = 1)]
        jobs: usize,
        #[arg(long)]
        write_on_improvement: bool,
        #[arg(long, default_value = "enumerate")]
        strategy: String,
        /// Run the Zeta5 whole-polynomial Hankel line (`polynomial-hankel-v1`) instead of Ferguson.
        #[arg(long)]
        polynomial: bool,
        /// With `--polynomial`, also compute exact `Δ_K` (very expensive).
        #[arg(long)]
        full_delta: bool,
        /// With `--polynomial`, grid-search `(N,q)` at `n=1` and record best `log P_K` (implies `--full-delta`).
        #[arg(long)]
        nq_sweep: bool,
    },
    /// Recompute and check the observation stored in a checkpoint.
    Check {
        target: String,
        #[arg(long)]
        checkpoint: Option<PathBuf>,
        /// Fast `n=1` polynomial Hankel golden (`log S_K`, scaling, leading-coeff formula). CI-safe.
        #[arg(long)]
        polynomial_golden: bool,
        /// With `--polynomial-golden`, also compute exact `Δ_K` (offline only, tens of minutes).
        #[arg(long)]
        full_delta: bool,
    },
    /// Verify a registered rational or irrational proof record.
    Verify {
        target: String,
        #[arg(long)]
        checkpoint: Option<PathBuf>,
    },
    /// Read-only toolchain and repository checks.
    Doctor,
}

fn map_err(error: CheckpointError) -> String {
    error.to_string()
}

fn require_repo_root() -> Result<PathBuf, String> {
    repo::find_repo_root(&std::env::current_dir().map_err(|e| e.to_string())?)
        .ok_or_else(|| "could not locate Hankel Searcher repository root from current directory".into())
}

fn resolve_path(
    repo_root: &PathBuf,
    target: &str,
    explicit: Option<PathBuf>,
) -> Result<PathBuf, String> {
    repo::resolve_checkpoint(repo_root, target, explicit)
}

fn ensure_checkpoint_target(cp: &hs_checkpoint::Checkpoint, target: &str) -> Result<(), String> {
    if !hs_checkpoint::is_known_target(target) {
        return Err(format!("unsupported target `{target}`"));
    }
    if cp.target != target {
        return Err(format!("checkpoint target `{}` does not match `{target}`", cp.target));
    }
    Ok(())
}

fn status(path: &PathBuf, target: &str) -> Result<(), String> {
    let cp = read_checkpoint(path).map_err(map_err)?;
    ensure_checkpoint_target(&cp, target)?;
    println!("target: {}", cp.target);
    println!("schema_version: {}", cp.schema_version);
    println!("status: {}", cp.status);
    println!("proof_status: {}", proof_status_label(&cp.proof_status));
    println!("next_candidate: {}", cp.search.next_candidate);
    println!("generator_id: {}", cp.search.generator_id);
    println!("parameter_space_id: {}", cp.search.parameter_space_id);
    if let Some(terms) = cp.search.series_terms {
        println!("series_terms: {}", terms);
    }
    if let Some(observation) = &cp.observed_best {
        println!("observed_best.n: {}", observation.n);
        if let Some(shift) = observation.shift {
            println!("observed_best.shift: {shift}");
        }
        if let Some(approximate) = rational_to_f64(&observation.error_upper.ratio()?) {
            println!("observed_best.error_upper (approx display): <= {:.12e}", approximate);
        }
        println!(
            "observed_best.error_upper (exact): {}/{}",
            observation.error_upper.num,
            observation.error_upper.den
        );
    } else {
        println!("observed_best: none");
    }
    if let Some(observation) = &cp.polynomial_observed_best {
        println!("polynomial_observed_best.n: {}", observation.n);
        println!(
            "polynomial_observed_best.scaling: K={} N={} h={} q={}",
            observation.k,
            observation.capital_n,
            observation.h,
            observation.q
        );
        println!("polynomial_observed_best.log_s_k: {:.3}", observation.log_s_k);
        if let Some(log_primitive) = observation.log_primitive_at_zeta5 {
            println!("polynomial_observed_best.log_primitive_at_zeta5: {:.3}", log_primitive);
            println!(
                "polynomial_observed_best.log_primitive / n^2: {:.3}",
                log_primitive / (observation.n * observation.n) as f64
            );
        }
    } else {
        println!("polynomial_observed_best: none");
    }
    println!("certified_best: {}", if cp.best.is_some() { "present" } else { "null" });
    println!("mu_status: {}", mu_status_label(&cp.mu.status));
    match cp.mu.status {
        hs_checkpoint::MuStatus::Exact => {
            if let Some(exact) = &cp.mu.exact {
                println!("mu_exact: {}/{}", exact.num, exact.den);
            }
        }
        hs_checkpoint::MuStatus::UpperBound => {
            if let Some(bound) = &cp.mu.upper_bound {
                println!("mu_upper_bound: {}/{}", bound.num, bound.den);
            }
            if let Some(verifier) = &cp.mu.verifier_id {
                println!("mu_verifier_id: {verifier}");
            }
        }
        hs_checkpoint::MuStatus::Unavailable => {}
    }
    println!("proof_record: {}", if cp.proof.is_some() { "present" } else { "null" });
    println!("time_to_proof_from_scratch: unknown (no completeness guarantee)");
    println!("workload_eta: {}", workload_eta_label(&cp));
    println!("checkpoint: {}", path.display());
    println!("updated_at: {}", cp.updated_at);
    Ok(())
}

fn targets() {
    println!("search contracts:");
    for contract in hs_checkpoint::SEARCH_CONTRACTS {
        let status = if contract.implemented { "implemented" } else { "planned" };
        println!(
            "  {} + {} ({}) [{status}]",
            contract.generator_id,
            contract.parameter_space_id,
            contract.label
        );
    }
    println!("checkpoint targets:");
    for target in hs_checkpoint::CHECKPOINT_TARGETS {
        println!("{target}");
        println!("  path: projects/hs-problems/checkpoints/{target}/checkpoint.json");
        if hs_checkpoint::has_polynomial_hankel(target) {
            println!("  improve: Ferguson finite-index, or `--polynomial` for paper Hankel construction");
            println!("  check: `hs check {target} --polynomial-golden` (fast, in cargo test)");
            println!("  offline full Δ_K: `hs check {target} --polynomial-golden --full-delta`");
            println!(
                "  offline bin: cargo run --release -p hs-problems --features offline-golden --bin {}-hankel-golden",
                target
            );
            println!("  verify: polynomial_irrationality when proof_status is irrational");
        }
        if hs_checkpoint::has_ferguson_search(target) {
            println!("  improve: finite-index Ferguson bounds or rational-parameter family");
            println!("  check: recompute finite-index or parameter prefix");
        } else if !hs_checkpoint::has_polynomial_hankel(target) {
            println!("  improve: unavailable (no Ferguson export registered)");
            println!("  check: unavailable (no finite-index pipeline registered)");
        }
        println!("  verify: rational_equality when proof_status is rational");
    }
}

fn improve_polynomial(
    path: &PathBuf,
    target: &str,
    steps: usize,
    time_budget: Option<Duration>,
    jobs: usize,
    write_on_improvement: bool,
    full_delta: bool,
    nq_sweep: bool,
) -> Result<(), String> {
    if !hs_checkpoint::has_polynomial_hankel(target) {
        return Err(format!("polynomial Hankel improve is not registered for `{target}`"));
    }
    let mut cp = read_checkpoint(path).map_err(map_err)?;
    ensure_checkpoint_target(&cp, target)?;
    let started = std::time::Instant::now();
    let report = if nq_sweep {
        if jobs > 1 {
            return Err("polynomial N-q sweep does not support --jobs > 1 yet".into());
        }
        let _ = time_budget;
        let _ = steps;
        zeta_polynomial::improve_polynomial_nq_sweep(&mut cp, hs_problems::PolynomialHankelSweepConfig::default())?
    } else {
        zeta_polynomial::improve_polynomial(&mut cp, steps, time_budget, jobs, full_delta)?
    };
    let elapsed_ms = started.elapsed().as_millis() as u64;
    let should_write = if write_on_improvement {
        report.polynomial_observation_updated
    } else {
        report.completed_steps > 0
    };
    if should_write {
        zeta_polynomial::record_benchmark(&mut cp, &report, elapsed_ms, jobs);
        write_checkpoint(path, &cp).map_err(map_err)?;
    }
    println!("searched: n={}..{}", report.start_n, report.end_n);
    println!("completed construction indices: {}", report.completed_steps);
    if let Some(observation) = &cp.polynomial_observed_best {
        println!(
            "polynomial_observed_best: n={} log_s_k={:.3}",
            observation.n,
            observation.log_s_k
        );
        if let Some(log_primitive) = observation.log_primitive_at_zeta5 {
            println!(
                "polynomial_observed_best.log_primitive / n^2: {:.3}",
                log_primitive / (observation.n * observation.n) as f64
            );
        }
    }
    if !should_write {
        println!("checkpoint: not written (--write-on-improvement and no polynomial observation update)");
    } else {
        println!("checkpoint: {}", path.display());
    }
    println!("proof status: no uniform bound registered");
    Ok(())
}

fn improve(
    path: &PathBuf,
    target: &str,
    steps: usize,
    terms: usize,
    time_budget: Option<Duration>,
    jobs: usize,
    write_on_improvement: bool,
    strategy: SearchStrategy,
) -> Result<(), String> {
    if steps == 0 {
        return Err("steps must be positive".into());
    }
    if jobs == 0 {
        return Err("jobs must be positive".into());
    }
    if !hs_checkpoint::is_known_target(target) {
        return Err(format!("unsupported target `{target}`"));
    }
    if !hs_checkpoint::has_ferguson_search(target) {
        return Err(format!("finite-index Ferguson search is not registered for `{target}`"));
    }
    let mut cp = read_checkpoint(path).map_err(map_err)?;
    ensure_checkpoint_target(&cp, target)?;
    if let Some(message) =
        hs_checkpoint::improve_error_for_search_contract(&cp.search.generator_id, &cp.search.parameter_space_id)
    {
        return Err(message);
    }
    let before = cp.observed_best.clone();
    let start_index = cp.search.next_candidate.clone();
    let parameter_contract = zeta::uses_parameter_contract(&cp);
    let started = std::time::Instant::now();
    let report = zeta::improve(&mut cp, steps, terms, time_budget, strategy, jobs)?;
    let elapsed_ms = started.elapsed().as_millis() as u64;
    let bound_improved = report.bound_improved;
    let total_improvements = report.improvements;
    let should_write = if write_on_improvement { bound_improved } else { true };
    if should_write {
        cp.search.benchmark = Some(SearchBenchmark {
            steps: report.completed_steps,
            elapsed_ms,
            jobs,
            strategy: strategy.label().into(),
            recorded_at: chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string(),
        });
        write_checkpoint(path, &cp).map_err(map_err)?;
    }
    println!("target: {}", cp.target);
    if parameter_contract {
        println!("level: rational-parameter bound ({}, jobs={jobs})", strategy.label());
        println!(
            "searched: {}",
            zeta::format_parameter_cursor(
                start_index.parse().unwrap_or(0),
                cp.search.next_candidate.parse().unwrap_or(0)
            )
        );
    } else {
        println!("level: finite-index bound ({}, jobs={jobs})", strategy.label());
        println!("searched: n={start_index}..{}", cp.search.next_candidate);
    }
    if bound_improved {
        if let (Some(old), Some(new)) = (&before, &cp.observed_best) {
            if let (Some(old_f), Some(new_f)) = (
                rational_to_f64(&old.error_upper.ratio()?),
                rational_to_f64(&new.error_upper.ratio()?),
            ) {
                if parameter_contract {
                    let old_shift = old.shift.unwrap_or(0);
                    let new_shift = new.shift.unwrap_or(0);
                    println!(
                        "best before: n={}, shift={}, error < {:.12e}",
                        old.n, old_shift, old_f
                    );
                    println!(
                        "best after: n={}, shift={}, error < {:.12e}",
                        new.n, new_shift, new_f
                    );
                } else {
                    println!("best before: n={}, error < {:.12e}", old.n, old_f);
                    println!("best after: n={}, error < {:.12e}", new.n, new_f);
                }
            }
        } else if let Some(new) = &cp.observed_best {
            if let Some(new_f) = rational_to_f64(&new.error_upper.ratio()?) {
                if parameter_contract {
                    println!(
                        "best after: n={}, shift={}, error < {:.12e}",
                        new.n,
                        new.shift.unwrap_or(0),
                        new_f
                    );
                } else {
                    println!("best after: n={}, error < {:.12e}", new.n, new_f);
                }
            }
        }
    } else {
        println!("best before: unchanged");
        println!("coverage progress: yes");
        println!("bound improvement: no");
    }
    println!("improved bounds this run: {total_improvements}");
    if !should_write {
        println!("checkpoint: not written (--write-on-improvement and no bound improvement)");
    } else {
        println!("checkpoint: {}", path.display());
    }
    println!("proof status: no uniform bound registered");
    Ok(())
}

fn check_polynomial_golden(target: &str, full_delta: bool) -> Result<(), String> {
    if !hs_checkpoint::has_polynomial_hankel(target) {
        return Err(format!("polynomial Hankel is not registered for `{target}`"));
    }
    if full_delta {
        hs_problems::init_progress_tracing();
        eprintln!("warning: full Δ_K golden is offline only — expect tens of minutes, not for CI");
        println!("checking polynomial Hankel golden for {target} (n=1, full Δ_K energy)");
        let energy = match target {
            "zeta-2" => hs_problems::zeta2_polynomial_golden_full()?,
            "zeta-3" => hs_problems::zeta3_polynomial_golden_full()?,
            "zeta-5" => {
                let report = hs_problems::zeta5_polynomial_golden_full()?;
                print_zeta5_full_golden(report);
                return Ok(());
            }
            other => return Err(format!("polynomial golden check is not registered for `{other}`")),
        };
        println!("polynomial Hankel golden (full): ok");
        println!("  max |coeff P_K| bits = {}", energy.max_primitive_coeff_bits);
        println!("  log S_K = {:.3}", energy.log_s_k);
        println!("  log Delta_K({target}) = {:.3}", energy.log_delta_at_zeta);
        println!("  log P_K({target}) / n^2 = {:.3}", energy.log_primitive_at_zeta);
    } else {
        println!("checking polynomial Hankel golden for {target} (n=1, fast gate)");
        match target {
            "zeta-2" => hs_problems::zeta2_polynomial_golden_fast()?,
            "zeta-3" => hs_problems::zeta3_polynomial_golden_fast()?,
            "zeta-5" => hs_problems::zeta5_polynomial_golden_fast()?,
            other => return Err(format!("polynomial golden check is not registered for `{other}`")),
        }
        println!("polynomial Hankel golden (fast): ok");
        println!("  paper scaling K=40 N=3 h=37");
        println!("  log S_K and (2.9) leading-coeff formula");
        println!("  note: pass `--full-delta` for exact Δ_K energy logs (offline only)");
    }
    Ok(())
}

fn print_zeta5_full_golden(energy: hs_problems::Zeta5EnergyReport) {
    println!("polynomial Hankel golden (full): ok");
    println!("  max |coeff P_K| bits = {}", energy.max_primitive_coeff_bits);
    println!("  log S_K = {:.3}", energy.log_s_k);
    println!("  log Delta_K(zeta5) = {:.3}", energy.log_delta_at_zeta5);
    println!("  log P_K(zeta5) / n^2 = {:.3}", energy.log_primitive_at_zeta5);
}

fn check(path: &PathBuf, target: &str) -> Result<(), String> {
    if !hs_checkpoint::has_ferguson_search(target) {
        return Err(format!("finite-index Ferguson check is not registered for `{target}`"));
    }
    let cp = read_checkpoint(path).map_err(map_err)?;
    ensure_checkpoint_target(&cp, target)?;
    zeta::check_observation(&cp)?;
    let observation = cp.observed_best.as_ref().expect("validated");
    if zeta::uses_parameter_contract(&cp) {
        println!(
            "checked n={} shift={} finite approximation bound",
            observation.n,
            observation.shift.unwrap_or(0)
        );
    } else {
        println!("checked n={} finite approximation bound", observation.n);
    }
    Ok(())
}

fn verify(path: &PathBuf, target: &str) -> Result<(), String> {
    let cp = read_checkpoint(path).map_err(map_err)?;
    ensure_checkpoint_target(&cp, target)?;
    let report = verify_checkpoint(&cp)?;
    match report.verdict {
        VerifyVerdict::Verified => {
            println!("{report}");
            if let Some(hint) = &report.mu_checkpoint_hint {
                println!("{hint}");
            }
            Ok(())
        }
        VerifyVerdict::Unsupported => {
            eprintln!("verify: unsupported for {0}: {report}", cp.target);
            std::process::exit(2);
        }
    }
}

fn doctor(repo_root: &PathBuf) -> Result<(), String> {
    println!("repo_root: {}", repo_root.display());
    println!("cwd: {}", std::env::current_dir().map_err(|e| e.to_string())?.display());
    let lean_prove = repo_root.join("projects/lean-prove/lakefile.toml");
    println!(
        "lean-prove: {}",
        if lean_prove.is_file() { "present" } else { "missing lakefile" }
    );
    println!("checkpoint targets:");
    for target in hs_checkpoint::CHECKPOINT_TARGETS {
        match repo::resolve_checkpoint(repo_root, target, None) {
            Ok(path) => match read_checkpoint(&path) {
                Ok(cp) => {
                    let ferguson = if hs_checkpoint::has_ferguson_search(target) { "yes" } else { "no" };
                    println!(
                        "  {target}: readable (schema_version={}, proof_status={}, ferguson={ferguson}, path={})",
                        cp.schema_version,
                        proof_status_label(&cp.proof_status),
                        path.display()
                    );
                }
                Err(error) => println!("  {target}: invalid ({error})"),
            },
            Err(error) => println!("  {target}: missing ({error})"),
        }
    }
    println!("cargo: {}", if which_cargo() { "available" } else { "not found in PATH" });
    Ok(())
}

fn which_cargo() -> bool {
    std::process::Command::new("cargo").arg("--version").output().map(|o| o.status.success()).unwrap_or(false)
}

fn proof_status_label(status: &ProofStatus) -> &'static str {
    match status {
        ProofStatus::Unknown => "unknown",
        ProofStatus::Rational => "rational",
        ProofStatus::Irrational => "irrational",
    }
}

fn workload_eta_label(cp: &hs_checkpoint::Checkpoint) -> String {
    match &cp.search.benchmark {
        Some(benchmark) => format!(
            "~{}ms for last {} candidate(s) at jobs={} ({}) on this machine, not a proof-time estimate",
            benchmark.elapsed_ms,
            benchmark.steps,
            benchmark.jobs,
            benchmark.strategy
        ),
        None => "unknown".into(),
    }
}

fn mu_status_label(status: &hs_checkpoint::MuStatus) -> &'static str {
    match status {
        hs_checkpoint::MuStatus::Unavailable => "unavailable",
        hs_checkpoint::MuStatus::Exact => "exact",
        hs_checkpoint::MuStatus::UpperBound => "upper_bound",
    }
}

fn main() {
    let cli = Cli::parse();
    let result = match cli.command {
        Command::Targets => {
            targets();
            Ok(())
        }
        Command::Doctor => require_repo_root().and_then(|root| doctor(&root)),
        Command::Status { target, checkpoint } => require_repo_root()
            .and_then(|root| resolve_path(&root, &target, checkpoint))
            .and_then(|path| status(&path, &target)),
        Command::Improve {
            target,
            checkpoint,
            steps,
            series_terms,
            time,
            jobs,
            write_on_improvement,
            strategy,
            polynomial,
            full_delta,
            nq_sweep,
        } => {
            require_repo_root()
                .and_then(|root| resolve_path(&root, &target, checkpoint))
                .and_then(|path| {
                    let budget = match time {
                        Some(value) => Some(duration::parse_duration(&value)?),
                        None => None,
                    };
                    if polynomial {
                        improve_polynomial(
                            &path,
                            &target,
                            steps,
                            budget,
                            jobs,
                            write_on_improvement,
                            full_delta || nq_sweep,
                            nq_sweep,
                        )
                    } else {
                        let strategy = SearchStrategy::parse(&strategy)?;
                        improve(
                            &path,
                            &target,
                            steps,
                            series_terms,
                            budget,
                            jobs,
                            write_on_improvement,
                            strategy,
                        )
                    }
                })
        }
        Command::Check {
            target,
            checkpoint,
            polynomial_golden,
            full_delta,
        } => {
            if polynomial_golden {
                require_repo_root().and_then(|_| check_polynomial_golden(&target, full_delta))
            } else {
                require_repo_root()
                    .and_then(|root| resolve_path(&root, &target, checkpoint))
                    .and_then(|path| check(&path, &target))
            }
        }
        Command::Verify { target, checkpoint } => require_repo_root()
            .and_then(|root| resolve_path(&root, &target, checkpoint))
            .and_then(|path| verify(&path, &target)),
    };
    if let Err(error) = result {
        eprintln!("hs: {error}");
        std::process::exit(1);
    }
}
