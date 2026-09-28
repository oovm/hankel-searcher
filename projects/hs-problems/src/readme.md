# hs-problems

Rust Ferguson certificate export for every `lean-prove/Problems/*` target.

| Problem | Constant | Export command |
|---------|----------|----------------|
| `Zeta2` | `ζ(2)` | `cargo run --release -p hs-problems --bin export-zeta2-certificates` |
| `Zeta3` | `ζ(3)` | `cargo run --release -p hs-problems --bin export-zeta3-certificates` |
| `Zeta5` | `ζ(5)` | `cargo run --release -p hs-problems --bin export-zeta5-certificates` |
| `Delta` | Euler-Gompertz `δ` | `cargo run --release -p hs-problems --bin export-delta-certificates` |
| `Gamma` | Euler-Mascheroni `γ` | `cargo run --release -p hs-problems --bin export-gamma-certificates` |
| `Catalan` | `G = β(2)` | `cargo run --release -p hs-problems --bin export-catalan-certificates` |

Output lands in `projects/lean-prove/Problems/<Problem>/Certificates.lean`.

Checkpoint JSON for collaborative `hs` runs lives under `projects/targets/<target-id>/`.
