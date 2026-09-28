use crate::bignum::{Integer, Rational, is_zero};
use crate::bignum::{One, Zero};
/// Matrix multiply for square rational matrices.
pub fn rational_mat_mul(left: &[Vec<Rational>], right: &[Vec<Rational>]) -> Vec<Vec<Rational>> {
    let n = left.len();
    debug_assert_eq!(left[0].len(), n);
    debug_assert_eq!(right.len(), n);
    debug_assert_eq!(right[0].len(), n);
    let mut out = vec![vec![Rational::ZERO; n]; n];
    for row in 0..n {
        for col in 0..n {
            let mut acc = Rational::ZERO;
            for k in 0..n {
                acc += left[row][k].clone() * right[k][col].clone();
            }
            out[row][col] = acc;
        }
    }
    out
}

/// Invert a square rational matrix via Gauss-Jordan elimination.
pub fn rational_mat_inv(matrix: &[Vec<Rational>]) -> Option<Vec<Vec<Rational>>> {
    let n = matrix.len();
    if n == 0 || matrix.iter().any(|row| row.len() != n) {
        return None;
    }

    let mut work = matrix
        .iter()
        .map(|row| row.iter().cloned().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut inv = vec![vec![Rational::ZERO; n]; n];
    for i in 0..n {
        inv[i][i] = Rational::ONE;
    }

    for col in 0..n {
        let pivot_row = (col..n).find(|row| !is_zero(&work[*row][col]))?;
        if pivot_row != col {
            work.swap(col, pivot_row);
            inv.swap(col, pivot_row);
        }
        let pivot = work[col][col].clone();
        for j in 0..n {
            work[col][j] /= pivot.clone();
            inv[col][j] /= pivot.clone();
        }
        for row in 0..n {
            if row == col {
                continue;
            }
            let factor = work[row][col].clone();
            if is_zero(&factor) {
                continue;
            }
            let pivot_row = work[col].clone();
            let inv_pivot_row = inv[col].clone();
            for j in 0..n {
                work[row][j] -= factor.clone() * pivot_row[j].clone();
                inv[row][j] -= factor.clone() * inv_pivot_row[j].clone();
            }
        }
    }
    Some(inv)
}

/// Characteristic polynomial `det(xI - matrix)` with ascending coefficients.
pub fn rational_charpoly(matrix: &[Vec<Rational>]) -> Vec<Rational> {
    let n = matrix.len();
    if n == 0 {
        return vec![Rational::ONE];
    }
    debug_assert!(matrix.iter().all(|row| row.len() == n));

    let mut powers = vec![identity_matrix(n)];
    for index in 1..=n {
        powers.push(rational_mat_mul(matrix, &powers[index - 1]));
    }

    let mut coeffs = vec![Rational::ZERO; n + 1];
    coeffs[n] = Rational::ONE;
    for k in 1..=n {
        let pk = matrix_trace(&powers[k]);
        let mut acc = pk;
        for j in 1..k {
            acc += coeffs[n - j].clone() * matrix_trace(&powers[k - j]);
        }
        coeffs[n - k] = -acc / Rational::from(Integer::from(k));
    }
    coeffs
}

fn matrix_trace(matrix: &[Vec<Rational>]) -> Rational {
    (0..matrix.len()).fold(Rational::ZERO, |acc, i| acc + matrix[i][i].clone())
}

fn identity_matrix(n: usize) -> Vec<Vec<Rational>> {
    let mut matrix = vec![vec![Rational::ZERO; n]; n];
    for i in 0..n {
        matrix[i][i] = Rational::ONE;
    }
    matrix
}

/// Scale every coefficient of an ascending polynomial.
pub fn scale_polynomial(coeffs: &[Rational], factor: &Rational) -> Vec<Rational> {
    coeffs.iter().map(|coeff| coeff * factor).collect()
}
