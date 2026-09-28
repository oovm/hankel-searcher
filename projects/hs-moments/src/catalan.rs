use crate::beta::{beta_moment, beta_moments, beta_moments_f64};
use crate::rational::{binomial, ratio};
use hs_types::{Integer, Rational, RationalMomentSequence};
use malachite::base::num::arithmetic::traits::Pow;
use malachite::base::num::basic::traits::Zero;

/// Catalan constant `G = β(2) = Σ_{k≥0} (-1)^k / (2k+1)^2`.
pub const CATALAN_G: f64 = 0.915_965_594_1;

/// 已弃用的奇分母候选核：`Σ C(n-1,i)(-1)^i / (2i+1)^2`。
/// 不满足 Ferguson 定理 1 的 `P_n/Q_n < G`（见 `hs-benchmark` 负对照测试）。
pub fn catalan_odd_denominator_moment(n: usize) -> Rational {
    assert!(n >= 1, "catalan moments are defined for n >= 1");
    let mut sum = Rational::ZERO;
    for i in 0..n {
        let denom = Integer::from(2 * i + 1).pow(2u64);
        let term = if i % 2 == 0 {
            ratio(binomial(n - 1, i), denom)
        } else {
            -ratio(binomial(n - 1, i), denom)
        };
        sum += term;
    }
    sum
}

/// 已弃用候选核的矩序列（搜索负对照）。
pub fn catalan_moments(count: usize) -> RationalMomentSequence {
    let values = (1..=count).map(catalan_odd_denominator_moment).collect();
    RationalMomentSequence::from_positive_moments(values)
}

/// Ferguson 正确的 `β(2)` 矩；`catalan_moment` 为其别名。
pub fn catalan_moment(n: usize) -> Rational {
    beta_moment(2, n)
}

/// Ferguson `β(2)` 矩序列。
pub fn catalan_beta_moments(count: usize) -> RationalMomentSequence {
    beta_moments(2, count)
}

/// `β(2)` 浮点矩。
pub fn catalan_moments_f64(count: usize) -> Vec<f64> {
    beta_moments_f64(2, count)
}
