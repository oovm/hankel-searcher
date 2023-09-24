import Gamma.Certificates
import Hankel.LinearForm

/-!
# Gamma.Irrationality

Hankel-Ferguson route to irrationality of the Euler-Mascheroni constant `γ` (in progress).
-/

namespace Gamma

open Hankel

example : certificates ≠ [] := certificates_nonempty

example : (certificate 0).isSome := by decide

theorem gamma_irrational_hankel : True := by
  trivial

end Gamma
