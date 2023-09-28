# zeta-5

`ζ(5)` irrationality search: Ferguson Hankel certificates in Rust, Lean formalization in progress.

## Architecture

```text
hs-moments (ζ(5) moments) → hs-types (Ferguson P/Q) → zeta-5 (export) → lean-prove/problems/Zeta5/
```

## Rust

```bash
cargo run --release -p zeta-5 --bin export-zeta5-certificates
cargo test --release -p zeta-5
```

## Lean

Shared project `projects/lean-prove` (toolchain `v4.16.0`). With elan on `PATH`:

```bash
cd projects/lean-prove
lake build
```

## Literature

- Timothy Ferguson, *Rational Approximations via Hankel Determinants* ([arXiv:2003.10616](https://arxiv.org/abs/2003.10616))
- Wadim Zudilin, at least one of `ζ(5)`, `ζ(7)`, `ζ(9)`, `ζ(11)` is irrational (2001)
