use num_bigint::BigInt;
use num_rational::Ratio;
use num_traits::{One, Zero};

/// Exact factorial `n!`.
pub fn factorial(n: usize) -> BigInt {
    if n == 0 {
        return BigInt::one();
    }
    (1..=n).fold(BigInt::one(), |acc, k| acc * BigInt::from(k))
}

/// Exact binomial coefficient `n choose k`.
pub fn binomial(n: usize, k: usize) -> BigInt {
    if k > n {
        return BigInt::zero();
    }
    if k == 0 || k == n {
        return BigInt::one();
    }
    let k = k.min(n - k);
    let mut num = BigInt::one();
    let mut den = BigInt::one();
    for i in 0..k {
        num *= BigInt::from(n - k + i + 1);
        den *= BigInt::from(i + 1);
    }
    num / den
}

/// Build a reduced rational from integers.
pub fn ratio(num: impl Into<BigInt>, den: impl Into<BigInt>) -> Ratio<BigInt> {
    Ratio::new(num.into(), den.into())
}
