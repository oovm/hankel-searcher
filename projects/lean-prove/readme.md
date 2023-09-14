# lean-prove

Shared Lean 4 library for Hankel irrationality formalization. Each constant lives under `problems/<Name>/`; shared definitions live in `Hankel/` and `LeanProve/`.

## Layout

```text
lean-prove/
  Hankel/           Ferguson markers, linear-form criterion
  LeanProve/        Shared Ferguson certificate structure
  problems/
    Zeta2/          ζ(2) certificates + irrationality scaffold
    Zeta3/          ζ(3)
    Zeta5/          ζ(5)
```

Rust crates (`zeta-2`, `zeta-3`, `zeta-5`) export `Certificates.lean` into the matching `problems/` folder.

## Build

```bash
cd projects/lean-prove
lake build
```

On Windows, run `scripts/setup-lean.ps1` from the repository root first.
