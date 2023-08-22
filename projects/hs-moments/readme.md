# hs-moments

Moment sequences `L(e_n)` for constants used in Hankel irrationality experiments.

Implemented functionals follow Timothy Ferguson, *Rational Approximations via Hankel Determinants* (arXiv:2003.10616):

| Constant | Functional | Reference |
|----------|------------|-----------|
| Euler-Gompertz `δ` | `L_δ(f) = ∫_0^∞ f(x+1) e^{-x}/(x+1) dx` | Ferguson §2 |
| Euler-Mascheroni `γ` | `L_γ` via `σ(x) = -x log(1-x)` | Ferguson §2 |
| Riemann zeta `ζ(k)` | Bose integral kernel on `[0,1]` | Ferguson §2 |
| Catalan `G` | Dirichlet beta kernel `∫_0^1 f(y) K_G(y) dy` | experimental |

Weighted Stieltjes kernels inspired by Marc Prévost (ζ(2), ζ(3) Padé/Hankel route) are exposed in `prevost`.
