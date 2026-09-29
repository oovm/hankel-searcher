//! Shared polynomial Hankel algebra for `ζ(s)` constructions (`K=40n`, `N=3n`, `q=6`, `h=37n`).

use crate::zeta5_polynomial::{Zeta5PaperParams, d_polynomial, evaluate_polynomial, zeta5_paper_params};
use hs_types::{Integer, Rational, det_linear_pencil, hankel_matrix, is_zero};
use malachite::base::num::arithmetic::traits::Pow;
use malachite::base::num::basic::traits::{One, Zero};

/// Moment sequences and paper scaling for a whole-polynomial Hankel construction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZetaPolynomialEntries {
    /// Zeta order `s` (`2`, `3`, or `5`).
    pub order: u32,
    /// Paper scaling `K`, `N`, `h` for this construction index.
    pub params: Zeta5PaperParams,
    /// Moment sequence `a_e` for `e = 0..2h-2`.
    pub a: Vec<Rational>,
    /// Moment sequence `b_e` for `e = 0..2h-2`.
    pub b: Vec<Rational>,
}

/// Build `(K, N, h, a, b)` for integer `ζ(s)` with `s ∈ {2, 3, 5}`.
pub fn zeta_polynomial_entries(order: u32, n: usize) -> Result<ZetaPolynomialEntries, String> {
    if !matches!(order, 2 | 3 | 5) {
        return Err(format!("polynomial Hankel order `{order}` is not implemented"));
    }
    let params = zeta5_paper_params(n);
    let k = params.k;
    let capital_n = params.capital_n;
    let h = params.h;
    let e_len = 2 * h - 1;
    let maxk = (6 * capital_n + e_len - 1).saturating_sub(k);
    let mus = (0..=maxk)
        .map(|index| mu_moment_rational(order, index))
        .collect::<Vec<_>>();
    let harmonic = harmonic_zeta_prefix(k, order);
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
        let (const_den, j_den) = pole_rational_offsets(order);
        for j in (capital_n + 1)..=k {
            let c = base[j - (capital_n + 1)].clone() * int_pow(-Integer::from(j * j), e);
            let cz = pole_zeta_coefficient(order, j);
            be += c.clone() * cz.clone();
            ae += c
                * (-cz.clone() * harmonic[j].clone()
                    - Rational::from_integers(Integer::ONE, Integer::from(const_den))
                    + Rational::from_integers(Integer::ONE, Integer::from(j * j_den as usize)));
        }
        a.push(ae);
        b.push(be);
    }

    Ok(ZetaPolynomialEntries {
        order,
        params,
        a,
        b,
    })
}

/// Hankel matrix `(a_{i+j})` of size `h × h`.
pub fn zeta_polynomial_hankel_a(entries: &ZetaPolynomialEntries) -> Vec<Vec<Rational>> {
    hankel_matrix(&entries.a, 0, entries.params.h)
}

/// Hankel matrix `(b_{i+j})` of size `h × h`.
pub fn zeta_polynomial_hankel_b(entries: &ZetaPolynomialEntries) -> Vec<Vec<Rational>> {
    hankel_matrix(&entries.b, 0, entries.params.h)
}

/// Determinant polynomial `Δ_K` with ascending coefficients.
pub fn zeta_polynomial_delta(entries: &ZetaPolynomialEntries) -> Result<Vec<Rational>, String> {
    let a_matrix = zeta_polynomial_hankel_a(entries);
    let b_matrix = zeta_polynomial_hankel_b(entries);
    det_linear_pencil(&a_matrix, &b_matrix)
}

/// Expected degree of `Δ_K`.
pub fn zeta_polynomial_delta_degree(entries: &ZetaPolynomialEntries) -> usize {
    entries.params.h
}

/// Leading coefficient from pole product (paper (2.9) pattern generalized by `s`).
pub fn zeta_polynomial_delta_leading_coeff(entries: &ZetaPolynomialEntries) -> Rational {
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
        lead *= pole_zeta_coefficient(entries.order, j) * dn_at.pow(entries.order as u64);
    }
    lead
}

/// `log S_K` paper normalization (independent of `ζ(s)` order for fixed `K,N,h`).
pub fn polynomial_hankel_log_s_k(params: &Zeta5PaperParams) -> f64 {
    let h = params.h as f64;
    let logfact = |m: usize| libm::lgamma(m as f64 + 1.0);
    let mut tail = 0.0;
    for index in 1..params.h {
        tail += logfact(2 * index);
    }
    2.0 * h * logfact(params.k)
        + (h - 1.0) * 4.0f64.ln()
        - 12.0 * h * logfact(params.capital_n)
        - 2.0 * tail
}

fn mu_moment_rational(order: u32, k: usize) -> Rational {
    let bernoulli = bernoulli_rational(2 * k + 2);
    let sign = if k % 2 == 0 { Rational::ONE } else { -Rational::ONE };
    match order {
        2 => sign * bernoulli / Rational::from(Integer::from(2)),
        3 => {
            let factor = Rational::from_integers(Integer::from(2 * k + 3), Integer::from(2));
            sign * factor * bernoulli
        }
        5 => {
            let numerator = Integer::from((2 * k + 3) * (2 * k + 4) * (2 * k + 5));
            sign * bernoulli * Rational::from_integers(numerator, Integer::from(24))
        }
        _ => Rational::ZERO,
    }
}

fn pole_zeta_coefficient(order: u32, j: usize) -> Rational {
    match order {
        2 => Rational::from_integers(Integer::from(j), Integer::from(2)),
        3 => Rational::from(Integer::from(j).pow(2u64)),
        5 => Rational::from(Integer::from(j).pow(4u64)),
        _ => Rational::ZERO,
    }
}

/// `(denominator for 1/const, denominator for 1/(j_den * j))` in the pole integral tail.
fn pole_rational_offsets(order: u32) -> (usize, usize) {
    match order {
        2 => (2, 4),
        3 => (2, 2),
        5 => (4, 2),
        _ => (1, 1),
    }
}

fn harmonic_zeta_prefix(k: usize, order: u32) -> Vec<Rational> {
    let mut prefix = vec![Rational::ZERO; k + 1];
    for j in 1..=k {
        prefix[j] = prefix[j - 1].clone()
            + Rational::from_integers(Integer::ONE, Integer::from(j).pow(order as u64));
    }
    prefix
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

pub(crate) fn poly_degree(poly: &[Rational]) -> isize {
    for index in (0..poly.len()).rev() {
        if !is_zero(&poly[index]) {
            return index as isize;
        }
    }
    -1
}

pub(crate) fn poly_mul_monomial(poly: &[Rational], shift: usize) -> Vec<Rational> {
    let mut out = vec![Rational::ZERO; poly.len() + shift];
    out[shift..].clone_from_slice(poly);
    out
}

pub(crate) fn poly_mul(left: &[Rational], right: &[Rational]) -> Vec<Rational> {
    let mut out = vec![Rational::ZERO; left.len() + right.len() - 1];
    for (i, left_coeff) in left.iter().enumerate() {
        for (j, right_coeff) in right.iter().enumerate() {
            out[i + j] += left_coeff.clone() * right_coeff.clone();
        }
    }
    out
}

pub(crate) fn poly_pow(base: &[Rational], exponent: usize) -> Vec<Rational> {
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
pub(crate) fn poly_div(dividend: &[Rational], divisor: &[Rational]) -> Vec<Rational> {
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

#[cfg(test)]
mod regression {
    use super::*;
    use hs_types::rational_from_str;

    #[test]
    fn zeta5_first_moments_match_hankel_py_golden() {
        let entries = zeta_polynomial_entries(5, 1).expect("zeta5 entries");
        const GOLDEN_A: [(&str, &str); 3] = [
            (
                "49821462748046694006853561350793143870593179493040606581",
                "188299614209343116973075133400157932727737142039536050380525012802664911752330958575583675635722426948645617643028480000000000000000000000",
            ),
            (
                "8958568640929906584589777392347266022642533941475983599",
                "2353745177616788962163439167501974159096714275494200629756562660033311396904136982194795945446530336858070220537856000000000000000000000",
            ),
            (
                "732019960601533841611306270658604877361157281383104997",
                "11316082584696100779631919074528721918734203247568272258445012788621689408192966260551903583877549696433029906432000000000000000000000",
            ),
        ];
        for (index, (numerator, denominator)) in GOLDEN_A.iter().enumerate() {
            let expected = rational_from_str(numerator, denominator).expect("golden rational");
            assert_eq!(entries.a[index], expected, "a[{index}] mismatch vs hankel.py");
            assert!(is_zero(&entries.b[index]), "b[{index}] should be zero in hankel.py golden");
        }
    }
}
