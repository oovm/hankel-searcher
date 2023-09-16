# hs-benchmark

Regression checks against Ferguson (2020) Table 1–2, `ζ(5)` convergence ladder (main search), and Criterion micro-benchmarks.

Run:

```bash
cargo test -p hs-benchmark --release
cargo run -p hs-benchmark --release --example zeta5_search
cargo bench -p hs-benchmark
```

Default tests cap exact Hankel depth (`γ` at `n≤12`, `ζ`/`δ` at `n≤15`, `ζ(5)` search at `n≤15`).
Deep Ferguson table tails and tight gap checks live behind `#[ignore]`:

```bash
cargo test -p hs-benchmark --release -- --ignored
ZETA5_MAX_N=20 cargo run -p hs-benchmark --release --example zeta5_search
```

Reference data:

- Timothy Ferguson, arXiv:2003.10616, Tables 1–2
- Wadim Zudilin, *A Determinantal Approach to Irrationality*, Constructive Approximation 45 (2016)
- Wadim Zudilin (2001), at least one of `ζ(5)`, `ζ(7)`, `ζ(9)`, `ζ(11)` is irrational
