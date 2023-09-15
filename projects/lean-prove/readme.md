# lean-prove

Shared Lean 4 library for Hankel irrationality formalization. Each constant lives under `problems/<Name>/`; shared definitions live in `Hankel/` and `LeanProve/`.

## Layout

```text
lean-prove/
  Hankel/           Ferguson markers, linear-form criterion
  LeanProve/        Shared Ferguson certificate structure
  problems/
    Zeta2/          ζ(2)
    Zeta3/          ζ(3)
    Zeta5/          ζ(5)
    Delta/          Euler-Gompertz δ
    Gamma/          Euler-Mascheroni γ
    Catalan/        G = β(2)
    L2ChiMinus3/    L(2, χ_{-3}) scaffold (certificates pending)
```

| Problem | Rust export |
|---------|-------------|
| `Zeta2` / `Zeta3` / `Zeta5` | `zeta-*` crates |
| `Delta` / `Gamma` / `Catalan` | `hs-problems` bins |

## Build

```bash
cd projects/lean-prove
lake build
```

On Windows, run `scripts/setup-lean.ps1` from the repository root first.
