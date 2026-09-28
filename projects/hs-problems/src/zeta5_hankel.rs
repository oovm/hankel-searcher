//! Gram/Hankel determinant pipeline for `ζ(5)` aligned with `mo271/Zeta5` `scripts/hankel.py`.

use crate::zeta5_polynomial::{Zeta5PaperParams, d_polynomial, evaluate_polynomial, zeta5_paper_params};
use hs_types::{
    bareiss_det, hankel_matrix, rational_charpoly, rational_mat_inv, rational_mat_mul, scale_polynomial,
};
use num_bigint::BigInt;
use num_rational::Ratio;
use num_traits::{One, Zero};

type Rational = Ratio<BigInt>;

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
        let mut ae = Rational::zero();
        for (index, coeff) in pe.iter().enumerate() {
            if index < mus.len() {
                ae += coeff * &mus[index];
            }
        }
        let mut be = Rational::zero();
        for j in (capital_n + 1)..=k {
            let c = &base[j - (capital_n + 1)] * int_pow(-BigInt::from(j * j), e);
            be += &c * Rational::from_integer(BigInt::from(j.pow(4)));
            ae += &c
                * (-Rational::from_integer(BigInt::from(j.pow(4))) * &h5[j]
                    - Rational::new(BigInt::one(), BigInt::from(4))
                    + Ratio::new(BigInt::one(), BigInt::from(2 * j as i64)));
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
/// **Expensive:** at `n=1` this inverts and takes the characteristic polynomial of a
/// `37×37` rational Hankel matrix. Offline only — not part of `cargo test`.
pub fn zeta5_delta_polynomial(entries: &Zeta5Entries) -> Vec<Rational> {
    let a_matrix = zeta5_hankel_a(entries);
    let b_matrix = zeta5_hankel_b(entries);
    let det_b = bareiss_det(&b_matrix);
    let b_inv = rational_mat_inv(&b_matrix).expect("B must be invertible");
    let neg_a = scale_matrix(&a_matrix, &-Rational::one());
    let c = rational_mat_mul(&b_inv, &neg_a);
    let charpoly = rational_charpoly(&c);
    scale_polynomial(&charpoly, &det_b)
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
        Rational::one()
    } else {
        -Rational::one()
    };
    let dn = d_polynomial(capital_n);
    let mut lead = sign;
    for j in (capital_n + 1)..=k {
        let dn_at = evaluate_polynomial(&dn, &Ratio::from_integer(-BigInt::from(j * j)));
        lead *= Ratio::from_integer(BigInt::from(j.pow(4))) * dn_at.pow(5);
    }
    lead
}

fn mu_coefficient(k: usize) -> Rational {
    let bernoulli = bernoulli_rational(2 * k + 2);
    let sign = if k % 2 == 0 { Rational::one() } else { -Rational::one() };
    let numerator = BigInt::from((2 * k + 3) * (2 * k + 4) * (2 * k + 5));
    sign * bernoulli * Ratio::new(numerator, BigInt::from(24))
}

fn bernoulli_rational(n: usize) -> Rational {
    if n == 0 {
        return Rational::one();
    }
    if n == 1 {
        return Ratio::new(-BigInt::one(), BigInt::from(2));
    }
    if n % 2 == 1 {
        return Rational::zero();
    }
    let mut bernoulli = vec![Rational::zero(); n + 1];
    bernoulli[0] = Rational::one();
    for m in 1..=n {
        let mut acc = Rational::zero();
        for k in 0..m {
            acc += binomial_rational(m + 1, k) * bernoulli[k].clone();
        }
        bernoulli[m] = -acc / Rational::from_integer(BigInt::from(m + 1));
    }
    bernoulli[n].clone()
}

fn binomial_rational(n: usize, k: usize) -> Rational {
    if k > n {
        return Rational::zero();
    }
    let mut numerator = BigInt::one();
    let mut denominator = BigInt::one();
    for index in 0..k {
        numerator *= BigInt::from(n - index);
        denominator *= BigInt::from(index + 1);
    }
    Ratio::new(numerator, denominator)
}

fn harmonic_zeta5_prefix(k: usize) -> Vec<Rational> {
    let mut prefix = vec![Rational::zero(); k + 1];
    for j in 1..=k {
        prefix[j] = prefix[j - 1].clone() + Ratio::new(BigInt::one(), BigInt::from(j.pow(5)));
    }
    prefix
}

fn base_values(capital_n: usize, k: usize) -> Vec<Rational> {
    let dn = d_polynomial(capital_n);
    let mut values = Vec::with_capacity(k - capital_n);
    for j in (capital_n + 1)..=k {
        let num = evaluate_polynomial(&dn, &Ratio::from_integer(-BigInt::from(j * j))).pow(6);
        let mut den = Rational::one();
        for t in 1..=k {
            if t != j {
                let diff = BigInt::from(t * t) - BigInt::from(j * j);
                den *= Ratio::from_integer(diff);
            }
        }
        values.push(num / den);
    }
    values
}

fn poly_degree(poly: &[Rational]) -> isize {
    for index in (0..poly.len()).rev() {
        if !poly[index].is_zero() {
            return index as isize;
        }
    }
    -1
}

fn poly_mul_monomial(poly: &[Rational], shift: usize) -> Vec<Rational> {
    let mut out = vec![Rational::zero(); poly.len() + shift];
    out[shift..].clone_from_slice(poly);
    out
}

fn poly_mul(left: &[Rational], right: &[Rational]) -> Vec<Rational> {
    let mut out = vec![Rational::zero(); left.len() + right.len() - 1];
    for (i, left_coeff) in left.iter().enumerate() {
        for (j, right_coeff) in right.iter().enumerate() {
            out[i + j] += left_coeff * right_coeff;
        }
    }
    out
}

fn poly_pow(base: &[Rational], exponent: usize) -> Vec<Rational> {
    if exponent == 0 {
        return vec![Rational::one()];
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
        return vec![Rational::zero()];
    }
    let dividend_degree = poly_degree(dividend);
    if dividend_degree < divisor_degree {
        return vec![Rational::zero()];
    }
    let mut remainder = trim_poly(dividend);
    let mut quotient = vec![
        Rational::zero();
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
            remainder.resize(needed, Rational::zero());
        }
        for (index, divisor_coeff) in divisor.iter().enumerate() {
            remainder[index + shift] -= term.clone() * divisor_coeff.clone();
        }
        remainder = trim_poly(&remainder);
    }
    trim_poly(&quotient)
}

fn scale_matrix(matrix: &[Vec<Rational>], factor: &Rational) -> Vec<Vec<Rational>> {
    matrix
        .iter()
        .map(|row| row.iter().map(|entry| entry * factor).collect())
        .collect()
}

fn int_pow(base: BigInt, exponent: usize) -> Rational {
    Ratio::from_integer(base.pow(exponent as u32))
}

fn trim_poly(poly: &[Rational]) -> Vec<Rational> {
    let mut out = poly.to_vec();
    while out.len() > 1 && out.last().is_some_and(Rational::is_zero) {
        out.pop();
    }
    if out.is_empty() {
        out.push(Rational::zero());
    }
    out
}

#[cfg(test)]
mod poly_tests {
    use super::*;

    #[test]
    fn poly_div_exact_divides_square_minus_one() {
        let dividend = vec![
            Ratio::from_integer(-BigInt::one()),
            Ratio::zero(),
            Ratio::one(),
        ];
        let divisor = vec![Ratio::from_integer(-BigInt::one()), Ratio::one()];
        let quotient = poly_div(&dividend, &divisor);
        assert_eq!(quotient, vec![Ratio::one(), Ratio::one()]);
    }

    #[test]
    fn d3_matches_python_reference() {
        let poly = d_polynomial(3);
        assert_eq!(poly.len(), 4);
        assert_eq!(poly[0], Ratio::from_integer(BigInt::from(36)));
        assert_eq!(poly[1], Ratio::from_integer(BigInt::from(49)));
        assert_eq!(poly[2], Ratio::from_integer(BigInt::from(14)));
        assert_eq!(poly[3], Ratio::one());
    }

    #[test]
    fn pe_division_exact_at_e22_for_n1() {
        let params = zeta5_paper_params(1);
        let dn = d_polynomial(params.capital_n);
        let dk = d_polynomial(params.k);
        let w = poly_pow(&dn, 6);
        let shifted = poly_mul_monomial(&w, 22);
        assert_eq!(poly_div(&shifted, &dk), vec![Ratio::one()]);
    }
}
