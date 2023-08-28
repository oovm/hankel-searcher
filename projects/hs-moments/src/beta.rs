use crate::rational::binomial;
use hs_types::RationalMomentSequence;
use num_bigint::BigInt;
use num_rational::Ratio;
use num_traits::{One, Zero};

/// Dirichlet beta `β(k) = Σ_{n≥0} (-1)^n / (2n+1)^k`；`k=2` 时等于 Catalan 常数 `G`。
///
/// Ferguson 型 Bose 积分（见 Ferguson arXiv:2003.10616 §2 与 `ζ(k)` 对偶）：
/// `L_{β,k}(f) = 2/(k-1)! ∫_0^∞ x^{k-1} e^{-x}/(e^x+e^{-x}) f(1-e^{-2x}) dx`，
/// 故 `L_{β,k}(e_n) = 2 Σ_{i=0}^{n-1} C(n-1,i)(-1)^i / (2(i+1))^k`。
pub fn beta_moment(k: u32, n: usize) -> Ratio<BigInt> {
    assert!(k >= 2, "beta moments are defined for k >= 2");
    assert!(n >= 1, "beta moments are defined for n >= 1");
    let mut sum = Ratio::from_integer(BigInt::zero());
    let one = Ratio::from_integer(BigInt::one());
    let two = BigInt::from(2);
    for i in 0..n {
        let sign = if i % 2 == 0 { one.clone() } else { -one.clone() };
        let denom = (two.clone() * BigInt::from(i + 1)).pow(k);
        let term = sign * Ratio::new(binomial(n - 1, i), denom);
        sum += term;
    }
    sum
}

/// Build Ferguson moments for `β(k)` (use `k = 2` for Catalan constant).
pub fn beta_moments(k: u32, count: usize) -> RationalMomentSequence {
    let values = (1..=count).map(|n| beta_moment(k, n)).collect();
    RationalMomentSequence::from_positive_moments(values)
}

/// Floating-point moments for energy-rate diagnostics.
pub fn beta_moments_f64(k: u32, count: usize) -> Vec<f64> {
    (1..=count)
        .map(|n| {
            let mut sum = 0.0;
            for i in 0..n {
                let sign = if i % 2 == 0 { 1.0 } else { -1.0 };
                let denom = (2.0 * (i + 1) as f64).powi(k as i32);
                sum += sign * binomial_f64(n - 1, i) / denom;
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
