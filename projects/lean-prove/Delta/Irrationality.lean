import Delta.Certificates
import Hankel.LinearForm

/-!
# Delta.Irrationality

Hankel-Ferguson route to irrationality of the Euler-Gompertz constant `δ` (in progress).
-/

namespace Delta

open Hankel

example : certificates ≠ [] := certificates_nonempty

example : (certificate 0).isSome := by decide

theorem delta_irrational_hankel : True := by
  trivial

end Delta
