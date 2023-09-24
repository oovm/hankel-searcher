# zeta-3

Irrationality verification for `ζ(3)`: Rust Hankel pipeline → `hs-lean-helper` Lean certificates → `lean-prove` formalization.

## Architecture

```text
hs-moments (ζ(3) moments) → hs-types (Ferguson P/Q) → zeta-3 (export) → lean-prove/Zeta3/
```

The Rust side follows Ferguson (arXiv:2003.10616) and the Prévost Padé/Hankel route (Apéry-style linear forms). The Lean side formalizes:

1. Hankel moment sequences and Ferguson determinants
2. Alignment with rational certificates from `export-zeta3-certificates` (`norm_num`)
3. Integer linear-form irrationality criteria
4. The main `ζ(3)` theorem (analytic decay steps can be filled in separately)

## Quick start

### Rust certificate export

```bash
cargo run --release -p zeta-3 --bin export-zeta3-certificates
cargo test --release -p zeta-3
```

### Lean (requires [elan](https://github.com/leanprover/elan))

```bash
cd projects/lean-prove
lake build
```

## Literature

- Roger Apéry (1978), irrationality of `ζ(3)`
- Timothy Ferguson, *Rational Approximations via Hankel Determinants* (2020)
- Marc Prévost, *A new proof of the irrationality of ζ(2) and ζ(3) using Padé approximants* (2002)
- Wadim Zudilin, *A Determinantal Approach to Irrationality* (2016)
