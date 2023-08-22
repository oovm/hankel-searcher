use crate::rational::binomial;
use hs_types::RationalMomentSequence;
use num_bigint::BigInt;
use num_rational::Ratio;
use num_traits::{One, Zero};

/// Catalan constant `G = Σ_{k>=0} (-1)^k / (2k+1)^2`.
pub const CATALAN_G: f64 = 0.915_965_594_1;

/// Experimental beta-kernel moments matching the `ζ(k)` Bose-integral pattern at `k=2`
/// with odd denominators: `Σ_{i=0}^{n-1} C(n-1,i) (-1)^i / (2i+1)^2`.
pub fn catalan_moment(n: usize) -> Ratio<BigInt> {
    assert!(n >= 1, "catalan moments are defined for n >= 1");
    let mut sum = Ratio::from_integer(BigInt::zero());
    let one = Ratio::from_integer(BigInt::one());
    for i in 0..n {
        let sign = if i % 2 == 0 { one.clone() } else { -one.clone() };
        let denom = BigInt::from(2 * i + 1).pow(2);
        let term = sign * Ratio::new(binomial(n - 1, i), denom);
        sum += term;
    }
    sum
}

/// Build the first `count` experimental Ferguson moments for `G`.
pub fn catalan_moments(count: usize) -> RationalMomentSequence {
    let values = (1..=count).map(catalan_moment).collect();
    RationalMomentSequence::from_positive_moments(values)
}

/// Floating-point moments for diagnostics.
pub fn catalan_moments_f64(count: usize) -> Vec<f64> {
    (1..=count)
        .map(|n| {
            let mut sum = 0.0;
            for i in 0..n {
                let sign = if i % 2 == 0 { 1.0 } else { -1.0 };
                sum += sign * binomial_f64(n - 1, i) / ((2 * i + 1) * (2 * i + 1)) as f64;
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
