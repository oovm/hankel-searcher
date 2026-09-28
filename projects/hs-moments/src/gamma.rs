use crate::rational::{binomial, factorial, ratio};
use hs_types::{Integer, Rational, RationalMomentSequence};
use malachite::base::num::arithmetic::traits::Pow;
use malachite::base::num::basic::traits::{One, Zero};

/// Euler-Mascheroni constant.
pub const EULER_MASCHERONI: f64 = 0.577_215_664_9;

/// `L_γ(e_n)` from Ferguson (2020), Example γ.
pub fn gamma_moment(n: usize) -> Rational {
    assert!(n >= 1, "gamma moments are defined for n >= 1");
    let fact = factorial(n - 1);
    let mut sum = Rational::ZERO;
    for i in 0..=n {
        let inner_sign = if i % 2 == 0 { Rational::ONE } else { -Rational::ONE };
        let coeff = Integer::from((n as i64) - 2 * (i as i64) - 1);
        let denom = Integer::from(i + 1).pow(u64::from(n as u32 + 1));
        let term = inner_sign * ratio(binomial(n, i) * coeff, denom);
        sum += term;
    }
    ratio(fact, Integer::ONE) * sum
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
