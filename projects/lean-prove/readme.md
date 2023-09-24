# lean-prove

Shared Lean 4 library for Hankel irrationality formalization. Shared definitions live in `Hankel/` and `LeanProve/`; each constant is a Lake library under `problems/<Name>/`.

## Layout

```text
lean-prove/
  Hankel/                 Ferguson markers, linear-form criterion
  LeanProve/              Shared Ferguson certificate structure
  problems/
    Zeta2/
      Zeta2.lean          root import for the `Zeta2` library
      Ferguson.lean
      Certificates.lean   auto-generated from Rust
      Irrationality.lean
    Zeta3/                ζ(3)
    Zeta5/                ζ(5)
    Delta/                Euler-Gompertz δ
    Gamma/                Euler-Mascheroni γ
    Catalan/              G = β(2)
    L2ChiMinus3/          L(2, χ_{-3}) scaffold (certificates pending)
```

Lake expects each problem library root at `problems/<Name>/<Name>.lean` when `srcDir = "problems/<Name>"`.

| Problem | Rust export |
|---------|-------------|
| `Zeta2` / `Zeta3` / `Zeta5` | `zeta-*` crates |
| `Delta` / `Gamma` / `Catalan` | `hs-problems` bins |

## Build

Requires [elan](https://github.com/leanprover/elan) on `PATH` (toolchain pinned by `lean-toolchain`).

```bash
cd projects/lean-prove
lake build
```
