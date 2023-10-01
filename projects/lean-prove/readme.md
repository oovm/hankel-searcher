# lean-prove

Shared Lean 4 library for Hankel irrationality formalization. Shared definitions live in `Hankel/` and `LeanProve/`; each constant is a Lake library under `problems/`.

## Layout

```text
lean-prove/
  Hankel/                 Ferguson markers, linear-form criterion
  LeanProve/              Shared Ferguson certificate structure
  problems/
    Zeta2.lean            root import for the `Zeta2` library
    Zeta2/
      Ferguson.lean
      Certificates.lean   auto-generated from Rust
      Irrationality.lean  constant-specific proof obligations
    Zeta3/                ζ(3)
    Zeta5/                ζ(5)
    Delta/                Euler-Gompertz δ
    Gamma/                Euler-Mascheroni γ
    Catalan/              G = β(2)
    L2ChiMinus3/          L(2, χ_{-3}) scaffold (certificates pending)
```

Lake maps each problem library with `srcDir = "problems"`: root at `problems/<Name>.lean`, modules in `problems/<Name>/`.

| Problem | Rust export |
|---------|-------------|
| `Zeta2` / `Zeta3` / `Zeta5` | `zeta-*` crates |
| `Delta` / `Gamma` / `Catalan` | `hs-problems` bins |

## Build

Requires [elan](https://github.com/leanprover/elan) on `PATH` (toolchain pinned by `lean-toolchain`).

If `elan toolchain install` times out (common when `releases.lean-lang.org` is slow), download `lean-4.16.0-windows.tar.zst` manually and register it:

```powershell
.\scripts\setup-lean.ps1 -InstallFromArchive "$env:USERPROFILE\Downloads\lean-4.16.0-windows.tar.zst" -Version v4.16.0
```

```bash
cd projects/lean-prove
lake build
```
