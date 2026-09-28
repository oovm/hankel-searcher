use crate::bignum::{Integer, Natural, Rational, is_zero};
use crate::bignum::{One, Zero};
use core::cmp::Ordering;
use malachite::base::num::arithmetic::traits::{Lcm, PowAssign, Sign};

/// Build the square Hankel matrix `(m_{i+j})` for `0 <= i,j < size`.
pub fn hankel_matrix<T: Clone>(moments: &[T], shift: usize, size: usize) -> Vec<Vec<T>> {
    let mut matrix = Vec::with_capacity(size);
    for row in 0..size {
        let mut line = Vec::with_capacity(size);
        for col in 0..size {
            line.push(moments[shift + row + col].clone());
        }
        matrix.push(line);
    }
    matrix
}

/// Exact determinant via the Bareiss fraction-free elimination algorithm.
pub fn bareiss_det<T>(matrix: &[Vec<T>]) -> T
where
    T: Clone + Zero + One + PartialEq + std::ops::Neg<Output = T>,
    T: std::ops::Div<Output = T>,
    T: std::ops::Mul<Output = T>,
    T: std::ops::Sub<Output = T>,
{
    let n = matrix.len();
    if n == 0 {
        return T::ONE;
    }
    if n == 1 {
        return matrix[0][0].clone();
    }

    let mut work = matrix
        .iter()
        .map(|row| row.iter().cloned().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut pivot = T::ONE;
    let mut sign = T::ONE;
    for k in 0..n - 1 {
        if is_zero(&work[k][k]) {
            let swap = (k + 1..n).find(|row| !is_zero(&work[*row][k]));
            match swap {
                Some(row) => {
                    work.swap(k, row);
                    sign = -sign;
                }
                None => return T::ZERO,
            }
        }

        for i in k + 1..n {
            for j in k + 1..n {
                let term = (work[i][j].clone() * work[k][k].clone() - work[i][k].clone() * work[k][j].clone())
                    / pivot.clone();
                work[i][j] = term;
            }
        }
        pivot = work[k][k].clone();
    }

    sign * work[n - 1][n - 1].clone()
}

/// Exact rational determinant via integer Bareiss after clearing denominators.
pub fn bareiss_det_rational(matrix: &[Vec<Rational>]) -> Rational {
    let n = matrix.len();
    if n == 0 {
        return Rational::ONE;
    }
    if n == 1 {
        return matrix[0][0].clone();
    }

    let mut lcm_denom = Natural::ONE;
    for row in matrix {
        for entry in row {
            lcm_denom = Natural::lcm(lcm_denom, entry.to_denominator());
        }
    }
    let lcm_integer = Integer::from(lcm_denom.clone());
    let scaled = matrix
        .iter()
        .map(|row| {
            row.iter()
                .map(|entry| {
                    if is_zero(entry) {
                        return Integer::ZERO;
                    }
                    let scale = &lcm_denom / entry.to_denominator();
                    let unsigned = entry.to_numerator().clone() * scale;
                    match entry.sign() {
                        Ordering::Greater => Integer::from(unsigned),
                        Ordering::Less => -Integer::from(unsigned),
                        Ordering::Equal => Integer::ZERO,
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let det_int = bareiss_det_integer(&scaled);
    let mut denominator = lcm_integer;
    denominator.pow_assign(u64::try_from(n).expect("matrix size fits in u64"));
    Rational::from_integers(det_int, denominator)
}

fn bareiss_det_integer(matrix: &[Vec<Integer>]) -> Integer {
    let n = matrix.len();
    if n == 0 {
        return Integer::ONE;
    }
    if n == 1 {
        return matrix[0][0].clone();
    }

    let mut work = matrix
        .iter()
        .map(|row| row.iter().cloned().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut prev_pivot = Integer::ONE;
    let mut sign = Integer::ONE;
    for k in 0..n - 1 {
        if is_zero(&work[k][k]) {
            let swap = (k + 1..n).find(|row| !is_zero(&work[*row][k]));
            match swap {
                Some(row) => {
                    work.swap(k, row);
                    sign = -sign;
                }
                None => return Integer::ZERO,
            }
        }

        for i in k + 1..n {
            for j in k + 1..n {
                let numerator =
                    work[i][j].clone() * &work[k][k] - work[i][k].clone() * &work[k][j];
                work[i][j] = numerator / &prev_pivot;
            }
        }
        prev_pivot = work[k][k].clone();
    }

    sign * work[n - 1][n - 1].clone()
}

/// Floating-point determinant via `faer` (pure Rust SIMD kernels, no BLAS sys).
pub fn f64_det(matrix: &[Vec<f64>]) -> f64 {
    let n = matrix.len();
    if n == 0 {
        return 1.0;
    }
    let m = faer::Mat::from_fn(n, n, |row, col| matrix[row][col]);
    m.determinant()
}

/// Floating-point Hankel determinant for asymptotic energy estimates.
pub fn hankel_det_f64(moments: &[f64], shift: usize, size: usize) -> f64 {
    let matrix = hankel_matrix(moments, shift, size);
    f64_det(&matrix)
}

/// Natural logarithm of `|det|` for log-energy diagnostics.
pub fn log_abs_hankel_det(moments: &[f64], shift: usize, size: usize) -> f64 {
    hankel_det_f64(moments, shift, size).abs().ln()
}

/// Estimate the quadratic decay rate `-log|H_n| / n^2` from a moment tail.
pub fn estimate_energy_rate(moments: &[f64], max_n: usize) -> Vec<(usize, f64)> {
    let mut rates = Vec::new();
    for n in 1..=max_n {
        let size = n + 1;
        if shifted_len(moments.len(), 0, size) {
            let log_det = log_abs_hankel_det(moments, 0, size);
            if log_det.is_finite() {
                rates.push((n, -log_det / (n as f64).powi(2)));
            }
        }
    }
    rates
}

fn shifted_len(total: usize, shift: usize, size: usize) -> bool {
    shift + 2 * size - 1 < total
}

/// Rational Hankel determinant.
pub fn hankel_det_rational(moments: &[Rational], shift: usize, size: usize) -> Rational {
    let matrix = hankel_matrix(moments, shift, size);
    bareiss_det_rational(&matrix)
}
