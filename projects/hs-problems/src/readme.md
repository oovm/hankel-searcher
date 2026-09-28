# hs-problems

Rust Ferguson certificate export for Hankel targets that are not tied to a dedicated `zeta-*` crate.

| Problem | Constant | Export command |
|---------|----------|----------------|
| `Delta` | Euler-Gompertz `δ` | `cargo run --release -p hs-problems --bin export-delta-certificates` |
| `Gamma` | Euler-Mascheroni `γ` | `cargo run --release -p hs-problems --bin export-gamma-certificates` |
| `Catalan` | `G = β(2)` | `cargo run --release -p hs-problems --bin export-catalan-certificates` |

Output lands in `projects/lean-prove/Problems/<Problem>/Certificates.lean`.
