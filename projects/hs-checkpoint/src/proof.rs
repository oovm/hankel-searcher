/// Rational equality certificate consumed by `exact-rational-v1`.
pub const PROOF_KIND_RATIONAL_EQUALITY: &str = "rational_equality";
/// Integer linear form certificate (legacy Ferguson-era envelope).
pub const PROOF_KIND_INTEGER_LINEAR_FORM: &str = "integer_linear_form";
/// Whole-polynomial irrationality certificate from the Zeta5 Hankel construction.
pub const PROOF_KIND_POLYNOMIAL_IRRATIONALITY: &str = "polynomial_irrationality";

/// Registered irrational proof verifiers.
pub const VERIFIER_POLYNOMIAL_IRRATIONALITY_V1: &str = "polynomial-irrationality-v1";

/// Return whether `kind` is a registered proof record kind.
pub fn is_known_proof_kind(kind: &str) -> bool {
    matches!(
        kind,
        PROOF_KIND_RATIONAL_EQUALITY | PROOF_KIND_INTEGER_LINEAR_FORM | PROOF_KIND_POLYNOMIAL_IRRATIONALITY
    )
}
