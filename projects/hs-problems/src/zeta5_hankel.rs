//! Gram/Hankel determinant pipeline for `ζ(5)` aligned with `mo271/Zeta5` `scripts/hankel.py`.

use crate::zeta5_polynomial::{Zeta5PaperParams, d_polynomial, evaluate_polynomial, zeta5_paper_params};
use hs_types::{Integer, Rational, det_linear_pencil, hankel_matrix, is_zero};
use malachite::base::num::arithmetic::traits::Pow;
use malachite::base::num::basic::traits::{One, Zero};

/// Moment sequences `(a_e, b_e)` for `e = 0..2h-2` from the Zeta5 paper construction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Zeta5Entries {
    /// Paper scaling `K`, `N`, `h` for this construction index.
    pub params: Zeta5PaperParams,
    /// Moment sequence `a_e` for `e = 0..2h-2`.
    pub a: Vec<Rational>,
    /// Moment sequence `b_e` for `e = 0..2h-2`.
    pub b: Vec<Rational>,
}

/// Build `(K, N, h, a, b)` matching `hankel.py#entries`.
pub fn zeta5_entries(n: usize) -> Zeta5Entries {
    let params = zeta5_paper_params(n);
    let k = params.k;
    let capital_n = params.capital_n;
    let h = params.h;
    let e_len = 2 * h - 1;
    let maxk = (6 * capital_n + e_len - 1).saturating_sub(k);
    let mus = (0..=maxk).map(|index| mu_coefficient(index)).collect::<Vec<_>>();
    let h5 = harmonic_zeta5_prefix(k);
    let base = base_values(capital_n, k);
    let dn = d_polynomial(capital_n);
    let dk = d_polynomial(k);
    let w = poly_pow(&dn, 6);

    let mut a = Vec::with_capacity(e_len);
    let mut b = Vec::with_capacity(e_len);
    for e in 0..e_len {
        let shifted = poly_mul_monomial(&w, e);
        let pe = poly_div(&shifted, &dk);
        let mut ae = Rational::ZERO;
        for (index, coeff) in pe.iter().enumerate() {
            if index < mus.len() {
                ae += coeff.clone() * mus[index].clone();
            }
        }
        let mut be = Rational::ZERO;
        for j in (capital_n + 1)..=k {
            let c = base[j - (capital_n + 1)].clone() * int_pow(-Integer::from(j * j), e);
            let j4 = Rational::from(Integer::from(j).pow(4u64));
            be += c.clone() * j4.clone();
            ae += c
                * (-j4.clone() * h5[j].clone()
                    - Rational::from_integers(Integer::ONE, Integer::from(4))
                    + Rational::from_integers(Integer::ONE, Integer::from(2 * j as i64)));
        }
        a.push(ae);
        b.push(be);
    }

    Zeta5Entries { params, a, b }
}

/// Hankel matrix `(a_{i+j})` of size `h × h`.
pub fn zeta5_hankel_a(entries: &Zeta5Entries) -> Vec<Vec<Rational>> {
    hankel_matrix(&entries.a, 0, entries.params.h)
}

/// Hankel matrix `(b_{i+j})` of size `h × h`.
pub fn zeta5_hankel_b(entries: &Zeta5Entries) -> Vec<Vec<Rational>> {
    hankel_matrix(&entries.b, 0, entries.params.h)
}

/// Determinant polynomial `Δ_K` with ascending coefficients, matching `hankel.py#det_poly`.
///
/// Uses Bareiss on the linear pencil `xB + A` (same as Flint `det_poly`). Offline only.
pub fn zeta5_delta_polynomial(entries: &Zeta5Entries) -> Vec<Rational> {
    let a_matrix = zeta5_hankel_a(entries);
    let b_matrix = zeta5_hankel_b(entries);
    det_linear_pencil(&a_matrix, &b_matrix).expect("Δ_K pencil determinant")
}

/// Expected degree of `Δ_K`.
pub fn zeta5_delta_degree(entries: &Zeta5Entries) -> usize {
    entries.params.h
}

/// Leading coefficient check from paper (2.9).
pub fn zeta5_delta_leading_coeff(entries: &Zeta5Entries) -> Rational {
    let params = entries.params;
    let h = params.h;
    let capital_n = params.capital_n;
    let k = params.k;
    let sign = if (h * (h - 1) / 2) % 2 == 0 {
        Rational::ONE
    } else {
        -Rational::ONE
    };
    let dn = d_polynomial(capital_n);
    let mut lead = sign;
    for j in (capital_n + 1)..=k {
        let dn_at = evaluate_polynomial(&dn, &Rational::from(-Integer::from(j * j)));
        lead *= Rational::from(Integer::from(j).pow(4u64)) * dn_at.pow(5u64);
    }
    lead
}

fn mu_coefficient(k: usize) -> Rational {
    let bernoulli = bernoulli_rational(2 * k + 2);
    let sign = if k % 2 == 0 { Rational::ONE } else { -Rational::ONE };
    let numerator = Integer::from((2 * k + 3) * (2 * k + 4) * (2 * k + 5));
    sign * bernoulli * Rational::from_integers(numerator, Integer::from(24))
}

fn bernoulli_rational(n: usize) -> Rational {
    if n == 0 {
        return Rational::ONE;
    }
    if n == 1 {
        return Rational::from_integers(-Integer::ONE, Integer::from(2));
    }
    if n % 2 == 1 {
        return Rational::ZERO;
    }
    let mut bernoulli = vec![Rational::ZERO; n + 1];
    bernoulli[0] = Rational::ONE;
    for m in 1..=n {
        let mut acc = Rational::ZERO;
        for k in 0..m {
            acc += binomial_rational(m + 1, k) * bernoulli[k].clone();
        }
        bernoulli[m] = -acc / Rational::from(Integer::from(m + 1));
    }
    bernoulli[n].clone()
}

fn binomial_rational(n: usize, k: usize) -> Rational {
    if k > n {
        return Rational::ZERO;
    }
    let mut numerator = Integer::ONE;
    let mut denominator = Integer::ONE;
    for index in 0..k {
        numerator *= Integer::from(n - index);
        denominator *= Integer::from(index + 1);
    }
    Rational::from_integers(numerator, denominator)
}

fn harmonic_zeta5_prefix(k: usize) -> Vec<Rational> {
    let mut prefix = vec![Rational::ZERO; k + 1];
    for j in 1..=k {
        prefix[j] = prefix[j - 1].clone()
            + Rational::from_integers(Integer::ONE, Integer::from(j).pow(5u64));
    }
    prefix
}

fn base_values(capital_n: usize, k: usize) -> Vec<Rational> {
    let dn = d_polynomial(capital_n);
    let mut values = Vec::with_capacity(k - capital_n);
    for j in (capital_n + 1)..=k {
        let num = evaluate_polynomial(&dn, &Rational::from(-Integer::from(j * j))).pow(6u64);
        let mut den = Rational::ONE;
        for t in 1..=k {
            if t != j {
                let diff = Integer::from(t * t) - Integer::from(j * j);
                den *= Rational::from(diff);
            }
        }
        values.push(num / den);
    }
    values
}

fn poly_degree(poly: &[Rational]) -> isize {
    for index in (0..poly.len()).rev() {
        if !is_zero(&poly[index]) {
            return index as isize;
        }
    }
    -1
}

fn poly_mul_monomial(poly: &[Rational], shift: usize) -> Vec<Rational> {
    let mut out = vec![Rational::ZERO; poly.len() + shift];
    out[shift..].clone_from_slice(poly);
    out
}

fn poly_mul(left: &[Rational], right: &[Rational]) -> Vec<Rational> {
    let mut out = vec![Rational::ZERO; left.len() + right.len() - 1];
    for (i, left_coeff) in left.iter().enumerate() {
        for (j, right_coeff) in right.iter().enumerate() {
            out[i + j] += left_coeff.clone() * right_coeff.clone();
        }
    }
    out
}

fn poly_pow(base: &[Rational], exponent: usize) -> Vec<Rational> {
    if exponent == 0 {
        return vec![Rational::ONE];
    }
    let mut acc = base.to_vec();
    for _ in 1..exponent {
        acc = poly_mul(&acc, base);
    }
    acc
}

/// Polynomial quotient in `Q[x]`, matching `fmpq_poly` floor division in `hankel.py`.
fn poly_div(dividend: &[Rational], divisor: &[Rational]) -> Vec<Rational> {
    let divisor = trim_poly(divisor);
    let divisor_degree = poly_degree(&divisor);
    if divisor_degree < 0 {
        return vec![Rational::ZERO];
    }
    let dividend_degree = poly_degree(dividend);
    if dividend_degree < divisor_degree {
        return vec![Rational::ZERO];
    }
    let mut remainder = trim_poly(dividend);
    let mut quotient = vec![
        Rational::ZERO;
        remainder.len().saturating_sub(divisor.len()) + 1
    ];
    let lead_divisor = divisor[divisor_degree as usize].clone();
    loop {
        let remainder_degree = poly_degree(&remainder);
        if remainder_degree < divisor_degree {
            break;
        }
        let shift = (remainder_degree - divisor_degree) as usize;
        let lead_remainder = remainder[remainder_degree as usize].clone();
        let term = lead_remainder / lead_divisor.clone();
        quotient[shift] += term.clone();
        let needed = shift + divisor.len();
        if remainder.len() < needed {
            remainder.resize(needed, Rational::ZERO);
        }
        for (index, divisor_coeff) in divisor.iter().enumerate() {
            remainder[index + shift] -= term.clone() * divisor_coeff.clone();
        }
        remainder = trim_poly(&remainder);
    }
    trim_poly(&quotient)
}

fn int_pow(base: Integer, exponent: usize) -> Rational {
    Rational::from(base.pow(exponent as u64))
}

fn trim_poly(poly: &[Rational]) -> Vec<Rational> {
    let mut out = poly.to_vec();
    while out.len() > 1 && is_zero(out.last().expect("non-empty poly")) {
        out.pop();
    }
    if out.is_empty() {
        out.push(Rational::ZERO);
    }
    out
}

#[cfg(all(test, feature = "offline-golden"))]
mod offline_det_tests {
    use super::*;
    use hs_types::{bareiss_det, bareiss_det_rational};

    #[test]
    fn bareiss_rational_matches_generic_on_zeta5_a() {
        let entries = zeta5_entries(1);
        let a = zeta5_hankel_a(&entries);
        assert_eq!(bareiss_det(&a), bareiss_det_rational(&a));
    }

    #[test]
    fn delta_polynomial_matches_pencil_samples() {
        let entries = zeta5_entries(1);
        let a = zeta5_hankel_a(&entries);
        let b = zeta5_hankel_b(&entries);
        let delta = zeta5_delta_polynomial(&entries);
        for point in 0..=entries.params.h {
            let x = Rational::from(Integer::from(point));
            let mut power = Rational::ONE;
            let mut value = Rational::ZERO;
            for coeff in &delta {
                value += coeff.clone() * power.clone();
                power *= x.clone();
            }
            let matrix = a
                .iter()
                .zip(&b)
                .map(|(row_a, row_b)| {
                    row_a
                        .iter()
                        .zip(row_b)
                        .map(|(left, right)| left.clone() + x.clone() * right.clone())
                        .collect()
                })
                .collect::<Vec<_>>();
            let expected = bareiss_det_rational(&matrix);
            assert_eq!(value, expected, "mismatch at x={point}");
        }
    }
}

#[cfg(test)]
mod poly_tests {
    use super::*;

    #[test]
    fn poly_div_exact_divides_square_minus_one() {
        let dividend = vec![
            Rational::from(-Integer::ONE),
            Rational::ZERO,
            Rational::ONE,
        ];
        let divisor = vec![Rational::from(-Integer::ONE), Rational::ONE];
        let quotient = poly_div(&dividend, &divisor);
        assert_eq!(quotient, vec![Rational::ONE, Rational::ONE]);
    }

    #[test]
    fn d3_matches_python_reference() {
        let poly = d_polynomial(3);
        assert_eq!(poly.len(), 4);
        assert_eq!(poly[0], Rational::from(Integer::from(36)));
        assert_eq!(poly[1], Rational::from(Integer::from(49)));
        assert_eq!(poly[2], Rational::from(Integer::from(14)));
        assert_eq!(poly[3], Rational::ONE);
    }

    #[test]
    fn pe_division_exact_at_e22_for_n1() {
        let params = zeta5_paper_params(1);
        let dn = d_polynomial(params.capital_n);
        let dk = d_polynomial(params.k);
        let w = poly_pow(&dn, 6);
        let shifted = poly_mul_monomial(&w, 22);
        assert_eq!(poly_div(&shifted, &dk), vec![Rational::ONE]);
    }
}
