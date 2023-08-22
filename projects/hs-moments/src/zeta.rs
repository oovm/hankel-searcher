use crate::rational::binomial;
use hs_types::RationalMomentSequence;
use num_bigint::BigInt;
use num_rational::Ratio;
use num_traits::{One, Zero};

/// Reference value of `ζ(2)` for regression tests.
pub const ZETA_2: f64 = 1.644_934_066_8;
/// Reference value of `ζ(3)` for regression tests.
pub const ZETA_3: f64 = 1.202_056_903_1;
/// Reference value of `ζ(5)` for exploratory diagnostics.
pub const ZETA_5: f64 = 1.036_927_755_1;

/// `L_{ζ,k}(e_n) = Σ_{i=0}^{n-1} C(n-1,i) (-1)^i / (i+1)^k`.
pub fn zeta_moment(k: u32, n: usize) -> Ratio<BigInt> {
    assert!(k >= 2, "zeta moments are defined for k >= 2");
    assert!(n >= 1, "zeta moments are defined for n >= 1");
    let mut sum = Ratio::from_integer(BigInt::zero());
    let one = Ratio::from_integer(BigInt::one());
    for i in 0..n {
        let sign = if i % 2 == 0 { one.clone() } else { -one.clone() };
        let denom = BigInt::from(i + 1).pow(k);
        let term = sign * Ratio::new(binomial(n - 1, i), denom);
        sum += term;
    }
    sum
}

/// Build the first `count` Ferguson moments for `ζ(k)`.
pub fn zeta_moments(k: u32, count: usize) -> RationalMomentSequence {
    let values = (1..=count).map(|n| zeta_moment(k, n)).collect();
    RationalMomentSequence::from_positive_moments(values)
}

/// Floating-point moments for diagnostics.
pub fn zeta_moments_f64(k: u32, count: usize) -> Vec<f64> {
    (1..=count)
        .map(|n| {
            let mut sum = 0.0;
            for i in 0..n {
                let sign = if i % 2 == 0 { 1.0 } else { -1.0 };
                sum += sign * binomial_f64(n - 1, i) / (i + 1).pow(k) as f64;
            }
            sum
        })
        .collect()
}

fn binomial_f64(n: usize, k: usize) -> f64 {
    if k > n {
        return 0.0;
    }
    let k = k.min(n - k);
    let mut num = 1.0;
    let mut den = 1.0;
    for i in 0..k {
        num *= (n - k + i + 1) as f64;
        den *= (i + 1) as f64;
    }
    num / den
}
