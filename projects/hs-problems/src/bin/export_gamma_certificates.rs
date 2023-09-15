use hs_problems::export_gamma_certificates;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let max_n = env::args().nth(1).and_then(|s| s.parse().ok()).unwrap_or(6);
    let path = export_gamma_certificates(max_n)?;
    println!("wrote {}", path.display());
    Ok(())
}
