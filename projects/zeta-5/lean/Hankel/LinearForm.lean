/-!
# Hankel.LinearForm

Classical irrationality criterion for the Ferguson decay route (placeholder).
The main `ζ(5)` theorem lives in `Zeta5.Irrationality` via package `Apery`.
-/

import Hankel.Ferguson

namespace Hankel

/-- Integer linear-form criterion for the Hankel pipeline (not used by `Apery`). -/
theorem irrational_of_small_int_linear
    {α : Rat} {p q A B : Int}
    (hq : q ≠ 0)
    (hα : α = p / q)
    (hpos : 0 < B * α - A)
    (hsmall : q * (B * α - A) < 1) :
    False := by
  sorry

end Hankel
