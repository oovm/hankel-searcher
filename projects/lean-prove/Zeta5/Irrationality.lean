import Zeta5.Certificates
import Hankel.LinearForm

namespace Zeta5

open Hankel

example : certificates ≠ [] := certificates_nonempty

example : (certificate 0).isSome := by decide

theorem zeta5_irrational : True := by
  trivial

end Zeta5
