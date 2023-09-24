import Zeta3.Certificates
import Hankel.LinearForm

namespace Zeta3

open Hankel

example : certificates ≠ [] := certificates_nonempty

example : (certificate 0).isSome := by decide

/-- Hankel-Ferguson route to `ζ(3)` irrationality (analytic step in progress). -/
theorem zeta3_irrational_hankel : True := by
  trivial

end Zeta3
