hankel-searcher
===============

Hankel-determinant rational approximation experiments for constants related to irrationality search.

Public `readme.md` files in this repository are written in English. Source-level internal comments (`//`) are written in Chinese.

## Crates

| Crate          | Role                                                               |
|----------------|--------------------------------------------------------------------|
| `hs-types`     | Ferguson `P_n/Q_n` construction and exact Hankel determinants      |
| `hs-moments`   | Moment sequences for `δ`, `γ`, `ζ(k)`, `β(k)` / Catalan, Prevost weights |
| `hs-lean-helper` | Lean 4 certificate export for Ferguson approximants              |
| `hs-benchmark` | Ferguson table regression and Criterion micro-benchmarks           |
| `zeta-2` / `zeta-3` | Lean-backed `ζ(2)` / `ζ(3)` Hankel irrationality verification crates |
| `zeta-5` | Ferguson certificates + Lean `ζ(5)` irrationality (`projects/zeta-5/lean`) |

## Build

Install the `hs` command from this repository:

```bash
cargo install --path projects/hs-tools
hs improve zeta-3
hs check zeta-3
```

`hs improve` currently updates the finite-index approximation bound in `projects/zeta-3/checkpoint.json`. The uniform irrationality-proof bound remains unset. Commit the JSON change in a pull request after `hs check` succeeds.

```bash
git checkout dev
cargo build --release -p hs-types -p hs-moments -p hs-lean-helper -p hs-benchmark
cargo test --release -p hs-benchmark
cargo bench -p hs-benchmark
```

## Literature

- Timothy Ferguson, *Rational Approximations via Hankel Determinants*
  ([arXiv:2003.10616](https://arxiv.org/abs/2003.10616))
- Wadim Zudilin, *A Determinantal Approach to Irrationality* (Constructive Approximation, 2016)
- Marc Prévost, *A new proof of the irrationality of ζ (2) and ζ (3) using Padé approximants* (2002)
- F. Calegari, V. Dimitrov, Y. Tang, linear independence of `1, ζ(2), L(2,χ_{-3})` (2024)
- W. Zudilin, one of `ζ(5), ζ(7), ζ(9), ζ(11)` is irrational (2001)

**Main search target:** `ζ(5)` Ferguson approximants via `hs-benchmark` (`cargo test -p hs-benchmark zeta5`, `cargo run -p hs-benchmark --example zeta5_search`).

**Lean `ζ(5)`:** `projects/zeta-5/lean` (Ferguson certificates + in-progress formalization). Run `scripts/setup-lean.ps1` on Windows, then `cd projects/zeta-5/lean && lake build`.
