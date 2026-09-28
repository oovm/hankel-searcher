use clap::{Parser, Subcommand};
use hs_checkpoint::{CheckpointError, ProofStatus, read_checkpoint, write_checkpoint};
use hs_verify::{VerifyVerdict, verify_checkpoint};
use num_traits::ToPrimitive;
use std::path::PathBuf;
use std::time::{Duration, Instant};

mod duration;
mod repo;
mod zeta3;

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

fn status(path: &PathBuf) -> Result<(), String> {
    let cp = read_checkpoint(path).map_err(map_err)?;
    println!("target: {}", cp.target);
    println!("schema_version: {}", cp.schema_version);
    println!("status: {}", cp.status);
    println!("proof_status: {}", proof_status_label(&cp.proof_status));
    println!("next_candidate: {}", cp.search.next_candidate);
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
    println!("workload_eta: unknown");
    println!("checkpoint: {}", path.display());
    println!("updated_at: {}", cp.updated_at);
    Ok(())
}

fn targets() {
    println!("zeta-3");
    println!("  path: projects/zeta-3/checkpoint.json");
    println!("  improve: finite-index Ferguson bounds");
    println!("  check: recompute finite-index prefix");
    println!("  verify: rational_equality when proof_status is rational");
}

fn improve(
    path: &PathBuf,
    steps: usize,
    terms: usize,
    time_budget: Option<Duration>,
    jobs: usize,
    write_on_improvement: bool,
) -> Result<(), String> {
    if jobs > 1 {
        return Err("parallel improve is not implemented yet; use --jobs 1".into());
    }
    if steps == 0 {
        return Err("steps must be positive".into());
    }
    let mut cp = read_checkpoint(path).map_err(map_err)?;
    let before = cp.observed_best.clone();
    let start_index = cp.search.next_candidate.clone();
    let deadline = time_budget.map(|budget| Instant::now() + budget);
    let mut remaining = steps;
    let mut total_improvements = 0;
    while remaining > 0 {
        if let Some(deadline) = deadline {
            if Instant::now() >= deadline {
                break;
            }
        }
        let batch = remaining;
        let improvements = zeta3::improve(&mut cp, batch, terms)?;
        total_improvements += improvements;
        remaining = 0;
    }
    let bound_improved = match (&before, &cp.observed_best) {
        (None, Some(_)) => true,
        (Some(old), Some(new)) => new.error_upper.ratio()? < old.error_upper.ratio()?,
        _ => false,
    };
    let should_write = if write_on_improvement { bound_improved } else { true };
    if should_write {
        write_checkpoint(path, &cp).map_err(map_err)?;
    }
    println!("target: {}", cp.target);
    println!("level: finite-index bound");
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

fn check(path: &PathBuf) -> Result<(), String> {
    let cp = read_checkpoint(path).map_err(map_err)?;
    zeta3::check_observation(&cp)?;
    let observation = cp.observed_best.as_ref().expect("validated");
    println!("checked n={} finite approximation bound", observation.n);
    Ok(())
}

fn verify(path: &PathBuf) -> Result<(), String> {
    let cp = read_checkpoint(path).map_err(map_err)?;
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
    let checkpoint = repo::resolve_checkpoint(repo_root, "zeta-3", None)?;
    match read_checkpoint(&checkpoint) {
        Ok(cp) => println!(
            "zeta-3 checkpoint: readable (schema_version={}, proof_status={})",
            cp.schema_version,
            proof_status_label(&cp.proof_status)
        ),
        Err(error) => println!("zeta-3 checkpoint: invalid ({error})"),
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
            .and_then(|path| status(&path)),
        Command::Improve {
            target,
            checkpoint,
            steps,
            series_terms,
            time,
            jobs,
            write_on_improvement,
        } => {
            require_repo_root()
                .and_then(|root| resolve_path(&root, &target, checkpoint))
                .and_then(|path| {
                    let budget = match time {
                        Some(value) => Some(duration::parse_duration(&value)?),
                        None => None,
                    };
                    improve(&path, steps, series_terms, budget, jobs, write_on_improvement)
                })
        }
        Command::Check { target, checkpoint } => require_repo_root()
            .and_then(|root| resolve_path(&root, &target, checkpoint))
            .and_then(|path| check(&path)),
        Command::Verify { target, checkpoint } => require_repo_root()
            .and_then(|root| resolve_path(&root, &target, checkpoint))
            .and_then(|path| verify(&path)),
    };
    if let Err(error) = result {
        eprintln!("hs: {error}");
        std::process::exit(1);
    }
}
