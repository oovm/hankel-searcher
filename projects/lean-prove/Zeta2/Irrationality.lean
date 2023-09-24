import Zeta2.Certificates
import Hankel.LinearForm

/-!
# Zeta2.Irrationality

Analytic irrationality of `ζ(2)` via the Ferguson/Hankel route (in progress).
-/

namespace Zeta2

open Hankel

example : certificates ≠ [] := certificates_nonempty

example : (certificate 0).isSome := by decide

end Zeta2
