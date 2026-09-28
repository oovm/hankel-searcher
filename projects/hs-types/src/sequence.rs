use crate::bignum::Rational;
use crate::bignum::Zero;

/// A moment sequence `a_n` with the Ferguson convention `a_0 = 0`.
#[derive(Debug, Clone)]
pub struct MomentSequence<T> {
    /// `a_n` for `n >= 1`; index `0` stores `a_1`.
    moments: Vec<T>,
}

impl<T: Clone + Zero> MomentSequence<T> {
    /// Build a Ferguson moment sequence from `L(e_1), L(e_2), ...`.
    pub fn from_positive_moments(values: Vec<T>) -> Self {
        Self { moments: values }
    }

    /// Number of stored positive moments.
    pub fn len(&self) -> usize {
        self.moments.len()
    }

    /// Access `a_n` with Ferguson's `a_0 = 0` convention.
    pub fn a(&self, n: usize) -> T {
        if n == 0 {
            T::ZERO
        } else {
            self.moments[n - 1].clone()
        }
    }

    /// Contiguous Ferguson indexing `a_0..a_{limit}` inclusive.
    pub fn as_ferguson_prefix(&self, limit: usize) -> Vec<T> {
        let mut out = Vec::with_capacity(limit + 1);
        for n in 0..=limit {
            out.push(self.a(n));
        }
        out
    }
}

/// View of `a_{n + shift}` without copying the full tail.
#[derive(Debug, Clone)]
pub struct ShiftedMomentSequence<'a, T> {
    base: &'a MomentSequence<T>,
    shift: usize,
}

impl<'a, T: Clone + Zero> ShiftedMomentSequence<'a, T> {
    /// Shift all indices by `shift`.
    pub fn new(base: &'a MomentSequence<T>, shift: usize) -> Self {
        Self { base, shift }
    }

    /// Access `a_{n + shift}` with `a_0 = 0`.
    pub fn a(&self, n: usize) -> T {
        self.base.a(n + self.shift)
    }
}

impl<T: Clone + Zero> MomentSequence<T> {
    /// Borrow a shifted view.
    pub fn shifted(&self, shift: usize) -> ShiftedMomentSequence<'_, T> {
        ShiftedMomentSequence::new(self, shift)
    }
}

/// Convenience builder for exact rational moment sequences.
pub type RationalMomentSequence = MomentSequence<Rational>;
