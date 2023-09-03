import Zeta2.Certificates
import Hankel.LinearForm

/-!
# Zeta2.Irrationality

Analytic irrationality of `ζ(2)` via the Ferguson/Hankel route.

Rust-exported Ferguson certificates in `Zeta2.Certificates` supply the rational
`(P_n,Q_n)` side; the decay bound and linear-form step remain to be completed.
-/

namespace Zeta2

open Hankel

example : certificates ≠ [] := certificates_nonempty

example : (certificate 0).isSome := by decide

end Zeta2
