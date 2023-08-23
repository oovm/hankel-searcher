# hs-benchmark

Regression checks against Ferguson (2020) Table 1–2 and micro-benchmarks for Hankel determinant evaluation.

Run:

```bash
cargo test -p hs-benchmark --release
cargo bench -p hs-benchmark
```

Reference data:

- Timothy Ferguson, arXiv:2003.10616, Tables 1–2
- Wadim Zudilin, *A Determinantal Approach to Irrationality*, Constructive Approximation 45 (2016)
