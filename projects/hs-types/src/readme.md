# hs-types

Core Hankel determinant algebra for irrationality-oriented rational approximation.

Implements Ferguson's construction (arXiv:2003.10616):

- moment sequence `a_n = L(e_n)` with `a_0 = 0`
- `P_n = -det(a_{i+j})_{0 <= i,j <= n+1}`
- `Q_n = det(a_{i+j+2})_{0 <= i,j <= n}`
- convergents `P_n / Q_n -> L(e_0)`

References:

- Timothy Ferguson, *Rational Approximations via Hankel Determinants* (2020)
- Wadim Zudilin, *A Determinantal Approach to Irrationality* (2016)
- Marc Prévost, *A new proof of the irrationality of ζ(2) and ζ(3) using Padé approximants* (2002)
