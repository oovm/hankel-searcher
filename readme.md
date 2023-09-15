Hankel Searcher
===============

Hankel-determinant rational approximation experiments for constants related to irrationality search.

## Crates

| Crate               | Role                                                                          |
|---------------------|-------------------------------------------------------------------------------|
| `hs-types`          | Ferguson `P_n/Q_n` construction and exact Hankel determinants                 |
| `hs-moments`        | Moment sequences for `δ`, `γ`, `ζ(k)`, `β(k)` / Catalan, Prevost weights      |
| `hs-lean-helper`    | Lean 4 certificate export for Ferguson approximants                           |
| `lean-prove`        | Shared Lean library (`Hankel/`, `problems/*`)                                 |
| `hs-problems`       | Ferguson certificate export for `δ`, `γ`, and Catalan `G`                     |
| `hs-benchmark`      | Ferguson table regression and Criterion micro-benchmarks                      |
| `zeta-2` / `zeta-3` | Ferguson examples and Lean certificate experiments                            |
| `zeta-5`            | Ferguson certificates and incomplete Lean formalization                       |
| `hs-tools`          | Installable `hs` command for finite-index checkpoint improvement and checking |

## Build

Install the `hs` command from this repository:

```bash
cargo install --path projects/hs-tools
hs improve zeta-3
hs check zeta-3
```

`hs improve` currently updates the finite-index approximation bound in `projects/zeta-3/checkpoint.json`. The uniform
irrationality-proof bound remains unset. Commit the JSON change in a pull request after `hs check` succeeds.

## Current proof progress

The time below means **wall-clock time from a fresh search to an independently verified irrationality proof**. It is
unknown: the current candidate family has no theorem guaranteeing that the search contains such a proof. A fixed compute
budget or a benchmark of several indices does not estimate this time.

- **`ζ(3)` finite-index result:** The [checkpoint](projects/zeta-3/checkpoint.json) covers indices `0..3`. Its best
  recorded candidate is `n=3` with a rigorous bound `|ζ(3) - P_3/Q_3| < 0.001515864284004895`, obtained using an exact
  rational enclosure with 256 series terms. The complete rational upper bound is stored in the JSON. The next index is
  `4`.
- **Uniform proof bound:** None is registered (`best: null`). The finite-index bound does not establish the behavior of
  all sufficiently large indices.
- **Irrationality measure from this repository:** No upper bound is certified. Although `ζ(3)` is known to be irrational
  by other proofs, this checkpoint does not determine its irrationality measure `μ(ζ(3))` or improve a published bound.
- **`ζ(5)` status:** The existing convergence ladder compares finite approximants to a truncated `f64` reference value.
  It supplies no rigorous bound on the irrationality measure. The current Lean main declaration states `True`, so it
  does not prove irrationality of `ζ(5)`.
- **Time from scratch to a proof:** Unknown for both targets; no finite success-time guarantee is available. A
  reproducible runtime for a fixed candidate range has not been recorded here.

Future `hs` runs will report proof time, finite bounds, uniform bounds and irrationality-measure bounds separately. A
measure upper bound requires a verified family of integer linear forms, decay and coefficient-growth estimates, and a
nondegeneracy condition; a single good approximant is insufficient.

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

**Experimental `ζ(5)` target:** Ferguson approximants via `hs-benchmark` (`cargo test -p hs-benchmark zeta5`,
`cargo run -p hs-benchmark --example zeta5_search`). Its current gap uses an approximate reference constant.

**Lean:** `projects/lean-prove` holds shared `Hankel/` definitions and per-problem folders under `problems/`. Run
`scripts/setup-lean.ps1` on Windows, then `cd projects/lean-prove && lake build`. A successful build does not establish
any irrationality theorem.
