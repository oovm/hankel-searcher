use crate::rational::{binomial, factorial, ratio};
use hs_types::RationalMomentSequence;
use num_bigint::BigInt;
use num_rational::Ratio;
use num_traits::{One, Zero};

/// Euler-Mascheroni constant.
pub const EULER_MASCHERONI: f64 = 0.577_215_664_9;

/// `L_γ(e_n)` from Ferguson (2020), Example γ.
pub fn gamma_moment(n: usize) -> Ratio<BigInt> {
    assert!(n >= 1, "gamma moments are defined for n >= 1");
    let fact = factorial(n - 1);
    let mut sum = Ratio::from_integer(BigInt::zero());
    let one = Ratio::from_integer(BigInt::one());
    for i in 0..=n {
        let inner_sign = if i % 2 == 0 { one.clone() } else { -one.clone() };
        let coeff = BigInt::from((n as i64) - 2 * (i as i64) - 1);
        let denom = BigInt::from(i + 1).pow(n as u32 + 1);
        let term = inner_sign * Ratio::new(binomial(n, i) * coeff, denom);
        sum += term;
    }
    ratio(fact, BigInt::one()) * sum
}

/// Build the first `count` Ferguson moments for `γ`.
pub fn gamma_moments(count: usize) -> RationalMomentSequence {
    let values = (1..=count).map(gamma_moment).collect();
    RationalMomentSequence::from_positive_moments(values)
}

/// Floating-point moments for diagnostics.
pub fn gamma_moments_f64(count: usize) -> Vec<f64> {
    (1..=count)
        .map(|n| {
            let fact = factorial_f64(n - 1);
            let mut sum = 0.0;
            for i in 0..=n {
                let inner_sign = if i % 2 == 0 { 1.0 } else { -1.0 };
                let coeff = (n - 2 * i - 1) as f64;
                let denom = (i + 1).pow((n + 1) as u32) as f64;
                let term = inner_sign * binomial_f64(n, i) * coeff / denom;
                sum += term;
            }
            fact * sum
        })
        .collect()
}

fn factorial_f64(n: usize) -> f64 {
    if n == 0 {
        1.0
    } else {
        (1..=n).map(|k| k as f64).product()
    }
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
