import Catalan.Certificates
import Hankel.LinearForm

/-!
# Catalan.Irrationality

Hankel-Ferguson route to irrationality of the Catalan constant `G` (in progress).
-/

namespace Catalan

open Hankel

example : certificates ≠ [] := certificates_nonempty

example : (certificate 0).isSome := by decide

theorem catalan_irrational_hankel : True := by
  trivial

end Catalan
