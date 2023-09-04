use clap::{Parser, Subcommand};
use num_bigint::BigInt;
use num_rational::Ratio;
use num_traits::{Signed, ToPrimitive, Zero};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(name = "hs", about = "Hankel search tools")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Continue finite-index approximation search and save an improved observation.
    Improve {
        /// Currently supported target.
        target: String,
        /// Checkpoint path, relative to the current directory by default.
        #[arg(long)]
        checkpoint: Option<PathBuf>,
        /// Number of consecutive candidate indices to evaluate.
        #[arg(long, default_value_t = 1)]
        steps: usize,
        /// Number of terms used in the exact zeta(3) interval.
        #[arg(long, default_value_t = 256)]
        series_terms: usize,
    },
    /// Recompute and check the observation stored in a checkpoint.
    Check {
        target: String,
        #[arg(long)]
        checkpoint: Option<PathBuf>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct RationalData {
    num: String,
    den: String,
}

impl RationalData {
    fn from_ratio(value: &Ratio<BigInt>) -> Self {
        Self { num: value.numer().to_string(), den: value.denom().to_string() }
    }

    fn ratio(&self) -> Result<Ratio<BigInt>, String> {
        let num = self.num.parse::<BigInt>().map_err(|e| e.to_string())?;
        let den = self.den.parse::<BigInt>().map_err(|e| e.to_string())?;
        if den <= BigInt::zero() {
            return Err("rational denominator must be positive".into());
        }
        let ratio = Ratio::new(num, den);
        if RationalData::from_ratio(&ratio).num != self.num || RationalData::from_ratio(&ratio).den != self.den {
            return Err("rational must be canonical".into());
        }
        Ok(ratio)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Observation {
    kind: String,
    n: usize,
    series_terms: usize,
    approximant: RationalData,
    error_upper: RationalData,
}

#[derive(Debug, Serialize, Deserialize)]
struct Objective {
    linear_form_id: String,
    normalization_id: String,
    bound_kind: String,
    direction: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Search {
    generator_id: String,
    parameter_space_id: String,
    seed: String,
    next_candidate: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    series_terms: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Checkpoint {
    schema_version: u32,
    status: String,
    target: String,
    objective: Objective,
    search: Search,
    best: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    observed_best: Option<Observation>,
    updated_at: String,
}

fn checkpoint_path(target: &str, explicit: Option<PathBuf>) -> Result<PathBuf, String> {
    if target != "zeta-3" {
        return Err(format!("unsupported target: {target}"));
    }
    Ok(explicit.unwrap_or_else(|| PathBuf::from("projects/zeta-3/checkpoint.json")))
}

fn read_checkpoint(path: &Path) -> Result<Checkpoint, String> {
    let data = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let checkpoint: Checkpoint = serde_json::from_str(&data).map_err(|e| e.to_string())?;
    if checkpoint.schema_version != 1 || checkpoint.target != "zeta-3" {
        return Err("checkpoint version or target mismatch".into());
    }
    if checkpoint.best.is_some() {
        return Err("this tool does not validate certified best records".into());
    }
    if checkpoint.search.generator_id != "unassigned" && checkpoint.search.generator_id != "ferguson-index-v1" {
        return Err("unknown candidate generator".into());
    }
    if checkpoint.search.parameter_space_id != "unassigned" && checkpoint.search.parameter_space_id != "nonnegative-index-v1" {
        return Err("unknown parameter space".into());
    }
    Ok(checkpoint)
}

fn zeta3_interval(terms: usize) -> Result<(Ratio<BigInt>, Ratio<BigInt>), String> {
    if terms == 0 {
        return Err("series_terms must be positive".into());
    }
    let mut lower = Ratio::from_integer(BigInt::zero());
    for k in 1..=terms {
        let k = BigInt::from(k);
        lower += Ratio::new(BigInt::from(1), k.pow(3));
    }
    // The positive tail sum for k > M is bounded by integral_M^infinity x^-3 dx.
    let m = BigInt::from(terms);
    let upper = &lower + Ratio::new(BigInt::from(1), BigInt::from(2) * m.pow(2));
    Ok((lower, upper))
}

fn observe(n: usize, terms: usize, interval: &(Ratio<BigInt>, Ratio<BigInt>)) -> Result<Observation, String> {
    let count = n.checked_add(2).and_then(|x| x.checked_mul(2)).ok_or("index overflow")?;
    let pair = zeta_3::ferguson_approximant(n, count).map_err(|e| e.to_string())?;
    if pair.q.is_zero() {
        return Err("zero Ferguson denominator".into());
    }
    let approx = pair.p / pair.q;
    let low_error = (&interval.0 - &approx).abs();
    let high_error = (&interval.1 - &approx).abs();
    let bound = low_error.max(high_error);
    Ok(Observation {
        kind: "finite_approximation_error_upper".into(),
        n,
        series_terms: terms,
        approximant: RationalData::from_ratio(&approx),
        error_upper: RationalData::from_ratio(&bound),
    })
}

fn write_checkpoint(path: &Path, cp: &Checkpoint) -> Result<(), String> {
    let parent = path.parent().ok_or("checkpoint has no parent directory")?;
    let name = path.file_name().ok_or("checkpoint has no file name")?.to_string_lossy();
    let temporary = parent.join(format!(".{name}.{}.tmp", std::process::id()));
    let mut content = serde_json::to_vec_pretty(cp).map_err(|e| e.to_string())?;
    content.push(b'\n');
    fs::write(&temporary, content).map_err(|e| e.to_string())?;
    fs::rename(&temporary, path).map_err(|e| format!("checkpoint replace failed: {e}"))?;
    Ok(())
}

fn improve(path: &Path, steps: usize, terms: usize) -> Result<(), String> {
    if steps == 0 {
        return Err("steps must be positive".into());
    }
    let mut cp = read_checkpoint(path)?;
    if let Some(saved_terms) = cp.search.series_terms {
        if saved_terms != terms {
            return Err(format!("series_terms must match checkpoint value {saved_terms}"));
        }
    }
    let next = cp.search.next_candidate.parse::<usize>().map_err(|e| e.to_string())?;
    let end = next.checked_add(steps).ok_or("candidate index overflow")?;
    let interval = zeta3_interval(terms)?;
    let mut improvements = 0;
    for n in next..end {
        let observation = observe(n, terms, &interval)?;
        let is_better = match &cp.observed_best {
            Some(old) => observation.error_upper.ratio()? < old.error_upper.ratio()?,
            None => true,
        };
        if is_better {
            cp.observed_best = Some(observation);
            improvements += 1;
        }
    }
    cp.search.generator_id = "ferguson-index-v1".into();
    cp.search.parameter_space_id = "nonnegative-index-v1".into();
    cp.search.next_candidate = end.to_string();
    cp.search.series_terms = Some(terms);
    // The certified mathematical objective remains unassigned.
    cp.status = "draft".into();
    cp.updated_at = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
    write_checkpoint(path, &cp)?;
    println!("evaluated {steps} indices, found {improvements} improved finite bounds; next index {end}");
    if let Some(best) = &cp.observed_best {
        if let Some(approximate) = best.error_upper.ratio()?.to_f64() {
            println!("best n={} has error <= approximately {:.8e}; exact rational is in JSON", best.n, approximate);
        }
    }
    println!("checkpoint: {}", path.display());
    Ok(())
}

fn check(path: &Path) -> Result<(), String> {
    let cp = read_checkpoint(path)?;
    let observation = cp.observed_best.as_ref().ok_or("checkpoint has no observation")?;
    if observation.kind != "finite_approximation_error_upper" {
        return Err("unsupported observation kind".into());
    }
    let terms = cp.search.series_terms.unwrap_or(observation.series_terms);
    if terms != observation.series_terms {
        return Err("observation and search precision mismatch".into());
    }
    let next = cp.search.next_candidate.parse::<usize>().map_err(|e| e.to_string())?;
    if next == 0 {
        return Err("observation exists without searched candidates".into());
    }
    let interval = zeta3_interval(terms)?;
    let mut best: Option<Observation> = None;
    for n in 0..next {
        let candidate = observe(n, terms, &interval)?;
        let better = match &best {
            Some(old) => candidate.error_upper.ratio()? < old.error_upper.ratio()?,
            None => true,
        };
        if better {
            best = Some(candidate);
        }
    }
    let actual = best.ok_or("no candidates were checked")?;
    if actual.n != observation.n {
        return Err("saved best index is not minimal".into());
    }
    if actual.approximant.num != observation.approximant.num
        || actual.approximant.den != observation.approximant.den
        || actual.error_upper.num != observation.error_upper.num
        || actual.error_upper.den != observation.error_upper.den
    {
        return Err("saved observation does not match recomputation".into());
    }
    println!("checked n={} finite approximation bound", observation.n);
    Ok(())
}

fn main() {
    let cli = Cli::parse();
    let result = match cli.command {
        Command::Improve { target, checkpoint, steps, series_terms } => {
            checkpoint_path(&target, checkpoint).and_then(|path| improve(&path, steps, series_terms))
        }
        Command::Check { target, checkpoint } => checkpoint_path(&target, checkpoint).and_then(|path| check(&path)),
    };
    if let Err(error) = result {
        eprintln!("hs: {error}");
        std::process::exit(1);
    }
}
