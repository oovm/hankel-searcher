/-!
# LeanProve.Certificate

Shared Ferguson certificate row exported from Rust (`hs-lean-helper`).
-/

namespace LeanProve

/-- One scaled Ferguson certificate `(P_n, Q_n)` at index `n`. -/
structure FergusonCertificate where
  n : Nat
  pNum pDen qNum qDen scale intP intQ : Int
  deriving Repr, DecidableEq

end LeanProve
