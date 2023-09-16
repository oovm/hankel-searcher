# hs-moments

Moment sequences `L(e_n)` for constants used in Hankel irrationality experiments.

Implemented functionals follow Timothy Ferguson, *Rational Approximations via Hankel Determinants* (arXiv:2003.10616):

| Constant | Functional | Reference |
|----------|------------|-----------|
| Euler-Gompertz `δ` | `L_δ(f) = ∫_0^∞ f(x+1) e^{-x}/(x+1) dx` | Ferguson §2 |
| Euler-Mascheroni `γ` | `L_γ` via `σ(x) = -x log(1-x)` | Ferguson §2 |
| Riemann zeta `ζ(k)` | Bose integral kernel on `[0,1]` | Ferguson §2 |
| Catalan `G = β(2)` | Bose dual integral `2/(k-1)! ∫ x^{k-1} e^{-x}/(e^x+e^{-x}) f(1-e^{-2x}) dx` | Ferguson §2 dual |
| `ζ(5)` | Same Bose kernel as `ζ(k)` at `k = 5` | Ferguson §2 (main search target) |

The obsolete odd-denominator kernel `Σ C(n-1,i)(-1)^i/(2i+1)^2` is kept as `catalan_odd_denominator_moment` for negative control.

Weighted Stieltjes kernels inspired by Marc Prévost (ζ(2), ζ(3) Padé/Hankel route) and an exploratory ζ(5) pole family are exposed in `prevost`.
