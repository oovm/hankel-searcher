use hs_types::{Integer, Rational};
use malachite::base::num::basic::traits::{One, Zero};

/// Exact factorial `n!`.
pub fn factorial(n: usize) -> Integer {
    if n == 0 {
        return Integer::ONE;
    }
    (1..=n).fold(Integer::ONE, |acc, k| acc * Integer::from(k))
}

/// Exact binomial coefficient `n choose k`.
pub fn binomial(n: usize, k: usize) -> Integer {
    if k > n {
        return Integer::ZERO;
    }
    if k == 0 || k == n {
        return Integer::ONE;
    }
    let k = k.min(n - k);
    let mut num = Integer::ONE;
    let mut den = Integer::ONE;
    for i in 0..k {
        num *= Integer::from(n - k + i + 1);
        den *= Integer::from(i + 1);
    }
    num / den
}

/// Build a reduced rational from integers.
pub fn ratio(num: impl Into<Integer>, den: impl Into<Integer>) -> Rational {
    Rational::from_integers(num.into(), den.into())
}
