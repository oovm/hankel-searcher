# zeta-2

Irrationality verification for `ζ(2)`: Rust Hankel pipeline → `hs-lean-helper` Lean certificates → `lean-prove` formalization.

## Architecture

```text
hs-moments (ζ(2) moments) → hs-types (Ferguson P/Q) → zeta-2 (export) → lean-prove/problems/Zeta2/
```

The Rust side follows Ferguson (arXiv:2003.10616) and the Prévost Padé/Hankel route. The Lean side formalizes:

1. Hankel moment sequences and Ferguson determinants
2. Alignment with rational certificates from `export-zeta2-certificates` (`norm_num`)
3. Integer linear-form irrationality criteria
4. The main `ζ(2)` theorem (analytic decay steps can be filled in separately)

## Quick start

### Rust certificate export

```bash
cargo run --release -p zeta-2 --bin export-zeta2-certificates
cargo test --release -p zeta-2
```

### Lean (requires [elan](https://github.com/leanprover/elan))

```bash
cd projects/lean-prove
lake build
```

On Windows, after installing elan:

```powershell
.\scripts\setup-lean.ps1
```

## Literature

- Timothy Ferguson, *Rational Approximations via Hankel Determinants* (2020)
- Marc Prévost, *A new proof of the irrationality of ζ(2) and ζ(3) using Padé approximants* (2002)
- Wadim Zudilin, *A Determinantal Approach to Irrationality* (2016)
