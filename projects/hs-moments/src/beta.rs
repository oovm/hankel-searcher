use crate::rational::{binomial, ratio};
use hs_types::{Integer, Rational, RationalMomentSequence};
use malachite::base::num::arithmetic::traits::Pow;
use malachite::base::num::basic::traits::Zero;

/// Dirichlet beta `β(k) = Σ_{n≥0} (-1)^n / (2n+1)^k`；`k=2` 时等于 Catalan 常数 `G`。
///
/// **Normalization warning:** this Ferguson integral kernel satisfies
/// `beta_moment(k, n) = 2^{1-k} * zeta_moment(k, n)`, so the resulting `P_n/Q_n`
/// track `2^{1-k} ζ(k)`, not `β(k)` itself. Do not treat `k=2` exports as
/// approximating `G` without an explicit target rescaling contract.
///
/// Ferguson 型 Bose 积分（见 Ferguson arXiv:2003.10616 §2 与 `ζ(k)` 对偶）：
/// `L_{β,k}(f) = 2/(k-1)! ∫_0^∞ x^{k-1} e^{-x}/(e^x+e^{-x}) f(1-e^{-2x}) dx`，
/// 故 `L_{β,k}(e_n) = 2 Σ_{i=0}^{n-1} C(n-1,i)(-1)^i / (2(i+1))^k`。
pub fn beta_moment(k: u32, n: usize) -> Rational {
    assert!(k >= 2, "beta moments are defined for k >= 2");
    assert!(n >= 1, "beta moments are defined for n >= 1");
    let mut sum = Rational::ZERO;
    let two = Integer::from(2);
    for i in 0..n {
        let denom = (two.clone() * Integer::from(i + 1)).pow(u64::from(k));
        let term = if i % 2 == 0 {
            ratio(binomial(n - 1, i), denom)
        } else {
            -ratio(binomial(n - 1, i), denom)
        };
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
