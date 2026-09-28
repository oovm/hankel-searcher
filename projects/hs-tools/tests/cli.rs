use hs_checkpoint::{ProofRecord, ProofStatus, read_checkpoint, write_checkpoint};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};
use tempfile::tempdir_in;

fn hs_bin() -> String {
    std::env::var("CARGO_BIN_EXE_hs").expect("CARGO_BIN_EXE_hs")
}

fn repo_root() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

#[test]
fn verify_exits_unsupported_when_no_proof() {
    let output = Command::new(hs_bin())
        .current_dir(repo_root())
        .args(["verify", "zeta-3"])
        .output()
        .expect("spawn hs");
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("unsupported"));
}

#[test]
fn status_reads_project_checkpoint() {
    let output = Command::new(hs_bin())
        .current_dir(repo_root())
        .args(["status", "zeta-3"])
        .output()
        .expect("spawn hs");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("proof_status: unknown"));
    assert!(stdout.contains("schema_version: 2"));
}

#[test]
fn doctor_finds_repo_root() {
    let output = Command::new(hs_bin()).current_dir(repo_root()).arg("doctor").output().expect("spawn hs");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("repo_root:"));
    assert!(stdout.contains("zeta-3 checkpoint: readable"));
}

#[test]
fn check_rejects_tampered_observation() {
    let dir = tempdir_in(repo_root()).unwrap();
    let path = dir.path().join("checkpoint.json");
    let source = repo_root().join("projects/targets/zeta-3/checkpoint.json");
    std::fs::copy(source, &path).expect("copy checkpoint");
    let mut cp = read_checkpoint(&path).unwrap();
    let observation = cp.observed_best.as_mut().expect("observed");
    observation.error_upper.num = "1".into();
    write_checkpoint(&path, &cp).unwrap();
    let output = Command::new(hs_bin())
        .current_dir(repo_root())
        .args(["check", "zeta-3", "--checkpoint"])
        .arg(&path)
        .output()
        .expect("spawn hs");
    assert!(!output.status.success());
}

#[test]
fn verify_accepts_rational_proof_record() {
    let dir = tempdir_in(repo_root()).unwrap();
    let path = dir.path().join("checkpoint.json");
    let source = repo_root().join("projects/targets/zeta-3/checkpoint.json");
    std::fs::copy(source, &path).expect("copy checkpoint");
    let mut cp = read_checkpoint(&path).unwrap();
    cp.proof_status = ProofStatus::Rational;
    cp.proof = Some(ProofRecord {
        kind: "rational_equality".into(),
        verifier_id: Some("exact-rational-v1".into()),
        payload: serde_json::json!({ "numerator": "1", "denominator": "2" }),
    });
    cp.updated_at = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        .to_string();
    write_checkpoint(&path, &cp).unwrap();
    let output = Command::new(hs_bin())
        .current_dir(repo_root())
        .args(["verify", "zeta-3", "--checkpoint"])
        .arg(&path)
        .output()
        .expect("spawn hs");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("verified rational equality"));
}

#[test]
fn targets_lists_all_checkpoint_zeta() {
    let output = Command::new(hs_bin()).current_dir(repo_root()).arg("targets").output().expect("spawn hs");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("zeta-2"));
    assert!(stdout.contains("zeta-3"));
    assert!(stdout.contains("zeta-5"));
    assert!(stdout.contains("zeta-7"));
    assert!(stdout.contains("finite-index Ferguson bounds"));
}

#[test]
fn status_reads_zeta7_unknown_proof() {
    let output = Command::new(hs_bin())
        .current_dir(repo_root())
        .args(["status", "zeta-7"])
        .output()
        .expect("spawn hs");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("target: zeta-7"));
    assert!(stdout.contains("proof_status: unknown"));
}

#[test]
fn verify_zeta7_exits_unsupported_when_unknown() {
    let output = Command::new(hs_bin())
        .current_dir(repo_root())
        .args(["verify", "zeta-7"])
        .output()
        .expect("spawn hs");
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("unsupported"));
}

#[test]
fn check_validates_zeta7_observation() {
    let output = Command::new(hs_bin()).current_dir(repo_root()).args(["check", "zeta-7"]).output().expect("spawn hs");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("checked n="));
}

#[test]
fn status_reads_zeta2_checkpoint() {
    let output = Command::new(hs_bin())
        .current_dir(repo_root())
        .args(["status", "zeta-2"])
        .output()
        .expect("spawn hs");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("target: zeta-2"));
    assert!(stdout.contains("observed_best.n:"));
}

#[test]
fn check_validates_zeta5_observation() {
    let output = Command::new(hs_bin()).current_dir(repo_root()).args(["check", "zeta-5"]).output().expect("spawn hs");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("checked n="));
}

#[test]
fn improve_rejects_parallel_jobs() {
    let output = Command::new(hs_bin())
        .current_dir(repo_root())
        .args(["improve", "zeta-3", "--steps", "1", "--jobs", "2"])
        .output()
        .expect("spawn hs");
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("parallel improve"));
}
