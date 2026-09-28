use clap::{Parser, Subcommand};
use hs_checkpoint::{CheckpointError, ProofStatus, SearchBenchmark, read_checkpoint, write_checkpoint};
use hs_verify::{VerifyVerdict, verify_checkpoint};
use num_traits::ToPrimitive;
use std::path::PathBuf;
use std::time::Duration;

use hs_searcher::SearchStrategy;

mod duration;
mod repo;
mod zeta;

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
    },
    /// Recompute and check the observation stored in a checkpoint.
    Check {
        target: String,
        #[arg(long)]
        checkpoint: Option<PathBuf>,
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
        if let Some(approximate) = observation.error_upper.ratio()?.to_f64() {
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
    println!("certified_best: {}", if cp.best.is_some() { "present" } else { "null" });
    println!("mu_status: {}", mu_status_label(&cp.mu.status));
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
        if hs_checkpoint::has_ferguson_search(target) {
            println!("  improve: finite-index Ferguson bounds");
            println!("  check: recompute finite-index prefix");
        } else {
            println!("  improve: unavailable (no Ferguson export registered)");
            println!("  check: unavailable (no finite-index pipeline registered)");
        }
        println!("  verify: rational_equality when proof_status is rational");
    }
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
    println!("level: finite-index bound ({}, jobs={jobs})", strategy.label());
    println!("searched: n={start_index}..{}", cp.search.next_candidate);
    if bound_improved {
        if let (Some(old), Some(new)) = (&before, &cp.observed_best) {
            if let (Some(old_f), Some(new_f)) = (old.error_upper.ratio()?.to_f64(), new.error_upper.ratio()?.to_f64()) {
                println!("best before: n={}, error < {:.12e}", old.n, old_f);
                println!("best after: n={}, error < {:.12e}", new.n, new_f);
            }
        } else if let Some(new) = &cp.observed_best {
            if let Some(new_f) = new.error_upper.ratio()?.to_f64() {
                println!("best after: n={}, error < {:.12e}", new.n, new_f);
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

fn check(path: &PathBuf, target: &str) -> Result<(), String> {
    if !hs_checkpoint::has_ferguson_search(target) {
        return Err(format!("finite-index Ferguson check is not registered for `{target}`"));
    }
    let cp = read_checkpoint(path).map_err(map_err)?;
    ensure_checkpoint_target(&cp, target)?;
    zeta::check_observation(&cp)?;
    let observation = cp.observed_best.as_ref().expect("validated");
    println!("checked n={} finite approximation bound", observation.n);
    Ok(())
}

fn verify(path: &PathBuf, target: &str) -> Result<(), String> {
    let cp = read_checkpoint(path).map_err(map_err)?;
    ensure_checkpoint_target(&cp, target)?;
    let report = verify_checkpoint(&cp)?;
    match report.verdict {
        VerifyVerdict::Verified => {
            println!("{report}");
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
        } => {
            require_repo_root()
                .and_then(|root| resolve_path(&root, &target, checkpoint))
                .and_then(|path| {
                    let budget = match time {
                        Some(value) => Some(duration::parse_duration(&value)?),
                        None => None,
                    };
                    let strategy = SearchStrategy::parse(&strategy)?;
                    improve(&path, &target, steps, series_terms, budget, jobs, write_on_improvement, strategy)
                })
        }
        Command::Check { target, checkpoint } => require_repo_root()
            .and_then(|root| resolve_path(&root, &target, checkpoint))
            .and_then(|path| check(&path, &target)),
        Command::Verify { target, checkpoint } => require_repo_root()
            .and_then(|root| resolve_path(&root, &target, checkpoint))
            .and_then(|path| verify(&path, &target)),
    };
    if let Err(error) = result {
        eprintln!("hs: {error}");
        std::process::exit(1);
    }
}
