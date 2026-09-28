# lean-prove

Shared Lean 4 library for Hankel irrationality formalization.

| Lake library | Role |
|--------------|------|
| `Hankel` | Ferguson markers, linear-form criterion |
| `LeanProve` | Shared Ferguson certificate structure |
| `Problems` | All constants (`Problems/Zeta2/`, `Problems/Delta/`, …) |

## Layout

```text
lean-prove/
  Hankel/
  LeanProve/
  Problems.lean
  Problems/
    Zeta2/
      Ferguson.lean
      Certificates.lean   auto-generated from Rust
      Irrationality.lean
    Zeta3/                ζ(3)
    Zeta5/                ζ(5)
    Delta/                δ
    Gamma/                γ
    Catalan/              G
    L2ChiMinus3/
```

Modules are `Problems.Zeta2.Certificates`, `Problems.Delta.Irrationality`, and so on.

## Build

```bash
cd projects/lean-prove
lake build
```

Offline toolchain install: see `scripts/setup-lean.ps1`.
