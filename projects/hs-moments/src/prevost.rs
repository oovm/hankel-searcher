/// Prevost-style pole weights for weighted Hankel/Padé constructions.
///
/// See Marc Prévost, *A new proof of the irrationality of ζ(2) and ζ(3)*
/// and the ζ(5) Hankel route discussed by Ball–Rivoal successors (2025–2026).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WeightFamily {
    /// `W_{2,n}(t) = 1 / (t + (2n+1)^2)` for ζ(2).
    Zeta2,
    /// `W_{3,n}(t) = t / (t + (n+1)^2)` for ζ(3).
    Zeta3,
}

/// A single pole weight evaluated at `t`.
#[derive(Debug, Clone, Copy)]
pub struct PrevostWeight {
    family: WeightFamily,
    n: usize,
}

impl PrevostWeight {
    /// Build the Prevost weight for index `n`.
    pub fn new(family: WeightFamily, n: usize) -> Self {
        Self { family, n }
    }

    /// Evaluate `W_n(t)` for real `t > 0`.
    pub fn eval(&self, t: f64) -> f64 {
        match self.family {
            WeightFamily::Zeta2 => {
                let pole = (2 * self.n + 1).pow(2) as f64;
                1.0 / (t + pole)
            }
            WeightFamily::Zeta3 => {
                let pole = (self.n + 1).pow(2) as f64;
                t / (t + pole)
            }
        }
    }

    /// Pole location used by the family.
    pub fn pole(&self) -> f64 {
        match self.family {
            WeightFamily::Zeta2 => (2 * self.n + 1).pow(2) as f64,
            WeightFamily::Zeta3 => (self.n + 1).pow(2) as f64,
        }
    }
}

/// Convenience constructor.
pub fn prevost_weight(family: WeightFamily, n: usize) -> PrevostWeight {
    PrevostWeight::new(family, n)
}
