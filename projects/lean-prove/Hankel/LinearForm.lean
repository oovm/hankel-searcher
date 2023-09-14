/-!
# Hankel.LinearForm

Classical irrationality criterion used after Hankel decay estimates.
-/

import Hankel.Ferguson

namespace Hankel

/-- If `α = p/q` and `0 < q·(B·α - A) < 1` for integers `A,B`, then `α` is irrational at denominator `q`. -/
theorem irrational_of_small_int_linear
    {α : Rat} {p q A B : Int}
    (hq : q ≠ 0)
    (hα : α = p / q)
    (hpos : 0 < B * α - A)
    (hsmall : q * (B * α - A) < 1) :
    False := by
  sorry

end Hankel
