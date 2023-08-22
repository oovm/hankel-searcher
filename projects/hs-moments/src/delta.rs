use crate::rational::{factorial, ratio};
use hs_types::RationalMomentSequence;
use num_bigint::BigInt;
use num_rational::Ratio;
use num_traits::{One, Zero};

/// Euler-Gompertz constant `δ = ∫_0^∞ e^{-t}/(1+t) dt`.
pub const EULER_GOMPERTZ: f64 = 0.596_347_362_3;

/// `L_δ(e_n) = Σ_{i=0}^{n-1} (n-1)!/i!` for `n >= 1`.
pub fn delta_moment(n: usize) -> Ratio<BigInt> {
    assert!(n >= 1, "delta moments are defined for n >= 1");
    let mut sum = BigInt::zero();
    let fact = factorial(n - 1);
    for i in 0..n {
        sum += &fact / factorial(i);
    }
    ratio(sum, BigInt::one())
}

/// Build the first `count` Ferguson moments for `δ`.
pub fn delta_moments(count: usize) -> RationalMomentSequence {
    let values = (1..=count).map(delta_moment).collect();
    RationalMomentSequence::from_positive_moments(values)
}

/// Floating-point moments for energy-rate diagnostics.
pub fn delta_moments_f64(count: usize) -> Vec<f64> {
    (1..=count)
        .map(|n| {
            let mut sum = 0.0;
            let fact = factorial_f64(n - 1);
            for i in 0..n {
                sum += fact / factorial_f64(i);
            }
            sum
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
